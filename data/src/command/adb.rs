use std::process::{Command, Output};

#[derive(Debug)]
pub struct Adb {
    cmd: Command,
}

impl Default for Adb {
    #[inline]
    fn default() -> Self {
        Adb {
            cmd: Command::new("adb"),
        }
    }
}

impl Adb {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn devices(mut self) -> Output {
        self.cmd
            .arg("devices")
            .output()
            .expect("Faild to execute `adb device` command.")
    }
}
