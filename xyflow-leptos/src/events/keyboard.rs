//! Keyboard handling for the flow container
//!
//! The `SvelteFlow` container carries `tabindex="0"` so it receives focus
//! when clicked (clicking any non-focusable child focuses the container),
//! and this keydown handler provides the standard flow shortcuts:
//!
//! * `Delete` / `Backspace` — request deletion of the current selection
//!   (`FlowStore::request_delete_selection`: host callback when registered,
//!   store-owned removal otherwise)
//! * `Escape` — cancel an in-flight connection drag
//!
//! Keys typed into inputs/textareas/contenteditable inside custom nodes are
//! left alone.

use leptos::prelude::*;
use leptos::ev;
use leptos::wasm_bindgen::JsCast;
use crate::hooks::use_flow_store;

/// Whether the event originates from an editable element (so flow shortcuts
/// must not steal the keystroke).
fn is_editing_target(ev: &ev::KeyboardEvent) -> bool {
    let Some(target) = ev
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    else {
        return false;
    };
    let tag = target.tag_name().to_ascii_lowercase();
    tag == "input"
        || tag == "textarea"
        || tag == "select"
        || target
            .dyn_ref::<web_sys::HtmlElement>()
            .is_some_and(|el| el.is_content_editable())
}

/// Hook returning the keydown handler the `SvelteFlow` container attaches.
pub fn use_flow_keydown_handler() -> impl Fn(ev::KeyboardEvent) + Clone {
    let store = use_flow_store();

    move |ev: ev::KeyboardEvent| {
        if is_editing_target(&ev) {
            return;
        }

        match ev.key().as_str() {
            "Delete" | "Backspace" => {
                if store.request_delete_selection().is_some() {
                    ev.prevent_default();
                }
            }
            "Escape" => {
                if store
                    .state
                    .connection_in_progress
                    .get_untracked()
                    .is_some()
                {
                    store.cancel_connection();
                    ev.prevent_default();
                }
            }
            _ => {}
        }
    }
}
