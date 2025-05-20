use clap::Parser;
use colored::Colorize;
use aguda_rs::cli::Cli;
use aguda_rs::compile_aguda_program;
use aguda_rs::diagnostics::formatting::{format_errors, format_warnings};
use aguda_rs::utils::read_source_file;

fn main() {
    let args = Cli::parse();
    let result = read_source_file(&args.file).and_then(|src| {
        let (ast, warnings) = compile_aguda_program(&src, &args.file)
            .map_err(|errors| {
                if args.suppress_all || args.suppress_errors {
                    String::new()
                } else {
                    format_errors(errors, args.max_errors, args.suppress_hints, &args.file, &src)
                }
            })?;
        let mut output = String::new();
        if args.suppress_all {
            return Ok(output);
        }
        if !args.suppress_ast {
            output.push_str(&ast.to_text())
        }
        if !args.suppress_warnings && !warnings.is_empty() {
            let warnings_str = format_warnings(warnings, args.max_warnings, args.suppress_hints, &args.file, &src);
            output.push_str(&format!("\n{}", warnings_str));
        }
        Ok(output)
    });
    match result {
        Ok(output) => println!("{}\n{}", output, "Compilation successful!".green().bold()),
        Err(e) => eprintln!("{}\n{}", e, "Compilation failed!".red().bold()),
    }
}
