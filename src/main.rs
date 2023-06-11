use morse_traducer::Letter;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::str::{self, FromStr};

use clap::{self, Parser};

/// tuple struct with two string slices with static lifetime (aka: as long as the program runs)
#[derive(Debug)]
enum MorseTraductionType {
    Text,
    Audio,
}

impl FromStr for MorseTraductionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "text" => Ok(MorseTraductionType::Text),
            "audio" => Ok(MorseTraductionType::Audio),
            _ => Err(format!("Type of output not found: {}", s.to_string())),
        }
    }
}

#[derive(Debug)]
enum MorseCommand {
    Encode,
    Decode,
}

impl FromStr for MorseCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "e" | "encode" => Ok(MorseCommand::Encode),
            "d" | "decode" => Ok(MorseCommand::Decode),
            _ => Err(format!("Morse command not found: {}", s.to_string())),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct MorseArgs {
    /// Morse command:
    /// -encode
    /// -decode
    pub morse_command: MorseCommand,

    /// Type of traduction from human readable text to morse:
    /// -text
    /// -audio
    pub traduction_type: MorseTraductionType,

    /// Name of the file to read, if the value is "-" read from stdin
    #[clap(short, long)]
    in_file: String,

    /// Name of the file to read, if the value is "-" write to stdout
    #[clap(short, long, default_value = "-")]
    out_file: String,
}

trait MorseTranslator<T, W> {
    fn traduce(&mut self, command: MorseCommand) -> io::Result<()>;

    fn encode(raw_data: T) -> W;

    fn decode(raw_data: T) -> W;
}

fn get_reader(arg: &str) -> Box<dyn BufRead> {
    match arg {
        "-" => Box::new(io::stdin().lock()),
        x if x.is_empty() => Box::new(io::stdin().lock()),
        file_name => Box::new(BufReader::new(
            OpenOptions::new().read(true).open(file_name).unwrap(),
        )),
    }
}

fn get_writer(arg: &str) -> Box<dyn Write> {
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

struct TextMorseTranslator {
    // idea, create struct AudioMorseTranslation for audio implementation
    // create struct OptionMorseTranslation with functions:
    // - in_file(Option<&str>)  -> using get_reader
    // - out_file(Option<&str>) -> using get_writer
    // - traduction_type(MorseTraductionType)
    // - traduction_options(MorseCommand)
    // this patter will create and use a TextMorseTranslator
    // or an AudioMorseTranslation trasparently
    input_filename: String,
    output_filename: String,
    traduction_type: MorseTraductionType,
}

impl<'a> MorseTranslator<String, Vec<Letter<'a>>> for TextMorseTranslator {
    fn traduce(&mut self, command: MorseCommand) -> io::Result<()> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let traduce_cmd = match command {
            MorseCommand::Encode => Letter::concat_morse,
            MorseCommand::Decode => Letter::concat_morse,
        };

        let traduced_lines = get_reader(&self.input_filename)
            .lines()
            .map(|line| read_cmd(line.unwrap()));

        let mut output = get_writer(&self.output_filename);
        for line in traduced_lines.map(traduce_cmd) {
            output.write(&line)?;
        }
        output.flush()
    }

    fn encode(line: String) -> Vec<Letter<'a>> {
        line.bytes()
            .map(
                |byte| match Letter::from_str(str::from_utf8(&[byte]).unwrap()) {
                    Ok(letter) => letter,
                    Err(err) => panic!("Character not supported {:?}", err),
                },
            )
            .collect::<Vec<Letter<'_>>>()
    }

    fn decode(line: String) -> Vec<Letter<'a>> {
        line.split_whitespace()
            .map(|morse_letter| match Letter::from_str(morse_letter) {
                Ok(letter) => letter,
                Err(err) => panic!("Character not supported {:?}", err),
            })
            .collect::<Vec<Letter<'_>>>()
    }
}

fn make_bytes<T>(number: T) -> Vec<u8>
where
    T: Into<u64>,
{
    let number: u64 = number.into();
    let mut b: Vec<u8> = Vec::new();
    for i in 0..std::mem::size_of::<T>() {
        b.push(((number >> (8 * i)) & 0xff) as u8);
    }
    b
}

impl TextMorseTranslator {
    fn new() -> Self {
        TextMorseTranslator {
            input_filename: "".to_string(),
            output_filename: "".to_string(),
            traduction_type: MorseTraductionType::Text,
        }
    }

    fn in_file(&mut self, input_filename: &str) -> &mut Self {
        self.input_filename = input_filename.to_owned();
        self
    }

    fn out_file(&mut self, output_filename: &str) -> &mut Self {
        self.output_filename = output_filename.to_owned();
        self
    }
}

fn main() {
    let args = MorseArgs::parse();
    TextMorseTranslator::new()
        .out_file(&args.out_file)
        .in_file(&args.in_file)
        .traduce(args.morse_command)
        .unwrap();
}
