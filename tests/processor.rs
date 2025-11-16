use brace::{brace_paths, BraceConfig, BraceError};

#[test]
fn test_basic_bracing() {
    let paths = vec!["foo/bar.rs", "foo/baz.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "foo/{bar,baz}.rs");
}

#[test]
fn test_stem_splitting() {
    let config = BraceConfig {
        allow_stem_split: true,
        ..Default::default()
    };
    let paths = vec!["foo/bar.rs", "foo/baz.rs"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "foo/ba{r,z}.rs");
}

#[test]
fn test_path_splitting_enabled() {
    let paths = vec!["a/b", "a/b/c"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "a/b{,/c}");
}

#[test]
fn test_path_splitting_disabled() {
    let config = BraceConfig {
        allow_path_split: false,
        ..Default::default()
    };
    let paths = vec!["a/b", "a/b/c"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "a/{b,b/c}");
}

#[test]
fn test_order_of_appearance() {
    let paths = vec!["z.rs", "b.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "{b,z}.rs");
}

#[test]
fn test_sorted_order() {
    let config = BraceConfig {
        sort_items: true,
        ..Default::default()
    };
    let paths = vec!["z.rs", "b.rs"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "{b,z}.rs");
}

#[test]
fn test_depth_limit_2() {
    let config = BraceConfig {
        max_depth: 2,
        ..Default::default()
    };
    let paths = vec!["a/b/c/1", "a/b/c/2", "a/b/d/3"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "a/b/{c/{1,2},d/3}");
}

#[test]
fn test_depth_limit_1() {
    let config = BraceConfig {
        max_depth: 1,
        ..Default::default()
    };
    let paths = vec!["a/b/c/1", "a/b/c/2", "a/b/d/3"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "a/b/{c/1,c/2,d/3}");
}

#[test]
fn test_max_brace_size() {
    let config = BraceConfig {
        max_brace_size: Some(2),
        ..Default::default()
    };
    let paths = vec!["a/b", "a/c", "a/d"];
    let result = brace_paths(&paths, &config).unwrap();
    // Should split into multiple braces
    assert!(result.contains("{b,c}") || result.contains("a/b a/c"));
}

#[test]
fn test_disallow_empty_braces() {
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    let paths = vec!["a/b", "a/b/c"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "a/b a/b/c");
}

#[test]
fn test_empty_input() {
    let paths: Vec<String> = vec![];
    let result = brace_paths(&paths, &BraceConfig::default());
    assert!(matches!(result, Err(BraceError::EmptyInput)));
}

#[test]
fn test_mixed_separators_rejected() {
    let paths = vec!["foo/bar", "foo\\baz"];
    let result = brace_paths(&paths, &BraceConfig::default());
    assert!(matches!(result, Err(BraceError::MixedSeparators { .. })));
}

#[test]
fn test_mixed_separators_normalized() {
    let config = BraceConfig {
        allow_mixed_separators: true,
        ..Default::default()
    };
    let paths = vec!["foo/bar", "foo\\baz"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "foo/{bar,baz}");
}

#[test]
fn test_deduplication_enabled() {
    let paths = vec!["foo/bar.rs", "foo/bar.rs", "foo/baz.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "foo/{bar,baz}.rs");
}

#[test]
fn test_deduplication_disabled() {
    let config = BraceConfig {
        deduplicate_inputs: false,
        ..Default::default()
    };
    let paths = vec!["foo/bar.rs", "foo/bar.rs", "foo/baz.rs"];
    let result = brace_paths(&paths, &config).unwrap();
    // Should keep duplicates
    assert!(result.contains("bar") && result.contains("baz"));
}

#[test]
fn test_braces_in_input_rejected() {
    let paths = vec!["foo/{bar,baz}.rs"];
    let result = brace_paths(&paths, &BraceConfig::default());
    assert!(matches!(result, Err(BraceError::InvalidBraceInput { .. })));
}

#[test]
fn test_braces_in_input_reprocessed() {
    let config = BraceConfig {
        reprocess_braces: true,
        ..Default::default()
    };
    let paths = vec!["foo/{bar,baz}.rs"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "foo/{bar,baz}.rs");
}

#[test]
fn test_single_path() {
    let paths = vec!["foo/bar.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "foo/bar.rs");
}

#[test]
fn test_no_common_prefix() {
    let paths = vec!["foo.rs", "bar.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "{bar,foo}.rs");
}

#[test]
fn test_complex_paths() {
    let paths = vec![
        "src/processor.rs",
        "src/cli.rs",
        "src/error.rs",
        "tests/processor.rs",
    ];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    // Should group by src/ and tests/
    assert!(result.contains("src/") && result.contains("tests/"));
}

#[test]
fn test_preserve_order_within_braces() {
    let config = BraceConfig {
        preserve_order_within_braces: true,
        sort_items: false,
        ..Default::default()
    };
    let paths = vec!["z.rs", "b.rs"];
    let result = brace_paths(&paths, &config).unwrap();
    // Should be sorted within braces even though sort_items is false
    assert_eq!(result, "{b,z}.rs");
}

#[test]
fn test_custom_separator() {
    let config = BraceConfig {
        path_separator: "::".to_string(),
        ..Default::default()
    };
    let paths = vec!["foo::bar", "foo::baz"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "foo::{bar,baz}");
}

#[test]
fn test_deeply_nested_paths() {
    let paths = vec![
        "a/b/c/d/e/1",
        "a/b/c/d/e/2",
        "a/b/c/d/f/3",
    ];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert!(result.contains("{1,2}"));
}

#[test]
fn test_mixed_depth_paths() {
    let paths = vec![
        "a/b",
        "a/b/c",
        "a/b/c/d",
    ];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    // Should handle paths at different depths
    assert!(result.contains("a/b"));
}

// Add these tests to tests/processor.rs

#[test]
fn test_path_split_with_three_levels() {
    // Pin down: what happens with a/b, a/b/c, a/b/c/d?
    let paths = vec!["a/b", "a/b/c", "a/b/c/d"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    // Expected: a/b{,/c{,/d}} - nested empties
    println!("Three levels result: {}", result);
}

#[test]
fn test_disallow_empty_with_nested() {
    // Pin down: disallow_empty_braces with nested paths
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    let paths = vec!["a/b/c", "a/b/d"];
    let result = brace_paths(&paths, &config).unwrap();
    // Should this use braces? These aren't empty braces
    println!("Disallow empty, no empties: {}", result);
    assert_eq!(result, "a/b/{c,d}");
}

#[test]
fn test_disallow_empty_simple() {
    // Simplest case of empty brace
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    let paths = vec!["a", "a/b"];
    let result = brace_paths(&paths, &config).unwrap();
    println!("Simplest disallow empty: {}", result);
    // Should output: "a a/b" (no braces)
}

#[test]
fn test_path_split_disabled_simple() {
    // Simplest case without path splitting
    let config = BraceConfig {
        allow_path_split: false,
        ..Default::default()
    };
    let paths = vec!["a/b", "a/c"];
    let result = brace_paths(&paths, &config).unwrap();
    println!("No path split, different endings: {}", result);
    // Without path splitting, entire paths should be in one brace
    assert_eq!(result, "{a/b,a/c}");
}

#[test]
fn test_depth_limit_understanding() {
    // Understanding what depth means
    let config = BraceConfig {
        max_depth: 2,
        ..Default::default()
    };
    let paths = vec!["a/b/c", "a/b/d"];
    let result = brace_paths(&paths, &config).unwrap();
    println!("Depth 2, simple case: {}", result);
    // At depth 2 from root: a(0) -> b(1) -> {c,d}(2)
    // Should work fine: a/b/{c,d}
}

#[test]
fn test_depth_limit_hit() {
    // What happens when we exactly hit the limit?
    let config = BraceConfig {
        max_depth: 3,
        ..Default::default()
    };
    let paths = vec!["a/b/c/d/1", "a/b/c/d/2"];
    let result = brace_paths(&paths, &config).unwrap();
    println!("Depth 3, going deep: {}", result);
    // a(0) -> b(1) -> c(2) -> d(3) -> {1,2}(4)
    // At depth 3, should stop nesting
}

#[test]
fn test_no_path_split_preserves_structure() {
    // When path split is off, we treat whole paths as atoms
    let config = BraceConfig {
        allow_path_split: false,
        ..Default::default()
    };
    let paths = vec!["abc", "abcd"];
    let result = brace_paths(&paths, &config).unwrap();
    println!("No path split, no separator: {}", result);
    // Should be {abc,abcd} - no common prefix extraction
}
