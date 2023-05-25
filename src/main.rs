use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::str::FromStr;

use clap::{self, Parser};

#[derive(Debug)]
enum OutputType {
    Text,
    Audio,
}

impl FromStr for OutputType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "text" => Ok(OutputType::Text),
            "audio" => Ok(OutputType::Audio),
            _ => Err(format!("Type of output not found: {}", s.to_string())),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct MorseArgs {
    /// Type of traduction from human readable text to morse:
    /// -text
    /// -audio
    pub traduction_type: OutputType,

    /// Name of the file to read, if the value is "-" read from stdin
    #[clap(short, long)]
    in_file: String,

    /// Name of the file to read, if the value is "-" write to stdout
    #[clap(short, long)]
    out_file: String,
}

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
    let args = MorseArgs::parse();
    print!("{:?}", args);
    let arg = env::args().nth(1);
    let mut out_writer = get_writer(arg);
    out_writer
        .write(b"Hello World!")
        .expect("Some error while writing");
}
