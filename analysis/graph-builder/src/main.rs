mod graph;
mod c_parser;

use std::collections::{BTreeMap, BTreeSet};
use crate::c_parser::Program;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum GlobalId {
    Func { name: String, file: String },
}

impl std::fmt::Display for GlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Kompakte Anzeige: "pfad/zu/datei.c:funktionsname"
            GlobalId::Func { name, file } => write!(f, "{}:{}", file, name),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let project_path_raw = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    let project_root = std::path::Path::new(project_path_raw);

    let mut c_files = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |ext| ext == "c") {
            c_files.push(entry.path().to_path_buf());
        }
    }

    // HIER DIE ÄNDERUNG: Zwei Argumente übergeben
    let program = Program::from_files(c_files, project_root);
    let mut combined_adj: BTreeMap<GlobalId, BTreeSet<GlobalId>> = BTreeMap::new();
    let funcs = program.functions();

    // 1. Initialisiere alle Knoten
    for (name, f) in &funcs {
        let id = GlobalId::Func { name: f.name.clone(), file: f.file.clone() };
        combined_adj.insert(id, BTreeSet::new());
    }

    // 2. Kanten (Abhängigkeiten) hinzufügen
    for ((name, file), f) in &funcs {
        let id = GlobalId::Func { name: name.clone(), file: file.clone() };
        let mut deps = BTreeSet::new();
        
        for callee in &f.callees {
            // Wir suchen, ob der Callee irgendwo im Programm definiert ist
            // Hinweis: Bei mehreren Funktionen mit gleichem Namen nimmt diese 
            // einfache Suche die erste gefundene.
            if let Some(callee_info) = funcs.iter().find(|((n, _), _)| n == callee).map(|(_, info)| info) {
                deps.insert(GlobalId::Func {
                    name: callee_info.name.clone(),
                    file: callee_info.file.clone(),
                });
            }
        }
        combined_adj.insert(id, deps);
    }

    println!("ANALYSIS_START");
    let sccs = graph::compute_sccs(&combined_adj);
    
    for (i, scc) in sccs.iter().enumerate() {
        let names: Vec<String> = scc.iter().map(|id| format!("FUNC:{}", id)).collect();
        if !names.is_empty() {
            println!("STEP {:03}: {}", i + 1, names.join(", "));
        }
    }
    println!("ANALYSIS_END");
}