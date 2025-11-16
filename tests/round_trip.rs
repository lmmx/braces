use braces::{brace_paths, BraceConfig};

fn round_trip(paths: Vec<&str>) {
    let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
    let expanded = expand_braces(&result);

    let mut expanded_sorted = expanded;
    expanded_sorted.sort();

    let mut paths_sorted = paths.clone();
    paths_sorted.sort();

    assert_eq!(
        expanded_sorted, paths_sorted,
        "Expansion of '{}' should match original paths",
        result
    );
}

#[test]
fn test_round_trip_simple() {
    round_trip(vec!["a/", "a/foo"]);
}

#[test]
fn test_round_trip_directory_files_common_suffix() {
    round_trip(vec!["a/1.zip", "a/2.zip", "a/3.zip"]);
}

#[test]
fn test_round_trip_mixed_file_directory() {
    round_trip(vec!["foo.rs", "foo/", "foo/submod.rs"]);
}
