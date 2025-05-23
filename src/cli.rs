use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aguda-rs")]
#[command(version = "0.1.0")]
#[command(about = "AGUDA Compiler")]
pub struct Cli {
    /// Path to the source .agu file
    #[arg(short, long, default_value = "main.agu")]
    pub file: String,

    /// Maximum number of errors to display
    #[arg(long, default_value_t = 5)]
    pub max_errors: usize,

    /// Maximum number of warnings to display
    #[arg(long, default_value_t = 5)]
    pub max_warnings: usize,

    /// Suppress errors in the output
    #[arg(long, default_value_t = false)]
    pub suppress_errors: bool,

    /// Suppress warnings in the output
    #[arg(long, default_value_t = false)]
    pub suppress_warnings: bool,

    /// Suppress hints in the output
    #[arg(long, default_value_t = false)]
    pub suppress_hints: bool,

    /// Show the AST without running the program
    #[arg(long, default_value_t = false)]
    pub ast: bool,

    /// LLVM optimization level (0-3)
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..=3))]
    pub opt: u32,
}