//! Node renderer component

use leptos::prelude::*;
use crate::hooks::use_flow_store;
use crate::events::use_node_drag_handlers;

/// Renders a single node
///
/// This component renders a node with dragging and selection support.
/// All node state (position, selection, dragging) is read reactively from
/// the store, so the node tracks store updates (drags, programmatic moves).
#[component]
fn NodeComponent(
    /// The ID of the node to render
    node_id: String,
) -> impl IntoView {
    let store = use_flow_store();

    // Pointer-capture based drag handlers from the library
    let (on_pointer_down, on_pointer_move, on_pointer_up) =
        use_node_drag_handlers(node_id.clone());

    // Click handler for selection
    let on_click = {
        let node_id = node_id.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.stop_propagation();
            let multi_select = ev.ctrl_key() || ev.meta_key();

            let selected = store
                .get_nodes_untracked()
                .iter()
                .find(|n| n.id == node_id)
                .map(|n| n.selected)
                .unwrap_or(false);

            // If node is already selected and multi-select is active, deselect it
            if selected && multi_select {
                store.deselect_node(&node_id);
            } else {
                store.select_node(&node_id, multi_select);
            }
        }
    };

    // Reactive style: tracks position/size/drag state from the store.
    // Use transform instead of left/top for better performance.
    let style = {
        let node_id = node_id.clone();
        move || {
            let nodes = store.get_nodes();
            let Some(node) = nodes.iter().find(|n| n.id == node_id) else {
                return "display: none;".to_string();
            };
            let width = node.width.unwrap_or(150.0);
            let height = node.height.unwrap_or(40.0);
            let cursor = if node.dragging { "grabbing" } else { "grab" };
            format!(
                "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; cursor: {};",
                node.position.x, node.position.y, width, height, cursor
            )
        }
    };

    // Reactive class: tracks selection/drag/visibility state.
    let class = {
        let node_id = node_id.clone();
        move || {
            let nodes = store.get_nodes();
            let mut classes = vec!["xyflow__node", "leptos-flow__node"];
            if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                if node.selected {
                    classes.push("selected");
                }
                if node.dragging {
                    classes.push("dragging");
                }
                if node.hidden {
                    classes.push("hidden");
                }
                if let Some(ref class_name) = node.class_name {
                    classes.push(class_name);
                }
                classes.join(" ")
            } else {
                classes.join(" ")
            }
        }
    };

    let label = {
        let node_id = node_id.clone();
        move || {
            store
                .get_nodes()
                .iter()
                .find(|n| n.id == node_id)
                .and_then(|n| n.data.get("label").and_then(|v| v.as_str()).map(String::from))
                .unwrap_or_else(|| format!("Node {}", node_id))
        }
    };

    view! {
        <div
            class=class
            style=style
            data-id=node_id.clone()
            on:click=on_click
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up.clone()
            on:pointercancel=on_pointer_up
        >
            <div class="xyflow__node-content">
                {label}
            </div>
        </div>
    }
}

/// Renders all nodes in the flow
///
/// This component iterates over all nodes in the store and renders them.
#[component]
pub fn NodeRenderer() -> impl IntoView {
    let store = use_flow_store();

    // Get nodes reactively
    let nodes = move || store.get_nodes();

    view! {
        <div class="xyflow__nodes leptos-flow__nodes">
            <For
                each=nodes
                key=|node| node.id.clone()
                children=move |node| {
                    view! {
                        <NodeComponent node_id=node.id />
                    }
                }
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_node_renderer_exists() {
        // Placeholder test - real tests need browser environment
        assert!(true);
    }
}
