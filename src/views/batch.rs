use dioxus::prelude::*;

#[component]
pub fn Batch() -> Element {
    rsx! {
        div { class: "bg-purple-100",
            span { "Batches" }
        }
    }
}
