use dioxus::prelude::*;
use tracing::info;

use crate::utils::str_util::capitalize;

#[component]
pub fn FilterScreen(group: String, titles: Vec<String>) -> Element {
    info!(
        "FilterScreen called with group: {}, titles: {:?}",
        group, titles
    );

    rsx! {
        h3 { "{capitalize(&group)} filter" }
        for title in titles {
            div { id: "word-filter", display: "flex",
                label { flex: 1, r#for: "{title}", "{capitalize(&title)}: " }
                input {
                    flex: 5,
                    r#type: "text",
                    id: "{title}",
                    placeholder: "e.g. write find or remove log. mutiple use `|`",
                }
                input { r#type: "checkbox" }
            }
        }
    }
}
