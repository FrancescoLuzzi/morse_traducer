use crate::parser::{MorseCommand, MorseTraductionType};
use crate::polyphonia::SAMPLE_RATE;
use crate::utils::{get_reader, get_writer};
use crate::wav::write_wav;
use crate::Letter;
use std::cell::RefCell;
use std::default::Default;
use std::error::Error;
use std::io::{BufRead, Write};
use std::ops::DerefMut;
use std::rc::Rc;
use std::str::{self, FromStr};

pub trait MorseTranslator<T, W, R> {
    fn translate(&mut self, command: MorseCommand) -> Result<R, Box<dyn Error>>;

    fn translate_to_text(&mut self, command: MorseCommand) -> Result<R, Box<dyn Error>>;

    fn translate_to_audio(&mut self, command: MorseCommand) -> Result<R, Box<dyn Error>>;

    fn encode(raw_data: T) -> W;

    fn decode(raw_data: T) -> W;
}

pub struct StreamedMorseTranslator<'a> {
    // idea, create struct AudioMorseTranslation for audio implementation
    // create struct OptionMorseTranslation with functions:
    // - in_file(Option<&str>)  -> using get_reader
    // - out_file(Option<&str>) -> using get_writer
    // - traduction_type(MorseTraductionType)
    // - traduction_options(MorseCommand)
    // this patter will create and use a StreamedMorseTranslator
    // or an AudioMorseTranslation trasparently
    input_stream: Option<Vec<String>>,
    pub output_stream: Option<Rc<RefCell<dyn Write + 'a>>>,
    pub traduction_type: MorseTraductionType,
}

impl<'l> MorseTranslator<&str, Vec<Letter<'l>>, ()> for StreamedMorseTranslator<'_> {
    fn translate(&mut self, command: MorseCommand) -> Result<(), Box<dyn Error>> {
        match self.traduction_type {
            MorseTraductionType::Text => self.translate_to_text(command),
            MorseTraductionType::Audio => self.translate_to_audio(command),
        }
    }

    fn translate_to_audio(&mut self, command: MorseCommand) -> Result<(), Box<dyn Error>> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let translated_lines = self
            .input_stream
            .as_ref()
            .expect("Input stream not initialized, failing.")
            .iter()
            .flat_map(|line| read_cmd(line));
        let mut output = self
            .output_stream
            .as_ref()
            .expect("Output stream not inizialized, failing.")
            .borrow_mut();
        write_wav(
            Letter::concat_audio(translated_lines),
            SAMPLE_RATE,
            output.deref_mut(),
        )?;
        output.flush()?;
        Ok(())
    }

    fn translate_to_text(&mut self, command: MorseCommand) -> Result<(), Box<dyn Error>> {
        let read_cmd = match command {
            MorseCommand::Encode => Self::encode,
            MorseCommand::Decode => Self::decode,
        };

        let translate_cmd = match command {
            MorseCommand::Encode => Letter::concat_morse,
            MorseCommand::Decode => Letter::concat_text,
        };

        let translated_lines = self
            .input_stream
            .as_ref()
            .expect("Input stream not initialized, failing.")
            .iter()
            .map(|line| read_cmd(line));

        let mut output = self
            .output_stream
            .as_ref()
            .expect("Output stream not inizialized, failing.")
            .borrow_mut();
        let last_index = translated_lines.len() - 1;
        for (i, line) in translated_lines.map(translate_cmd).enumerate() {
            output.write_all(&line)?;
            if i != last_index {
                output.write_all(b"\n")?;
            }
        }
        output.flush()?;
        Ok(())
    }

    fn encode(line: &str) -> Vec<Letter<'l>> {
        line.bytes()
            .map(
                |byte| match Letter::from_str(str::from_utf8(&[byte]).unwrap()) {
                    Ok(letter) => letter,
                    Err(err) => panic!("Character not supported {:?}", err),
                },
            )
            .collect::<Vec<Letter<'_>>>()
    }

    fn decode(line: &str) -> Vec<Letter<'l>> {
        line.split_whitespace()
            .map(|morse_letter| match Letter::from_str(morse_letter) {
                Ok(letter) => letter,
                Err(err) => panic!("Character not supported {:?}", err),
            })
            .collect::<Vec<Letter<'_>>>()
    }
}

impl Default for StreamedMorseTranslator<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> StreamedMorseTranslator<'a> {
    pub fn new() -> Self {
        StreamedMorseTranslator {
            input_stream: None,
            output_stream: None,
            traduction_type: MorseTraductionType::Text,
        }
    }

    pub fn in_stream(&mut self, input_stream: Vec<String>) -> &mut Self {
        self.input_stream = Some(input_stream);
        self
    }

    pub fn in_file(&mut self, input_filename: &str) -> &mut Self {
        self.input_stream = Some(
            get_reader(input_filename)
                .lines()
                .flatten()
                .collect::<Vec<String>>(),
        );
        self
    }

    pub fn out_stream(&mut self, out_stream: Rc<RefCell<dyn Write>>) -> &mut Self {
        self.output_stream = Some(out_stream);
        self
    }

    pub fn out_file(&mut self, output_filename: &str) -> &mut Self {
        self.output_stream = Some(Rc::new(RefCell::new(get_writer(output_filename))));
        self
    }

    pub fn traduction_type(&mut self, traduction_type: MorseTraductionType) -> &mut Self {
        self.traduction_type = traduction_type;
        self
    }
}
