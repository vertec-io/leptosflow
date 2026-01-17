//! Default Node Overwrite Example
//!
//! Demonstrates how to customize the default node component by creating
//! a custom node renderer that replaces the standard node appearance.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::get_drag_signal;

/// Default node overwrite example showing custom node styling
#[component]
pub fn DefaultNodeOverwriteExample() -> impl IntoView {
    // Create initial nodes - these use the default "type" but will be rendered
    // with our custom overwritten component
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({"label": "Custom Default 1", "type": "default"})),
        Node::new("2".to_string(), Position::new(100.0, 175.0))
            .with_data(json!({"label": "Custom Default 2", "type": "default"})),
        Node::new("3".to_string(), Position::new(100.0, 300.0))
            .with_data(json!({"label": "Custom Default 3", "type": "default"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string()),
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
            let node_id = drag_state.node_id.clone();
            store.update_node(&node_id, |n| {
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
                // Background with cross pattern
                <Background variant=BackgroundVariant::Cross />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes with our custom overwritten default component
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <OverwrittenDefaultNode
                                    node=node.clone()
                                    store=store
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15);">
                        <strong style="display: block; margin-bottom: 8px;">"Default Node Overwrite"</strong>
                        <p style="margin: 0 0 8px 0; font-size: 12px; color: #666;">
                            "Custom styling for default nodes"
                        </p>
                        <div style="font-size: 11px; color: #888;">
                            <div style="margin: 4px 0;">"• Gradient background"</div>
                            <div style="margin: 4px 0;">"• Rounded corners"</div>
                            <div style="margin: 4px 0;">"• Custom shadow"</div>
                            <div style="margin: 4px 0;">"• Icon prefix"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Custom overwritten default node component
/// This replaces the standard default node appearance with a custom design
#[component]
fn OverwrittenDefaultNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(crate::shared::DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            // Mark node as dragging
            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
        }
    };

    // Get reactive node position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
        >
            // Custom styled node - this is our "overwritten default"
            <div
                class="overwritten-default-node"
                style="
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    border: none;
                    border-radius: 12px;
                    padding: 12px 20px;
                    min-width: 140px;
                    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
                    color: white;
                    font-weight: 500;
                    text-align: center;
                    position: relative;
                "
            >
                // Target handle (top)
                <Handle
                    node_id=node.id.clone()
                    r#type=HandleType::Target
                    position=HandlePosition::Top
                    connection_mode=ConnectionMode::Strict
                />

                // Node content with icon
                <div style="display: flex; align-items: center; justify-content: center; gap: 8px;">
                    <span style="font-size: 14px;">"◆"</span>
                    <span>{label}</span>
                </div>

                // Source handle (bottom)
                <Handle
                    node_id=node.id.clone()
                    r#type=HandleType::Source
                    position=HandlePosition::Bottom
                    connection_mode=ConnectionMode::Strict
                />
            </div>
        </div>
    }
}
