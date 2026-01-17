//! Hook to access the flow store

use leptos::prelude::use_context;
use crate::store::FlowStore;

/// Get the flow store from context
///
/// This hook provides access to the store containing all flow state and actions.
/// It must be called within a component inside a `SvelteFlow` provider.
///
/// # Panics
///
/// Panics if called outside of a `SvelteFlow` provider context.
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let store = use_flow_store();
///     let nodes = store.get_nodes();
///     view! { <div>{nodes.len()}</div> }
/// }
/// ```
pub fn use_flow_store() -> FlowStore {
    use_context::<FlowStore>()
        .expect("use_flow_store must be called inside a SvelteFlow provider")
}
