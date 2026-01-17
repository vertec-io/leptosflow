//! Hook to access nodes from the flow

use crate::types::Node;
use super::use_flow_store::use_flow_store;

/// Get the nodes from the flow store
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let nodes = use_nodes();
///     view! {
///         <div>{nodes.len()}</div>
///     }
/// }
/// ```
pub fn use_nodes() -> Vec<Node> {
    let store = use_flow_store();
    store.get_nodes()
}
