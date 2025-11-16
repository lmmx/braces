mod helpers;

use braces::BraceConfig;
use helpers::*;

#[test]
fn test_max_brace_size() {
    let config = BraceConfig {
        max_brace_size: Some(2),
        ..Default::default()
    };
    // Should split into multiple braces
    assert_braces(vec!["a/b", "a/c", "a/d"], "{a/{b/,c/},a/d/}", &config);
}
