use braces::{brace_paths, BraceConfig};

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
fn test_custom_separator() {
    let config = BraceConfig {
        path_separator: "::".to_string(),
        ..Default::default()
    };
    let paths = vec!["foo::bar", "foo::baz"];
    let result = brace_paths(&paths, &config).unwrap();
    assert_eq!(result, "foo::{bar,baz}");
}
