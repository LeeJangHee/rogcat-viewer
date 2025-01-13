use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn DeviceScreen() -> Element {
    info!("DeviceScreen called.");
    let DUMMY_DEVICES: Vec<String> = vec!["test1", "test2", "test3", "test4"]
        .iter()
        .map(|&d| d.to_string())
        .collect();

    rsx! {
        fieldset { id: "device-screen", width: "10%",
            legend { "Device Select" }
            button { id: "reset", onclick: move |_| info!("reset button clicked."), "reset" }
            for d in DUMMY_DEVICES {
                pre { "{d}" }
            }
        }
    }
}
