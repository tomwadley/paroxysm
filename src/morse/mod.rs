mod run_morse;
mod morse_control;
mod char_to_symbol;

use common::control::GatttoolController;
use morse::run_morse::run_morse;

const DOT_LEN: u64 = 200;
const VIBRATION: u8 = 10;

const TEXT: &'static str = include_str!("resources/schneemann.txt");

pub fn morse(bd_addr: &str) {
    let controller = GatttoolController::new(bd_addr, 0);
    let morse_control = morse_control::MorseControl::new(DOT_LEN, VIBRATION, Box::new(controller));

    run_morse(morse_control, TEXT);
}
