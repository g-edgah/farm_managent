use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "h-screen rounded-md w-full bg-gray-200 py-4 px-6",
            span { "dashboardy" }
        }
    }
}
