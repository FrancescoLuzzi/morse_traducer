use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::str::{self, FromStr};

use clap::{self, Parser};

/// tuple struct with two string slices with static lifetime (aka: as long as the program runs)
#[derive(Debug)]
struct Letter<'a>(&'a str, &'a str);

impl<'a> Letter<'a> {
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
    const SPACE: Self = Self(" ", "/");

    pub fn concat_morse(args: Vec<Letter<'_>>) -> String {
        let mut output = String::from("");
        if let Some(letter) = args.first() {
            let Letter(_, morse) = letter;
            output = String::from(*morse);
        }
        for letter in args.iter().skip(1) {
            let Letter(_, morse) = letter;
            output = output + " " + morse;
        }
        output
    }
}

impl FromStr for Letter<'_> {
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
            " " | "/" => Ok(Letter::SPACE),
            _ => Err(format!(
                "No representation found for the string: {}",
                s.to_string()
            )),
        }
    }
}

impl PartialEq for Letter<'_> {
    fn eq(&self, other: &Self) -> bool {
        let Self(human1, morse1) = self;
        let Self(human2, morse2) = other;
        human1 == human2 && morse1 == morse2
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
    #[clap(short, long, default_value = "-")]
    out_file: String,
}

struct MorseTraducer {
    input: Box<dyn BufRead>,
    output: Box<dyn Write>,
}

impl MorseTraducer {
    fn new(input: Box<dyn BufRead>, output: Box<dyn Write>) -> Self {
        Self { input, output }
    }
    fn copy(&mut self) -> io::Result<()> {
        let shifted_buffer = self.input.as_mut().lines().map(|line| {
            Letter::concat_morse(
                line.unwrap()
                    .bytes()
                    .map(
                        |byte| match Letter::from_str(str::from_utf8(&[byte]).unwrap()) {
                            Ok(letter) => letter,
                            Err(err) => panic!("Character not supported {:?}", err),
                        },
                    )
                    .collect::<Vec<Letter<'_>>>(),
            )
        });
        for line in shifted_buffer {
            self.output.write((line + "\n").as_bytes())?;
        }
        self.output.flush()?;
        Ok(())
    }
}
fn get_reader(arg: Option<&str>) -> Box<dyn BufRead> {
    match arg.as_deref() {
        None | Some("-") => Box::new(io::stdin().lock()),
        Some(file_name) => Box::new(BufReader::new(
            OpenOptions::new()
                .read(true)
                .create(true)
                .open(file_name)
                .unwrap(),
        )),
    }
}

fn get_writer(arg: Option<&str>) -> Box<dyn Write> {
    match arg.as_deref() {
        None | Some("-") => Box::new(io::stdout().lock()),
        Some(file_name) => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_name)
                .unwrap(),
        ),
    }
}

fn main() {
    let args = MorseArgs::parse();
    print!("{:?}", args);
    let out_writer = get_writer(Some(&args.out_file));
    let in_reader = get_reader(Some(&args.in_file));
    let mut morse_traducer = MorseTraducer::new(in_reader, out_writer);
    morse_traducer.copy().unwrap();
}
