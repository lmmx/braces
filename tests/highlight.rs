#![cfg(feature = "cli")]
use braces::highlight::highlight_braces;

#[test]
fn test_single_brace_group() {
    let input = "{a,b,c}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mb\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mc\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_nested_braces() {
    let input = "a/{b/{c,d},e}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"a/\u{1b}[36m{\u{1b}[0mb/\u{1b}[33m{\u{1b}[0m\u{1b}[33mc\u{1b}[0m\u{1b}[33m,\u{1b}[0m\u{1b}[33md\u{1b}[0m\u{1b}[33m}\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36me\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_cycling_colors() {
    let input = "{a,{b,{c,{d,{e,{f,g}}}}}}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[33m{\u{1b}[0m\u{1b}[33mb\u{1b}[0m\u{1b}[33m,\u{1b}[0m\u{1b}[35m{\u{1b}[0m\u{1b}[35mc\u{1b}[0m\u{1b}[35m,\u{1b}[0m\u{1b}[32m{\u{1b}[0m\u{1b}[32md\u{1b}[0m\u{1b}[32m,\u{1b}[0m\u{1b}[34m{\u{1b}[0m\u{1b}[34me\u{1b}[0m\u{1b}[34m,\u{1b}[0m\u{1b}[36m{\u{1b}[0m\u{1b}[36mf\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mg\u{1b}[0m\u{1b}[36m}\u{1b}[0m\u{1b}[34m}\u{1b}[0m\u{1b}[32m}\u{1b}[0m\u{1b}[35m}\u{1b}[0m\u{1b}[33m}\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_comma_coloring() {
    let input = "{a,b,c}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mb\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mc\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_no_braces() {
    let input = "plain/text/path";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"plain/text/path");
}

#[test]
fn test_empty_braces() {
    let input = "a/b{,/c}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"a/b\u{1b}[36m{\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36m/c\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_multiple_groups() {
    let input = "{a,b}/{c,d}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mb\u{1b}[0m\u{1b}[36m}\u{1b}[0m/\u{1b}[36m{\u{1b}[0m\u{1b}[36mc\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36md\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_unbalanced_braces_open() {
    let input = "{a,b";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mb\u{1b}[0m");
}

#[test]
fn test_unbalanced_braces_close() {
    let input = "a,b}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"a,b\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_complex_real_world() {
    let input = "src/{lib,main,{util/{mod,helpers},tests/{unit,integration}}}.rs";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"src/\u{1b}[36m{\u{1b}[0m\u{1b}[36mlib\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mmain\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[33m{\u{1b}[0m\u{1b}[33mutil/\u{1b}[0m\u{1b}[35m{\u{1b}[0m\u{1b}[35mmod\u{1b}[0m\u{1b}[35m,\u{1b}[0m\u{1b}[35mhelpers\u{1b}[0m\u{1b}[35m}\u{1b}[0m\u{1b}[33m,\u{1b}[0m\u{1b}[33mtests/\u{1b}[0m\u{1b}[35m{\u{1b}[0m\u{1b}[35munit\u{1b}[0m\u{1b}[35m,\u{1b}[0m\u{1b}[35mintegration\u{1b}[0m\u{1b}[35m}\u{1b}[0m\u{1b}[33m}\u{1b}[0m\u{1b}[36m}\u{1b}[0m.rs");
}

#[test]
fn test_triple_nesting() {
    let input = "{a/{b/{c,d},e},f}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36ma/\u{1b}[0m\u{1b}[33m{\u{1b}[0m\u{1b}[33mb/\u{1b}[0m\u{1b}[35m{\u{1b}[0m\u{1b}[35mc\u{1b}[0m\u{1b}[35m,\u{1b}[0m\u{1b}[35md\u{1b}[0m\u{1b}[35m}\u{1b}[0m\u{1b}[33m,\u{1b}[0m\u{1b}[33me\u{1b}[0m\u{1b}[33m}\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36mf\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}

#[test]
fn test_empty_string() {
    let input = "";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_only_braces() {
    let input = "{}";
    let output = highlight_braces(input);
    insta::assert_snapshot!(output, @"\u{1b}[36m{\u{1b}[0m\u{1b}[36m}\u{1b}[0m");
}
