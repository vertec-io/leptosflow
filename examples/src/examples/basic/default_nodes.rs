//! Default Nodes Example
//!
//! Demonstrates the three default node types: input, default, and output.
//! - Input node: has only a source handle (bottom)
//! - Default node: has both source (bottom) and target (top) handles
//! - Output node: has only a target handle (top)

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{DraggableNode, get_drag_signal, SourceCodeViewer};

/// Default node types example showing input, default, and output nodes
#[component]
pub fn DefaultNodesExample() -> impl IntoView {
    // Create initial nodes with different types
    let initial_nodes = vec![
        Node::new("input".to_string(), Position::new(150.0, 25.0))
            .with_data(json!({"label": "Input Node", "type": "input", "class": "light"})),
        Node::new("default".to_string(), Position::new(150.0, 125.0))
            .with_data(json!({"label": "Default Node", "type": "default", "class": "light"})),
        Node::new("output".to_string(), Position::new(150.0, 225.0))
            .with_data(json!({"label": "Output Node", "type": "output", "class": "light"})),
    ];

    // Create edges connecting the nodes in a flow
    let initial_edges = vec![
        Edge::new("e-input-default".to_string(), "input".to_string(), "default".to_string()),
        Edge::new("e-default-output".to_string(), "default".to_string(), "output".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_drag_signal();

    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let current_x = ev.client_x() as f64;
            let current_y = ev.client_y() as f64;
            let (start_x, start_y) = drag_state.start_mouse;
            let (node_start_x, node_start_y) = drag_state.start_pos;

            // Calculate delta accounting for zoom
            let viewport = store.get_viewport();
            let dx = (current_x - start_x) / viewport.zoom;
            let dy = (current_y - start_y) / viewport.zoom;

            // Update node position
            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(node_start_x + dx, node_start_y + dy);
            });
        }
    };

    let on_global_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            store.update_node(&drag_state.node_id, |n| {
                n.dragging = false;
            });
            drag_signal.set(None);
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background with dots
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <DraggableNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info panel (top-right)
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
                        <strong style="display: block; margin-bottom: 8px;">"Default Node Types"</strong>
                        <div style="font-size: 12px; line-height: 1.5;">
                            <div style="margin-bottom: 4px;">
                                <span style="color: #1a192b; font-weight: 500;">"Input: "</span>
                                "Source handle only (bottom)"
                            </div>
                            <div style="margin-bottom: 4px;">
                                <span style="color: #1a192b; font-weight: 500;">"Default: "</span>
                                "Both handles (top + bottom)"
                            </div>
                            <div>
                                <span style="color: #1a192b; font-weight: 500;">"Output: "</span>
                                "Target handle only (top)"
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>

            <SourceCodeViewer
                source=include_str!("default_nodes.rs")
                title="default_nodes.rs"
            />
        </div>
    }
}
