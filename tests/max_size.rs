use braces::{brace_paths, BraceConfig};

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
