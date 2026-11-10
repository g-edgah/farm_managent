use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let mut page = use_signal(|| "register");

    fn tab(active: bool) -> String {
        let base = "w-35 rounded-md h-10 flex items-center justify-center text-lg";
        if active {
            format!("{base} bg-purple-500 text-white")
        } else {
            format!("{base} bg-gray-200")
        }
    }

    rsx! {
        div { class: "bg-purple-100 h-screen w-screen flex",
            div { class: "side w-2/10 h-full border-r border-gray-300 flex flex-col justify-between items-center pt-15 bg-purple-200 max-w-40",

                div { class: " flex flex-col gap-5 ",
                    div {
                        class: tab(page() == "register"),
                        onclick: move |_| { page.set("registery") },
                        if page() == "register" {
                            "Login/register"
                        }
                    }

                    div {
                        class: tab(page() == "register"),
                        onclick: move |_| { page.set("y") },
                        "Login/register"
                    }

                    div {
                        class: tab(page() == "register"),
                        onclick: move |_| { page.set("registery") },
                        "Login/register"
                    }
                }

                div { class: "",

                    div {
                        class: "mb-100 bg-purple-100 w-35 rounded-md border border-gray-300  h-10 flex items-center justify-center text-lg",
                        onclick: move |_| { page.set("register") },
                        "Login/register"

                    }
                }

            }

            div { class: "main w-full h-full ",
                div { class: "profile_pic" }
                span { class: "profile_name" }
            }
        }
    }
}
