#![cfg(feature = "highlight")]

use braces::{brace_paths, pretty_braces, BraceConfig};

#[test]
fn config_highlight_applies_to_output() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar", "foo/baz"], &config).unwrap();

    insta::assert_snapshot!(result);
}

#[test]
fn config_highlight_false_no_ansi() {
    let config = BraceConfig {
        highlight: false,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar", "foo/baz"], &config).unwrap();

    insta::assert_snapshot!(result, @"foo/{bar,baz}");
}

#[test]
fn config_highlight_with_nesting() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["a/b/c/1", "a/b/c/2", "a/b/d/3"], &config).unwrap();

    insta::assert_snapshot!(result);
}

#[test]
fn config_highlight_with_pretty() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar/1", "foo/bar/2", "foo/baz/3"], &config).unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty);
}

#[test]
fn config_highlight_pretty_deep_nesting() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["a/b/c/d/1", "a/b/c/d/2", "a/b/c/e/3", "a/b/f/4"], &config).unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty);
}

#[test]
fn config_no_highlight_with_pretty() {
    let config = BraceConfig {
        highlight: false,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar/1", "foo/bar/2", "foo/baz/3"], &config).unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty, @r"
    foo/{
         bar/{
              1,
              2
             }
         ,
         baz/3
        }
    ");
}

#[test]
fn config_highlight_complex_real_world() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(
        &[
            "src/lib.rs",
            "src/main.rs",
            "src/util/mod.rs",
            "src/util/helpers.rs",
            "src/tests/unit.rs",
            "src/tests/integration.rs",
        ],
        &config,
    )
    .unwrap();

    insta::assert_snapshot!(result);
}

#[test]
fn config_highlight_pretty_complex_real_world() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(
        &[
            "src/lib.rs",
            "src/main.rs",
            "src/util/mod.rs",
            "src/util/helpers.rs",
            "src/tests/unit.rs",
            "src/tests/integration.rs",
        ],
        &config,
    )
    .unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty);
}

#[test]
fn config_highlight_with_sorting() {
    let config = BraceConfig {
        highlight: true,
        sort_items: true,
        ..Default::default()
    };
    let result = brace_paths(&["z/c", "z/a", "z/b"], &config).unwrap();

    insta::assert_snapshot!(result);
}

#[test]
fn config_highlight_with_empty_braces() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["a/b", "a/b/c"], &config).unwrap();

    insta::assert_snapshot!(result);
}
