#![allow(dead_code)]
use braces::{brace_paths, BraceConfig};

/// Test that paths produce expected brace output
pub fn assert_braces(paths: Vec<&str>, expected: &str, config: &BraceConfig) {
    let result = brace_paths(&paths, config).unwrap();
    assert_eq!(result, expected, "Bracing {:?}", paths);
}

/// Test with default config
pub fn assert_braces_default(paths: Vec<&str>, expected: &str) {
    assert_braces(paths, expected, &BraceConfig::default());
}
