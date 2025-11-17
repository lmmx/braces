mod helpers;

use braces::{brace_paths, BraceConfig};
use helpers::*;

#[test]
fn test_segment_splitting_enabled() {
    assert_braces_default(vec!["a/b", "a/b/c"], "a/b/{c,}");
}

#[test]
fn test_segment_splitting_disabled() {
    let config = BraceConfig {
        allow_segment_split: false,
        ..Default::default()
    };
    assert_braces(vec!["a/b", "a/b/c"], "a/{b,b/c}", &config);
}

#[test]
fn test_no_common_prefix() {
    assert_braces_default(vec!["a.rs", "b.rs"], "{a,b}.rs");
}

#[test]
fn test_preserve_order_within_braces() {
    let config = BraceConfig {
        preserve_order_within_braces: true,
        sort_items: false,
        ..Default::default()
    };
    assert_braces(vec!["z.rs", "b.rs"], "{z,b}.rs", &config);
}

#[test]
fn test_segment_split_with_three_levels() {
    assert_braces_default(vec!["a/b", "a/b/c", "a/b/c/d"], "a/b/{c/{d,},}");
}

#[test]
fn test_no_segment_split_preserves_structure() {
    // When path split is off, we treat whole paths as atoms
    let config = BraceConfig {
        allow_segment_split: false,
        ..Default::default()
    };
    let paths = vec!["abc", "abcd"];
    let result = brace_paths(&paths, &config).unwrap();
    println!("No path split, no separator: {}", result);
    // No common prefix extraction
    assert_eq!(result, "{abc,abcd}");
}
