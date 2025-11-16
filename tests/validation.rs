mod helpers;

use braces::{brace_paths, BraceConfig, BraceError};
use helpers::*;

#[test]
fn test_disallow_empty_braces_1_to_2_levels() {
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    assert_braces(vec!["a", "a/b"], "{a/b,a}", &config);
}

#[test]
fn test_disallow_empty_braces_2_to_3_levels() {
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    assert_braces(vec!["a/b", "a/b/c"], "a/{b/c,b}", &config);
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
    assert_braces_default(
        vec!["foo/bar.rs", "foo/bar.rs", "foo/baz.rs"],
        "foo/{bar,baz}.rs",
    );
}

#[test]
fn test_deduplication_disabled() {
    let config = BraceConfig {
        deduplicate_inputs: false,
        ..Default::default()
    };
    assert_braces(
        vec!["foo/bar.rs", "foo/bar.rs", "foo/baz.rs"],
        "foo/{bar,bar,baz}",
        &config,
    );
}

#[test]
fn test_braces_in_input_rejected() {
    let paths = vec!["foo/{bar,baz}.rs"];
    let result = brace_paths(&paths, &BraceConfig::default());
    assert!(matches!(result, Err(BraceError::InvalidBraceInput { .. })));
}

#[test]
fn test_disallow_empty_with_nested() {
    let config = BraceConfig {
        disallow_empty_braces: true,
        ..Default::default()
    };
    assert_braces(vec!["a/b", "a/b/c"], "a/{b/c,b}", &config);
}
