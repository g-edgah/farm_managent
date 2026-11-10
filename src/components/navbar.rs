use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaCircleUser;
use dioxus_free_icons::Icon;

#[component]
pub fn Navbar() -> Element {
    let mut page = use_signal(|| "home");
    rsx! {
        div {
            class: "bg-purple-200 flex justify-between items-center gap-4 px-5 border-b border-gray-300 h-10",
            id: "navbar",
            div { class: "flex justify-between items-center gap-4 px-5 h-full",
                Link {
                    to: Route::Batch {},
                    class: if page() == "batch" { "bg-purple-100 text-black h-full text-center flex items-center justify-center px-1" } else { "px-1 text-black h-full text-center flex items-center justify-center" },
                    onclick: move |_| { page.set("batch") },
                    "Batch"
                }
                Link {
                    to: Route::Home {},
                    class: if page() == "home" { "bg-purple-100 text-black h-full text-center flex items-center justify-center px-1" } else { "px-1 text-black h-full text-center flex items-center justify-center" },
                    onclick: move |_| { page.set("home") },
                    span { class: "text-center", "Homey" }
                }
                Link {
                    to: Route::Money {},
                    class: if page() == "money" { "bg-purple-100 text-black h-full text-center flex items-center justify-center px-1" } else { "px-1 text-black h-full text-center flex items-center justify-center" },
                    onclick: move |_| { page.set("money") },
                    "Money"
                }
                Link {
                    to: Route::Birds {},
                    class: if page() == "birds" { "bg-purple-100 text-black h-full text-center flex items-center justify-center px-1" } else { "px-1 text-black h-full text-center flex items-center justify-center" },
                    onclick: move |_| { page.set("birds") },
                    "Birds"
                }
                Link {
                    to: Route::Settings {},
                    class: if page() == "settings" { "bg-purple-100 text-black h-full text-center flex items-center justify-center px-1" } else { "px-1 text-black h-full text-center flex items-center justify-center" },
                    onclick: move |_| { page.set("settings") },
                    "Settings"
                }
            }
            div { class: "flex justify-between items-center gap-4 px-5 h-full",
                Link {
                    to: Route::Profile {},
                    class: if page() == "profile" { "bg-purple-100 text-black h-full text-center flex items-center justify-center px-1" } else { "px-1 text-black h-full text-center flex items-center justify-center" },
                    onclick: move |_| { page.set("profile") },

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
