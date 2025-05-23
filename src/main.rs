use std::process::exit;
use clap::Parser;
use colored::Colorize;
use aguda_rs::cli::Cli;
use aguda_rs::{compile_aguda_program, run_aguda_program};
use aguda_rs::diagnostics::errors::AgudaError;
use aguda_rs::diagnostics::formatting::{format_aguda_errors, format_warnings};
use aguda_rs::utils::read_aguda_file;

fn main() {
    let args = Cli::parse();
    let src = read_aguda_file(&args.file)
        .unwrap_or_else(|_| {
            eprintln!("{}: {}", "Error".red().bold(), "Failed to read source file");
            exit(1);
        });
    match run_aguda_compiler(args, &src) {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err)
    }
}

fn run_aguda_compiler(args: Cli, src: &str) -> Result<String, String> {
    let fmt_errors = |errors: Vec<AgudaError>| {
        if args.suppress_errors {
            String::new()
        } else {
            format_aguda_errors(errors, args.max_errors, args.suppress_hints, &args.file, &src)
        }
    };

    // compilation
    let opt_str = if args.opt > 0 { format!("with -O{}", args.opt) } else { String::new() };
    println!("{} {} {}", "Compiling".green().bold(), args.file, opt_str);

    let (ast, warnings) = compile_aguda_program(&src, &args.file, args.opt)
        .map_err(|errs| {
            let aguda_errs = errs.into_iter().map(AgudaError::from).collect();
            fmt_errors(aguda_errs)
        })?;

    // warnings
    if !args.suppress_warnings && !warnings.is_empty() {
        println!("{}", format_warnings(warnings, args.max_warnings, args.suppress_hints, &args.file, &src));
    }

    // ast output
    if args.ast {
        return Ok(ast.to_text());
    }

    // execution
    println!("{} {}", "Running".green().bold(), args.file.replace(".agu", ".ll"));
    let run_out = run_aguda_program(&args.file)
        .map_err(|e| fmt_errors(vec![AgudaError::from(e)]))?;

    let stdout = String::from_utf8_lossy(&run_out.stdout);
    Ok(stdout.trim().to_string())
}
