//! Minimal ANSI color support

/// ANSI color codes
pub struct Color {
    code: u8,
}

impl Color {
    pub const fn new(code: u8) -> Self {
        Self { code }
    }

    pub fn paint(&self, text: &str) -> String {
        format!("\x1b[{}m{}\x1b[0m", self.code, text)
    }
}

// Standard colors
pub const CYAN: Color = Color::new(36);
pub const YELLOW: Color = Color::new(33);
pub const MAGENTA: Color = Color::new(35);
pub const GREEN: Color = Color::new(32);
pub const BLUE: Color = Color::new(34);
