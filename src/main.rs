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
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    use std::str::from_utf8;

    fn translate_out(
        translator: &mut StreamedMorseTranslator,
        input: Vec<String>,
        translate_cmd: MorseCommand,
    ) {
        translator
            .in_stream(input)
            // default to MorseTraductionType::Text
            .translate(translate_cmd)
            .unwrap();
    }

    let out: Rc<RefCell<Box<Vec<u8>>>> = Rc::new(RefCell::new(Box::default()));
    let input = vec!["Hello World".into()];
    let mut translator = StreamedMorseTranslator::default();
    translator.out_stream(out.clone());
    translate_out(&mut translator, input, MorseCommand::Encode);
    //launch with cargo test -- --nocapture
    print!("{:?}", from_utf8(&out.deref().borrow()));
}
