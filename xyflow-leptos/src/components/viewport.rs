//! Viewport component that applies pan and zoom transforms

use leptos::prelude::*;
use crate::store::FlowStore;
use crate::hooks::use_flow_store;
use crate::events::{use_pan_handlers, use_zoom_handler};

/// The viewport component that wraps all flow content
///
/// This component applies the viewport transform (pan and zoom) to all child elements.
/// It renders a div with a CSS transform that translates and scales the content.
///
/// # Example
///
/// ```ignore
/// view! {
///     <Viewport>
///         <NodeRenderer />
///         <EdgeRenderer />
///     </Viewport>
/// }
/// ```
#[component]
pub fn Viewport(
    /// Optional store - if not provided, uses context
    #[prop(optional)]
    store: Option<FlowStore>,
    /// Child components to render inside the viewport
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Use provided store or get from context
    let store = store.unwrap_or_else(use_flow_store);

    // Create a reactive transform string based on viewport state
    let transform = move || {
        let viewport = store.get_viewport();
        format!(
            "translate({}px, {}px) scale({})",
            viewport.x, viewport.y, viewport.zoom
        )
    };

    // Set up event handlers for pan and zoom
    let (on_mouse_down, on_mouse_move, on_mouse_up) = use_pan_handlers();
    let on_wheel = use_zoom_handler();

    view! {
        <div
            class="xyflow__viewport leptos-flow__viewport"
            style:transform=transform
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:wheel=on_wheel
        >
            {children.map(|children| children())}
        </div>
    }
}

#[cfg(test)]
mod tests {
    // Note: Component tests require a browser environment
    // These are placeholder tests for the structure

    #[test]
    fn test_viewport_component_exists() {
        // This test just ensures the component compiles
        // Real rendering tests would need wasm-bindgen-test
        assert!(true);
    }
}

