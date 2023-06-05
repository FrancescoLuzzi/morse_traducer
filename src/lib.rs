use std::str::FromStr;
#[derive(Debug)]
pub struct Letter<'a>(&'a str, &'a str);

impl<'a> Letter<'a> {
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

    pub fn concat_text(args: Vec<Letter<'_>>) -> String {
        let mut output = String::from("");
        if let Some(letter) = args.first() {
            let Letter(text, _) = letter;
            output = String::from(*text);
        }
        for letter in args.iter().skip(1) {
            let Letter(text, _) = letter;
            output = output + text;
        }
        output
    }
}

pub mod morse_alphabet {
    use crate::Letter;

    pub(crate) const A: Letter = Letter("a", ".-");
    pub(crate) const B: Letter = Letter("b", "-...");
    pub(crate) const C: Letter = Letter("c", "-.-.");
    pub(crate) const D: Letter = Letter("d", "-..");
    pub(crate) const E: Letter = Letter("e", ".");
    pub(crate) const F: Letter = Letter("f", "..-.");
    pub(crate) const G: Letter = Letter("g", "--.");
    pub(crate) const H: Letter = Letter("h", "....");
    pub(crate) const I: Letter = Letter("i", "..");
    pub(crate) const J: Letter = Letter("j", ".---");
    pub(crate) const K: Letter = Letter("k", "-.-");
    pub(crate) const L: Letter = Letter("l", ".-..");
    pub(crate) const M: Letter = Letter("m", "--");
    pub(crate) const N: Letter = Letter("n", "-.");
    pub(crate) const O: Letter = Letter("o", "---");
    pub(crate) const P: Letter = Letter("p", ".--.");
    pub(crate) const Q: Letter = Letter("q", "--.-");
    pub(crate) const R: Letter = Letter("r", ".-.");
    pub(crate) const S: Letter = Letter("s", "...");
    pub(crate) const T: Letter = Letter("t", "-");
    pub(crate) const U: Letter = Letter("u", "..-");
    pub(crate) const V: Letter = Letter("v", "...-");
    pub(crate) const W: Letter = Letter("w", ".--");
    pub(crate) const X: Letter = Letter("x", "-..-");
    pub(crate) const Y: Letter = Letter("y", "-.--");
    pub(crate) const Z: Letter = Letter("z", "--..");
    pub(crate) const ONE: Letter = Letter("1", ".----");
    pub(crate) const TWO: Letter = Letter("2", "..---");
    pub(crate) const THREE: Letter = Letter("3", "...--");
    pub(crate) const FOUR: Letter = Letter("4", "....-");
    pub(crate) const FIVE: Letter = Letter("5", ".....");
    pub(crate) const SIX: Letter = Letter("6", "-....");
    pub(crate) const SEVEN: Letter = Letter("7", "--...");
    pub(crate) const EIGHT: Letter = Letter("8", "---..");
    pub(crate) const NINE: Letter = Letter("9", "----.");
    pub(crate) const ZERO: Letter = Letter("0", "-----");
    pub(crate) const SPACE: Letter = Letter(" ", "/");
}

impl FromStr for Letter<'_> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a" | ".-" => Ok(morse_alphabet::A),
            "b" | "-..." => Ok(morse_alphabet::B),
            "c" | "-.-." => Ok(morse_alphabet::C),
            "d" | "-.." => Ok(morse_alphabet::D),
            "e" | "." => Ok(morse_alphabet::E),
            "f" | "..-." => Ok(morse_alphabet::F),
            "g" | "--." => Ok(morse_alphabet::G),
            "h" | "...." => Ok(morse_alphabet::H),
            "i" | ".." => Ok(morse_alphabet::I),
            "j" | ".---" => Ok(morse_alphabet::J),
            "k" | "-.-" => Ok(morse_alphabet::K),
            "l" | ".-.." => Ok(morse_alphabet::L),
            "m" | "--" => Ok(morse_alphabet::M),
            "n" | "-." => Ok(morse_alphabet::N),
            "o" | "---" => Ok(morse_alphabet::O),
            "p" | ".--." => Ok(morse_alphabet::P),
            "q" | "--.-" => Ok(morse_alphabet::Q),
            "r" | ".-." => Ok(morse_alphabet::R),
            "s" | "..." => Ok(morse_alphabet::S),
            "t" | "-" => Ok(morse_alphabet::T),
            "u" | "..-" => Ok(morse_alphabet::U),
            "v" | "...-" => Ok(morse_alphabet::V),
            "w" | ".--" => Ok(morse_alphabet::W),
            "x" | "-..-" => Ok(morse_alphabet::X),
            "y" | "-.--" => Ok(morse_alphabet::Y),
            "z" | "--.." => Ok(morse_alphabet::Z),
            "1" | ".----" => Ok(morse_alphabet::ONE),
            "2" | "..---" => Ok(morse_alphabet::TWO),
            "3" | "...--" => Ok(morse_alphabet::THREE),
            "4" | "....-" => Ok(morse_alphabet::FOUR),
            "5" | "....." => Ok(morse_alphabet::FIVE),
            "6" | "-...." => Ok(morse_alphabet::SIX),
            "7" | "--..." => Ok(morse_alphabet::SEVEN),
            "8" | "---.." => Ok(morse_alphabet::EIGHT),
            "9" | "----." => Ok(morse_alphabet::NINE),
            "0" | "-----" => Ok(morse_alphabet::ZERO),
            " " | "/" => Ok(morse_alphabet::SPACE),
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
