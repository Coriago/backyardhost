use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::server::kv::list_entries_server;

#[cfg(not(feature = "server"))]
#[get("/api/entries")]
pub async fn list_entries_server() -> Result<Vec<crate::models::ContactEntry>, ServerFnError> {
    unreachable!()
}

const ENTRY_LIST_CSS: Asset = asset!("/assets/styling/entry_list.css");

#[component]
pub fn EntryList() -> Element {
    let refresh_trigger = use_context::<Signal<u64>>();
    let entries = use_resource(move || async move {
        let _trigger = refresh_trigger();
        list_entries_server().await
    });

    rsx! {
        document::Link { rel: "stylesheet", href: ENTRY_LIST_CSS }

        div { id: "entry-list",
            h4 { "Submitted Entries" }
            match entries() {
                None => rsx! { div { "Loading..." } },
                Some(Ok(entries)) => {
                    if entries.is_empty() {
                        rsx! { div { class: "empty-list", "No entries yet" } }
                    } else {
                        rsx! {
                            for entry in entries {
                                div { class: "entry-item",
                                    div { class: "entry-name", "{entry.name}" }
                                    div { class: "entry-email", "{entry.email}" }
                                    div { class: "entry-message", "{entry.message}" }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => rsx! { div { "Error loading entries: {e}" } }
            }
        }
    }
}
