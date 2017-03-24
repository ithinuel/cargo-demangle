#[macro_use]
extern crate clap;
extern crate rustc_demangle;
extern crate regex;

use std::io::{Read, Write};
use std::fs::File;
use regex::{Regex, Captures};
use rustc_demangle::demangle;

const CARGO: &'static str = "cargo";

fn main() {
    let matches = clap_app!(CARGO =>
        (bin_name: CARGO)
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Edits in place the given file demangling symbols")
        (@subcommand demangle =>
            (@arg filename: +required "Sets the input file to use")
        )
    ).get_matches();
    if let Some(info) = matches.subcommand_matches("demangle") {
        if let Some(filename) = info.value_of("filename") {
            do_demangle(filename.to_string());
        }
    }
}

fn do_demangle(filename: String) {
    let re = Regex::new(r"(?m)(?P<symbol>(_ZN[0-9]+.*E))").unwrap();

    let mut txt = String::new();
    let mut file = File::open(&filename).unwrap();
    file.read_to_string(&mut txt).unwrap();
    drop(file);

    let result = re.replace_all(&txt, |caps: &Captures| format!("{}", demangle(&caps["symbol"])));

    let mut file = File::create(&filename).unwrap();
    file.write_all(result.as_bytes()).unwrap();
    drop(file);
}
