use dioxus::prelude::*;
use rogcat_viewer::ui::{device_screen::DeviceScreen, filter_screen::FilterScreen};
use rogcat_viewer::utils::str_util::map_to;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { display: "flex", gap: "10px",
            DeviceScreen {}
            div { width: "80%",
                FilterScreen {
                    group: String::from("word"),
                    titles: map_to(vec!["find", "remove"]),
                }

                FilterScreen {
                    group: String::from("tag"),
                    titles: map_to(vec!["pid", "tid", "show", "remove"]),
                }
            }
        }
    }
}
