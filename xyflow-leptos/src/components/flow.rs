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
    nodes: RwSignal<Vec<Node>>,
    edges: RwSignal<Vec<Edge>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Create the store from the signals
    let store = FlowStore::from_signals(nodes, edges);

    // Provide the store to child components
    provide_context(store);

    view! {
        <div class="svelte-flow">
            {children.map(|children| children())}
        </div>
    }
}
