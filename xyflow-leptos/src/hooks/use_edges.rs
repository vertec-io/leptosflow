//! Hook to access edges from the flow

use crate::types::Edge;
use super::use_flow_store::use_flow_store;

/// Get the edges from the flow store
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let edges = use_edges();
///     view! {
///         <div>{edges.len()}</div>
///     }
/// }
/// ```
pub fn use_edges() -> Vec<Edge> {
    let store = use_flow_store();
    store.get_edges()
}
