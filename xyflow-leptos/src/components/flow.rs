//! Main SvelteFlow component

use leptos::prelude::*;
use crate::types::{Node, Edge};
use crate::store::FlowStore;

/// The main flow component
///
/// This component sets up the flow context and renders the flow view.
///
/// # Props
///
/// * `nodes` - The flow nodes (as RwSignal)
/// * `edges` - The flow edges (as RwSignal)
/// * `children` - Child components to render (Controls, MiniMap, etc.)
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn App() -> impl IntoView {
///     let nodes = RwSignal::new(vec![
///         Node::new("1".to_string(), Position::new(0.0, 0.0)),
///     ]);
///     let edges = RwSignal::new(vec![]);
///
///     view! {
///         <SvelteFlow nodes edges>
///             <Controls />
///             <Background />
///         </SvelteFlow>
///     }
/// }
/// ```
#[component]
pub fn SvelteFlow(
    /// Node signal (ignored when `store` is provided)
    #[prop(optional)] nodes: Option<RwSignal<Vec<Node>>>,
    /// Edge signal (ignored when `store` is provided)
    #[prop(optional)] edges: Option<RwSignal<Vec<Edge>>>,
    /// Pre-built store to use instead of creating one from the signals.
    /// Lets the consumer keep a handle for `fit_view`, drag-end callbacks, etc.
    #[prop(optional)] store: Option<FlowStore>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use the provided store, or create one from the signals
    let store = store.unwrap_or_else(|| {
        FlowStore::from_signals(
            nodes.unwrap_or_else(|| RwSignal::new(Vec::new())),
            edges.unwrap_or_else(|| RwSignal::new(Vec::new())),
        )
    });

    // Provide the store to child components
    provide_context(store);

    view! {
        <div class="xyflow svelte-flow" node_ref=store.state.container_ref>
            {children.map(|children| children())}
        </div>
    }
}
