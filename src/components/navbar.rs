use crate::Route;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let batch_open = use_signal(|| false);
    rsx! {
        div { class: "bg-purple-100 flex gap-4 px-5", id: "navbar",
            Link { to: Route::Batch {}, "Batch" }
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Money {}, "Money" }
            Link { to: Route::Birds {}, "Birds" }
            Link { to: Route::Settings {}, "Settings" }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
