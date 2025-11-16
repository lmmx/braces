use braces::{brace_paths, BraceConfig};

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
fn test_single_path() {
    let paths = vec!["foo/bar.rs"];
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    assert_eq!(result, "foo/bar.rs");
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
