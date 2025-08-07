#!/bin/sh
#
# This is the compiler driver.
#
# This driver is intended to comply with the requirements
# of the book's test script. This driver is described on
# page 7 of the book.

set -e

input_path="$1"

# From the book:
# > [The driver] must produce an executable in the same directory
# > as the input file, with the same name (minus the file extension). In other
# > words, if you run ./YOUR_COMPILER /path/to/program.c, it should produce an
# > executable at /path/to/program and terminate with an exit code of 0
input_dir="$(dirname $input_path)"
input_basename="$(basename $input_path)"
output_path="$input_dir/${input_path%.*}"

# Preprocess with gcc
gcc -E -P "$input_path" -o out.i

# This is currently a stub. Later this will invoke the actual compiler.
gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none out.i

# Assemble and link
gcc out.s -o "$output_path"
