use dioxus::prelude::*;

#[component]
pub fn Batch() -> Element {
    rsx! {
        div { class: "bg-purple-100 h-screen w-screen p-5",
            span { "Batches" }
        }
    }
}
