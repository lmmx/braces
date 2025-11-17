// src/processor/ppb.rs
//! Pretty-print braces expansion syntax with indentation

/// Count visible characters (excluding ANSI escape codes)
fn visible_len(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.next() == Some('[') {
                // Skip until we find a letter (the command character)
                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            count += 1;
        }
    }

    count
}

/// Check if a string has any visible characters (excluding ANSI codes and whitespace)
fn has_visible_content(s: &str) -> bool {
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.next() == Some('[') {
                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else if !c.is_whitespace() {
            return true;
        }
    }

    false
}

/// Pretty-print a braces expression with indentation
///
/// Takes a braces expression like `"a/{b,c/{d,e},f}"` and formats it with
/// each brace level indented for readability.
///
/// # Example
/// ```
/// use braces::processor::ppb::pretty_braces;
///
/// let expr = "a/{b,c/{d,e},f}";
/// let pretty = pretty_braces(expr);
/// // Output:
/// // a/{
/// //   b,
/// //   c/{
/// //     d,
/// //     e
/// //   },
/// //   f
/// // }
/// ```
pub fn pretty_braces(expr: &str) -> String {
    let mut lines = Vec::new();
    let mut stack: Vec<usize> = Vec::new(); // tracks column of each '{'
    let mut line = String::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Handle ANSI escape sequences
        if c == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '[' {
            // Copy the entire escape sequence
            line.push(c);
            i += 1;
            line.push(chars[i]); // '['
            i += 1;
            while i < chars.len() {
                line.push(chars[i]);
                if chars[i].is_ascii_alphabetic() {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        match c {
            '{' => {
                // Add opening brace to current line
                line.push(c);
                lines.push(line.clone());

                // Record column position of the brace (for alignment) - use visible length
                stack.push(visible_len(&line));

                // Start new line indented to brace column
                line = " ".repeat(stack.last().copied().unwrap_or(0));
                i += 1;
            }
            '}' => {
                // Push current line if it has visible content (strip trailing comma)
                if has_visible_content(&line) {
                    // Remove trailing comma if present
                    let trimmed = line.trim_end();
                    if let Some(stripped) = trimmed.strip_suffix(',') {
                        line = format!("{}{}", stripped, " ".repeat(line.len() - trimmed.len()));
                    }
                    lines.push(line);
                }

                // Create closing brace line at previous indentation
                if let Some(indent) = stack.pop() {
                    line = format!("{}}}", " ".repeat(indent.saturating_sub(1)));
                } else {
                    line = "}".to_string();
                }
                lines.push(line.clone());

                // Continue on new line at current indentation level
                line = " ".repeat(stack.last().copied().unwrap_or(0));
                i += 1;
            }
            ',' => {
                // Add comma and go to new line
                line.push(c);
                lines.push(line);

                // Start new line at current indentation
                line = " ".repeat(stack.last().copied().unwrap_or(0));
                i += 1;
            }
            _ => {
                // Regular character - add to current line
                line.push(c);
                i += 1;
            }
        }
    }

    // Push final line if it has visible content
    if has_visible_content(&line) {
        lines.push(line);
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_simple_braces() {
        let input = "{a,b,c}";
        assert_snapshot!(pretty_braces(input), @r"
        {
         a,
         b,
         c
        }
        ");
    }

    #[test]
    fn test_nested_braces() {
        let input = "foo/{bar,baz/{qux,quux}}";
        assert_snapshot!(pretty_braces(input), @r"
        foo/{
             bar,
             baz/{
                  qux,
                  quux
                 }
            }
        ");
    }

    #[test]
    fn test_empty_braces() {
        let input = "a/{}";
        assert_snapshot!(pretty_braces(input), @r"
        a/{
          }
        ");
    }

    #[test]
    fn test_multiple_levels() {
        let input = "{a/{b,c},d/{e/{f,g},h}}";
        assert_snapshot!(pretty_braces(input), @r"
        {
         a/{
            b,
            c
           }
         ,
         d/{
            e/{
               f,
               g
              }
            ,
            h
           }
        }
        ");
    }

    #[test]
    fn test_no_braces() {
        let input = "simple/path/file.txt";
        assert_snapshot!(pretty_braces(input), @"simple/path/file.txt");
    }
}
