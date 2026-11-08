use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Money() -> Element {
    rsx! {

        div {
            "blog"

            // Content
            h1 { "This is blog !" }
            p {
                "In blog we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            Link { to: Route::Birds {}, "Previous" }
            span { " <---> " }
            Link { to: Route::Birds {}, "Next" }
        }
    }
}
