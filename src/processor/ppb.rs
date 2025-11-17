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

/// Skip ANSI escape sequence and append to current line
fn skip_ansi_sequence(chars: &mut std::iter::Peekable<std::str::Chars>, current_line: &mut String) {
    current_line.push('\x1b');
    current_line.push(chars.next().unwrap()); // '['
    while let Some(&ch) = chars.peek() {
        current_line.push(chars.next().unwrap());
        if ch.is_ascii_alphabetic() {
            break;
        }
    }
}

/// Flush current line to output if it has visible content
fn flush_line_if_needed(output: &mut String, current_line: &mut String) {
    let stripped = strip_ansi(current_line);
    if !stripped.trim().is_empty() {
        // Remove trailing comma if present
        if let Some(pos) = current_line.trim_end().rfind(',') {
            current_line.truncate(pos);
        }
        output.push_str(current_line);
        output.push('\n');
    }
}

/// Get indentation string for current stack depth
fn get_indent(indent_stack: &[usize]) -> String {
    " ".repeat(indent_stack.last().copied().unwrap_or(0))
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
        // Preserve but skip over ANSI color codes
        if c == '\x1b' && chars.peek() == Some(&'[') {
            skip_ansi_sequence(&mut chars, &mut current_line);
            continue;
        }

        match c {
            '{' => {
                // Opening brace: output current line and increase indent
                current_line.push('{');
                output.push_str(&current_line);
                output.push('\n');

                indent_stack.push(strip_ansi(&current_line).len());
                current_line = get_indent(&indent_stack);
            }
            '}' => {
                // Closing brace: flush pending content and decrease indent
                flush_line_if_needed(&mut output, &mut current_line);

                let indent = indent_stack.pop().unwrap_or(0).saturating_sub(1);
                output.push_str(&" ".repeat(indent));
                output.push('}');
                output.push('\n');

                current_line = get_indent(&indent_stack);
            }
            ',' => {
                // Comma: output current item and start new line at same indent
                current_line.push(',');
                output.push_str(&current_line);
                output.push('\n');
                current_line = get_indent(&indent_stack);
            }
            _ => {
                current_line.push(c);
            }
        }
    }

    // Handle any remaining content
    if !strip_ansi(&current_line).trim().is_empty() {
        output.push_str(&current_line);
    } else if output.ends_with('\n') {
        output.pop();
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
