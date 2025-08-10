# writing-a-c-compiler

This repo contains the code for the C compiler project for the
[_Writing a C Compiler_](https://norasandler.com/book/) book by Nora Sandler.

## Cloning

You can clone this without the book's tests with
```bash
git clone git@github.com:shanebishop/writing-a-c-compiler.git
```
Or, with an appropriately modern version of Git, you can also
clone the book's tests with
```bash
git clone --recurse-submodules git@github.com:shanebishop/writing-a-c-compiler.git
```
To pull submodules after an intial clone, run
```bash
git submodule update --init --recursive
```

## Run tests

To run unit tests, run `cargo test`.

To run the book's tests, run `./run_book_tests.sh`.

To run fuzzing, run `cargo +nightly fuzz run fuzz-tests -- -max_total_time=2m`.
Adjust the fuzzing total time as desired.

## Code organization

The code has been organized into the following crates:
* `driver` - the binary crate that has the compiler driver
* `lexer` - the lexer library
* `parser` - the parser library

This code organization has the following benefits:
* The different parts can be reused as libraries in an application
  that wants to perform compilation or parts of compilation without
  invoking a child process.
* Faster incremental build times. (Each Rust crate is compiled as a
  separate translation unit. This means if a change is internal to
  a particular crate, only that crate needs to be rebuilt. This has
  the consequence that some optimizations cannot be done at the
  compilation stage (but could be done at the link stage), but I
  favour the tradeoff for compile time, as this is a project for
  learning and not for production use.)
