use common::control::{Controller,GatttoolController};
use std::thread;
use std::time::Duration;
use std::ascii::AsciiExt;
use std::io;
use std::io::Write;

const DOT_LEN: u64 = 200;
const VIBRATION: u8 = 10;

pub fn morse(bd_addr: &str) {
    let mut controller = GatttoolController::new(bd_addr, 0);

    run_morse(&mut controller, "PARIS CODEX");
}

fn run_morse(controller: &mut Controller, text: &str) -> String {
    let mut t = String::new();

    for c in text.chars() {
        print!("{}", c.to_ascii_uppercase());
        io::stdout().flush();

        if c == ' ' {
            t += word_space();
        } else {
            for symbol in to_morse_symbol(c).chars() {
                match symbol {
                    '−' => t += dash(controller),
                    '·' => t += dot(controller),
                    _ => panic!("Invalid symbol: {}", symbol)
                };
                t += symbol_space();
            }
            t += letter_space();
        }
    }

    print!("\n");
    t.trim_right_matches(".").to_string()
}

fn dash(controller: &mut Controller) -> &'static str {
    controller.set_vibration(VIBRATION);
    thread::sleep(Duration::from_millis(3 * DOT_LEN));
    controller.set_vibration(0);
    "==="
}

fn dot(controller: &mut Controller) -> &'static str {
    controller.set_vibration(VIBRATION);
    thread::sleep(Duration::from_millis(1 * DOT_LEN));
    controller.set_vibration(0);
    "="
}

fn symbol_space() -> &'static str {
    thread::sleep(Duration::from_millis(1 * DOT_LEN));
    "."
}

fn letter_space() -> &'static str {
    thread::sleep(Duration::from_millis(2 * DOT_LEN));
    ".."
}

fn word_space() -> &'static str {
    thread::sleep(Duration::from_millis(4 * DOT_LEN));
    "...."
}

fn to_morse_symbol(c: char) -> &'static str {
    match c.to_ascii_uppercase() {
        'A' => "·−",
        'B' => "−···",
        'C' => "−·−·",
        'D' => "−··",
        'E' => "·",
        'F' => "··−·",
        'G' => "−−·",
        'H' => "····",
        'I' => "··",
        'J' => "·−−−",
        'K' => "−·−",
        'L' => "·−··",
        'M' => "−−",
        'N' => "−·",
        'O' => "−−−",
        'P' => "·−−·",
        'Q' => "−−·−",
        'R' => "·−·",
        'S' => "···",
        'T' => "−",
        'U' => "··−",
        'V' => "···−",
        'W' => "·−−",
        'X' => "−··−",
        'Y' => "−·−−",
        'Z' => "−−··",
        _ => panic!("Invalid character: {}", c)
    }
}

#[cfg(test)]
mod tests {
    extern crate config;
    use common::control::{Controller,GatttoolController};

    struct FakeController;
    impl Controller for FakeController {
        fn set_vibration(&mut self, vibration: u8) {
            // no-op
        }
    }

    #[test]
    fn morse_code() {
        let mut controller = FakeController {};

        let timing_str = super::run_morse(&mut controller, "MORSE CODE");
        assert_eq!(timing_str, "===.===...===.===.===...=.===.=...=.=.=...=.......===.=.===.=...===.===.===...===.=.=...=")
    }
}