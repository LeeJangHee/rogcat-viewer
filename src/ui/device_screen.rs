use dioxus::prelude::*;
use domain::get_devices::{Device as DomainDevice, GetDevices};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Props)]
struct Device {
    id: String,
    status: String,
}

#[component]
pub fn DeviceScreen() -> Element {
    info!("DeviceScreen called.");
    let get_devices = GetDevices::new();
    let devices = use_signal(|| Vec::<Device>::new());

    let get_devices_clone = get_devices.clone();
    let onclick = move |_| {
        info!("reset on_click event.");
        devices.clone().set(mapper(&get_devices_clone));
    };

    let get_devices_clone = get_devices.clone();
    use_effect(move || {
        devices.clone().set(mapper(&get_devices_clone));
    });

    rsx! {
        fieldset { width: "10%",
            legend { "Device Select" }
            button { id: "reset", onclick: onclick, "reset" }
            for d in devices() {
                if d.status == "device".to_owned() {
                    div {
                        input { id: "{d.id}", r#type: "radio", name: "device" }
                        label { r#for: "{d.id}", "{d.id}" }
                    }
                }
            }
        }
    }
}

fn mapper(get_devices: &GetDevices) -> Vec<Device> {
    get_devices
        .invoke()
        .iter()
        .map(|d: &DomainDevice| Device {
            id: d.id(),
            status: d.status(),
        })
        .collect()
}
