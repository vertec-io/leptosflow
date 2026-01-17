//! Zoom event handlers for viewport zooming

use leptos::ev;
use crate::hooks::use_flow_store;

/// Hook that provides a wheel event handler for zooming the viewport
///
/// This hook sets up a wheel event handler that zooms the viewport in/out
/// based on the mouse wheel delta.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn Viewport() -> impl IntoView {
///     let on_wheel = use_zoom_handler();
///     
///     view! {
///         <div on:wheel=on_wheel>
///             // viewport content
///         </div>
///     }
/// }
/// ```
pub fn use_zoom_handler() -> impl Fn(ev::WheelEvent) + Clone {
    let store = use_flow_store();

    move |event: ev::WheelEvent| {
        // Prevent default scrolling behavior
        event.prevent_default();
        
        // Get the wheel delta
        let delta_y = event.delta_y();
        
        // Calculate zoom factor
        // Negative delta = zoom in, positive delta = zoom out
        // We use a small factor to make zooming smooth
        let zoom_factor = if delta_y < 0.0 {
            1.1 // Zoom in by 10%
        } else {
            0.9 // Zoom out by 10%
        };
        
        // Apply zoom
        store.zoom_by(zoom_factor);
    }
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
        let current_viewport = store.get_viewport();
        let mut new_viewport = current_viewport;
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

