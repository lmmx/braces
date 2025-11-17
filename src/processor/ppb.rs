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
    let mut output = String::new();
    let mut indent_stack: Vec<usize> = Vec::new();
    let mut current_line = String::new();
    let mut chars = expr.chars().peekable();

    while let Some(c) = chars.next() {
        // Handle ANSI escape sequences
        if c == '\x1b' && chars.peek() == Some(&'[') {
            current_line.push(c);
            current_line.push(chars.next().unwrap()); // '['
            while let Some(&ch) = chars.peek() {
                current_line.push(chars.next().unwrap());
                if ch.is_ascii_alphabetic() {
                    break;
                }
            }
            continue;
        }

        match c {
            '{' => {
                current_line.push('{');
                output.push_str(&current_line);
                output.push('\n');

                indent_stack.push(strip_ansi(&current_line).len());
                current_line = " ".repeat(*indent_stack.last().unwrap());
            }
            '}' => {
                // Flush current line if it has visible content
                let stripped = strip_ansi(&current_line);
                if !stripped.trim().is_empty() {
                    // Remove trailing comma if present
                    if current_line.trim_end().ends_with(',') {
                        let trimmed_len = current_line.trim_end().len();
                        current_line.truncate(trimmed_len - 1);
                    }
                    output.push_str(&current_line);
                    output.push('\n');
                }

                // Add closing brace
                let indent = indent_stack.pop().unwrap_or(0).saturating_sub(1);
                output.push_str(&" ".repeat(indent));
                output.push('}');
                output.push('\n');

                current_line = " ".repeat(indent_stack.last().copied().unwrap_or(0));
            }
            ',' => {
                current_line.push(',');
                output.push_str(&current_line);
                output.push('\n');
                current_line = " ".repeat(indent_stack.last().copied().unwrap_or(0));
            }
            _ => {
                current_line.push(c);
            }
        }
    }

    // Flush any remaining content
    if !strip_ansi(&current_line).trim().is_empty() {
        output.push_str(&current_line);
    } else if output.ends_with('\n') {
        output.pop(); // Remove trailing newline
    }

    output
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
