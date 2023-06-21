use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

pub fn get_reader(arg: &str) -> Box<dyn BufRead> {
    match arg {
        "-" => Box::new(io::stdin().lock()),
        x if x.is_empty() => Box::new(io::stdin().lock()),
        file_name => Box::new(BufReader::new(
            OpenOptions::new().read(true).open(file_name).unwrap(),
        )),
    }
}

pub fn get_writer(arg: &str) -> Box<dyn Write> {
    match arg {
        "-" => Box::new(io::stdout().lock()),
        x if x.is_empty() => Box::new(io::stdout().lock()),
        file_name => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_name)
                .unwrap(),
        ),
    }
}
