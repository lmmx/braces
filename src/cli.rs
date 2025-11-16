#[cfg(feature = "cli")]
use std::io::{self, BufRead};

#[cfg(feature = "cli")]
fn main() {
    use brace::{brace_paths, BraceConfig};

    let args: Vec<String> = std::env::args().skip(1).collect();

    // Check for help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // Parse config from args
    let mut config = BraceConfig::default();
    let mut paths = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--sort" => config.sort_items = true,
            "--stem-split" => config.allow_stem_split = true,
            "--no-path-split" => config.allow_segment_split = false,
            "--disallow-empty" => config.disallow_empty_braces = true,
            "--no-dedup" => config.deduplicate_inputs = false,
            "--reprocess" => config.reprocess_braces = true,
            "--allow-mixed-sep" => config.allow_mixed_separators = true,
            "--preserve-order" => config.preserve_order_within_braces = true,
            "--separator" => {
                i += 1;
                if i < args.len() {
                    config.path_separator = args[i].clone();
                }
            }
            "--max-depth" => {
                i += 1;
                if i < args.len() {
                    if let Ok(depth) = args[i].parse() {
                        config.max_depth = depth;
                    }
                }
            }
            "--max-brace-size" => {
                i += 1;
                if i < args.len() {
                    if let Ok(size) = args[i].parse() {
                        config.max_brace_size = Some(size);
                    }
                }
            }
            _ if !arg.starts_with("--") => {
                paths.push(arg.clone());
            }
            _ => {}
        }
        i += 1;
    }

    // Read from stdin if no paths provided
    if paths.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(path) = line {
                let path = path.trim();
                if !path.is_empty() {
                    paths.push(path.to_string());
                }
            }
        }
    }

    if paths.is_empty() {
        eprintln!("Error: No paths provided");
        std::process::exit(1);
    }

    match brace_paths(&paths, &config) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "cli")]
fn print_help() {
    println!("brace - Convert path lists into brace expansion syntax");
    println!();
    println!("USAGE:");
    println!("    brace [OPTIONS] [PATHS...]");
    println!("    echo -e \"path1\\npath2\" | brace [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --sort                  Sort items within braces");
    println!("    --stem-split           Enable stem-level character splitting");
    println!("    --no-path-split        Disable path splitting (no empty components)");
    println!("    --disallow-empty       Output separate paths instead of empty braces");
    println!("    --no-dedup             Don't remove duplicate paths");
    println!("    --reprocess            Expand and reprocess existing braces");
    println!("    --allow-mixed-sep      Normalize mixed separators");
    println!("    --preserve-order       Sort within braces even when --sort not used");
    println!("    --separator SEP        Set path separator (default: /)");
    println!("    --max-depth N          Maximum brace nesting depth (default: 5)");
    println!("    --max-brace-size N     Maximum items per brace");
    println!("    -h, --help             Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    brace foo/bar.rs foo/baz.rs");
    println!("    echo -e \"foo/bar.rs\\nfoo/baz.rs\" | brace --sort");
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI feature not enabled. Rebuild with --features cli");
    std::process::exit(1);
}
