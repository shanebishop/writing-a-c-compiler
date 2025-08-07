use clap::Parser;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::{Command, exit};

/// Clap program arguments
#[derive(Parser, Debug)]
#[command(about = "A C compiler", long_about = None)]
struct Args {
    /// Path to C source file to compile
    source_path: String,

    /// Run the lexer, but stop before parsing
    #[arg(short, long)]
    lex: bool,

    /// Run the lexer and parser, but stop before
    /// assembly generation
    #[arg(short, long)]
    parse: bool,

    /// Perform lexing, parsing, and assembly
    /// generation, but stop before code emission
    #[arg(short, long)]
    codegen: bool,
}

fn main() {
    let args = Args::parse();
    driver(args);
}

fn driver(args: Args) {
    let source_path = Path::new(&args.source_path);
    if !source_path.is_file() {
        eprintln!("fatal: \"{}\" is not a file.", args.source_path);
        exit(1);
    }

    // From the book:
    // > [The driver] must produce an executable in the same directory
    // > as the input file, with the same name (minus the file extension). In other
    // > words, if you run ./YOUR_COMPILER /path/to/program.c, it should produce an
    // > executable at /path/to/program and terminate with an exit code of 0
    let input_dir = source_path.parent().unwrap_or(Path::new("/"));
    // Unwrap is safe, due to is_file check above
    let input_basename = source_path.file_stem().unwrap();

    let preprocessed_path = "out.i";

    // Preprocess with gcc
    let args = ["-E", "-P", &args.source_path, "-o", preprocessed_path];
    run_gcc(&args, "Failed to run gcc preprocessing");

    // TODO Write unit tests for driver, like testing for a non-zero exit code if preprocessing fails (like in #inc)
    // TODO Add logic for conditional parsing and codegen
}

fn run_gcc(args: &[&str], err_msg_prefix: &str) {
    let status = Command::new("gcc").args(args).status().unwrap_or_else(|e| {
        eprintln!("{err_msg_prefix}: {e}");
        exit(1);
    });

    if !status.success() {
        // On Unix, status.code() returns None if the process was killed
        // by a signal. If the process was killed by a signal, we still
        // want to terminate.
        exit(status.code().unwrap_or(1));
    }
}
