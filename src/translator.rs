use crate::parser::{MorseCommand, MorseTraductionType};
use crate::polyphonia::SAMPLE_RATE;
use crate::utils::{get_reader, get_writer};
use crate::wav::write_wav;
use crate::Letter;
use std::default::Default;
use std::io::{self, BufRead, Write};
use std::str::{self, FromStr};

pub trait MorseTranslator<T, W> {
    fn traduce(&mut self, command: MorseCommand) -> io::Result<()>;

    fn traduce_to_text(&mut self, command: MorseCommand) -> io::Result<()>;

    fn traduce_to_audio(&mut self, command: MorseCommand) -> io::Result<()>;

    fn encode(raw_data: T) -> W;

    fn decode(raw_data: T) -> W;
}

pub struct TextMorseTranslator {
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
        match self.traduction_type {
            MorseTraductionType::Text => self.traduce_to_text(command),
            MorseTraductionType::Audio => self.traduce_to_audio(command),
        }
    }

    fn traduce_to_audio(&mut self, command: MorseCommand) -> io::Result<()> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let traduced_lines = get_reader(&self.input_filename)
            .lines()
            .flat_map(|line| read_cmd(line.unwrap()));
        let mut output = get_writer(&self.output_filename);
        write_wav(
            Letter::concat_audio(traduced_lines),
            SAMPLE_RATE,
            &mut output,
        )?;
        output.flush()
    }

    fn traduce_to_text(&mut self, command: MorseCommand) -> io::Result<()> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let traduce_cmd = match command {
            MorseCommand::Encode => Letter::concat_morse,
            MorseCommand::Decode => Letter::concat_text,
        };

        let traduced_lines = get_reader(&self.input_filename)
            .lines()
            .map(|line| read_cmd(line.unwrap()));

        let mut output = get_writer(&self.output_filename);
        for line in traduced_lines.map(traduce_cmd) {
            output.write_all(&line)?;
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

impl Default for TextMorseTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl TextMorseTranslator {
    pub fn new() -> Self {
        TextMorseTranslator {
            input_filename: "".to_string(),
            output_filename: "".to_string(),
            traduction_type: MorseTraductionType::Text,
        }
    }

    pub fn in_file(&mut self, input_filename: &str) -> &mut Self {
        self.input_filename = input_filename.to_owned();
        self
    }

    pub fn out_file(&mut self, output_filename: &str) -> &mut Self {
        self.output_filename = output_filename.to_owned();
        self
    }

    pub fn traduction_type(&mut self, traduction_type: MorseTraductionType) -> &mut Self {
        self.traduction_type = traduction_type;
        self
    }
}
