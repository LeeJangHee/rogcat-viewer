use data::command::adb::Adb;
use tracing::{debug, info};

#[derive(Debug, Clone, Default)]
pub struct GetDevices {
    adb: Adb
}

#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    id: String,
    status: String,
}

impl GetDevices {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn invoke(&self) -> Vec<Device> {
        debug!("GetDevices invoke() called.");
        let output: std::process::Output = self.clone().adb.devices();

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let device_std: Vec<&str> = stdout
                .lines()
                .skip(1)
                .filter(|line| !line.trim().is_empty())
                .collect();

            let mut devices = Vec::<Device>::new();
            for d in device_std {
                let s: Vec<&str> = d.split('\t').collect();
                if s.len() >= 2 {
                    let id = s[0].to_owned();
                    let status = s[1].to_owned();
                    devices.push(Device { id, status })
                }
            }
            println!("Command Output:\n{:?}", devices);
            devices
        } else {
            println!("Command Output failed.");
            vec![]
        }
    }
}

impl Device {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }
}