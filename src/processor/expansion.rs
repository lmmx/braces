use super::normalise::{can_stem_split, find_common_string_prefix, find_common_string_suffix};
use super::trie::Node;
use super::BraceConfig;
use std::collections::{HashMap, HashSet};

/// Compute brace representations from trie
pub fn compute_reprs(
    nodes: &[Node],
    root_idx: usize,
    sep: &str,
    config: &BraceConfig,
) -> (
    std::collections::HashMap<usize, String>,
    std::collections::HashMap<usize, Vec<String>>,
) {
    let mut stack = vec![root_idx];
    let mut visited = HashSet::new();
    let mut post = vec![];

    while let Some(idx) = stack.pop() {
        if visited.contains(&idx) {
            post.push(idx);
            continue;
        }
        visited.insert(idx);
        stack.push(idx);
        for &child in nodes[idx].children.values().rev() {
            if !visited.contains(&child) {
                stack.push(child);
            }
        }
    }

    let mut reprs = std::collections::HashMap::new();
    let mut raw_leaves: HashMap<usize, Vec<String>> = HashMap::new();

    for &idx in &post {
        let node = &nodes[idx];

        let mut child_repr_items = vec![];
        let mut child_raws = vec![];

        for (child_label, child_idx) in node.children.iter() {
            let label_str = &child_label.0;

            // Get the child's representation if it exists, otherwise use the label
            child_repr_items.push(
                reprs
                    .get(child_idx)
                    .cloned()
                    .unwrap_or_else(|| label_str.clone()),
            );

            if let Some(r) = raw_leaves.get(child_idx) {
                child_raws.extend(r.clone());
            } else {
                child_raws.push(label_str.clone());
            }
        }

        // Compute raw leaves for this node
        let node_raws: Vec<String> = if node.children.is_empty() {
            if node.label.is_empty() {
                vec![String::new()]
            } else {
                vec![node.label.clone()]
            }
        } else {
            let mut out = vec![];
            for r in child_raws.iter() {
                if node.label.is_empty() {
                    out.push(r.clone())
                } else if r.is_empty() {
                    out.push(node.label.clone())
                } else {
                    out.push(format!("{}{}{}", node.label, sep, r));
                }
            }
            // Don't add node.label again if we already have a trailing sep child
            let has_trailing_sep_child = node
                .children
                .iter()
                .any(|(_, child_idx)| nodes[*child_idx].is_trailing_sep);
            if node.is_leaf && !has_trailing_sep_child {
                out.push(node.label.clone())
            }
            out
        };
        raw_leaves.insert(idx, node_raws.clone());

        // Compose final representation
        let repr = if node.depth > config.max_depth {
            // depth limit: use raw leaves
            let suffixes: Vec<String> = node_raws
                .iter()
                .map(|s| {
                    if node.label.is_empty() {
                        s.clone()
                    } else if s.starts_with(&format!("{}{}", node.label, sep)) {
                        s[node.label.len() + sep.len()..].to_string()
                    } else if s == &node.label {
                        String::new()
                    } else {
                        s.clone()
                    }
                })
                .collect();

            if config.disallow_empty_braces
                && suffixes.iter().any(|s| s.is_empty())
                && suffixes.len() > 1
            {
                format!("{{{}}}", node_raws.join(","))
            } else {
                compose_label_and_items(
                    &node.label,
                    sep,
                    &suffixes,
                    config.max_brace_size,
                    config.sort_items,
                )
            }
        } else {
            let mut items = child_repr_items.clone();

            // Only add empty string if this node is a leaf AND doesn't have a trailing sep child
            let has_trailing_sep_child = node
                .children
                .iter()
                .any(|(_, child_idx)| nodes[*child_idx].is_trailing_sep);

            // Add empty string for leaf nodes that don't have trailing sep children
            // This represents the case where path ends at this node (e.g., "a" in ["a", "a/b"])
            if node.is_leaf && !node.is_trailing_sep && !has_trailing_sep_child {
                items.push(String::new())
            }

            // For nodes with trailing sep children, the empty string is already in items
            // from the child processing above

            if config.disallow_empty_braces && items.iter().any(|s| s.is_empty()) && items.len() > 1
            {
                format!("{{{}}}", node_raws.join(","))
            } else if config.allow_stem_split && can_stem_split(&items) {
                let prefix = find_common_string_prefix(&items);
                let suffix = find_common_string_suffix(&items);
                let mut vars = items
                    .iter()
                    .map(|s| {
                        s.strip_prefix(&prefix)
                            .unwrap_or(s)
                            .strip_suffix(&suffix)
                            .unwrap_or(s)
                            .to_string()
                    })
                    .collect::<Vec<_>>();
                if config.sort_items {
                    vars.sort();
                }
                let inner = if vars.len() == 1 {
                    vars[0].clone()
                } else {
                    format!("{{{}}}", vars.join(","))
                };
                if node.label.is_empty() {
                    format!("{}{}{}", prefix, inner, suffix)
                } else if !prefix.is_empty() || !suffix.is_empty() {
                    format!("{}{}{}{}{}", node.label, sep, prefix, inner, suffix)
                } else {
                    format!("{}{}{}", node.label, sep, inner)
                }
            } else {
                compose_label_and_items(
                    &node.label,
                    sep,
                    &items,
                    config.max_brace_size,
                    config.sort_items,
                )
            }
        };

        reprs.insert(idx, repr);
    }

    (reprs, raw_leaves)
}

/// Compose node label + separator + items into string
fn compose_label_and_items(
    label: &str,
    sep: &str,
    items: &[String],
    max_brace_size: Option<usize>,
    sort_items: bool,
) -> String {
    let mut cleaned: Vec<String> = items.to_vec();
    if sort_items {
        cleaned.sort();
    }

    let compose_inner = |slice: &[String]| {
        // Handle empty slice
        if slice.is_empty() {
            String::new()
        } else if slice.len() == 1 {
            // Don't wrap single empty strings in braces
            if slice[0].is_empty() {
                String::new()
            } else {
                slice[0].clone()
            }
        } else {
            format!("{{{}}}", slice.join(","))
        }
    };

    if let Some(max) = max_brace_size {
        if cleaned.len() <= max {
            if label.is_empty() {
                compose_inner(&cleaned)
            } else {
                // Don't add separator if inner is empty
                let inner = compose_inner(&cleaned);
                if inner.is_empty() {
                    label.to_string()
                } else {
                    format!("{}{}{}", label, sep, inner)
                }
            }
        } else {
            let mut groups = vec![];
            for chunk in cleaned.chunks(max) {
                if label.is_empty() {
                    groups.push(compose_inner(chunk))
                } else {
                    // Apply same fix here
                    let inner = compose_inner(chunk);
                    if inner.is_empty() {
                        groups.push(label.to_string())
                    } else {
                        groups.push(format!("{}{}{}", label, sep, inner))
                    }
                }
            }
            format!("{{{}}}", groups.join(","))
        }
    } else if label.is_empty() {
        compose_inner(&cleaned)
    } else {
        // Don't add separator if inner is empty
        let inner = compose_inner(&cleaned);
        if inner.is_empty() {
            label.to_string()
        } else {
            format!("{}{}{}", label, sep, inner)
        }
    }
}

/// Expand braces (a{b,c}d -> abd, acd)
pub fn expand_braces(pattern: &str) -> Vec<String> {
    if !pattern.contains('{') {
        return vec![pattern.to_string()];
    }
    let mut results = vec![String::new()];
    let mut chars = pattern.chars().peekable();
    let mut current = String::new();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut depth = 1;
            let mut inner = String::new();
            for c in chars.by_ref() {
                if c == '{' {
                    depth += 1;
                    inner.push(c);
                } else if c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    inner.push(c);
                } else {
                    inner.push(c);
                }
            }
            let options: Vec<&str> = inner.split(',').collect();
            let mut new_results = vec![];
            for r in &results {
                for opt in &options {
                    let mut tmp = r.clone();
                    tmp.push_str(&current);
                    tmp.push_str(opt);
                    new_results.push(tmp);
                }
            }
            results = new_results;
            current.clear();
        } else {
            current.push(ch);
        }
    }
    for r in &mut results {
        r.push_str(&current);
    }
    results
}
