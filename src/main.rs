use clap::Parser;
use colored::Colorize;
use aguda_rs::cli::Cli;
use aguda_rs::compile_aguda_program;
use aguda_rs::utils::read_source_file;

fn main() {
    let args = Cli::parse();
    let result = read_source_file(args.file).and_then(|src| {
        compile_aguda_program(src, args.max_errors, !args.no_print_ast)
    });
    match result {
        Ok(output) => println!("{}\n{}", output, "Compilation successful!".green().bold()),
        Err(err) => eprintln!("{}\n{}", err, "Compilation failed!".red().bold()),
    }
}
