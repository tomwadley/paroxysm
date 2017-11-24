use std::process::Command;
use std::process::Child;
use std::iter::FromIterator;

pub struct Controller {
    device_addr: String,
    p: Child,
}

impl Controller {
    pub fn new(device_addr: &str, init_vibration: u8) -> Controller {
        Controller {
            device_addr: String::from(device_addr),
            p: Controller::run_cmd(device_addr, init_vibration)
        }
    }

    pub fn set_vibration(&mut self, vibration: u8) {
        self.p.kill();
        self.p = Controller::run_cmd(&self.device_addr, vibration);
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
            .arg(Controller::vibrate_arg(vibration))
            .arg("--listen")
            .spawn().unwrap()
    }

    fn vibrate_arg(vibration: u8) -> String {
        let arg = format!("Vibrate:{};", vibration);
        let chars_as_hex = arg.chars().map(|c| format!("{:x}", c as u8));
        String::from_iter(chars_as_hex)
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.p.kill().expect("Failed to kill process");
    }
}

#[cfg(test)]
mod tests {
    extern crate config;

    use std::thread;
    use std::time;

    #[test]
    fn controls() {
        let bd_addr = load_config().get_str("bd_addr").unwrap();
        let mut controller = super::Controller::new(&bd_addr, 10);
        thread::sleep(time::Duration::from_secs(5));
        controller.set_vibration(5);
        thread::sleep(time::Duration::from_secs(5));
    }

    fn load_config() -> config::Config {
        let mut conf = config::Config::new();
        conf.merge(config::File::with_name("Config")).unwrap();
        conf
    }
}