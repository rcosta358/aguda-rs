use clap::Parser;
use colored::Colorize;
use aguda_rs::cli::Cli;
use aguda_rs::compile_aguda_program;
use aguda_rs::errors::formatting::format_compile_errors;
use aguda_rs::utils::read_source_file;

fn main() {
    let args = Cli::parse();
    let result = read_source_file(args.file.clone()).and_then(|src| {
        let ast = compile_aguda_program(&src).
            map_err(|e| format_compile_errors(e, args.max_errors, &args.file, &src))?;
        let output = if args.suppress_ast { None } else { Some(ast.to_text()) };
        Ok(output)
    });
    match result {
        Ok(output) => println!("{}\n{}", output.unwrap_or("".to_string()), "Compilation successful!".green().bold()),
        Err(e) => eprintln!("{}\n{}", e, "Compilation failed!".red().bold()),
    }
}
