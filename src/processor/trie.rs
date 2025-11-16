use crate::BraceConfig;
use std::collections::BTreeMap;

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
    let mut nodes = vec![Node {
        label: String::new(),
        children: BTreeMap::new(),
        is_leaf: false,
        depth: 0,
    }];

    for path in paths {
        let comps: Vec<String> = if config.allow_segment_split && !sep.is_empty() {
            path.split(sep).map(|s| s.to_string()).collect()
        } else {
            // When segment split is disabled, still extract common prefix
            let cur_path = path.as_str();
            let mut components = Vec::new();

            // Find common prefix with existing paths at root
            if !sep.is_empty() && cur_path.contains(sep) {
                let parts: Vec<&str> = cur_path.split(sep).collect();
                if parts.len() > 1 {
                    // Take first component as potential common prefix
                    components.push(parts[0].to_string());
                    components.push(parts[1..].join(sep));
                } else {
                    components.push(cur_path.to_string());
                }
            } else {
                components.push(cur_path.to_string());
            }
            components
        };

        let mut cur = 0;
        for (i, comp) in comps.iter().enumerate() {
            let child_idx = if let Some(&idx) = nodes[cur].children.get(comp) {
                idx
            } else {
                let idx = nodes.len();
                nodes[cur].children.insert(comp.clone(), idx);
                nodes.push(Node {
                    label: comp.clone(),
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
