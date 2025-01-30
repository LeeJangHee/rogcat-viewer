use data::command::adb::Adb;
use tracing::debug;


#[derive(Debug, Clone, Default)]
pub struct ChoiceAdbDevice {
    adb: Adb
}

impl ChoiceAdbDevice {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn invoke(mut self, adb_serial: String) {
        debug!("ChoiceAdbDevice inovke() called with {adb_serial}.");
        self.adb.set_device(adb_serial);
    }
}