use clap::Parser;
use colored::Colorize;
use aguda_rs::cli::Cli;
use aguda_rs::compile_aguda_program;
use aguda_rs::errors::formatting::{format_compile_errors, format_warnings};
use aguda_rs::utils::read_source_file;

fn main() {
    let args = Cli::parse();
    let result = read_source_file(&args.file).and_then(|src| {
        let (ast, warnings) = compile_aguda_program(&src).
            map_err(|errors| format_compile_errors(errors, args.max_errors, &args.file, &src))?;

        let mut output = String::new();
        if !args.suppress_ast {
            output.push_str(&ast.to_text())
        }
        if !args.suppress_warnings && !warnings.is_empty() {
            let warnings_str = format_warnings(warnings, args.max_warnings, &args.file, &src);
            output.push_str(&format!("\n{}", warnings_str));
        }
        Ok(output)
    });
    match result {
        Ok(output) => println!("{}\n{}", output, "Compilation successful!".green().bold()),
        Err(e) => eprintln!("{}\n{}", e, "Compilation failed!".red().bold()),
    }
}
