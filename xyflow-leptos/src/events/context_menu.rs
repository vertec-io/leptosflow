//! Context-menu (right-click) event handling
//!
//! A single `contextmenu` listener on the flow container classifies the
//! click target — edge, node, or pane — by walking up to the closest
//! `.xyflow__edge` / `.xyflow__node` ancestor and reading its `data-id`.
//! This works for the built-in renderers AND for custom node/edge views,
//! which already emit those classes and `data-id` attributes; no per-node
//! wiring is needed in the consumer.

use leptos::prelude::*;
use leptos::ev;
use leptos::wasm_bindgen::JsCast;
use crate::hooks::use_flow_store;
use crate::store::ContextMenuEvent;
use crate::utils::coordinate::screen_to_flow_position_with_ref;

/// Hook returning the `contextmenu` handler the `SvelteFlow` container
/// attaches. Fires the store's `on_edge_context_menu` /
/// `on_node_context_menu` / `on_pane_context_menu` callback for the
/// resolved target, with the pointer position in both screen (client) and
/// flow coordinates.
///
/// The native browser menu is only suppressed when a callback is registered
/// for that target kind — unconfigured flows keep default browser behavior.
pub fn use_context_menu_handler() -> impl Fn(ev::MouseEvent) + Clone {
    let store = use_flow_store();

    move |ev: ev::MouseEvent| {
        // Edge groups sit under nodes in the DOM; check the edge class
        // first so the invisible wide interaction stroke wins over the
        // node rectangle only when it was actually the event target.
        let target: Option<web_sys::Element> = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok());

        let closest = |selector: &str| -> Option<web_sys::Element> {
            target.as_ref().and_then(|el| el.closest(selector).ok().flatten())
        };

        let (callback, id) = if let Some(edge_el) = closest(".xyflow__edge") {
            (
                store.state.on_edge_context_menu.get_untracked(),
                edge_el.get_attribute("data-id"),
            )
        } else if let Some(node_el) = closest(".xyflow__node") {
            (
                store.state.on_node_context_menu.get_untracked(),
                node_el.get_attribute("data-id"),
            )
        } else {
            (store.state.on_pane_context_menu.get_untracked(), None)
        };

        let Some(callback) = callback else {
            return; // no handler registered: keep the native menu
        };

        ev.prevent_default();
        ev.stop_propagation();

        let screen_x = ev.client_x() as f64;
        let screen_y = ev.client_y() as f64;
        let viewport = store.get_viewport_untracked();
        let flow = screen_to_flow_position_with_ref(
            screen_x,
            screen_y,
            &viewport,
            store.state.container_ref,
        );

        callback.run(ContextMenuEvent {
            id,
            screen_x,
            screen_y,
            flow_x: flow.x,
            flow_y: flow.y,
        });
    }
}
