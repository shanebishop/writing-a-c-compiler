use clap::Parser;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::{Command, exit};

use errors::DriverError;

/// Clap program arguments
#[derive(Parser, Debug, Default)]
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
    let res = driver(args);

    if let Err(e) = res {
        eprintln!("{}", e.msg);
        exit(e.exit_code);
    }
}

fn driver<'a>(driver_args: Args) -> Result<(), DriverError<'a>> {
    let source_path = Path::new(&driver_args.source_path);
    if !source_path.is_file() {
        return Err(DriverError {
            exit_code: 1,
            msg: Cow::Owned(format!(
                "fatal: \"{}\" is not a file.",
                driver_args.source_path
            )),
        });
    }

    // From the book:
    // > [The driver] must produce an executable in the same directory
    // > as the input file, with the same name (minus the file extension). In other
    // > words, if you run ./YOUR_COMPILER /path/to/program.c, it should produce an
    // > executable at /path/to/program and terminate with an exit code of 0
    let input_dir = source_path.parent().unwrap_or(Path::new("/"));
    // Unwrap is safe, due to is_file check above
    let input_basename_stem = source_path.file_stem().map(Path::new).unwrap();
    let output_path = input_dir.join(input_basename_stem);
    let output_path = output_path.as_os_str();

    let source_path = source_path.as_os_str();

    println!("Preprocessing...");
    let mut preprocessed_path = OsString::from(output_path);
    preprocessed_path.push(".i");
    let args = [
        OsStr::new("-E"),
        OsStr::new("-P"),
        source_path,
        OsStr::new("-o"),
        &preprocessed_path,
    ];
    let res = run_gcc(&args);
    if let Err(e) = res {
        return Err(DriverError {
            msg: Cow::Owned(format!("Failed to run gcc preprocessing: {}.", e.msg)),
            ..e
        });
    }

    // TODO Remove this stubbing
    println!("Compiling...");
    let mut assembly_path = OsString::from(output_path);
    assembly_path.push(".s");
    let args = [OsStr::new("-S"), OsStr::new("-O"), &preprocessed_path, OsStr::new("-o"), &assembly_path];
    let res = run_gcc(&args);
    if let Err(e) = res {
        return Err(DriverError {
            msg: Cow::Owned(format!("Failed to compile: {}.", e.msg)),
            ..e
        });
    }

    // Lexing will go here

    if driver_args.lex {
        return Ok(());
    }

    // Parsing will go here

    if driver_args.parse {
        return Ok(());
    }

    // Codegen will go here

    if driver_args.codegen {
        return Ok(());
    }

    // Assemble and link
    println!("Assembling and linking...");
    let res = run_gcc(&[
        &assembly_path,
        OsStr::new("-o"),
        &output_path,
    ]);
    if let Err(e) = res {
        return Err(DriverError {
            msg: Cow::Owned(format!("Failed to assemble and link: {}.", e.msg)),
            ..e
        });
    }

    Ok(())
}

fn run_gcc<'a, I, S>(args: I) -> Result<(), DriverError<'a>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    // Uncomment to debug run_gcc command
    // let args = dbg!(args.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>());

    let status = match Command::new("gcc").args(args).status() {
        Ok(status) => status,
        Err(e) => {
            return Err(DriverError {
                exit_code: 1,
                msg: Cow::Owned(format!("{e}")),
            });
        }
    };

    if !status.success() {
        // On Unix, status.code() returns None if the process was killed
        // by a signal. If the process was killed by a signal, we still
        // want to terminate.
        return Err(DriverError {
            exit_code: status.code().unwrap_or(1),
            msg: if let Some(code) = status.code() {
                Cow::Owned(format!("gcc terminated with exit code {code}"))
            } else {
                Cow::Borrowed("gcc killed by some signal")
            },
        });
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const BASIC_MAIN: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/../test_c_source/basic_main.c");

    #[test]
    fn test_run_gcc() {
        run_gcc(&["-E", "-P", BASIC_MAIN, "-o", "/dev/null"]).unwrap();

        let err = run_gcc(&["-E", "-P", "invalid_path.c", "-o", "/dev/null"]).unwrap_err();
        assert_eq!(
            err,
            DriverError {
                exit_code: 1,
                msg: Cow::Borrowed("gcc terminated with exit code 1")
            }
        );
    }

    #[test]
    fn test_driver_happy_paths() {
        let args = Args {
            source_path: BASIC_MAIN.to_string(),
            ..Default::default()
        };
        driver(args).unwrap();

        let args = Args {
            source_path: BASIC_MAIN.to_string(),
            lex: true,
            ..Default::default()
        };
        driver(args).unwrap();

        let args = Args {
            source_path: BASIC_MAIN.to_string(),
            parse: true,
            ..Default::default()
        };
        driver(args).unwrap();

        let args = Args {
            source_path: BASIC_MAIN.to_string(),
            codegen: true,
            ..Default::default()
        };
        driver(args).unwrap();
    }

    #[test]
    fn test_invalid_preprocessor_token() {
        let args = Args {
            source_path: concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../test_c_source/invalid_preprocessor_token.c"
            )
            .to_string(),
            ..Default::default()
        };
        let err = driver(args).unwrap_err();
        assert_eq!(
            err,
            DriverError {
                exit_code: 1,
                msg: Cow::Borrowed(
                    "Failed to run gcc preprocessing: gcc terminated with exit code 1."
                )
            }
        );
    }

    #[test]
    fn test_invalid_source_token() {
        let args = Args {
            source_path: concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../test_c_source/invalid_source_token.c"
            )
            .to_string(),
            ..Default::default()
        };
        let err = driver(args).unwrap_err();
        assert_eq!(
            err,
            DriverError {
                exit_code: 1,
                msg: Cow::Borrowed("Failed to parse. See errors above.")
            }
        );
    }

    #[test]
    fn test_undefined_symbol_token() {
        let source_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../test_c_source/undefined_symbol.c"
        )
        .to_string();
        let args = Args {
            source_path: source_path.clone(),
            ..Default::default()
        };
        let err = driver(args).unwrap_err();
        assert_eq!(
            err,
            DriverError {
                exit_code: 1,
                msg: Cow::Owned(format!("fatal: \"{source_path}\" is not a file."))
            }
        );
    }
}
