# braces

[![crates.io](https://img.shields.io/crates/v/braces.svg)](https://crates.io/crates/braces)
[![documentation](https://docs.rs/braces/badge.svg)](https://docs.rs/braces)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/braces.svg)](./LICENSE)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/braces/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/braces/master)
[![free of syn](https://img.shields.io/badge/free%20of-syn-hotpink)](https://github.com/fasterthanlime/free-of-syn)

Convert file paths into compact braces expansion syntax.

## Examples
```rust
use braces::{brace_paths, BraceConfig};

// Basic usage
let paths = vec!["foo/bar.rs", "foo/baz.rs"];
let result = brace_paths(&paths, &BraceConfig::default())?;
assert_eq!(result, "foo/{bar,baz}.rs");
```

### Stem Splitting
Factor out common character prefixes within path segments:
```rust
let config = BraceConfig {
    allow_stem_split: true,
    ..Default::default()
};
let paths = vec!["foo/bar.rs", "foo/baz.rs"];
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "foo/ba{r,z}.rs");
```

### Segment Splitting
Control whether to create empty alternatives when one path is a prefix of another:
```rust
// Enabled (default) - creates empty alternative
let paths = vec!["a/b", "a/b/c"];
let result = brace_paths(&paths, &BraceConfig::default())?;
assert_eq!(result, "a/b{,/c}");

// Disabled - keeps paths whole
let config = BraceConfig {
    allow_segment_split: false,
    ..Default::default()
};
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "a/{b,b/c}");
```

### Sorting
```rust
// Default: sorted within braces
let paths = vec!["z.rs", "b.rs"];
let result = brace_paths(&paths, &BraceConfig::default())?;
assert_eq!(result, "{b,z}.rs");

// Preserve input order
let config = BraceConfig {
    preserve_order_within_braces: true,
    ..Default::default()
};
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "{z,b}.rs");
```

### Depth Limiting
Limit brace nesting to prevent performance issues:
```rust
let config = BraceConfig {
    max_depth: 2,
    ..Default::default()
};
let paths = vec!["a/b/c/1", "a/b/c/2", "a/b/d/3"];
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "a/b/{c/{1,2},d/3}");

// With max_depth: 1
// Result: a/b/{c/1,c/2,d/3}
```

## CLI Usage
```bash
# From arguments
braces foo/bar.rs foo/baz.rs
# Output: foo/{bar,baz}.rs

# From stdin
echo -e "foo/bar.rs\nfoo/baz.rs" | braces
# Output: foo/{bar,baz}.rs

# With options
braces --sort --stem-split foo/bar.rs foo/baz.rs
```

## Configuration

`BraceConfig` options:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `path_separator` | `String` | `"/"` | Path separator to use |
| `max_depth` | `usize` | `5` | Maximum brace nesting depth |
| `max_brace_size` | `Option<usize>` | `None` | Maximum items per brace group |
| `allow_stem_split` | `bool` | `false` | Factor out character-level prefixes |
| `allow_segment_split` | `bool` | `true` | Allow empty alternatives like `{,/c}` |
| `sort_items` | `bool` | `false` | Sort items alphabetically |
| `disallow_empty_braces` | `bool` | `false` | Output separate paths instead of empty alternatives |
| `preserve_order_within_braces` | `bool` | `false` | Maintain exact input order within braces |
| `allow_mixed_separators` | `bool` | `false` | Normalize different separators to `path_separator` |
| `deduplicate_inputs` | `bool` | `true` | Remove duplicate paths before processing |
| `reprocess_braces` | `bool` | `false` | Expand and reprocess existing braces syntax |

## Installation
```bash
cargo add braces
```

Or install the CLI:
```bash
cargo install braces --features cli
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
