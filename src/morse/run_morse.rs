use morse::morse_control::MorseControl;
use morse::char_to_symbol::to_morse_symbol;
use std::io;
use std::io::Write;

const WHITESPACE: &'static [char] = &[' ', '\n', '—', '-', '’', '(', ')', '…', '.', '?', '“', '”', ','];

pub fn run_morse(mut morse_control: MorseControl, text: &str) -> String {
    let mut t = String::new();

    for c in text.chars() {
        print!("{}", c);
        let _ = io::stdout().flush();

        if WHITESPACE.contains(&c) {
            t += morse_control.word_space();
        } else {
            for symbol in to_morse_symbol(c).chars() {
                match symbol {
                    '−' => t += morse_control.dash(),
                    '·' => t += morse_control.dot(),
                    _ => panic!("Invalid symbol: {}", symbol)
                };
                t += morse_control.symbol_space();
            }
            t += morse_control.letter_space();
        }
    }

    print!("\n");
    t.trim_right_matches(".").to_string()
}

#[cfg(test)]
mod tests {
    extern crate config;
    use common::control::Controller;
    use morse::morse_control::MorseControl;

    struct FakeController {
        history: Vec<u8>
    }

    impl Controller for FakeController {
        fn set_vibration(&mut self, vibration: u8) {
            self.history.push(vibration);
        }
    }

    #[test]
    fn morse_code() {
        let mut stub = FakeController { history: vec![] };
        let morse_control = MorseControl::new(0, 5, &mut stub);
        let timing_str = super::run_morse(morse_control, "MORSE CODE");

        assert_eq!(timing_str, "===.===...===.===.===...=.===.=...=.=.=...=.......===.=.===.=...===.===.===...===.=.=...=")
    }

    #[test]
    fn runs_entire_text() {
        const TEXT: &'static str = include_str!("resources/schneemann.txt");
        let mut stub = FakeController { history: vec![] };
        let morse_control = MorseControl::new(0, 5, &mut stub);

        super::run_morse(morse_control, TEXT);
    }

    #[test]
    fn sets_vibration() {
        let mut stub = FakeController { history: vec![] };
        {
            let morse_control = MorseControl::new(0, 5, &mut stub);
            super::run_morse(morse_control, "ee e");
        }

        assert_eq!(stub.history, vec![5, 0, 5, 0, 5, 0]);
    }
}