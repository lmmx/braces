mod ordered_map;

use crate::BraceConfig;
use ordered_map::OrderedMap;

/// Trie node: uses IndexMap-like OrderedMap whose key is (label, ID)
///
/// The ID allows us to treat duplicates as distinct nodes if not deduplicating.
#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub children: OrderedMap<(String, usize), usize>,
    pub is_leaf: bool,
    pub is_trailing_sep: bool,
    pub depth: usize,
}

/// Build trie of paths
pub fn build_trie(paths: &[String], sep: &str, config: &BraceConfig) -> (Vec<Node>, usize) {
    let mut nodes = vec![Node {
        label: String::new(),
        children: OrderedMap::new(),
        is_leaf: false,
        is_trailing_sep: false,
        depth: 0,
    }];

    let mut next_id = 0;

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
            let is_last = i + 1 == comps.len();

            // Only add unique ID if not deduplicating AND this is the last component
            let key = if !config.deduplicate_inputs && is_last {
                let id = next_id;
                next_id += 1;
                (comp.clone(), id)
            } else {
                (comp.clone(), 0) // Use 0 as a dummy ID for non-leaf nodes
            };

            let child_idx = if let Some(&idx) = nodes[cur].children.get(&key) {
                idx
            } else {
                let idx = nodes.len();
                nodes[cur].children.insert(key, idx);
                nodes.push(Node {
                    label: comp.clone(),
                    children: OrderedMap::new(),
                    is_leaf: false,
                    is_trailing_sep: false,
                    depth: nodes[cur].depth + 1,
                });
                idx
            };
            cur = child_idx;
            if is_last {
                nodes[cur].is_leaf = true;
                // Mark as trailing separator if the component is empty AND it's the last one
                nodes[cur].is_trailing_sep = comp.is_empty() && path.ends_with(sep);
            }
        }
    }

    (nodes, 0)
}
