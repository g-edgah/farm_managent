use dioxus::prelude::*;

mod components;
mod views;

#[cfg(feature = "server")]
mod server;

use components::Navbar;
use views::{Batch, Birds, Home, Money, Profile, Settings};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[layout(Navbar)]

        #[ route("/batch") ]
        Batch {},
        
        #[route("/")]
        Home {},
        
        #[route("/money")]
        Money {},
        
        #[route("/birds")]
        Birds {},
        
        #[route("/settings")]
        Settings {},

        #[route("/profile")]
        Profile {},
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        dotenvy::dotenv().ok();
        let router = dioxus::server::router(app);
        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
