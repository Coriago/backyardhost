use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::server::kv::submit_entry_server;

#[cfg(not(feature = "server"))]
#[post("/api/submit")]
pub async fn submit_entry_server(
    name: String,
    email: String,
    message: String,
) -> Result<crate::models::ContactEntry, ServerFnError> {
    unreachable!()
}

const CONTACT_FORM_CSS: Asset = asset!("/assets/styling/contact_form.css");

#[component]
pub fn ContactForm() -> Element {
    let mut refresh_trigger = use_context::<Signal<u64>>();
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut status = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: CONTACT_FORM_CSS }

        div { id: "contact-form",
            h4 { "Contact Us" }

            div { class: "form-group",
                label { "Name" }
                input {
                    r#type: "text",
                    name: "name",
                    placeholder: "Your name",
                    value: name,
                    oninput: move |e| name.set(e.value())
                }
            }

            div { class: "form-group",
                label { "Email" }
                input {
                    r#type: "email",
                    name: "email",
                    placeholder: "Your email",
                    value: email,
                    oninput: move |e| email.set(e.value())
                }
            }

            div { class: "form-group",
                label { "Message" }
                textarea {
                    name: "message",
                    placeholder: "Your message",
                    value: message,
                    oninput: move |e| message.set(e.value())
                }
            }

            button {
                class: "submit-button",
                onclick: move |_| async move {
                    let n = name();
                    let e = email();
                    let m = message();

                    if n.is_empty() || e.is_empty() || m.is_empty() {
                        status.set("All fields are required.".to_string());
                        return;
                    }

                    match submit_entry_server(n, e, m).await {
                        Ok(_) => {
                            name.set(String::new());
                            email.set(String::new());
                            message.set(String::new());
                            status.set("Entry saved!".to_string());
                            *refresh_trigger.write() += 1;
                        }
                        Err(err) => {
                            status.set(format!("Error: {}", err));
                        }
                    }
                },
                "Submit"
            }

            div { class: "form-status", "{status}" }
        }
    }
}
