use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::str::FromStr;

use clap::{self, Parser};

/// tuple struct with two string slices with static lifetime (aka: as long as the program runs)
struct Letter(&'static str, &'static str);

impl Letter {
    const A: Self = Self("a", ".-");
    const B: Self = Self("b", "-...");
    const C: Self = Self("c", "-.-.");
    const D: Self = Self("d", "-..");
    const E: Self = Self("e", ".");
    const F: Self = Self("f", "..-.");
    const G: Self = Self("g", "--.");
    const H: Self = Self("h", "....");
    const I: Self = Self("i", "..");
    const J: Self = Self("j", ".---");
    const K: Self = Self("k", "-.-");
    const L: Self = Self("l", ".-..");
    const M: Self = Self("m", "--");
    const N: Self = Self("n", "-.");
    const O: Self = Self("o", "---");
    const P: Self = Self("p", ".--.");
    const Q: Self = Self("q", "--.-");
    const R: Self = Self("r", ".-.");
    const S: Self = Self("s", "...");
    const T: Self = Self("t", "-");
    const U: Self = Self("u", "..-");
    const V: Self = Self("v", "...-");
    const W: Self = Self("w", ".--");
    const X: Self = Self("x", "-..-");
    const Y: Self = Self("y", "-.--");
    const Z: Self = Self("z", "--..");
    const ONE: Self = Self("1", ".----");
    const TWO: Self = Self("2", "..---");
    const THREE: Self = Self("3", "...--");
    const FOUR: Self = Self("4", "....-");
    const FIVE: Self = Self("5", ".....");
    const SIX: Self = Self("6", "-....");
    const SEVEN: Self = Self("7", "--...");
    const EIGHT: Self = Self("8", "---..");
    const NINE: Self = Self("9", "----.");
    const ZERO: Self = Self("0", "-----");
    const SPACE: Self = Self(" ", " ");
}

impl FromStr for Letter {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a" | ".-" => Ok(Letter::A),
            "b" | "-..." => Ok(Letter::B),
            "c" | "-.-." => Ok(Letter::C),
            "d" | "-.." => Ok(Letter::D),
            "e" | "." => Ok(Letter::E),
            "f" | "..-." => Ok(Letter::F),
            "g" | "--." => Ok(Letter::G),
            "h" | "...." => Ok(Letter::H),
            "i" | ".." => Ok(Letter::I),
            "j" | ".---" => Ok(Letter::J),
            "k" | "-.-" => Ok(Letter::K),
            "l" | ".-.." => Ok(Letter::L),
            "m" | "--" => Ok(Letter::M),
            "n" | "-." => Ok(Letter::N),
            "o" | "---" => Ok(Letter::O),
            "p" | ".--." => Ok(Letter::P),
            "q" | "--.-" => Ok(Letter::Q),
            "r" | ".-." => Ok(Letter::R),
            "s" | "..." => Ok(Letter::S),
            "t" | "-" => Ok(Letter::T),
            "u" | "..-" => Ok(Letter::U),
            "v" | "...-" => Ok(Letter::V),
            "w" | ".--" => Ok(Letter::W),
            "x" | "-..-" => Ok(Letter::X),
            "y" | "-.--" => Ok(Letter::Y),
            "z" | "--.." => Ok(Letter::Z),
            "1" | ".----" => Ok(Letter::ONE),
            "2" | "..---" => Ok(Letter::TWO),
            "3" | "...--" => Ok(Letter::THREE),
            "4" | "....-" => Ok(Letter::FOUR),
            "5" | "....." => Ok(Letter::FIVE),
            "6" | "-...." => Ok(Letter::SIX),
            "7" | "--..." => Ok(Letter::SEVEN),
            "8" | "---.." => Ok(Letter::EIGHT),
            "9" | "----." => Ok(Letter::NINE),
            "0" | "-----" => Ok(Letter::ZERO),
            " " => Ok(Letter::SPACE),
            _ => Err(format!(
                "No representation found for the string: {}",
                s.to_string()
            )),
        }
    }
}

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
