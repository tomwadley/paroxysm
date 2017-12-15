use std::process::{Command,Child,Stdio};
use std::iter::FromIterator;
use std::thread;
use std::time::Duration;

pub trait Controller {
    fn set_vibration(&mut self, vibration: u8);
}

pub struct GatttoolController {
    device_addr: String,
    p: Child,
}

impl Controller for GatttoolController {
    fn set_vibration(&mut self, vibration: u8) {
        let _ = self.p.kill();
        self.p = GatttoolController::run_cmd(&self.device_addr, vibration);
    }
}

impl GatttoolController {
    pub fn new(device_addr: &str, init_vibration: u8) -> GatttoolController {
        let p = GatttoolController::run_cmd(device_addr, init_vibration);

        // Give it a chance to wake up
        thread::sleep(Duration::from_millis(2000));

        GatttoolController { device_addr: String::from(device_addr), p }
    }

    fn run_cmd(device_addr: &str, vibration: u8) -> Child {
        Command::new("gatttool")
            .arg("-b")
            .arg(device_addr)
            .arg("-t")
            .arg("random")
            .arg("--char-write-req")
            .arg("-a")
            .arg("0x000e")
            .arg("-n")
            .arg(GatttoolController::vibrate_arg(vibration))
            .arg("--listen")
            .stdout(Stdio::null())
            .spawn().unwrap()
    }

    fn vibrate_arg(vibration: u8) -> String {
        let arg = format!("Vibrate:{};", vibration);
        let chars_as_hex = arg.chars().map(|c| format!("{:x}", c as u8));
        String::from_iter(chars_as_hex)
    }
}

impl Drop for GatttoolController {
    fn drop(&mut self) {
        self.p.kill().expect("Failed to kill process");
    }
}

#[cfg(test)]
mod tests {
    extern crate config;

    use std::thread;
    use std::time;
    use super::Controller;

    #[test]
    fn controls() {
        let bd_addr = load_config().get_str("bd_addr").unwrap();
        let mut controller = super::GatttoolController::new(&bd_addr, 20);
        thread::sleep(time::Duration::from_secs(1));
        controller.set_vibration(5);
        thread::sleep(time::Duration::from_secs(1));
        controller.set_vibration(0);
        thread::sleep(time::Duration::from_secs(1));
    }

    fn load_config() -> config::Config {
        let mut conf = config::Config::new();
        conf.merge(config::File::with_name("Config")).unwrap();
        conf
    }
}