use clap::{clap_app, crate_authors, crate_version};
use regex::{Captures, Regex};
use rustc_demangle::demangle;
use std::fs::File;
use std::io::{stdin, stdout, Cursor, Read, Write};

const CARGO: &str = "cargo";

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
    )
    .get_matches();
    if let Some(info) = matches.subcommand_matches("demangle") {
        let mut c_out = Cursor::new(Vec::new());
        let filename = info.value_of("filename");

        if let Some(name) = filename {
            do_demangle(&mut File::open(&name).unwrap(), &mut c_out);
        } else {
            do_demangle(&mut stdin(), &mut c_out);
        }

        if let (Some(name), true) = (filename, info.is_present("in_place")) {
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

    let result = re.replace_all(&txt, |caps: &Captures| {
        format!("{}", demangle(&caps["symbol"]))
    });

    output.write_all(result.as_bytes()).unwrap();
}
