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

#[test]
fn test_main() {
    use morse_traducer::parser::MorseCommand;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::str::from_utf8;

    let out = Rc::new(RefCell::new(Vec::new()));
    let input = vec!["Hello World".into()];
    TextMorseTranslator::default()
        .out_stream(out.clone())
        .in_stream(input)
        // default to MorseTraductionType::Text
        .traduce(MorseCommand::Encode)
        .unwrap();
    //launch with cargo test -- --nocapture
    print!("{:?}", from_utf8(out.borrow().deref()));
}
