// src/processor/ppb.rs
//! Pretty-print braces expansion syntax with indentation

/// Strip ANSI escape codes from a string
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next(); // consume '['
                          // Skip until we hit a letter (the command)
            while let Some(&ch) = chars.peek() {
                chars.next();
                if ch.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }

    result
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

        // Handle ANSI escape sequences - copy them but don't process
        if c == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '[' {
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
                line.push(c);
                lines.push(line.clone());
                // Use stripped length for indentation
                stack.push(strip_ansi(&line).len());
                line = " ".repeat(stack.last().copied().unwrap_or(0));
                i += 1;
            }
            '}' => {
                // Check if line has visible content (ignoring ANSI codes and whitespace)
                if !strip_ansi(&line).trim().is_empty() {
                    let trimmed = line.trim_end();
                    if let Some(stripped) = trimmed.strip_suffix(',') {
                        line = format!("{}{}", stripped, " ".repeat(line.len() - trimmed.len()));
                    }
                    lines.push(line);
                }

                if let Some(indent) = stack.pop() {
                    line = format!("{}}}", " ".repeat(indent.saturating_sub(1)));
                } else {
                    line = "}".to_string();
                }
                lines.push(line.clone());

                line = " ".repeat(stack.last().copied().unwrap_or(0));
                i += 1;
            }
            ',' => {
                line.push(c);
                lines.push(line);
                line = " ".repeat(stack.last().copied().unwrap_or(0));
                i += 1;
            }
            _ => {
                line.push(c);
                i += 1;
            }
        }
    }

    // Check if final line has visible content
    if !strip_ansi(&line).trim().is_empty() {
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
