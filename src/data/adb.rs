use std::process::{Child, Command, Output, Stdio};
use tracing::{debug, info};
use utils::macros::StructName;

#[derive(Debug, Clone, StructName)]
pub struct Adb {
    exec_cmd: String,
    device: Option<String>,
}

impl Default for Adb {
    #[inline]
    fn default() -> Self {
        Adb {
            exec_cmd: String::from("adb"),
            device: None,
        }
    }
}

impl Adb {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn devices(self) -> Output {
        debug!("[{}] devices called.", self.simple_name());
        Command::new(self.exec_cmd)
            .arg("devices")
            .output()
            .expect("Failed to execute `adb device` command.")
    }

    pub fn logcat(self) -> Child {
        info!("[{}] logcat called.", self.simple_name());
        let mut args = vec!["logcat", "-v", "threadtime"];
        if let Some(d) = &self.device {
            args.push("-s");
            args.push(d.as_str());
        }
        Command::new(self.exec_cmd)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute `adb logcat -v threadtime` command.")
    }

    pub fn set_device(&mut self, device: String) {
        debug!(
            "[{}] set_device called with {}.",
            self.simple_name(),
            &device
        );
        self.device = if !device.is_empty() {
            Some(device)
        } else {
            None
        };
    }
}
