use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    io::Write,
};
use lang_c::{
    ast::*,
    driver::{self, Config, Parse},
    span::Span,
    visit::{self, Visit},
};

// Diese Definitionen neutralisieren GCC-Erweiterungen direkt beim Präprozessor-Aufruf
const GCC_SHIM: &str = r#"
#define __builtin_va_list void*
#define __builtin_va_start(ap, last)
#define __builtin_va_end(ap)
#define __builtin_va_arg(ap, type) ((type)0)
#define __builtin_expect(exp, c) (exp)
#define __attribute__(x)
#define __extension__
#define __inline inline
#define __restrict restrict
#define __asm__(x)
typedef int size_t;
typedef void* FILE;
typedef void* va_list;
"#;

pub struct Function {
    pub name: String,
    pub file: String, // NEU: Dateipfad hinzugefügt
    pub callees: Vec<String>,
}

pub struct Program {
    pub parses: BTreeMap<String, Parse>,
    pub function_set: BTreeSet<String>,
}

pub struct Preprocessed {
    pub path: PathBuf,
    pub code: String,
}

impl Program {
    pub fn from_files(files: Vec<PathBuf>, project_root: &Path) -> Self {
        let mut preprocessed_files = Vec::new();
        
        // 1. Alle Unterordner für automatische Includes finden
        let mut include_args = Vec::new();
        if let Ok(entries) = walkdir::WalkDir::new(project_root).into_iter().collect::<Result<Vec<_>, _>>() {
            for entry in entries {
                if entry.file_type().is_dir() {
                    include_args.push(format!("-I{}", entry.path().display()));
                }
            }
        }

        // 2. Erstelle eine temporäre Shim-Datei, um Pfadprobleme zu vermeiden
        let shim_path = std::env::temp_dir().join("gcc_compat_shim.h");
        let mut f = std::fs::File::create(&shim_path).expect("Konnte temporäre Shim-Datei nicht erstellen");
        f.write_all(GCC_SHIM.as_bytes()).unwrap();

        for file_path in files {
            let filename = file_path.file_name().unwrap().to_str().unwrap();
            
            let output = std::process::Command::new("gcc")
                .arg("-E")
                .arg("-P")
                .args(&include_args)
                .arg("-include")
                .arg(&shim_path) // Nutze den absoluten Pfad zur temporären Datei
                .arg(&file_path)
                .output()
                .expect("gcc -E Aufruf fehlgeschlagen");

            if !output.status.success() {
                eprintln!("\x1b[91m[GCC FEHLER]\x1b[0m {}: {}", filename, String::from_utf8_lossy(&output.stderr));
                continue;
            }

            let code = String::from_utf8_lossy(&output.stdout).to_string();
            if code.trim().is_empty() {
                eprintln!("\x1b[93m[WARNUNG]\x1b[0m {}: Präprozessor lieferte leeren Code.", filename);
            }

            preprocessed_files.push(Preprocessed { path: file_path, code });
        }
        Self::new(preprocessed_files)
    }

    fn new(files: Vec<Preprocessed>) -> Self {
        let mut parses = BTreeMap::new();
        let mut function_set = BTreeSet::new();
        let config = Config::with_gcc();

        eprintln!("\n\x1b[93m--- PARSER DIAGNOSE ---\x1b[0m");

        for file in files {
            let path_str = file.path.to_str().unwrap().to_string();
            let filename = file.path.file_name().unwrap().to_str().unwrap();

            match driver::parse_preprocessed(&config, file.code) {
                Ok(parse) => {
                    let mut count = 0;
                    // Wir nutzen den Visitor für maximale Gründlichkeit
                    struct FuncFinder<'a> { set: &'a mut BTreeSet<String>, count: &'a mut usize }
                    impl<'ast, 'a> Visit<'ast> for FuncFinder<'a> {
                        fn visit_function_definition(&mut self, f: &'ast FunctionDefinition, _: &'ast Span) {
                            let name = get_declarator_name(&f.declarator.node);
                            if name != "unknown" {
                                self.set.insert(name.to_string());
                                *self.count += 1;
                            }
                        }
                    }
                    
                    let mut finder = FuncFinder { set: &mut function_set, count: &mut count };
                    visit::visit_translation_unit(&mut finder, &parse.unit);

                    eprintln!("\x1b[92m[OK]\x1b[0m {}: {} Funktionen gefunden", filename, count);
                    parses.insert(path_str, parse);
                }
                Err(e) => {
                    eprintln!("\x1b[91m[FEHLER]\x1b[0m {}: {:?}", filename, e);
                }
            }
        }
        eprintln!("\x1b[93m------------------------\x1b[0m\n");

        Self { parses, function_set }
    }

    pub fn functions(&self) -> BTreeMap<(String, String), Function> {
        let mut map = BTreeMap::new();
        for (path_str, parse) in &self.parses {
            struct Extractor<'a> { 
                map: &'a mut BTreeMap<(String, String), Function>, 
                all_funcs: &'a BTreeSet<String>,
                current_file: String 
            }
            impl<'ast, 'a> Visit<'ast> for Extractor<'a> {
                fn visit_function_definition(&mut self, f: &'ast FunctionDefinition, _: &'ast Span) {
                    let name = get_declarator_name(&f.declarator.node).to_string();
                    if name != "unknown" {
                        let file = self.current_file.clone();
                        self.map.insert((name.clone(), file.clone()), Function {
                            name: name.clone(),
                            file,
                            callees: get_callees(f, self.all_funcs),
                        });
                    }
                }
            }
            let mut ext = Extractor { 
                map: &mut map, 
                all_funcs: &self.function_set,
                current_file: path_str.clone() 
            };
            visit::visit_translation_unit(&mut ext, &parse.unit);
        }
        map
    }

    pub fn typedefs(&self) -> BTreeMap<String, String> { BTreeMap::new() }
    pub fn structs(&self) -> BTreeMap<String, String> { BTreeMap::new() }
}

fn get_declarator_name(d: &Declarator) -> &str {
    match &d.kind.node {
        DeclaratorKind::Identifier(i) => i.node.name.as_str(),
        DeclaratorKind::Declarator(inner) => get_declarator_name(&inner.node),
        _ => "unknown",
    }
}

fn get_callees(f: &FunctionDefinition, all_funcs: &BTreeSet<String>) -> Vec<String> {
    struct CalleeVisitor<'a>(Vec<String>, &'a BTreeSet<String>);
    impl<'ast, 'a> Visit<'ast> for CalleeVisitor<'a> {
        fn visit_call_expression(&mut self, e: &'ast CallExpression, _: &'ast Span) {
            if let Expression::Identifier(i) = &e.callee.node {
                let name = i.node.name.clone();
                if self.1.contains(&name) { self.0.push(name); }
            }
            visit::visit_call_expression(self, e, &e.callee.span);
        }
    }
    let mut v = CalleeVisitor(vec![], all_funcs);
    visit::visit_function_definition(&mut v, f, &f.declarator.span);
    v.0
}