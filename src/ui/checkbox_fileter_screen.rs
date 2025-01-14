use dioxus::prelude::*;
use tracing::info;

use utils::capitalize;

#[component]
pub fn CheckboxFilterScreen(group: String, titles: Vec<String>) -> Element {
    info!(
        "CheckboxFilterScreen is called with group:{:?}, titles:{:?}",
        group, titles
    );
    rsx! {
        fieldset {
            legend { "{capitalize(&group)}" }
            for title in titles {
                input { r#type: "checkbox", id: "{title}", name: "{title}" }
                label { r#for: "{title}", "{title}" }
            }
        }
    }
}
