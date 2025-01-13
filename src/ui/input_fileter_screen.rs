use dioxus::prelude::*;
use tracing::info;

use crate::utils::str_util::capitalize;

#[component]
pub fn InputFilterScreen(group: String, titles: Vec<String>) -> Element {
    info!(
        "InputFilterScreen called with group: {}, titles: {:?}",
        group, titles
    );

    rsx! {
        fieldset {
            legend { "{capitalize(&group)}" }
            for title in titles {
                div { display: "flex",
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
}
