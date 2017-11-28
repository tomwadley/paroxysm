use morse::morse_control::MorseControl;
use morse::char_to_symbol::to_morse_symbol;
use std::io;
use std::io::Write;

const WHITESPACE: &'static [char] = &[' ', '\n', '—', '-', '’', '(', ')', '…', '.', '?', '“', '”', ','];

pub fn run_morse(mut morse_control: MorseControl, text: &str) -> String {
    let mut t = String::new();

    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush();

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

    struct FakeController;
    impl Controller for FakeController {
        fn set_vibration(&mut self, _vibration: u8) {
            // no-op
        }
    }

    #[test]
    fn morse_code() {
        let timing_str = super::run_morse(get_morse_control(), "MORSE CODE");

        assert_eq!(timing_str, "===.===...===.===.===...=.===.=...=.=.=...=.......===.=.===.=...===.===.===...===.=.=...=")
    }

    #[test]
    fn runs_entire_text() {
        const TEXT: &'static str = include_str!("resources/schneemann.txt");

        super::run_morse(get_morse_control(), TEXT);
    }

    fn get_morse_control() -> MorseControl {
        let controller = FakeController {};
        MorseControl::new(0, 5, Box::new(controller))
    }
}