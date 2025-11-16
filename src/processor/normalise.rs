use crate::error::{BraceError, Result};
use std::collections::HashSet;

/// Stem splitting eligibility
pub fn can_stem_split(items: &[String]) -> bool {
    if items.len() < 2 { return false; }
    let p = find_common_string_prefix(items);
    let s = find_common_string_suffix(items);
    !p.is_empty() || !s.is_empty()
}

/// Validate separators
pub fn validate_separators(paths: &[String], expected: &str) -> Result<()> {
    let seps = vec!["/", "\\", ":"];
    let mut found = HashSet::new();
    for path in paths {
        for sep in &seps {
            if path.contains(sep) && !expected.contains(sep) { found.insert(sep.to_string()); }
        }
    }
    if !found.is_empty() { return Err(BraceError::MixedSeparators { found: found.into_iter().collect(), expected: expected.to_string() }); }
    Ok(())
}

/// Normalize separators
pub fn normalise_separators(path: &str, target: &str) -> String {
    let seps = vec!["/", "\\", ":"];
    let mut res = path.to_string();
    for sep in seps { if sep != target { res = res.replace(sep, target); } }
    res
}

/// Common suffix across strings
pub fn find_common_suffix(strings: &[String]) -> String {
    if strings.is_empty() { return String::new(); }
    let first: Vec<char> = strings[0].chars().collect();
    let mut common = vec![];
    'outer: for i in (0..first.len()).rev() {
        let c = first[i];
        for s in strings.iter().skip(1) {
            let sc: Vec<char> = s.chars().collect();
            if sc.len() <= i || sc[sc.len() - first.len() + i] != c { break 'outer; }
        }
        common.insert(0, c);
    }
    common.into_iter().collect()
}

/// Common prefix
pub fn find_common_string_prefix(strings: &[String]) -> String {
    if strings.is_empty() { return String::new(); }
    let first: Vec<char> = strings[0].chars().collect();
    let mut common = vec![];
    for (i, c) in first.iter().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(*c)) { common.push(*c); } else { break; }
    }
    common.into_iter().collect()
}

/// Common suffix
pub fn find_common_string_suffix(strings: &[String]) -> String {
    if strings.is_empty() { return String::new(); }
    let first: Vec<char> = strings[0].chars().collect();
    let mut common = vec![];
    for i in 0..first.len() {
        let c = first[first.len() - i - 1];
        if strings.iter().all(|s| s.chars().rev().nth(i) == Some(c)) { common.push(c); } else { break; }
    }
    common.into_iter().rev().collect()
}

/// Strip trailing separator
pub fn clean_trailing_sep<'a>(s: &'a str, _sep: &str) -> &'a str {
    s.strip_suffix(_sep).unwrap_or(s)
}
