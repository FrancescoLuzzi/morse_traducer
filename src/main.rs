use clap::Parser;
use morse_traducer::parser::MorseArgs;
use morse_traducer::translator::{MorseTranslator, StreamedMorseTranslator};

fn main() {
    let args = MorseArgs::parse();
    StreamedMorseTranslator::default()
        .out_file(&args.out_file)
        .in_file(&args.in_file)
        .traduction_type(args.traduction_type)
        .translate(args.morse_command)
        .unwrap();
}

#[test]
fn test_main() {
    use morse_traducer::parser::MorseCommand;
    use std::ops::DerefMut;
    use std::str::from_utf8;

    let mut out: Box<Vec<u8>> = Box::default();
    let input = vec!["Hello World".into()];
    StreamedMorseTranslator::default()
        .out_stream(out.deref_mut())
        .in_stream(input)
        // default to MorseTraductionType::Text
        .translate(MorseCommand::Encode)
        .unwrap();
    //launch with cargo test -- --nocapture
    print!("{:?}", from_utf8(&out));
}
