use braces::{brace_paths, BraceConfig};

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
fn test_deeply_nested_paths() {
    let paths = vec!["a/b/c/d/e/1", "a/b/c/d/e/2", "a/b/c/d/f/3"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert!(result.contains("{1,2}"));
}

#[test]
fn test_mixed_depth_paths() {
    let paths = vec!["a/b", "a/b/c", "a/b/c/d"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    // Should handle paths at different depths
    assert!(result.contains("a/b"));
}
