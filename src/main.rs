use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn get_writer(arg: Option<String>) -> Box<dyn Write> {
    match arg.as_deref() {
        None | Some("-") => Box::new(io::stdout().lock()),
        Some(string) => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(string)
                .unwrap(),
        ),
    }
}

fn main() {
    let arg = env::args().nth(1);
    let mut out_writer = get_writer(arg);
    out_writer
        .write(b"Hello World!")
        .expect("Some error while writing");
}
