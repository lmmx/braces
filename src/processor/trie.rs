use std::collections::BTreeMap;
use crate::BraceConfig;

/// Trie node
#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub children: BTreeMap<String, usize>,
    pub is_leaf: bool,
    pub depth: usize,
}

/// Build trie of paths
pub fn build_trie(paths: &[String], sep: &str, config: &BraceConfig) -> (Vec<Node>, usize) {
    let mut nodes = vec![Node { label: String::new(), children: BTreeMap::new(), is_leaf: false, depth: 0 }];

    for path in paths {
        let comps: Vec<&str> = if !config.allow_path_split || sep.is_empty() {
            vec![path.as_str()]
        } else {
            path.split(sep).collect()
        };

        let mut cur = 0;
        for (i, comp) in comps.iter().enumerate() {
            let child_idx = if let Some(&idx) = nodes[cur].children.get(*comp) {
                idx
            } else {
                let idx = nodes.len();
                nodes[cur].children.insert((*comp).to_string(), idx);
                nodes.push(Node {
                    label: (*comp).to_string(),
                    children: BTreeMap::new(),
                    is_leaf: false,
                    depth: nodes[cur].depth + 1,
                });
                idx
            };
            cur = child_idx;
            if i + 1 == comps.len() {
                nodes[cur].is_leaf = true;
            }
        }
    }

    (nodes, 0)
}
