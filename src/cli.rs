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
    #[arg(short, long, default_value_t = 5)]
    pub max_errors: usize,

    /// Skip printing the AST to stdout
    #[arg(long, default_value_t = false)]
    pub no_print_ast: bool,
}
