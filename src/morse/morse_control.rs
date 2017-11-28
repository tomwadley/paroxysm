use common::control::Controller;
use std::thread;
use std::time::Duration;

pub struct MorseControl {
    dot_len: u64,
    vibration: u8,
    controller: Box<Controller>,
}

impl MorseControl {
    pub fn new(dot_len: u64, vibration: u8, controller: Box<Controller>) -> MorseControl {
        MorseControl { dot_len, vibration, controller }
    }

    pub fn dash(&mut self) -> &'static str {
        self.controller.set_vibration(self.vibration);
        thread::sleep(Duration::from_millis(3 * self.dot_len));
        self.controller.set_vibration(0);
        "==="
    }

    pub fn dot(&mut self) -> &'static str {
        self.controller.set_vibration(self.vibration);
        thread::sleep(Duration::from_millis(1 * self.dot_len));
        self.controller.set_vibration(0);
        "="
    }

    pub fn symbol_space(&self) -> &'static str {
        thread::sleep(Duration::from_millis(1 * self.dot_len));
        "."
    }

    pub fn letter_space(&self) -> &'static str {
        thread::sleep(Duration::from_millis(2 * self.dot_len));
        ".."
    }

    pub fn word_space(&self) -> &'static str {
        thread::sleep(Duration::from_millis(4 * self.dot_len));
        "...."
    }
}