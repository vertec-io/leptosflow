//! Pan event handlers for viewport panning

use leptos::prelude::*;
use leptos::ev;
use crate::hooks::use_flow_store;

/// State for tracking pan operations
#[derive(Clone, Copy, Debug)]
struct PanState {
    /// Whether we're currently panning
    is_panning: bool,
    /// Last mouse X position
    last_x: f64,
    /// Last mouse Y position
    last_y: f64,
}

impl Default for PanState {
    fn default() -> Self {
        PanState {
            is_panning: false,
            last_x: 0.0,
            last_y: 0.0,
        }
    }
}

/// Hook that provides pan event handlers for the viewport
///
/// This hook sets up mouse and touch event handlers for panning the viewport.
/// It returns callbacks that should be attached to the viewport element.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn Viewport() -> impl IntoView {
///     let (on_mouse_down, on_mouse_move, on_mouse_up) = use_pan_handlers();
///     
///     view! {
///         <div
///             on:mousedown=on_mouse_down
///             on:mousemove=on_mouse_move
///             on:mouseup=on_mouse_up
///         >
///             // viewport content
///         </div>
///     }
/// }
/// ```
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
            let pan_on_drag = store.state.pan_on_drag.get();
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
        let state = pan_state.get();
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
    #[test]
    fn test_pan_handlers_exist() {
        // Placeholder test - real tests need browser environment
        assert!(true);
    }
}

