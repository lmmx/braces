#![cfg(feature = "cli")]
use braces::highlight::highlight_braces;

#[test]
fn test_empty_string() {
    let output = highlight_braces("");
    insta::assert_snapshot!(output);
}

#[test]
fn test_only_braces() {
    let output = highlight_braces("{}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_null_delimited_input() {
    let output = braces::brace_paths(
        &["you/pulled/off", "your/only", "your/\ntrick"],
        &braces::BraceConfig::default(),
    )
    .unwrap();
    insta::assert_snapshot!(output);
}

#[test]
fn test_null_delimited_simple() {
    let input = "a/b\0a/c\0a/d";
    let paths: Vec<&str> = input.split('\0').collect();
    let output = braces::brace_paths(&paths, &braces::BraceConfig::default()).unwrap();
    insta::assert_snapshot!(output);
}

#[test]
fn test_null_delimited_with_newlines() {
    let paths = vec!["path/with\nnewline", "path/with\ntab", "path/normal"];
    let output = braces::brace_paths(&paths, &braces::BraceConfig::default()).unwrap();
    insta::assert_snapshot!(output);
}

#[test]
fn test_single_brace_group() {
    let output = highlight_braces("{a,b,c}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_nested_braces() {
    let output = highlight_braces("a/{b/{c,d},e}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_cycling_colors() {
    let output = highlight_braces("{a,{b,{c,{d,{e,{f,g}}}}}}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_comma_coloring() {
    let output = highlight_braces("{a,b,c}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_no_braces() {
    let output = highlight_braces("plain/text/path");
    insta::assert_snapshot!(output);
}

#[test]
fn test_empty_braces() {
    let output = highlight_braces("a/b{,/c}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_multiple_groups() {
    let output = highlight_braces("{a,b}/{c,d}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_unbalanced_braces_open() {
    let output = highlight_braces("{a,b");
    insta::assert_snapshot!(output);
}

#[test]
fn test_unbalanced_braces_close() {
    let output = highlight_braces("a,b}");
    insta::assert_snapshot!(output);
}

#[test]
fn test_complex_real_world() {
    let output =
        highlight_braces("src/{lib,main,{util/{mod,helpers},tests/{unit,integration}}}.rs");
    insta::assert_snapshot!(output);
}

#[test]
fn test_triple_nesting() {
    let output = highlight_braces("{a/{b/{c,d},e},f}");
    insta::assert_snapshot!(output);
}
