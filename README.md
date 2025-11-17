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
````bash
# From arguments
braces foo/bar.rs foo/baz.rs
# Output: foo/{bar,baz}.rs

# From stdin
echo -e "foo/bar.rs\nfoo/baz.rs" | braces
# Output: foo/{bar,baz}.rs

# With options
braces --sort --stem-split foo/bar.rs foo/baz.rs

# With syntax highlighting (requires highlight feature)
braces --highlight --pretty foo/bar.rs foo/baz.rs
````

### CLI Options

```bash
braces [OPTIONS] [PATHS...]
```

| Option | Description |
|--------|-------------|
| `--pretty` | Pretty-print the output with indentation |
| `--highlight` | Highlight brace groups with colors (requires `highlight` feature) |
| `--sort` | Sort items within braces alphabetically |
| `--stem-split` | Enable stem-level character splitting |
| `--no-segment-split` | Disable segment splitting (no empty components) |
| `--disallow-empty` | Output separate paths instead of empty braces |
| `--no-dedup` | Don't remove duplicate paths |
| `--reprocess` | Expand and reprocess existing braces |
| `--allow-mixed-sep` | Normalize mixed separators |
| `--preserve-order` | Maintain exact input order within braces |
| `--separator SEP` | Set path separator (default: `/`) |
| `--max-depth N` | Maximum brace nesting depth (default: `5`) |
| `--max-brace-size N` | Maximum items per brace group |
| `-0, --null` | Read NUL-separated input (like `xargs -0`) |
| `-h, --help` | Print help message |

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
| `highlight` | `bool` | `false` | Highlight braces with colors (requires `highlight` feature) |

## Installation

Add to your project:
```bash
cargo add braces
```

The easiest way to install the CLI is via [cargo binstall][cbin] (which pulls pre-built binaries from the [release][ghr]).

[cbin]: https://github.com/cargo-bins/cargo-binstall
[ghr]: https://github.com/lmmx/braces/releases

```bash
cargo binstall braces
```

To build it yourself:

```bash
# Basic CLI
cargo install braces --features cli

# CLI with syntax highlighting
cargo install braces --features cli-highlight

# CLI with all features
cargo install braces --features cli-full
```

### Features

The crate supports the following feature flags:

- `cli` - Enables the command-line interface
- `highlight` - Enables syntax highlighting for brace output
- `cli-highlight` - Convenience feature enabling both `cli` and `highlight`
- `cli-full` - Alias for `cli-highlight` with all CLI features

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
