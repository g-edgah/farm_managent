use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaCircleUser;
use dioxus_free_icons::Icon;

#[component]
pub fn Navbar() -> Element {
    let page = use_signal(|| "home");
    rsx! {
        div {
            class: "bg-purple-200 flex justify-between items-center gap-4 px-5 border-b border-gray-300 h-10",
            id: "navbar",
            div { class: "flex justify-between items-center gap-4 px-5",
                Link {
                    to: Route::Batch {},
                    class: if page() == "batch" { "bg-purple-500 text-white" } else { "bg-gray-200 text-gray-600" },
                    onclick: move |_| { page.set("batch") },
                    "Batch"
                }
                Link {
                    to: Route::Home {},
                    class: if page() == "home" { "bg-purple-500 text-white" } else { "bg-gray-200 text-gray-600" },
                    "Home"
                }
                Link {
                    to: Route::Money {},
                    class: if page() == "money" { "bg-purple-500 text-white" } else { "bg-gray-200 text-gray-600" },
                    "Money"
                }
                Link {
                    to: Route::Birds {},
                    class: if page() == "birds" { "bg-purple-500 text-white" } else { "bg-gray-200 text-gray-600" },
                    "Birds"
                }
                Link {
                    to: Route::Settings {},
                    class: if page() == "settings" { "bg-purple-500 text-white" } else { "bg-gray-200 text-gray-600" },
                    "Settings"
                }
            }
            div {
                Link {
                    to: Route::Profile {},
                    class: if page() == "profile" { "bg-purple-500 text-white" } else { "bg-gray-200 text-gray-600" },

                    Icon {
                        width: 20,
                        height: 20,
                        fill: "black",
                        icon: FaCircleUser,
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
