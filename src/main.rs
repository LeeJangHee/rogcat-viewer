use dioxus::prelude::*;
use rogcat_viewer::ui::{
    adjust_log_screen::AdjustLogScreen, checkbox_fileter_screen::CheckboxFilterScreen,
    device_screen::DeviceScreen, input_fileter_screen::InputFilterScreen,
};
use utils::map_to;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { display: "flex", gap: "10px",
            DeviceScreen {}
            div { flex: 80,
                InputFilterScreen {
                    group: String::from("word"),
                    titles: map_to(vec!["find", "remove"]),
                }

                InputFilterScreen {
                    group: String::from("tag"),
                    titles: map_to(vec!["pid", "tid", "show", "remove"]),
                }
            }
            div { flex: 20,
                CheckboxFilterScreen {
                    group: String::from("log"),
                    titles: map_to(vec!["verbose", "debug", "info", "warn", "error", "fatal"]),
                }

                CheckboxFilterScreen {
                    group: String::from("show-column"),
                    titles: map_to(
                        vec!["mark", "line", "date", "time", "level", "pid", "thread", "tag", "message"],
                    ),
                }

                InputFilterScreen {
                    group: String::from("highlight"),
                    titles: map_to(vec!["highlight"]),
                }
            }
        }
        AdjustLogScreen {}
    }
}
