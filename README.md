# brace

Convert lists of file paths into compact brace expansion syntax.

## Examples
```rust
use brace::{brace_paths, BraceConfig};

let paths = vec!["foo/bar.rs", "foo/baz.rs"];
let result = brace_paths(&paths, &BraceConfig::default())?;
assert_eq!(result, "foo/{bar,baz}.rs");
```

### Stem Splitting
```rust
let config = BraceConfig {
    allow_stem_split: true,
    ..Default::default()
};
let paths = vec!["foo/bar.rs", "foo/baz.rs"];
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "foo/ba{r,z}.rs");
```

### Path Splitting
```rust
// With path splitting (default)
let paths = vec!["a/b", "a/b/c"];
let result = brace_paths(&paths, &BraceConfig::default())?;
assert_eq!(result, "a/b{,/c}");

// Without path splitting
let config = BraceConfig {
    allow_path_split: false,
    ..Default::default()
};
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "a/{b,b/c}");
```

### Sorting
```rust
// Order of appearance
let paths = vec!["z.rs", "b.rs"];
let result = brace_paths(&paths, &BraceConfig::default())?;
assert_eq!(result, "{z,b}.rs");

// Sorted
let config = BraceConfig {
    sort_items: true,
    ..Default::default()
};
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "{b,z}.rs");
```

### Depth Limiting
```rust
// Default depth limit of 5
let config = BraceConfig {
    max_depth: 2,
    ..Default::default()
};
let paths = vec!["a/b/c/1", "a/b/c/2", "a/b/d/3"];
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "a/b/{c/{1,2},d/3}");

// Depth limit of 1
let config = BraceConfig {
    max_depth: 1,
    ..Default::default()
};
let result = brace_paths(&paths, &config)?;
assert_eq!(result, "a/{b/c/1,b/c/2,b/d/3}");
```

## CLI Usage
```bash
# From arguments
brace foo/bar.rs foo/baz.rs
# Output: foo/{bar,baz}.rs

# From stdin
echo -e "foo/bar.rs\nfoo/baz.rs" | brace
# Output: foo/{bar,baz}.rs

# With options
brace --sort --stem-split foo/bar.rs foo/baz.rs
# Output: foo/ba{r,z}.rs
```

## Configuration

All options in `BraceConfig`:

- `path_separator`: String (default: "/")
- `max_depth`: usize (default: 5) - maximum brace nesting depth
- `max_brace_size`: Option<usize> (default: None) - maximum items per brace
- `allow_stem_split`: bool (default: false) - enable stem-level character splitting
- `allow_path_split`: bool (default: true) - allow empty components like `{,/c}`
- `sort_items`: bool (default: false) - sort items within braces
- `disallow_empty_braces`: bool (default: false) - output separate paths instead of `{,/c}`
- `preserve_order_within_braces`: bool (default: false) - when false and not sorting, still sort within braces for readability
- `allow_mixed_separators`: bool (default: false) - normalize mixed separators to path_separator
- `deduplicate_inputs`: bool (default: true) - remove duplicate paths
- `reprocess_braces`: bool (default: false) - expand and reprocess existing braces in input

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.