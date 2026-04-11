use crate::components::{ContactForm, EntryList};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let refresh_trigger = use_signal(|| 0u64);
    use_context_provider(|| refresh_trigger);

    rsx! {
        ContactForm {}
        EntryList {}
    }
}
