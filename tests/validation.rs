use braces::{brace_paths, BraceConfig, BraceError};

#[test]
fn test_disallow_empty_braces() {
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    let paths = vec!["a/b", "a/b/c"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "a/{b/c,b}");
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
    assert_eq!(result, "{a/b,a}");
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
