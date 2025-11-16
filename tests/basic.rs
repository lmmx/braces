mod helpers;

use braces::BraceConfig;
use helpers::*;

#[test]
fn test_basic_bracing() {
    assert_braces_default(vec!["a/b.rs", "a/c.rs"], "a/{b,c}.rs");
}

#[test]
fn test_trailing_slash_preserved() {
    assert_braces_default(vec!["a/", "a/b"], "a/{,b}");
}

#[test]
fn test_stem_splitting() {
    let config = BraceConfig {
        allow_stem_split: true,
        ..Default::default()
    };
    assert_braces(vec!["foo/bar.rs", "foo/baz.rs"], "foo/ba{r,z}.rs", &config);
}

#[test]
fn test_order_of_appearance() {
    assert_braces_default(vec!["z.rs", "b.rs"], "{z,b}.rs");
}

#[test]
fn test_sorted_order() {
    let config = BraceConfig {
        sort_items: true,
        ..Default::default()
    };
    assert_braces(vec!["z.rs", "b.rs"], "{b,z}.rs", &config);
}

#[test]
fn test_single_path() {
    assert_braces_default(vec!["foo/bar.rs"], "foo/bar.rs");
}

#[test]
fn test_complex_paths() {
    // Should group by x/ and y/
    // assert!(result.contains("x/") && result.contains("y/"));
    assert_braces_default(
        vec!["x/a.rs", "x/b.rs", "x/c.rs", "y/a.rs"],
        "{x/{a,b,c},y/a}.rs",
    );
}
