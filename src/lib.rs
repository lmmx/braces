//! Convert lists of file paths into compact brace expansion syntax.
//!
//! # Examples
//!
//! ```
//! use braces::{brace_paths, BraceConfig};
//!
//! let paths = vec!["foo/bar.rs", "foo/baz.rs"];
//! let result = brace_paths(&paths, &BraceConfig::default()).unwrap();
//! assert_eq!(result, "foo/{bar,baz}.rs");
//! ```

mod error;
mod processor;

pub use error::{BraceError, Result};
pub use processor::expansion::expand_braces;
pub use processor::{brace_paths, BraceConfig};
