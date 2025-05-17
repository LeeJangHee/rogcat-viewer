#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::{debug, info};

#[component]
pub fn TableScreen() -> Element {
    info!("TableScreen is called.");
    rsx! {
        div {
            table { width: "100%", table_layout: "fixed",
                TitleScreen {}
                Contents {}
            }
        }
    }
}

fn TitleScreen() -> Element {
    info!("TableTitleScreen is called.");
    rsx! {
        thead {
            tr {
                th { "line" }
                th { "date" }
                th { "time" }
                th { "log-level" }
                th { "pid" }
                th { "thread" }
                th { "tag" }
                th { "message" }
            }
        }
    }
}

fn Contents() -> Element {
    info!("Contents is called.");
    rsx! {
        tbody {
            tr {
                td { "line" }
                td { "date" }
                td { "time" }
                td { "log-level" }
                td { "pid" }
                td { "thread" }
                td { "tag" }
                td { "message" }
            }
        }
    }
}
