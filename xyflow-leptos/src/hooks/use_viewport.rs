//! Hook to access viewport state

use crate::types::Viewport;
use super::use_flow_store::use_flow_store;

/// Get the viewport from the flow store
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let viewport = use_viewport();
///     view! {
///         <div>
///             {format!("x: {}, y: {}, zoom: {}", viewport.x, viewport.y, viewport.zoom)}
///         </div>
///     }
/// }
/// ```
pub fn use_viewport() -> Viewport {
    let store = use_flow_store();
    store.get_viewport()
}
