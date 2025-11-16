use crate::error::{BraceError, Result};
use std::collections::HashSet;

pub(crate) mod expansion;
mod normalise;
mod trie;

use expansion::{compute_reprs, expand_braces};
use normalise::{find_common_suffix, normalise_separators, validate_separators};
use trie::build_trie;

/// Configuration for brace expansion
///
/// Controls how paths are compressed into brace notation. For example,
/// `["a/b.rs", "a/c.rs"]` can become `"a/{b,c}.rs"` depending on settings.
#[derive(Debug, Clone)]
pub struct BraceConfig {
    /// Path separator to use (default: `"/"`).
    ///
    /// This separator is used to split paths into segments and will be the
    /// separator in the output. When `allow_mixed_separators` is false, input
    /// paths with different separators will cause an error.
    pub path_separator: String,

    /// Maximum nesting depth of braces (default: `5`).
    ///
    /// Limits how deeply braces can be nested to prevent performance issues.
    /// When the limit is reached, remaining items are kept as full subpaths.
    ///
    /// # Examples
    /// With `max_depth = 2`:
    /// - `["a/b/c/1", "a/b/c/2", "a/b/d/3"]` → `"a/b/{c/{1,2},d/3}"`
    ///
    /// With `max_depth = 1`:
    /// - `["a/b/c/1", "a/b/c/2", "a/b/d/3"]` → `"a/b/{c/1,c/2,d/3}"`
    pub max_depth: usize,

    /// Maximum number of items allowed in a single brace group (default: `None`).
    ///
    /// When set, splits large brace groups into multiple groups to prevent
    /// extremely long output. If a brace would exceed this size, it's split
    /// into multiple braces each within the limit.
    ///
    /// # Example
    /// With `max_brace_size = Some(2)`:
    /// - `"a/{b,c,d}"` → `"a/{b,c} a/{d}"`
    pub max_brace_size: Option<usize>,

    /// Allow splitting within filename stems (default: `false`).
    ///
    /// When enabled, common prefixes within path segments (not just at
    /// separator boundaries) can be factored out.
    ///
    /// # Examples
    /// When `true`:
    /// - `["foo/bar.rs", "foo/baz.rs"]` → `"foo/ba{r,z}.rs"`
    ///
    /// When `false`:
    /// - `["foo/bar.rs", "foo/baz.rs"]` → `"foo/{bar,baz}.rs"`
    pub allow_stem_split: bool,

    /// Allow splitting path segments to factor out common prefixes (default: `true`).
    ///
    /// When enabled, allows factoring out segments even when one path is a
    /// prefix of another, creating braces with empty alternatives.
    ///
    /// # Examples
    /// When `true`:
    /// - `["a/b", "a/b/c"]` → `"a/b{,/c}"`
    ///
    /// When `false`:
    /// - `["a/b", "a/b/c"]` → `"a/{b,b/c}"`
    pub allow_segment_split: bool,

    /// Sort items within brace groups alphabetically (default: `false`).
    ///
    /// When enabled, items within each brace group are sorted. When disabled,
    /// items appear in their order from the input (unless
    /// `preserve_order_within_braces` is also false).
    ///
    /// # Examples
    /// When `true`:
    /// - `["z.rs", "b.rs"]` → `"{b,z}.rs"`
    ///
    /// When `false` (with `preserve_order_within_braces = true`):
    /// - `["z.rs", "b.rs"]` → `"{z,b}.rs"`
    pub sort_items: bool,

    /// Disallow braces with empty alternatives (default: `false`).
    ///
    /// When enabled, prevents output like `"a/b{,/c}"` by outputting paths
    /// separately instead.
    ///
    /// # Examples
    /// When `true`:
    /// - `["a/b", "a/b/c"]` → `"a/b a/b/c"` (space-separated)
    ///
    /// When `false`:
    /// - `["a/b", "a/b/c"]` → `"a/b{,/c}"`
    pub disallow_empty_braces: bool,

    /// Preserve input order within braces when not sorting (default: `false`).
    ///
    /// When `false` and `sort_items` is also `false`, items may still be
    /// reordered for consistency. When `true`, the exact input order is maintained.
    pub preserve_order_within_braces: bool,

    /// Allow and normalize mixed path separators in input (default: `false`).
    ///
    /// When `false`, input paths with separators different from `path_separator`
    /// will cause an error. When `true`, all separators are normalized to
    /// `path_separator`.
    pub allow_mixed_separators: bool,

    /// Remove duplicate paths from input before processing (default: `true`).
    ///
    /// When enabled, duplicate paths are removed. When disabled, duplicates
    /// are preserved in the output.
    pub deduplicate_inputs: bool,

    /// Expand existing braces in input before reprocessing (default: `false`).
    ///
    /// When `false`, input containing brace syntax will cause an error (prevents
    /// injection attacks). When `true`, existing braces are expanded and then
    /// re-compressed according to the current configuration.
    ///
    /// # Example
    /// When `true`:
    /// - Input: `"a/{b,c}.rs"` → Expanded to `["a/b.rs", "a/c.rs"]` → Reprocessed
    pub reprocess_braces: bool,
}

impl Default for BraceConfig {
    fn default() -> Self {
        Self {
            path_separator: "/".to_string(),
            max_depth: 5,
            max_brace_size: None,
            allow_stem_split: false,
            allow_segment_split: true,
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
        paths = paths
            .into_iter()
            .map(|p| normalise_separators(&p, &config.path_separator))
            .collect();
    }

    // Handle braces in input if reprocess disabled
    if !config.reprocess_braces && paths.iter().any(|p| p.contains('{') || p.contains('}')) {
        return Err(BraceError::InvalidBraceInput {
            path: paths
                .iter()
                .find(|p| p.contains('{') || p.contains('}'))
                .unwrap()
                .clone(),
            reason: "reprocess_braces is disabled".to_string(),
        });
    }

    if config.reprocess_braces {
        paths = paths.into_iter().flat_map(|p| expand_braces(&p)).collect();
    }

    // Deduplicate while preserving order (only if enabled)
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
        paths
            .iter()
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
