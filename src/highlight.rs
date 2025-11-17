//! Syntax highlighting for brace expressions

use owo_colors::{colors::*, OwoColorize};

const BRACE_COLORS: &[&dyn owo_colors::DynColor] = &[
    &Cyan,
    &Yellow,
    &Magenta,
    &Green,
    &Blue,
];

/// Highlight braces in the output with cycling colors per nesting level
pub fn highlight_braces(text: &str) -> String {
    let mut result = String::new();
    let mut depth = 0;

    for ch in text.chars() {
        match ch {
            '{' => {
                let color = BRACE_COLORS[depth % BRACE_COLORS.len()];
                result.push_str(&format!("{}", ch.color(*color)));
                depth += 1;
            }
            '}' => {
                depth = depth.saturating_sub(1);
                let color = BRACE_COLORS[depth % BRACE_COLORS.len()];
                result.push_str(&format!("{}", ch.color(*color)));
            }
            ',' => {
                // Optionally color commas to match their brace group level
                if depth > 0 {
                    let color = BRACE_COLORS[(depth - 1) % BRACE_COLORS.len()];
                    result.push_str(&format!("{}", ch.color(*color)));
                } else {
                    result.push(ch);
                }
            }
            _ => result.push(ch),
        }
    }

    result
}
