use clap::Parser;
use morse_traducer::parser::MorseArgs;
use morse_traducer::translator::{MorseTranslator, TextMorseTranslator};

fn main() {
    let args = MorseArgs::parse();
    TextMorseTranslator::default()
        .out_file(&args.out_file)
        .in_file(&args.in_file)
        .traduction_type(args.traduction_type)
        .traduce(args.morse_command)
        .unwrap();
}
