#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{debug, info};

#[component]
pub fn AdjustLogScreen() -> Element {
    rsx! {
        FontTypeScreen {}
        FontSizeScreen {}
        TextEncodeScreen {}
        GotoScreen {}
        CommandScreen {}
        ButtonScreen {}
    }
}

fn FontTypeScreen() -> Element {
    info!("FontTypeScreen is called.");
    let TEST_FONTS: Vec<String> = vec![1, 2, 3, 4]
        .iter()
        .map(|&i| format!("fornt{}", i))
        .collect();

    rsx! {
        label { "Font Type: " }
        select { name: "fonts", id: "select-font-id",
            for font in TEST_FONTS {
                option { value: "{font}", "{font}" }
            }
        }
    }
}

fn FontSizeScreen() -> Element {
    use regex::Regex;

    let re = Regex::new(r"[^\d]+$").unwrap();
    let mut font_size = use_signal(|| 12.to_string());
    let on_font_size_click = move |_| {
        info!("font size click with {}", &font_size());
    };
    let font_size_listener = move |event: Event<FormData>| {
        let value = event.value();
        let input_text = re.replace_all(value.trim(), "").to_string();
        debug!("after regex: {}", input_text);
        font_size.set(input_text);
    };

    rsx! {
        label { "Font Size: " }
        input {
            r#type: "text",
            value: "{font_size()}",
            oninput: font_size_listener,
        }
        button { id: "font_size_ok", onclick: on_font_size_click, "OK" }
    }
}

fn TextEncodeScreen() -> Element {
    info!("TextEncodeScreen is called.");
    rsx! {
        label { "Text Encode: " }
        select {
            option { "UTF-8" }
        }
    }
}

fn GotoScreen() -> Element {
    info!("GotoScreen is called.");
    rsx! {
        label { "Goto: " }
        input { r#type: "text" }
    }
}

fn CommandScreen() -> Element {
    info!("CommandScreen is called.");
    rsx! {
        label { "Cmd: " }
        select {
            option { "logcat -v threadtime" }
            option { "logcat -v threadtime" }
            option { "logcat -v threadtime" }
            option { "logcat -v threadtime" }
        }
    }
}

fn ButtonScreen() -> Element {
    info!("ButtonScreen is called.");
    rsx! {
        button { "Clear" }
        button { "Run" }
        button { "Pause" }
        button { "Stop" }
    }
}
