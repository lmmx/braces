use braces::{brace_paths, BraceConfig};

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
