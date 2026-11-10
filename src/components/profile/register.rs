use crate::server::register_user;
use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    let mut page = use_signal(|| "register");
    let mut register = use_signal(|| "");
    let mut login = use_signal(|| "");
    let mut reg_details = use_signal(|| RegDetails::default());
    let mut error = use_signal(|| None::<String>);
    let mut submitting = use_signal(|| false);
    let mut success = use_signal(|| false);

    let onsubmit_reg = move |e: FormEvent| {
        e.prevent_default();

        // input validation
        // username length
        let details = reg_details();

        dioxus_logger::tracing::info!("{:?}", details);

        if details.username.trim().len() < 3 {
            error.set(Some("username must be at least 3 chaacters".into()));
            dioxus_logger::tracing::info!("username fail");
            return;
        }

        dioxus_logger::tracing::info!("username pass");
        //password length
        if details.password.trim().len() < 8 {
            error.set(Some(
                "password cannot be less than 8 characters long".into(),
            ));
            dioxus_logger::tracing::info!("pass fail");
            return;
        }

        dioxus_logger::tracing::info!("pass pass");

        //password length
        if details.password.trim() != details.confirm_password.trim() {
            error.set(Some("passwords provided do not match".into()));
            dioxus_logger::tracing::info!("confirm fail");
            return;
        }

        dioxus_logger::tracing::info!("confirm pass");

        //if validation checks are passed
        error.set(None);
        submitting.set(true);

        spawn(async move {
            match register_user(details.username, details.password, details.confirm_password).await
            {
                Ok(()) => {
                    success.set(true);
                    reg_details.set(RegDetails::default());
                    dioxus_logger::tracing::info!("server pass");
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                    dioxus_logger::tracing::info!("server fail");
                }
            }
            submitting.set(false);
        });
    };

    rsx! {
        if page() == "register" {
            div { class: "bg-purple-100 flex flex-col gap-5",
                div { class: "w-full text-center",
                    span { class: "text-xl font-bold", "Register" }
                }
                div { class: "bg-purple-200 pt-17 pb-10 px-5 w-110 rounded-md",
                    form { class: "flex flex-col gap-10", onsubmit: onsubmit_reg,
                        div { class: "username flex flex-col gap-3",

                            span { class: "pl-1 text-lg font-semibold", "username" }

                            input {
                                class: "rounded-md border border-gray-300 bg-purple-100 px-3 h-10",
                                r#type: "text",
                                placeholder: "enter username",
                                value: "{reg_details.read().username}",
                                oninput: move |e| reg_details.with_mut(|d| d.username = e.value()),
                            }
                        }
                        div { class: "password flex flex-col gap-3",

                            span { class: "pl-1 text-lg font-semibold", "password" }

                            input {
                                class: "rounded-md border border-gray-300 bg-purple-100 px-3 h-10",
                                r#type: "text",
                                placeholder: "********",
                                value: "{reg_details.read().password}",
                                oninput: move |e| reg_details.with_mut(|d| d.password = e.value()),
                            }
                        }

                        div { class: "confirm_password flex flex-col gap-3",

                            span { class: "pl-1 text-lg font-semibold", "confirm password" }

                            input {
                                class: "rounded-md border border-gray-300 bg-purple-100 px-3 h-10",
                                r#type: "text",
                                placeholder: "********",
                                value: "{reg_details.read().confirm_password}",
                                oninput: move |e| reg_details.with_mut(|d| d.confirm_password = e.value()),
                            }
                        }

                        div { class: "mt-7 w-full flex justify-center items-center",

                            button {
                                r#type: "submit",
                                class: "hover:bg-purple-400 w-30 h-10 bg-purple-300 rounded-md",
                                span { class: "text-lg font-semibold", "submit" }
                            }
                        }

                    }
                }
                div { class: "flex gap-3",
                    span { "already have an account?" }
                    button {
                        class: "cursor-pointer",
                        onclick: move |_| { page.set("login") },
                        "log in"
                    }
                }
            }
        } else if page() == "login" {
            div { class: "bg-purple-100",
                div { class: "bg-purple-300 w-130 h-150 rounded-md",
                    span { "log in" }
                }
                div { class: "flex gap-2",
                    span { "already have an account?" }
                    button {
                        class: "cursor-pointer",
                        onclick: move |_| { page.set("register") },
                        "register"
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RegDetails {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LoginDetails {
    pub username: String,
    pub password: String,
    //pub remember: Bool,
}
