//! Wheel event handling for the viewport: trackpad pan + pinch/ctrl zoom

use leptos::prelude::*;
use leptos::ev;
use crate::hooks::use_flow_store;
use crate::utils::math::{wheel_delta_scale, wheel_zoom_factor};

/// Hook that provides the wheel handler for the flow container, matching
/// xyflow/react-flow trackpad conventions:
///
/// * `wheel` without `ctrlKey` **pans** by `(deltaX, deltaY)` — a two-finger
///   trackpad scroll moves the canvas.
/// * `wheel` with `ctrlKey` (trackpad pinch fires this, and so does
///   ctrl+scroll) **zooms**, centered on the cursor: the flow point under the
///   pointer stays stationary while the scale changes exponentially.
///
/// Deltas are normalized per `deltaMode` (line mode is ~16 px per unit) so
/// Firefox line-scrolling feels like Chrome pixel-scrolling.
///
/// Attach to the **untransformed** flow container (`SvelteFlow` does this),
/// not the transformed `.xyflow__viewport` element — the transformed element
/// slides out from under the cursor as soon as the view pans or zooms.
///
/// `preventDefault` is called so the page never scrolls or browser-zooms.
/// Do NOT attach this with `on:wheel`: leptos delegates bubbling events to a
/// single window-level listener, and browsers force wheel listeners on
/// window/document/body to be passive, which silently disables
/// `preventDefault`. Attach it directly to the element via web_sys with
/// `AddEventListenerOptions { passive: false }`, as `SvelteFlow` does.
///
/// All signal reads are untracked: this runs in event handlers and must not
/// leave reactive subscriptions in scope.
pub fn use_wheel_handler() -> impl Fn(ev::WheelEvent) + Clone {
    let store = use_flow_store();

    move |event: ev::WheelEvent| {
        // Keep the browser from scrolling/zooming the page
        event.prevent_default();

        let scale = wheel_delta_scale(event.delta_mode());
        let dx = event.delta_x() * scale;
        let dy = event.delta_y() * scale;

        if event.ctrl_key() || event.meta_key() {
            // Pinch gesture / ctrl+scroll: zoom centered on the cursor.
            // Cursor position relative to the flow container's top-left,
            // the same space the viewport CSS transform maps into.
            let (px, py) = match store.state.container_ref.get_untracked() {
                Some(container) => {
                    let rect = container.get_bounding_client_rect();
                    (
                        event.client_x() as f64 - rect.left(),
                        event.client_y() as f64 - rect.top(),
                    )
                }
                // Container not mounted yet: zoom around the origin
                None => (0.0, 0.0),
            };
            store.zoom_at(wheel_zoom_factor(dy), px, py);
        } else {
            // Two-finger scroll: pan. Content follows scroll direction
            // (scroll down moves the canvas content up), like xyflow's
            // panOnScroll.
            store.pan_by(-dx, -dy);
        }
    }
}

/// Deprecated name for [`use_wheel_handler`].
#[deprecated(note = "use `use_wheel_handler`; wheel now pans, and ctrl+wheel/pinch zooms at the cursor")]
pub fn use_zoom_handler() -> impl Fn(ev::WheelEvent) + Clone {
    use_wheel_handler()
}

/// Hook that provides zoom in/out functions for programmatic zooming
///
/// Returns a tuple of (zoom_in, zoom_out, zoom_to) functions.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn Controls() -> impl IntoView {
///     let (zoom_in, zoom_out, zoom_to) = use_zoom_controls();
///     
///     view! {
///         <button on:click=move |_| zoom_in()>"+"</button>
///         <button on:click=move |_| zoom_out()>"-"</button>
///         <button on:click=move |_| zoom_to(1.0)>"Reset"</button>
///     }
/// }
/// ```
pub fn use_zoom_controls() -> (
    impl Fn() + Clone,
    impl Fn() + Clone,
    impl Fn(f64) + Clone,
) {
    let store = use_flow_store();

    let zoom_in = move || {
        store.zoom_by(1.2);
    };

    let zoom_out = move || {
        store.zoom_by(0.8);
    };

    let zoom_to = move |zoom: f64| {
        // Untracked: called from event handlers; a tracked read here would
        // subscribe the caller's scope to the viewport signal.
        let mut new_viewport = store.get_viewport_untracked();
        new_viewport.zoom = zoom;
        store.set_viewport(new_viewport);
    };

    (zoom_in, zoom_out, zoom_to)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_zoom_handler_exists() {
        // Placeholder test - real tests need browser environment
        assert!(true);
    }
}

