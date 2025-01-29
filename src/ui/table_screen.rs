use dioxus::prelude::*;
use tracing::{debug, info};

#[component]
pub fn TableScreen() -> Element {
    info!("TableScreen is called.");
    rsx! {
        div {
            table {
                thead {
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
    }
}

fn title_screen() -> Element {
    info!("TableTitleScreen is called.");
    rsx! {
        thead {
        }
    }
}

fn contents() -> Element {
    info!("Contents is called.");
    rsx! {
        tbody {
        }
    }
}

// table column
