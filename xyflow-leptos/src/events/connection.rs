//! Connection creation event handlers (pointer-capture based)

use leptos::prelude::*;
use leptos::ev;
use leptos::wasm_bindgen::JsCast;
use crate::hooks::use_flow_store;
use crate::store::{ConnectionCandidate, FlowStore};
use crate::types::{Position, HandleType, ConnectionMode, IsValidConnection};
use crate::utils::coordinate::screen_to_flow_position_with_ref;
use crate::utils::handle::{evaluate_connection_candidate, CONNECTION_RADIUS};

/// Hook returning the pointer handlers that drive a connection drag from a
/// handle: `(on_pointer_down, on_pointer_move, on_pointer_up)`.
///
/// The `Handle` component attaches these to its element. Like node dragging
/// ([`crate::events::use_node_drag_handlers`]), the pointerdown handler
/// captures the pointer on the handle element, so the whole drag —
/// move updates, drop, cancel — keeps tracking on the same element with no
/// document-level listeners:
///
/// * pointerdown  → `FlowStore::start_connection` (line starts at the
///   handle's measured center)
/// * pointermove  → hit-test against the store's measured handle bounds,
///   validate, `FlowStore::update_connection` (the connection line follows
///   the cursor and snaps to candidate handles)
/// * pointerup    → re-evaluate at the drop position, then
///   `FlowStore::complete_connection` (fires `on_connect` / adds an edge) or,
///   with no valid candidate, cancels cleanly
/// * pointercancel → `FlowStore::cancel_connection`
///
/// Validity combines the connection mode, the built-in rules, the
/// per-handle `is_valid_connection` fn, and the store-level predicate
/// registered via [`FlowStore::set_is_valid_connection`].
///
/// # Arguments
///
/// * `node_id` - ID of the node containing the handle
/// * `handle_id` - Optional handle ID (None for default handle)
/// * `handle_type` - Type of the handle (Source or Target)
/// * `connection_mode` - Strict (opposite types only) or Loose
/// * `is_valid_connection` - Optional per-handle validation fn
/// * `connectable_start` - Whether drags may start from this handle
///   (`is_connectable && is_connectable_start`); when false, pointerdown
///   only stops propagation so the node underneath does not start dragging
pub fn use_connection_handlers(
    node_id: String,
    handle_id: Option<String>,
    handle_type: HandleType,
    connection_mode: ConnectionMode,
    is_valid_connection: Option<IsValidConnection>,
    connectable_start: bool,
) -> (
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_down
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_move
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_up / on_pointer_cancel
) {
    let store = use_flow_store();

    // Pointer that owns the in-flight connection drag (per-handle, no globals)
    let active_pointer: RwSignal<Option<i32>> = RwSignal::new(None);

    // Pointer down - start the connection and capture the pointer
    let on_pointer_down = {
        let node_id = node_id.clone();
        let handle_id = handle_id.clone();
        move |ev: ev::PointerEvent| {
            // Always keep the event from reaching the node's drag handler:
            // interacting with a handle is never a node drag.
            ev.stop_propagation();

            if !connectable_start || ev.button() != 0 {
                return;
            }

            // Suppress the compatibility mouse events (text selection, pane
            // pan fallback) for the duration of the drag.
            ev.prevent_default();

            // Capture the pointer on the handle element so move/up keep
            // firing here even when the cursor leaves the handle.
            if let Some(target) = ev.current_target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    let _ = element.set_pointer_capture(ev.pointer_id());
                }
            }

            active_pointer.set(Some(ev.pointer_id()));

            // prevent_default suppressed native focus-on-click; focus the
            // container explicitly so Escape can cancel this drag.
            store.focus_container();

            store.start_connection(
                node_id.clone(),
                handle_id.clone(),
                handle_type,
                handle_start_position(&store, &ev, &node_id, handle_id.as_deref(), handle_type),
            );
        }
    };

    // Pointer move - track the cursor, snap + validate against handle bounds
    let on_pointer_move = {
        let node_id = node_id.clone();
        let handle_id = handle_id.clone();
        move |ev: ev::PointerEvent| {
            if active_pointer.get_untracked() != Some(ev.pointer_id()) {
                return;
            }
            ev.prevent_default();

            evaluate_and_update(
                &store,
                &ev,
                &node_id,
                handle_id.as_deref(),
                handle_type,
                connection_mode,
                is_valid_connection,
            );
        }
    };

    // Pointer up / cancel - complete on a valid candidate, cancel otherwise
    let on_pointer_up = {
        let node_id = node_id.clone();
        let handle_id = handle_id.clone();
        move |ev: ev::PointerEvent| {
            if active_pointer.get_untracked() != Some(ev.pointer_id()) {
                return;
            }
            active_pointer.set(None);

            if let Some(target) = ev.current_target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    let _ = element.release_pointer_capture(ev.pointer_id());
                }
            }

            if ev.type_() == "pointercancel" {
                store.cancel_connection();
                return;
            }

            // Re-evaluate at the drop position so completion never trusts a
            // stale move, then complete (no-op cancel when invalid/no
            // candidate — the in-flight state is always cleared).
            evaluate_and_update(
                &store,
                &ev,
                &node_id,
                handle_id.as_deref(),
                handle_type,
                connection_mode,
                is_valid_connection,
            );
            store.complete_connection();
        }
    };

    (on_pointer_down, on_pointer_move, on_pointer_up)
}

/// Hit-test the pointer against the store's measured handle bounds, run the
/// validity layers, and push the result into the in-flight connection state.
fn evaluate_and_update(
    store: &FlowStore,
    ev: &ev::PointerEvent,
    from_node_id: &str,
    from_handle_id: Option<&str>,
    from_handle_type: HandleType,
    connection_mode: ConnectionMode,
    is_valid_connection: Option<IsValidConnection>,
) {
    let viewport = store.get_viewport_untracked();
    let position = screen_to_flow_position_with_ref(
        ev.client_x() as f64,
        ev.client_y() as f64,
        &viewport,
        store.state.container_ref,
    );

    let nodes = store.get_nodes_untracked();
    let host_validator = store.state.is_valid_connection.get_untracked();
    let extra = host_validator.map(|callback| move |conn: &crate::types::Connection| {
        callback.run(conn.clone())
    });

    let evaluation = evaluate_connection_candidate(
        &nodes,
        position,
        CONNECTION_RADIUS,
        from_node_id,
        from_handle_id,
        from_handle_type,
        connection_mode,
        is_valid_connection,
        extra.as_ref().map(|f| f as &dyn Fn(&crate::types::Connection) -> bool),
    );

    // Snap the line's free end to the candidate handle center when in range
    let to_position = evaluation
        .candidate
        .as_ref()
        .and_then(|closest| {
            nodes
                .iter()
                .find(|n| n.id == closest.node_id)
                .map(|node| closest.handle.center_absolute(&node.position))
        })
        .unwrap_or(position);

    let candidate = evaluation.candidate.map(|closest| ConnectionCandidate {
        node_id: closest.node_id,
        handle_id: closest.handle.id,
        handle_type: closest.handle.handle_type,
    });

    store.update_connection(to_position, candidate, evaluation.is_valid);
}

/// Flow-space position the connection line starts from: the handle's
/// measured bound (registered by the `Handle` component) when available,
/// falling back to the DOM rect of the pressed element, then to the cursor.
fn handle_start_position(
    store: &FlowStore,
    ev: &ev::PointerEvent,
    node_id: &str,
    handle_id: Option<&str>,
    handle_type: HandleType,
) -> Position {
    let nodes = store.get_nodes_untracked();
    if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
        if let Some(bound) = node
            .internals
            .handle_bounds
            .as_ref()
            .and_then(|bounds| bounds.find_handle(handle_id, handle_type))
        {
            return bound.center_absolute(&node.position);
        }
    }

    // Fallback: center of the element the pointer went down on
    let viewport = store.get_viewport_untracked();
    let (screen_x, screen_y) = ev
        .current_target()
        .and_then(|target| {
            target.dyn_ref::<web_sys::Element>().map(|element| {
                let rect = element.get_bounding_client_rect();
                (
                    rect.left() + rect.width() / 2.0,
                    rect.top() + rect.height() / 2.0,
                )
            })
        })
        .unwrap_or((ev.client_x() as f64, ev.client_y() as f64));

    screen_to_flow_position_with_ref(screen_x, screen_y, &viewport, store.state.container_ref)
}

/// Get the position of a handle in flow coordinates
///
/// This calculates the center position of a handle element.
/// In a full implementation, this would use DOM measurements.
///
/// # Arguments
///
/// * `node_position` - Position of the parent node
/// * `node_width` - Width of the parent node
/// * `node_height` - Height of the parent node
/// * `handle_position` - Position of the handle (Top, Right, Bottom, Left)
///
/// # Returns
///
/// Position of the handle center in flow coordinates
pub fn calculate_handle_position(
    node_position: Position,
    node_width: f64,
    node_height: f64,
    handle_position: &str,
) -> Position {
    match handle_position {
        "top" => Position::new(
            node_position.x + node_width / 2.0,
            node_position.y,
        ),
        "right" => Position::new(
            node_position.x + node_width,
            node_position.y + node_height / 2.0,
        ),
        "bottom" => Position::new(
            node_position.x + node_width / 2.0,
            node_position.y + node_height,
        ),
        "left" => Position::new(
            node_position.x,
            node_position.y + node_height / 2.0,
        ),
        _ => Position::new(
            node_position.x + node_width / 2.0,
            node_position.y + node_height / 2.0,
        ),
    }
}
