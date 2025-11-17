mod helpers;

use braces::BraceConfig;
use helpers::*;

#[test]
fn test_depth_limit_1_with_extra_groupable() {
    let config = BraceConfig {
        max_depth: 1,
        ..Default::default()
    };
    assert_braces(
        vec!["a/b/c/1", "a/b/c/2", "a/b/d/3"],
        "a/b/{c/1,c/2,d/3}",
        &config,
    );
}

#[test]
fn test_depth_limit_2_with_unused_group() {
    let config = BraceConfig {
        max_depth: 2,
        ..Default::default()
    };
    assert_braces(vec!["a/b/c", "a/b/d"], "a/b/{c,d}", &config);
}

#[test]
fn test_depth_limit_2_with_max_group_at_limit() {
    let config = BraceConfig {
        max_depth: 2,
        ..Default::default()
    };
    assert_braces(
        vec!["a/b/c/d", "a/b/c/e", "a/b/d/f"],
        "a/b/{c/{d,e},d/f}",
        &config,
    );
}

#[test]
fn test_depth_limit_2_with_max_group_above_limit() {
    let config = BraceConfig {
        max_depth: 2,
        ..Default::default()
    };
    assert_braces(
        vec!["a/b/c/d", "a/b/c/e", "a/b/f"],
        "a/b/{c/{d,e},f}",
        &config,
    );
}

#[test]
fn test_depth_limit_exceeded() {
    let config = BraceConfig {
        max_depth: 1,
        ..Default::default()
    };
    assert_braces(
        vec!["a/b/c/d", "a/b/c/e", "a/b/f/g", "a/b/f/h"],
        "a/b/{c/d,c/e,f/g,f/h}",
        &config,
    );
}

#[test]
fn test_deeply_nested_paths() {
    assert_braces_default(
        vec!["a/b/c/d/e/1", "a/b/c/d/e/2", "a/b/c/d/f/3"],
        "a/b/c/d/{e/{1,2},f/3}",
    );
}

#[test]
fn test_mixed_depth_paths() {
    assert_braces_default(vec!["a/b", "a/b/c", "a/b/c/d"], "a/b/{c/{d,},}");
}
