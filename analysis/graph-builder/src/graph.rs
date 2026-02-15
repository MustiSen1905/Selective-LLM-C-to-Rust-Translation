use std::collections::{BTreeMap, BTreeSet};

pub fn compute_sccs<T: Clone + Ord>(
    adj: &BTreeMap<T, BTreeSet<T>>,
) -> Vec<Vec<T>> {
    let mut index = 0;
    let mut stack = Vec::new();
    let mut on_stack = BTreeSet::new();
    let mut indices = BTreeMap::new();
    let mut lowlink = BTreeMap::new();
    let mut sccs = Vec::new();

    fn strongconnect<T: Clone + Ord>(
        v: &T,
        adj: &BTreeMap<T, BTreeSet<T>>,
        index: &mut usize,
        stack: &mut Vec<T>,
        on_stack: &mut BTreeSet<T>,
        indices: &mut BTreeMap<T, usize>,
        lowlink: &mut BTreeMap<T, usize>,
        sccs: &mut Vec<Vec<T>>,
    ) {
        indices.insert(v.clone(), *index);
        lowlink.insert(v.clone(), *index);
        *index += 1;
        stack.push(v.clone());
        on_stack.insert(v.clone());

        if let Some(successors) = adj.get(v) {
            for w in successors {
                if !indices.contains_key(w) {
                    strongconnect(w, adj, index, stack, on_stack, indices, lowlink, sccs);
                    let val = *lowlink.get(w).unwrap();
                    let v_low = lowlink.get_mut(v).unwrap();
                    *v_low = std::cmp::min(*v_low, val);
                } else if on_stack.contains(w) {
                    let val = *indices.get(w).unwrap();
                    let v_low = lowlink.get_mut(v).unwrap();
                    *v_low = std::cmp::min(*v_low, val);
                }
            }
        }

        if lowlink.get(v) == indices.get(v) {
            let mut scc = Vec::new();
            while let Some(w) = stack.pop() {
                on_stack.remove(&w);
                scc.push(w.clone());
                if &w == v { break; }
            }
            sccs.push(scc);
        }
    }

    for node in adj.keys() {
        if !indices.contains_key(node) {
            strongconnect(node, adj, &mut index, &mut stack, &mut on_stack, &mut indices, &mut lowlink, &mut sccs);
        }
    }
    sccs
}