//! Node renderer component

use leptos::prelude::*;
use crate::hooks::use_flow_store;
use crate::types::Node;
use crate::events::use_node_drag_handlers;

/// Renders a single node
///
/// This component renders a node with dragging and selection support.
#[component]
fn NodeComponent(
    /// The node to render
    node: Node,
) -> impl IntoView {
    let node_id = node.id.clone();
    let store = use_flow_store();

    // Get drag handlers
    let (on_mouse_down, on_mouse_move, on_mouse_up) = use_node_drag_handlers(node_id.clone());

    // Click handler for selection
    let on_click = {
        let node_id = node_id.clone();
        let store = store.clone();
        move |ev: leptos::ev::MouseEvent| {
            // Check if Ctrl/Cmd key is pressed for multi-select
            let multi_select = ev.ctrl_key() || ev.meta_key();

            // If node is already selected and multi-select is active, deselect it
            if node.selected && multi_select {
                store.deselect_node(&node_id);
            } else {
                store.select_node(&node_id, multi_select);
            }
        }
    };

    // Use transform instead of left/top for better performance
    let style = {
        let transform = format!("translate({}px, {}px)", node.position.x, node.position.y);
        let width = node.width.unwrap_or(150.0);
        let height = node.height.unwrap_or(40.0);
        let cursor = if node.dragging { "grabbing" } else { "grab" };
        format!(
            "position: absolute; transform: {}; width: {}px; height: {}px; cursor: {};",
            transform, width, height, cursor
        )
    };

    let class = {
        let mut classes = vec!["xyflow__node", "leptos-flow__node"];
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
    };

    view! {
        <div
            class=class
            style=style
            data-id=node.id.clone()
            on:click=on_click
            on:mousedown=on_mouse_down
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
        >
            <div class="xyflow__node-content">
                {format!("Node {}", node.id)}
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
                        <NodeComponent node=node />
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

