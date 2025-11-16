use crate::error::{BraceError, Result};
use std::collections::HashSet;

mod expansion;
mod trie;
mod normalise;

use expansion::{compute_reprs, expand_braces};
use normalise::{find_common_suffix, normalise_separators, validate_separators};
use trie::build_trie;

/// Configuration for brace expansion
#[derive(Debug, Clone)]
pub struct BraceConfig {
    pub path_separator: String,
    pub max_depth: usize,
    pub max_brace_size: Option<usize>,
    pub allow_stem_split: bool,
    pub allow_path_split: bool,
    pub sort_items: bool,
    pub disallow_empty_braces: bool,
    pub preserve_order_within_braces: bool,
    pub allow_mixed_separators: bool,
    pub deduplicate_inputs: bool,
    pub reprocess_braces: bool,
}

impl Default for BraceConfig {
    fn default() -> Self {
        Self {
            path_separator: "/".to_string(),
            max_depth: 5,
            max_brace_size: None,
            allow_stem_split: false,
            allow_path_split: true,
            sort_items: false,
            disallow_empty_braces: false,
            preserve_order_within_braces: false,
            allow_mixed_separators: false,
            deduplicate_inputs: true,
            reprocess_braces: false,
        }
    }
}

/// Public entry: expand paths into braces
pub fn brace_paths(paths: &[impl AsRef<str>], config: &BraceConfig) -> Result<String> {
    if paths.is_empty() {
        return Err(BraceError::EmptyInput);
    }

    // Convert to owned strings
    let mut paths: Vec<String> = paths.iter().map(|p| p.as_ref().to_string()).collect();

    // Normalize separators
    if !config.allow_mixed_separators {
        validate_separators(&paths, &config.path_separator)?;
    } else {
        paths = paths.into_iter()
            .map(|p| normalise_separators(&p, &config.path_separator))
            .collect();
    }

    // Handle braces in input if reprocess disabled
    if !config.reprocess_braces && paths.iter().any(|p| p.contains('{') || p.contains('}')) {
        return Err(BraceError::InvalidBraceInput {
            path: paths.iter().find(|p| p.contains('{') || p.contains('}')).unwrap().clone(),
            reason: "reprocess_braces is disabled".to_string(),
        });
    }

    if config.reprocess_braces {
        paths = paths.into_iter().flat_map(|p| expand_braces(&p)).collect();
    }

    // Deduplicate while preserving order
    if config.deduplicate_inputs {
        let mut seen = HashSet::new();
        let mut ordered = Vec::with_capacity(paths.len());
        for p in paths.into_iter() {
            if seen.insert(p.clone()) {
                ordered.push(p);
            }
        }
        paths = ordered;
    }

    // Strip common suffix for cleaner braces
    let common_suffix = find_common_suffix(&paths);
    let stripped_paths: Vec<String> = if !common_suffix.is_empty() {
        paths.iter()
            .map(|s| s.strip_suffix(&common_suffix).unwrap_or(s).to_string())
            .collect()
    } else {
        paths.clone()
    };

    // Build trie
    let (nodes, root_idx) = build_trie(&stripped_paths, &config.path_separator, config);

    // Compute representations
    let (reprs, _) = compute_reprs(&nodes, root_idx, &config.path_separator, config);

    let mut result = reprs.get(&root_idx).cloned().unwrap_or_default();
    if !common_suffix.is_empty() {
        result.push_str(&common_suffix);
    }

    Ok(result)
}
