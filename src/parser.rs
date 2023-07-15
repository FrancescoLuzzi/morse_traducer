use std::str::{self, FromStr};

use clap::{self, Parser};

/// tuple struct with two string slices with static lifetime (aka: as long as the program runs)
#[derive(Debug, Clone)]
pub enum MorseTraductionType {
    Text,
    Audio,
}

impl FromStr for MorseTraductionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "text" => Ok(MorseTraductionType::Text),
            "audio" => Ok(MorseTraductionType::Audio),
            _ => Err(format!("Type of output not found: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MorseCommand {
    Encode,
    Decode,
}

impl FromStr for MorseCommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "e" | "encode" => Ok(MorseCommand::Encode),
            "d" | "decode" => Ok(MorseCommand::Decode),
            _ => Err(format!("Morse command not found: {}", s)),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MorseArgs {
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
    pub in_file: String,

    /// Name of the file to read, if the value is "-" write to stdout
    #[clap(short, long, default_value = "-")]
    pub out_file: String,
}
