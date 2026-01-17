//! Custom Nodes Example
//!
//! Demonstrates how to create custom node components with different colors.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::get_drag_signal;

/// Custom nodes example with colored nodes
#[component]
pub fn CustomNodesExample() -> impl IntoView {
    // Create initial nodes with color data
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({"label": "Input Node", "type": "input", "color": "#6ede87"})),
        Node::new("2".to_string(), Position::new(100.0, 175.0))
            .with_data(json!({"label": "Processing", "type": "default", "color": "#6865A5"})),
        Node::new("3".to_string(), Position::new(100.0, 300.0))
            .with_data(json!({"label": "Output Node", "type": "output", "color": "#ff6b6b"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_animated(true),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string())
            .with_animated(true),
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
            <div class="xyflow leptos-flow custom-nodes-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background with lines
                <Background variant=BackgroundVariant::Lines />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render colored nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <ColoredNode
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
                    <div style="background: white; padding: 10px; border-radius: 4px; box-shadow: 0 2px 6px rgba(0,0,0,0.1);">
                        <strong>"Custom Nodes Demo"</strong>
                        <p style="margin: 5px 0; font-size: 12px;">"Nodes with custom colors"</p>
                        <div style="margin-top: 8px; font-size: 11px;">
                            <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                <div style="width: 12px; height: 12px; background: #6ede87; border-radius: 2px;"></div>
                                <span>"Input"</span>
                            </div>
                            <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                <div style="width: 12px; height: 12px; background: #6865A5; border-radius: 2px;"></div>
                                <span>"Processing"</span>
                            </div>
                            <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                <div style="width: 12px; height: 12px; background: #ff6b6b; border-radius: 2px;"></div>
                                <span>"Output"</span>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Colored node component with custom background color
#[component]
fn ColoredNode(
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
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#ffffff")
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

    // Determine handles based on type
    let has_source = node_type != "output";
    let has_target = node_type != "input";

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
        >
            <div
                class="xyflow__node-default colored-node"
                style=format!(
                    "background: {}; border-color: {}; color: {}; padding: 10px 20px; border-radius: 5px; border-width: 2px; border-style: solid; min-width: 100px; text-align: center;",
                    color,
                    darken_color(&color),
                    get_text_color(&color)
                )
            >
                {has_target.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div class="xyflow__node-label" style="font-weight: 500;">
                    {label}
                </div>

                {has_source.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Bottom
                        connection_mode=ConnectionMode::Strict
                    />
                })}
            </div>
        </div>
    }
}

/// Simple helper to darken a hex color for border
fn darken_color(hex: &str) -> String {
    // Parse hex color and darken by 20%
    if hex.starts_with('#') && hex.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            let r = ((r as f32) * 0.7) as u8;
            let g = ((g as f32) * 0.7) as u8;
            let b = ((b as f32) * 0.7) as u8;
            return format!("#{:02x}{:02x}{:02x}", r, g, b);
        }
    }
    hex.to_string()
}

/// Determine text color (white or black) based on background luminance
fn get_text_color(hex: &str) -> &'static str {
    if hex.starts_with('#') && hex.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            // Calculate relative luminance
            let luminance = 0.299 * (r as f32) + 0.587 * (g as f32) + 0.114 * (b as f32);
            if luminance > 186.0 {
                return "#333333";  // Dark text for light backgrounds
            }
        }
    }
    "#ffffff"  // White text for dark backgrounds
}
