#![cfg(feature = "highlight")]

use braces::{brace_paths, pretty_braces, BraceConfig};

#[test]
fn config_highlight_applies_to_output() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar", "foo/baz"], &config).unwrap();

    insta::assert_snapshot!(result, @"foo/\u{1b}[36m{\u{1b}[0m\u{1b}[36mbar\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mbaz\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
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

    insta::assert_snapshot!(result, @"a/b/\u{1b}[36m{\u{1b}[0m\u{1b}[36mc/\u{1b}[0m\u{1b}[33m{\u{1b}[0m\u{1b}[33m1\u{1b}[0m\u{1b}[33m,\u{1b}[0m\u{1b}[33m2\u{1b}[0m\u{1b}[33m}\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36md/3\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn config_highlight_with_pretty() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar/1", "foo/bar/2", "foo/baz/3"], &config).unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty, @r###"
    foo/\u{1b}[36m{\u{1b}[0m
      \u{1b}[36mbar/\u{1b}[0m\u{1b}[33m{\u{1b}[0m
        \u{1b}[33m1\u{1b}[0m\u{1b}[33m,\u{1b}[0m
        \u{1b}[33m2\u{1b}[0m
      \u{1b}[33m}\u{1b}[0m\u{1b}[36m,\u{1b}[0m
      \u{1b}[36mbaz/3\u{1b}[0m
    \u{1b}[36m}\u{1b}[0m
    "###);
}

#[test]
fn config_highlight_pretty_deep_nesting() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["a/b/c/d/1", "a/b/c/d/2", "a/b/c/e/3", "a/b/f/4"], &config).unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty, @r###"
    a/b/\u{1b}[36m{\u{1b}[0m
      \u{1b}[36mc/\u{1b}[0m\u{1b}[33m{\u{1b}[0m
        \u{1b}[33md/\u{1b}[0m\u{1b}[35m{\u{1b}[0m
          \u{1b}[35m1\u{1b}[0m\u{1b}[35m,\u{1b}[0m
          \u{1b}[35m2\u{1b}[0m
        \u{1b}[35m}\u{1b}[0m\u{1b}[33m,\u{1b}[0m
        \u{1b}[33me/3\u{1b}[0m
      \u{1b}[33m}\u{1b}[0m\u{1b}[36m,\u{1b}[0m
      \u{1b}[36mf/4\u{1b}[0m
    \u{1b}[36m}\u{1b}[0m
    "###);
}

#[test]
fn config_no_highlight_with_pretty() {
    let config = BraceConfig {
        highlight: false,
        ..Default::default()
    };
    let result = brace_paths(&["foo/bar/1", "foo/bar/2", "foo/baz/3"], &config).unwrap();
    let pretty = pretty_braces(&result);

    insta::assert_snapshot!(pretty, @r###"
    foo/{
      bar/{
        1,
        2
      },
      baz/3
    }
    "###);
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

    insta::assert_snapshot!(result, @"src/\u{1b}[36m{\u{1b}[0m\u{1b}[36mlib\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mmain\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[33m{\u{1b}[0m\u{1b}[33mutil/\u{1b}[0m\u{1b}[35m{\u{1b}[0m\u{1b}[35mmod\u{1b}[0m\u{1b}[35m,\u{1b}[0m\u{1b}[35mhelpers\u{1b}[0m\u{1b}[35m}\u{1b}[0m\u{1b}[33m,\u{1b}[0m\u{1b}[33mtests/\u{1b}[0m\u{1b}[35m{\u{1b}[0m\u{1b}[35munit\u{1b}[0m\u{1b}[35m,\u{1b}[0m\u{1b}[35mintegration\u{1b}[0m\u{1b}[35m}\u{1b}[0m\u{1b}[33m}\u{1b}[0m\u{1b}[36m}\u{1b}[0m.rs");
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

    insta::assert_snapshot!(pretty, @r###"
    src/\u{1b}[36m{\u{1b}[0m
      \u{1b}[36mlib\u{1b}[0m\u{1b}[36m,\u{1b}[0m
      \u{1b}[36mmain\u{1b}[0m\u{1b}[36m,\u{1b}[0m
      \u{1b}[33m{\u{1b}[0m
        \u{1b}[33mutil/\u{1b}[0m\u{1b}[35m{\u{1b}[0m
          \u{1b}[35mmod\u{1b}[0m\u{1b}[35m,\u{1b}[0m
          \u{1b}[35mhelpers\u{1b}[0m
        \u{1b}[35m}\u{1b}[0m\u{1b}[33m,\u{1b}[0m
        \u{1b}[33mtests/\u{1b}[0m\u{1b}[35m{\u{1b}[0m
          \u{1b}[35munit\u{1b}[0m\u{1b}[35m,\u{1b}[0m
          \u{1b}[35mintegration\u{1b}[0m
        \u{1b}[35m}\u{1b}[0m
      \u{1b}[33m}\u{1b}[0m
    \u{1b}[36m}\u{1b}[0m.rs
    "###);
}

#[test]
fn config_highlight_with_sorting() {
    let config = BraceConfig {
        highlight: true,
        sort_items: true,
        ..Default::default()
    };
    let result = brace_paths(&["z/c", "z/a", "z/b"], &config).unwrap();

    insta::assert_snapshot!(result, @"z/\u{1b}[36m{\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mb\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mc\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn config_highlight_with_empty_braces() {
    let config = BraceConfig {
        highlight: true,
        ..Default::default()
    };
    let result = brace_paths(&["a/b", "a/b/c"], &config).unwrap();

    insta::assert_snapshot!(result, @"a/b\u{1b}[36m{\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36m/c\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}
