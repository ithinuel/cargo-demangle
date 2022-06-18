# cargo-demangle

![Crates.io](https://img.shields.io/crates/v/cargo-demangle) ![Crates.io](https://img.shields.io/crates/l/cargo-demangle)

Demangles the detected symbols using ![Crates.io](https://img.shields.io/crates/d/rustc-demangle).

## Usage

First install:

`cargo install cargo-demangle`

Then:

A. Demangle from stdin  
   `cargo objdump --release -- -d | cargo demangle | bat -l asm`
B. Demangle from a file print to stdout  
   `cargo demangle somefile.asm`
C. Demangle from a file and write the result to the same file  
   `cargo demangle -i somefile.asm`
