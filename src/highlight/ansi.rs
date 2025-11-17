//! Minimal ANSI color support

#[cfg(not(feature = "owo"))]
mod internal {
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

    // Standard colors
    pub const CYAN: Color = Color::new(36);
    pub const YELLOW: Color = Color::new(33);
    pub const MAGENTA: Color = Color::new(35);
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
}

#[cfg(feature = "owo")]
mod internal {
    pub use owo_colors::{colors::*, OwoColorize as Highlight};
    pub type Color = &'static dyn owo_colors::DynColor;

    pub const CYAN: Color = &Cyan;
    pub const YELLOW: Color = &Yellow;
    pub const MAGENTA: Color = &Magenta;
    pub const GREEN: Color = &Green;
    pub const BLUE: Color = &Blue;
}

pub use internal::*;
