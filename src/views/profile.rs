use dioxus::prelude::*;

use crate::components::{Details, Notifications, Privacy, Register};

#[component]
pub fn Profile() -> Element {
    #[derive(Clone, PartialEq)]
    enum Page {
        details,
        notifications,
        privacy,
        register,
    }

    let mut page = use_signal(|| Page::register);

    fn tab(active: bool) -> String {
        let base = "w-35 rounded-md h-10 flex items-center justify-center text-lg cursor-pointer";
        if active {
            format!("{base} bg-purple-100 border border-gray-300")
        } else {
            format!("{base}")
        }
    }

    rsx! {
        div { class: "bg-purple-100 h-screen w-screen flex",
            div { class: "side w-2/10 h-full border-r border-gray-300 flex flex-col justify-between items-center pt-15 bg-purple-200 max-w-40",

                div { class: " flex flex-col gap-5 ",
                    div {
                        class: tab(page() == Page::details),
                        onclick: move |_| { page.set(Page::details) },
                        "profile"

                    }

                    div {
                        class: tab(page() == Page::notifications),
                        onclick: move |_| { page.set(Page::notifications) },
                        "notifications"
                    }

                    div {
                        class: tab(page() == Page::privacy),
                        onclick: move |_| { page.set(Page::privacy) },
                        "privacy"
                    }
                }

                div { class: "",

                    div {
                        class: format!("{} mb-100", tab(page() == Page::register)),
                        onclick: move |_| { page.set(Page::register) },
                        "Login/register"
                    }
                }

            }

            div { class: "main w-full h-full flex justify-center items-center",
                match page() {
                    Page::details => rsx! {
                        Details {}
                    },
                    Page::notifications => rsx! {
                        Notifications {}
                    },
                    Page::privacy => rsx! {
                        Privacy {}
                    },
                    Page::register => rsx! {
                        Register {}
                    },
                }
            }
        }
    }
}
