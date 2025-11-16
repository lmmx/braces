use braces::{brace_paths, expand_braces, BraceConfig};

/// Test that brace expansion round-trips correctly
fn assert_round_trip(paths: Vec<&str>, config: &BraceConfig) {
    let result = brace_paths(&paths, config).unwrap();
    let expanded = expand_braces(&result);

    let mut expanded_sorted = expanded;
    expanded_sorted.sort();

    let mut paths_sorted = paths.clone();
    paths_sorted.sort();

    assert_eq!(
        expanded_sorted, paths_sorted,
        "Expansion (LHS) of '{}' should match original paths (RHS)",
        result
    );
}

/// Test that paths produce expected brace output
fn assert_braces(paths: Vec<&str>, expected: &str, config: &BraceConfig) {
    let result = brace_paths(&paths, config).unwrap();
    assert_eq!(result, expected, "Bracing {:?}", paths);
}

/// Test with default config
fn assert_braces_default(paths: Vec<&str>, expected: &str) {
    assert_braces(paths, expected, &BraceConfig::default());
}

/// Test round-trip with default config
fn assert_round_trip_default(paths: Vec<&str>) {
    assert_round_trip(paths, &BraceConfig::default());
}

// === Round-trip tests ===

#[test]
fn test_round_trip_trailing_slash() {
    assert_round_trip_default(vec!["a/", "a/foo"]);
}

#[test]
fn test_round_trip_directory_files_common_suffix() {
    assert_round_trip_default(vec!["a/1.zip", "a/2.zip", "a/3.zip"]);
}

#[test]
fn test_round_trip_mixed_file_directory() {
    assert_round_trip_default(vec!["foo.rs", "foo/", "foo/submod.rs"]);
}

#[test]
fn test_round_trip_nested_directories() {
    assert_round_trip_default(vec!["a/", "a/b.rs", "a/c/", "a/c/d.rs"]);
}

#[test]
fn test_round_trip_no_common_prefix() {
    assert_round_trip_default(vec!["foo.rs", "bar.rs"]);
}
