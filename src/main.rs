#[macro_use]
extern crate clap;
extern crate rustc_demangle;
extern crate regex;

use std::io::{Read, Write, Cursor, stdin, stdout};
use std::fs::File;
use regex::{Regex, Captures};
use rustc_demangle::demangle;

const CARGO: &'static str = "cargo";

fn main() {
    let matches = clap_app!(CARGO =>
        (bin_name: CARGO)
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Demangles symbols in input stream")
        (@subcommand demangle =>
            (@arg filename: "Read from file")
            (@arg in_place: -i "Edit in place")
        )
    ).get_matches();
    if let Some(info) = matches.subcommand_matches("demangle") {
        let mut c_out = Cursor::new(Vec::new());
        let filename = info.value_of("filename");

        if let Some(name) = filename {
            do_demangle(&mut File::open(&name).unwrap(), &mut c_out);
        } else {
            do_demangle(&mut stdin(), &mut c_out);
        }

        if info.is_present("in_place") && filename.is_some() {
            let name = filename.unwrap();
            let mut file = File::create(name).unwrap();
            file.write_all(c_out.get_ref()).unwrap();
        } else {
            let stdout = stdout();
            // we dont care about potential broken pipes
            let _ = stdout.lock().write_all(c_out.get_ref());
        }
    }
}

fn do_demangle<I: Read, O: Write>(input: &mut I, output: &mut O) {
    let re = Regex::new(r"(?m)(?P<symbol>_ZN[0-9]+.*E)").unwrap();

    let mut txt = String::new();
    input.read_to_string(&mut txt).unwrap();

    let result = re.replace_all(&txt, |caps: &Captures| format!("{}", demangle(&caps["symbol"])));

    output.write_all(result.as_bytes()).unwrap();
}
