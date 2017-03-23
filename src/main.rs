#[macro_use]
extern crate clap;
extern crate rustc_demangle;
extern crate regex;

use std::env;
use std::io::{Read, Write};
use std::fs::File;
use regex::Regex;
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
    let mut symbols = std::collections::BTreeMap::new();

    let re = Regex::new(r"(?m)(?P<symbol>(_ZN[0-9]+.*E))").unwrap();

    let mut txt = String::new();
    let mut file = File::open(&filename).unwrap();
    file.read_to_string(&mut txt).unwrap();
    drop(file);

    for cap in re.captures_iter(&txt) {
        let symbol = &cap["symbol"];
        if !symbols.contains_key(symbol) {
            symbols.insert(symbol.to_string(), format!("{}", demangle(symbol)));
        }
    }

    for (mangled, demangled) in symbols.iter() {
        txt = txt.replace(mangled, demangled);
    }

    let mut file = File::create(&filename).unwrap();
    file.write_all(txt.as_bytes()).unwrap();
    drop(file);
}
