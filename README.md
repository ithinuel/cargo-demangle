# cargo-demangle

![Crates.io](https://img.shields.io/crates/v/cargo-demangle) ![Crates.io](https://img.shields.io/crates/l/cargo-demangle)

Demangles the detected symbols using [rustc-demangle](https://crates.io/crates/rustc-demangle).

## Usage

First install:

`cargo install cargo-demangle`

Then:

* Demangle from stdin  
  `cargo objdump --release -- -d | cargo demangle | bat -l asm`
* Demangle from a file print to stdout  
  `cargo demangle somefile.asm`
* Demangle from a file and write the result to the same file  
  `cargo demangle -i somefile.asm`

