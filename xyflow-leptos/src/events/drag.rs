//! Node dragging event handlers (pointer-capture based)

use leptos::prelude::*;
use leptos::ev;
use leptos::wasm_bindgen::JsCast;
use crate::hooks::use_flow_store;
use crate::types::Position;

/// State for tracking a drag operation
#[derive(Clone, Copy, Debug, Default)]
struct DragState {
    /// Whether we're currently dragging
    is_dragging: bool,
    /// Pointer that started the drag (ignore moves from other pointers)
    pointer_id: i32,
    /// Starting pointer position (screen coordinates)
    start_pointer: (f64, f64),
    /// Starting node position (flow coordinates)
    start_node_pos: Position,
    /// Whether the node actually moved during the drag
    moved: bool,
}

/// Hook for node dragging functionality.
///
/// Returns `(on_pointer_down, on_pointer_move, on_pointer_up)` handlers to
/// attach to a node's root element:
///
/// ```ignore
/// let (on_pointer_down, on_pointer_move, on_pointer_up) =
///     use_node_drag_handlers(node_id.clone());
/// view! {
///     <div
///         class="xyflow__node"
///         on:pointerdown=on_pointer_down
///         on:pointermove=on_pointer_move
///         on:pointerup=on_pointer_up.clone()
///         on:pointercancel=on_pointer_up
///     >
///         ...
///     </div>
/// }
/// ```
///
/// The pointerdown handler captures the pointer on the node element, so the
/// drag keeps tracking even when the cursor leaves the node — no document
/// level listeners or global state required. Screen deltas are converted to
/// flow coordinates using the current zoom, and positions are written to the
/// store as the drag progresses.
///
/// When the drag finishes (and the node actually moved), the store's
/// `on_node_drag_end` callback fires with `(node_id, final_position)` —
/// register one via [`crate::FlowStore::set_on_node_drag_end`] to persist
/// positions.
pub fn use_node_drag_handlers(
    node_id: String,
) -> (
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_down
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_move
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_up / on_pointer_cancel
) {
    let store = use_flow_store();

    // Per-node drag state (no globals)
    let drag_state = RwSignal::new(DragState::default());

    // Pointer down - start drag and capture the pointer
    let on_pointer_down = {
        let node_id = node_id.clone();
        move |ev: ev::PointerEvent| {
            // Primary button only
            if ev.button() != 0 {
                return;
            }

            // prevent_default suppresses the compatibility mouse events, so
            // the viewport's pan handler does not also engage; stop_propagation
            // keeps ancestor flows (sub-flows) from reacting.
            ev.prevent_default();
            ev.stop_propagation();

            // Capture the pointer on the element the handler is attached to
            if let Some(target) = ev.current_target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    let _ = element.set_pointer_capture(ev.pointer_id());
                }
            }

            // prevent_default suppressed native focus-on-click; focus the
            // container explicitly so keyboard shortcuts (Delete on the
            // selection this click makes, Escape) keep working.
            store.focus_container();

            let nodes = store.get_nodes_untracked();
            if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                drag_state.set(DragState {
                    is_dragging: true,
                    pointer_id: ev.pointer_id(),
                    start_pointer: (ev.client_x() as f64, ev.client_y() as f64),
                    start_node_pos: node.position,
                    moved: false,
                });

                store.update_node(&node_id, |node| {
                    node.dragging = true;
                });
            }
        }
    };

    // Pointer move - update position (delta in screen px divided by zoom)
    let on_pointer_move = {
        let node_id = node_id.clone();
        move |ev: ev::PointerEvent| {
            let state = drag_state.get_untracked();
            if !state.is_dragging || ev.pointer_id() != state.pointer_id {
                return;
            }

            ev.prevent_default();

            let zoom = store.get_viewport_untracked().zoom.max(f64::EPSILON);
            let dx = (ev.client_x() as f64 - state.start_pointer.0) / zoom;
            let dy = (ev.client_y() as f64 - state.start_pointer.1) / zoom;

            let new_position = Position::new(
                state.start_node_pos.x + dx,
                state.start_node_pos.y + dy,
            );

            if dx != 0.0 || dy != 0.0 {
                drag_state.update(|s| s.moved = true);
            }

            store.update_node(&node_id, |node| {
                node.position = new_position;
            });
        }
    };

    // Pointer up / cancel - end drag, release capture, notify
    let on_pointer_up = {
        let node_id = node_id.clone();
        move |ev: ev::PointerEvent| {
            let state = drag_state.get_untracked();
            if !state.is_dragging || ev.pointer_id() != state.pointer_id {
                return;
            }

            if let Some(target) = ev.current_target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    let _ = element.release_pointer_capture(ev.pointer_id());
                }
            }

            store.update_node(&node_id, |node| {
                node.dragging = false;
            });

            drag_state.set(DragState::default());

            // Notify the consumer so positions can be persisted
            if state.moved {
                if let Some(callback) = store.state.on_node_drag_end.get_untracked() {
                    let final_pos = store
                        .get_nodes_untracked()
                        .iter()
                        .find(|n| n.id == node_id)
                        .map(|n| n.position)
                        .unwrap_or(state.start_node_pos);
                    callback.run((node_id.clone(), final_pos));
                }
            }
        }
    };

    (on_pointer_down, on_pointer_move, on_pointer_up)
}
