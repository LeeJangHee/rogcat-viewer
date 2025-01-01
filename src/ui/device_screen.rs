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
        div { id: "device-screen", border: "2px solid #ffffff", width: "20%",
            button { id: "reset", onclick: move |_| info!("reset button clicked."), "reset" }
            for d in DUMMY_DEVICES {
                pre { "{d}" }
            }
        }
    }
}
