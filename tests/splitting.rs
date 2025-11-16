use brace::{brace_paths, BraceConfig};

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
fn test_no_common_prefix() {
    let paths = vec!["foo.rs", "bar.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "{bar,foo}.rs");
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
fn test_path_split_with_three_levels() {
    // Pin down: what happens with a/b, a/b/c, a/b/c/d?
    let paths = vec!["a/b", "a/b/c", "a/b/c/d"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    // Expected: a/b{,/c{,/d}} - nested empties
    println!("Three levels result: {}", result);
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
