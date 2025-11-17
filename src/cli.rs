// src/cli.rs
#[cfg(feature = "cli")]
use std::io::{self, BufRead};

#[cfg(feature = "cli")]
fn main() {
    use braces::{brace_paths, pretty_braces, BraceConfig};

    #[cfg(feature = "highlight")]
    use anstream::println;
    #[cfg(not(feature = "highlight"))]
    use std::println;

    let args: Vec<String> = std::env::args().skip(1).collect();

    // Check for help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // Parse config from args
    let mut config = BraceConfig::default();
    let mut paths = Vec::new();
    let mut pretty_print = false;
    let mut read_null = false;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--pretty" => pretty_print = true,
            "--sort" => config.sort_items = true,
            "--stem-split" => config.allow_stem_split = true,
            "--no-segment-split" => config.allow_segment_split = false,
            "--disallow-empty" => config.disallow_empty_braces = true,
            "--no-dedup" => config.deduplicate_inputs = false,
            "--reprocess" => config.reprocess_braces = true,
            "--allow-mixed-sep" => config.allow_mixed_separators = true,
            "--preserve-order" => config.preserve_order_within_braces = true,
            #[cfg(feature = "highlight")]
            "--highlight" => config.highlight = true,
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
            "-0" | "--null" => read_null = true,
            _ if arg.starts_with("--") => {
                eprintln!("Error: Unknown option: {}", arg);
                print_help();
                std::process::exit(1);
            }
            _ => {
                paths.push(arg.clone());
            }
        }
        i += 1;
    }

    // Read from stdin if no paths provided
    if paths.is_empty() {
        let stdin = io::stdin();
        let handle = stdin.lock();
        if read_null {
            // NUL-delimited input
            for chunk in handle.split(0).map_while(Result::ok) {
                if !chunk.is_empty() {
                    // No trimming, take NUL-terminated paths literally
                    let path = String::from_utf8_lossy(&chunk).into_owned();
                    paths.push(path);
                }
            }
        } else {
            // Newline-delimited input (`\n` or `\r\n`)
            for line in handle.lines().map_while(Result::ok) {
                if !line.is_empty() {
                    paths.push(line.to_string());
                }
            }
        }
    }

    if paths.is_empty() {
        eprintln!("Error: No paths provided");
        std::process::exit(1);
    }

    match brace_paths(&paths, &config) {
        Ok(result) => {
            let output = if pretty_print {
                pretty_braces(&result)
            } else {
                result
            };

            #[cfg(feature = "highlight")]
            let highlighted = if config.highlight {
                braces::highlight::highlight_braces(&output)
            } else {
                output
            };

            #[cfg(not(feature = "highlight"))]
            let highlighted = output;

            println!("{}", highlighted);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "cli")]
fn print_help() {
    println!("braces - Convert path lists into braces expansion syntax");
    println!();
    println!("USAGE:");
    println!("    braces [OPTIONS] [PATHS...]");
    println!("    echo -e \"path1\\npath2\" | braces [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --pretty              Pretty-print the output with indentation");
    #[cfg(feature = "highlight")]
    println!("    --highlight           Highlight brace groups with colours");
    println!("    --sort                Sort items within braces");
    println!("    --stem-split          Enable stem-level character splitting");
    println!("    --no-segment-split    Disable segment splitting (no empty components)");
    println!("    --disallow-empty      Output separate paths instead of empty braces");
    println!("    --no-dedup            Don't remove duplicate paths");
    println!("    --reprocess           Expand and reprocess existing braces");
    println!("    --allow-mixed-sep     Normalise mixed separators");
    println!("    --preserve-order      Sort within braces even when --sort not used");
    println!("    --separator SEP       Set path separator (default: /)");
    println!("    --max-depth N         Maximum brace nesting depth (default: 5)");
    println!("    --max-brace-size N    Maximum items per brace");
    println!("    -0, --null            Read NUL-separated input (like xargs -0)");
    println!("    -h, --help            Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    braces foo/bar.rs foo/baz.rs");
    println!("    echo -e \"foo/bar.rs\\nfoo/baz.rs\" | braces --sort");
    #[cfg(feature = "highlight")]
    println!("    braces --highlight --pretty foo/{{bar,baz}}.rs");
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI feature not enabled. Rebuild with --features cli");
    std::process::exit(1);
}
