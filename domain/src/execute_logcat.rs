use std::io::{BufRead, BufReader};
use tracing::debug;
use data::command::adb::Adb;

#[derive(Debug, Clone, Default)]
pub struct ExecuteLogcat {
    adb: Adb,
}

impl ExecuteLogcat {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn invoke(self) {
        debug!("ExecuteLogcat inveke() called.");
        let mut logcat = self.adb.logcat();
        let stdout = logcat
            .stdout
            .take()
            .expect("Failed to take stdout message.");
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Some(opt_line) = lines.next() {
            match opt_line {
                Ok(l) => println!("{l}"),
                Err(e) => {
                    eprintln!("logcat error: {e}");
                    break;
                },
            }
        }
    }
}
