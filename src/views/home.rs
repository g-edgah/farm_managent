use crate::components::{Birds, Dashboard, Money};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "bg-purple-100 w-screen flex flex-col gap-5 p-5 grow",

            Dashboard {}
            Money {}
            Birds {}
        }
    }
}
