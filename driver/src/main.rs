use clap::Parser;

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
}
