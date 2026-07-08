//! Pan event handlers for viewport panning

use leptos::prelude::*;
use leptos::ev;
use crate::hooks::use_flow_store;

/// Movement (in screen px) before a pointer-down on the pane becomes a pan.
///
/// Below this the gesture is treated as a click, so edge selection and
/// pane-click handlers keep working.
const PAN_START_THRESHOLD_PX: f64 = 3.0;

/// State for a pointer-capture pane pan
#[derive(Clone, Copy, Debug, Default)]
struct PanePanState {
    /// A primary-button pointer went down on the pane
    active: bool,
    /// The movement threshold was exceeded and we are actually panning
    panning: bool,
    /// Pointer that started the gesture (ignore other pointers)
    pointer_id: i32,
    /// Pointer position at pointer-down (screen coordinates)
    start: (f64, f64),
    /// Last pointer position (screen coordinates)
    last: (f64, f64),
}

/// Hook for panning the viewport by dragging the empty pane.
///
/// Returns `(on_pointer_down, on_pointer_move, on_pointer_up)` to attach to
/// the **untransformed** flow container (`SvelteFlow` wires these up).
/// Interaction rules:
///
/// * Only the primary button engages, and only when the store's
///   `pan_on_drag` is enabled.
/// * Node drags and handle connections are unaffected: their pointer-down
///   handlers call `stop_propagation`, so the container never sees them.
/// * A pan only starts after the pointer moves [`PAN_START_THRESHOLD_PX`];
///   plain clicks (edge selection, pane clicks) are untouched because the
///   pointer is only captured once the threshold is exceeded.
/// * Once panning, the pointer is captured on the container (via the
///   store's `container_ref` — `current_target` is unreliable under Leptos'
///   window-delegated event dispatch) so the drag keeps tracking outside
///   the element.
///
/// All signal reads are untracked: these run in event handlers and must not
/// leave reactive subscriptions in the caller's scope.
pub fn use_pane_pan_handlers() -> (
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_down
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_move
    impl Fn(ev::PointerEvent) + Clone, // on_pointer_up / on_pointer_cancel
) {
    let store = use_flow_store();
    let pan_state = RwSignal::new(PanePanState::default());

    let on_pointer_down = move |ev: ev::PointerEvent| {
        // Primary button only
        if ev.button() != 0 {
            return;
        }
        if !store.state.pan_on_drag.get_untracked() {
            return;
        }
        let pos = (ev.client_x() as f64, ev.client_y() as f64);
        pan_state.set(PanePanState {
            active: true,
            panning: false,
            pointer_id: ev.pointer_id(),
            start: pos,
            last: pos,
        });
        // No prevent_default / capture yet: a click that never moves must
        // still reach edges and pane-click handlers untouched.
    };

    let on_pointer_move = move |ev: ev::PointerEvent| {
        let state = pan_state.get_untracked();
        if !state.active || ev.pointer_id() != state.pointer_id {
            return;
        }

        let current = (ev.client_x() as f64, ev.client_y() as f64);

        if !state.panning {
            let dist_x = current.0 - state.start.0;
            let dist_y = current.1 - state.start.1;
            if (dist_x * dist_x + dist_y * dist_y).sqrt() < PAN_START_THRESHOLD_PX {
                return;
            }
            // Threshold exceeded: this is a pan, not a click. Capture the
            // pointer on the container so the drag survives leaving it.
            // (Not ev.current_target(): leptos delegates bubbling events to
            // a window-level listener, so current_target is the window.)
            if let Some(container) = store.state.container_ref.get_untracked() {
                let _ = container.set_pointer_capture(ev.pointer_id());
            }
        }

        ev.prevent_default();
        store.pan_by(current.0 - state.last.0, current.1 - state.last.1);
        pan_state.update(|s| {
            s.panning = true;
            s.last = current;
        });
    };

    let on_pointer_up = move |ev: ev::PointerEvent| {
        let state = pan_state.get_untracked();
        if !state.active || ev.pointer_id() != state.pointer_id {
            return;
        }
        if state.panning {
            if let Some(container) = store.state.container_ref.get_untracked() {
                let _ = container.release_pointer_capture(ev.pointer_id());
            }
        }
        pan_state.set(PanePanState::default());
    };

    (on_pointer_down, on_pointer_move, on_pointer_up)
}

/// State for tracking legacy mouse-event pans
#[derive(Clone, Copy, Debug, Default)]
struct PanState {
    /// Whether we're currently panning
    is_panning: bool,
    /// Last mouse X position
    last_x: f64,
    /// Last mouse Y position
    last_y: f64,
}

/// Legacy mouse-event pan handlers.
///
/// Superseded by [`use_pane_pan_handlers`], which uses pointer capture (the
/// drag keeps tracking when the cursor leaves the element) and a movement
/// threshold (plain clicks still work). Kept for API compatibility.
#[deprecated(note = "use `use_pane_pan_handlers` (pointer capture + click-preserving move threshold)")]
pub fn use_pan_handlers() -> (
    impl Fn(ev::MouseEvent) + Clone,
    impl Fn(ev::MouseEvent) + Clone,
    impl Fn(ev::MouseEvent) + Clone,
) {
    let store = use_flow_store();
    let pan_state = RwSignal::new(PanState::default());

    // Mouse down - start panning
    let on_mouse_down = move |event: ev::MouseEvent| {
        // Only pan with left mouse button
        if event.button() == 0 {
            let pan_on_drag = store.state.pan_on_drag.get_untracked();
            if pan_on_drag {
                pan_state.update(|state| {
                    state.is_panning = true;
                    state.last_x = event.client_x() as f64;
                    state.last_y = event.client_y() as f64;
                });
                event.prevent_default();
            }
        }
    };

    // Mouse move - perform panning
    let on_mouse_move = move |event: ev::MouseEvent| {
        let state = pan_state.get_untracked();
        if state.is_panning {
            let current_x = event.client_x() as f64;
            let current_y = event.client_y() as f64;

            let dx = current_x - state.last_x;
            let dy = current_y - state.last_y;

            // Pan the viewport
            store.pan_by(dx, dy);

            // Update last position
            pan_state.update(|state| {
                state.last_x = current_x;
                state.last_y = current_y;
            });

            event.prevent_default();
        }
    };

    // Mouse up - stop panning
    let on_mouse_up = move |_event: ev::MouseEvent| {
        pan_state.update(|state| {
            state.is_panning = false;
        });
    };

    (on_mouse_down, on_mouse_move, on_mouse_up)
}

#[cfg(test)]
mod tests {
    use super::PAN_START_THRESHOLD_PX;

    #[test]
    fn test_pan_threshold_is_small_but_nonzero() {
        // Plain clicks (selection clears, edge clicks) must survive a couple
        // of pixels of jitter, but panning must engage almost immediately.
        assert!(PAN_START_THRESHOLD_PX >= 1.0);
        assert!(PAN_START_THRESHOLD_PX <= 5.0);
    }
}
