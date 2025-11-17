//! Minimal ANSI color support

/// ANSI color codes
#[derive(Clone, Copy)]
pub struct Color {
    code: u8,
}

impl Color {
    pub const fn new(code: u8) -> Self {
        Self { code }
    }
}

// Standard colors - chosen for maximum contrast and color-blind friendliness
pub const CYAN: Color = Color::new(36);
pub const YELLOW: Color = Color::new(33);
pub const RED: Color = Color::new(31);
pub const GREEN: Color = Color::new(32);
pub const BLUE: Color = Color::new(34);

/// Extension trait to add `.color()` method to any type that can be displayed
pub trait Highlight {
    fn color(&self, color: Color) -> String;
}

impl<T: std::fmt::Display> Highlight for T {
    fn color(&self, color: Color) -> String {
        format!("\x1b[{}m{}\x1b[0m", color.code, self)
    }
}
