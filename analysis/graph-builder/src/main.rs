mod graph;
mod c_parser;

use std::collections::{BTreeMap, BTreeSet};
use crate::c_parser::Program;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum GlobalId {
    Func(String),
}

impl std::fmt::Display for GlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalId::Func(s) => write!(f, "{}", s),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let project_path_raw = args.get(1).map(|s| s.as_str()).unwrap_or(".");
    
    // Wir machen daraus einen echten Pfad für Rust
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

    // 1. Initialisiere ALLE gefundenen Funktionen als Knoten
    for name in funcs.keys() {
        combined_adj.insert(GlobalId::Func(name.clone()), BTreeSet::new());
    }

    // 2. Kanten (Abhängigkeiten) hinzufügen
    for (name, f) in &funcs {
        let id = GlobalId::Func(name.clone());
        let mut deps = BTreeSet::new();
        for callee in &f.callees {
            if program.function_set.contains(callee) {
                deps.insert(GlobalId::Func(callee.clone()));
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