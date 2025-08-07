# writing-a-c-compiler

This repo contains the code for the C compiler project for the
[_Writing a C Compiler_](https://norasandler.com/book/) book by Nora Sandler.

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
