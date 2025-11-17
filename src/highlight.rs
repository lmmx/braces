//! Syntax highlighting for brace expressions

mod ansi;

use ansi::{Highlight, BLUE, CYAN, GREEN, RED, YELLOW};

const BRACE_COLORS: &[ansi::Color] = &[CYAN, YELLOW, RED, GREEN, BLUE];

/// Highlight braces in the output with cycling colors per nesting level
pub fn highlight_braces(text: &str) -> String {
    let mut result = String::new();
    let mut depth = 0;

    for ch in text.chars() {
        match ch {
            '{' => {
                let color = BRACE_COLORS[depth % BRACE_COLORS.len()];
                result.push_str(&ch.color(color).to_string());
                depth += 1;
            }
            '}' => {
                depth = depth.saturating_sub(1);
                let color = BRACE_COLORS[depth % BRACE_COLORS.len()];
                result.push_str(&ch.color(color).to_string());
            }
            ',' => {
                if depth > 0 {
                    let color = BRACE_COLORS[(depth - 1) % BRACE_COLORS.len()];
                    result.push_str(&ch.color(color).to_string());
                } else {
                    result.push(ch);
                }
            }
            _ => result.push(ch),
        }
    }

    result
}
