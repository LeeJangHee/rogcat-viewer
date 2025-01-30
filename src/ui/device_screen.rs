#![allow(non_snake_case)]
use dioxus::prelude::*;
use domain::{
    choice_adb_device::ChoiceAdbDevice,
    get_devices::{Device as DomainDevice, GetDevices},
};
use std::sync::Arc;

use tracing::{debug, info};
use utils::prelude::Mapper;

#[derive(Debug, Clone, PartialEq, Props)]
struct Device {
    id: String,
    status: String,
}

#[component]
pub fn DeviceScreen() -> Element {
    info!("DeviceScreen called.");
    let get_devices = Arc::new(Box::new(GetDevices::new()));
    let devices = use_signal(|| Vec::<Device>::new());

    let get_devices_clone = get_devices.clone();
    let on_clicked = move |_| {
        info!("reset on_click event.");
        devices.clone().set(mapper(&get_devices_clone));
    };
    let on_select_device = move |e: Event<FormData>| {
        info!("on_input event handler called.\n{e:#?}");
        let choice_adb_device = Box::new(ChoiceAdbDevice::new());
        if !e.value().is_empty() {
            debug!("event value : {:?}", &e.value());
            choice_adb_device.invoke(e.value());
        }
    };

    let get_devices_clone = get_devices.clone();
    use_effect(move || {
        devices.clone().set(mapper(&get_devices_clone));
    });

    rsx! {
        div { class: "content", width: "10%", style: "overflow: auto;",
            p {
                b { "Device Select" }
            }
            button { class: "button is-small", onclick: on_clicked, "Search" }
            for d in devices() {
                if d.status == "device".to_owned() {
                    div {
                        input {
                            id: "{d.id}",
                            r#type: "radio",
                            name: "device",
                            value: "{d.id}",
                            oninput: on_select_device,
                        }
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
        .map(|d: &DomainDevice| Device::map_to(d.clone()))
        .collect()
}

impl Mapper<DomainDevice> for Device {
    fn map_to(data: DomainDevice) -> Device {
        Device {
            id: data.id(),
            status: data.status(),
        }
    }
}
