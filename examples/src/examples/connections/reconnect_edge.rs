//! Reconnect Edge Example
//!
//! Demonstrates how to reconnect an existing edge to a different handle.
//!
//! This example shows:
//! - Dragging edge endpoint to new target
//! - Visual feedback showing reconnection in progress
//! - Edge updates to new connection on drop
//! - Reconnection event logging

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

// Global signal for reconnection state
static RECONNECT_STATE: OnceLock<RwSignal<Option<ReconnectState>>> = OnceLock::new();

fn get_reconnect_signal() -> RwSignal<Option<ReconnectState>> {
    *RECONNECT_STATE.get_or_init(|| RwSignal::new(None))
}

/// State tracking edge reconnection
#[derive(Clone, Debug)]
struct ReconnectState {
    /// The edge being reconnected
    edge_id: String,
    /// Which end is being reconnected: "source" or "target"
    end_type: String,
    /// Original source/target node id
    original_node: String,
    /// Current mouse position in flow coordinates
    current_pos: Position,
}

/// Reconnect Edge example
#[component]
pub fn ReconnectEdgeExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("source1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Source 1", "node_type": "source"})),
        Node::new("source2".to_string(), Position::new(50.0, 200.0))
            .with_data(json!({"label": "Source 2", "node_type": "source"})),
        Node::new("target1".to_string(), Position::new(350.0, 50.0))
            .with_data(json!({"label": "Target 1", "node_type": "target"})),
        Node::new("target2".to_string(), Position::new(350.0, 200.0))
            .with_data(json!({"label": "Target 2", "node_type": "target"})),
        Node::new("middle".to_string(), Position::new(200.0, 125.0))
            .with_data(json!({"label": "Middle", "node_type": "default"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1".to_string(), "source1".to_string(), "target1".to_string())
            .with_label("Edge 1".to_string()),
        Edge::new("e2".to_string(), "source2".to_string(), "target2".to_string())
            .with_label("Edge 2".to_string()),
        Edge::new("e3".to_string(), "source1".to_string(), "middle".to_string())
            .with_label("Edge 3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Connection event log
    let action_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        let timestamp = js_sys::Date::now();
        let time_str = format!("{:.0}", timestamp % 100000.0);
        action_log.update(|logs| {
            logs.insert(0, format!("[{}] {}", time_str, msg));
            if logs.len() > 15 {
                logs.pop();
            }
        });
    };

    // Track reconnection count
    let reconnect_count = RwSignal::new(0_i32);

    // Get reconnect signal
    let reconnect_signal = get_reconnect_signal();

    // Global drag handlers
    let drag_signal = get_drag_signal();

    let on_global_mousemove = {
        let add_log = add_log.clone();
        move |ev: leptos::ev::MouseEvent| {
            // Handle node dragging
            if let Some(drag_state) = drag_signal.get() {
                let current_x = ev.client_x() as f64;
                let current_y = ev.client_y() as f64;
                let (start_x, start_y) = drag_state.start_mouse;
                let (node_start_x, node_start_y) = drag_state.start_pos;

                let viewport = store.get_viewport();
                let dx = (current_x - start_x) / viewport.zoom;
                let dy = (current_y - start_y) / viewport.zoom;

                store.update_node(&drag_state.node_id, |n| {
                    n.position = Position::new(node_start_x + dx, node_start_y + dy);
                });
            }

            // Handle edge reconnection dragging
            if let Some(mut reconnect_state) = reconnect_signal.get() {
                // Get flow container position
                let viewport = store.get_viewport();

                // Calculate flow coordinates from mouse position
                // This is a simplified calculation - in reality you'd need the container offset
                let flow_x = (ev.client_x() as f64 - 280.0 - viewport.x) / viewport.zoom;
                let flow_y = (ev.client_y() as f64 - viewport.y) / viewport.zoom;

                reconnect_state.current_pos = Position::new(flow_x, flow_y);
                reconnect_signal.set(Some(reconnect_state));
            }
        }
    };

    let on_global_mouseup = {
        let add_log = add_log.clone();
        move |ev: leptos::ev::MouseEvent| {
            // Handle node drag end
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
            }

            // Handle edge reconnection end
            if let Some(reconnect_state) = reconnect_signal.get() {
                // Find if we're over a valid node handle
                let nodes = store.get_nodes();
                let viewport = store.get_viewport();

                let flow_x = (ev.client_x() as f64 - 280.0 - viewport.x) / viewport.zoom;
                let flow_y = (ev.client_y() as f64 - viewport.y) / viewport.zoom;

                // Check if we're near a node's handle
                let mut found_target: Option<String> = None;
                for node in &nodes {
                    if node.id == reconnect_state.original_node {
                        continue; // Skip the original connection node
                    }

                    // Get node type to determine valid connection targets
                    let node_type = node.data.get("node_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    // Determine if this node can accept the connection based on end type
                    let is_valid_target = match reconnect_state.end_type.as_str() {
                        "target" => node_type == "target" || node_type == "default",
                        "source" => node_type == "source" || node_type == "default",
                        _ => false,
                    };

                    if !is_valid_target {
                        continue;
                    }

                    let node_width = node.width.unwrap_or(120.0);
                    let node_height = node.height.unwrap_or(40.0);

                    // Check if mouse is near this node's connection area
                    let handle_x = if reconnect_state.end_type == "target" {
                        node.position.x // Left side for target handles
                    } else {
                        node.position.x + node_width // Right side for source handles
                    };
                    let handle_y = node.position.y + node_height / 2.0;

                    let distance = ((flow_x - handle_x).powi(2) + (flow_y - handle_y).powi(2)).sqrt();

                    if distance < 30.0 {
                        found_target = Some(node.id.clone());
                        break;
                    }
                }

                if let Some(new_node_id) = found_target {
                    // Update the edge
                    let edge_id = reconnect_state.edge_id.clone();
                    let end_type = reconnect_state.end_type.clone();
                    let original = reconnect_state.original_node.clone();

                    store.state.edges.update(|edges| {
                        if let Some(edge) = edges.iter_mut().find(|e| e.id == edge_id) {
                            if end_type == "target" {
                                edge.target = new_node_id.clone();
                            } else {
                                edge.source = new_node_id.clone();
                            }
                        }
                    });

                    add_log(format!(
                        "Reconnected '{}': {} {} -> {}",
                        edge_id, end_type, original, new_node_id
                    ));
                    reconnect_count.update(|c| *c += 1);
                } else {
                    add_log(format!(
                        "Reconnection cancelled for '{}'",
                        reconnect_state.edge_id
                    ));
                }

                reconnect_signal.set(None);
            }
        }
    };

    // Handle background click to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        store.clear_node_selection();
        store.clear_edge_selection();
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow reconnect-edge-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
                 on:click=on_background_click
            >
                // Background with dots
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges with reconnect handles
                    <ReconnectEdgeRenderer store=store add_log=add_log.clone() />

                    // Render reconnection preview line
                    <ReconnectPreviewLine store=store />

                    // Render connection line while creating new connections
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <ReconnectNode
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
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 280px; max-width: 320px;">
                        <strong style="display: block; margin-bottom: 8px; font-size: 14px;">"🔄 Reconnect Edge"</strong>

                        // Reconnection state display
                        <div style="margin-bottom: 12px; padding: 10px; background: #f8f9fa; border-radius: 6px; border: 1px solid #e9ecef;">
                            {move || {
                                let reconnect = reconnect_signal.get();
                                if let Some(state) = reconnect {
                                    view! {
                                        <div style="font-size: 12px;">
                                            <div style="display: flex; align-items: center; margin-bottom: 8px;">
                                                <span style="display: inline-block; width: 8px; height: 8px; background: #f59e0b; border-radius: 50%; margin-right: 8px; animation: pulse 0.5s ease-in-out infinite;"></span>
                                                <span style="color: #d97706; font-weight: 600;">"Reconnecting..."</span>
                                            </div>
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; margin-bottom: 4px; border-left: 3px solid #667eea;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"Edge"</span>
                                                <span style="font-weight: 500; color: #111827;">{state.edge_id.clone()}</span>
                                            </div>
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; border-left: 3px solid #a855f7;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"Reconnecting"</span>
                                                <span style="font-weight: 500; color: #111827;">
                                                    {format!("{} endpoint", state.end_type)}
                                                </span>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div style="display: flex; align-items: center; color: #6b7280; font-size: 12px;">
                                            <span style="display: inline-block; width: 8px; height: 8px; background: #9ca3af; border-radius: 50%; margin-right: 8px;"></span>
                                            "Not reconnecting"
                                        </div>
                                        <div style="margin-top: 8px; padding: 8px; background: #e3f2fd; border-radius: 4px; font-size: 11px; color: #1565c0;">
                                            "Drag a colored dot at the edge endpoint to reconnect it to a different node."
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>

                        // Statistics
                        <div style="margin-bottom: 12px; padding: 10px; background: #dbeafe; border-radius: 6px; border: 1px solid #93c5fd;">
                            <div style="display: flex; justify-content: space-between; align-items: center;">
                                <span style="font-size: 12px; color: #1e40af;">"Reconnections:"</span>
                                <span style="font-size: 16px; font-weight: 700; color: #1d4ed8;">{move || reconnect_count.get()}</span>
                            </div>
                        </div>

                        // How it works section
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"How It Works:"</div>
                            <ul style="font-size: 11px; color: #6b7280; margin: 0; padding-left: 16px; list-style-type: disc;">
                                <li style="margin-bottom: 4px;">"Edges have small dots at source (blue) and target (orange) ends"</li>
                                <li style="margin-bottom: 4px;">"Drag a dot to start reconnecting"</li>
                                <li style="margin-bottom: 4px;">"Drop on a valid node handle to reconnect"</li>
                                <li>"Drop elsewhere to cancel"</li>
                            </ul>
                        </div>

                        // Node types legend
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Node Types:"</div>
                            <div style="display: flex; flex-wrap: wrap; gap: 8px; font-size: 11px;">
                                <div style="display: flex; align-items: center;">
                                    <span style="display: inline-block; width: 12px; height: 12px; background: #6ede87; border-radius: 2px; margin-right: 4px;"></span>
                                    "Source"
                                </div>
                                <div style="display: flex; align-items: center;">
                                    <span style="display: inline-block; width: 12px; height: 12px; background: #64748b; border-radius: 2px; margin-right: 4px;"></span>
                                    "Default"
                                </div>
                                <div style="display: flex; align-items: center;">
                                    <span style="display: inline-block; width: 12px; height: 12px; background: #6865A5; border-radius: 2px; margin-right: 4px;"></span>
                                    "Target"
                                </div>
                            </div>
                        </div>

                        // Action log
                        <div>
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Action Log:"</div>
                            <div style="max-height: 100px; overflow-y: auto; font-size: 10px; font-family: monospace; background: #1e293b; color: #94a3b8; border-radius: 4px; padding: 8px;">
                                {move || {
                                    let logs = action_log.get();
                                    if logs.is_empty() {
                                        view! { <div style="color: #64748b;">"// Waiting for events..."</div> }.into_any()
                                    } else {
                                        logs.iter().map(|entry| {
                                            let style = if entry.contains("Reconnected") {
                                                "padding: 2px 0; border-bottom: 1px solid #334155; color: #4ade80;"
                                            } else {
                                                "padding: 2px 0; border-bottom: 1px solid #334155;"
                                            };
                                            view! {
                                                <div style=style>
                                                    {entry.clone()}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </Panel>

                // CSS for animations
                <style>
                    {"
                    @keyframes pulse {
                        0%, 100% { opacity: 1; transform: scale(1); }
                        50% { opacity: 0.6; transform: scale(1.2); }
                    }
                    .reconnect-handle {
                        cursor: grab;
                        transition: transform 0.15s ease;
                    }
                    .reconnect-handle:hover {
                        transform: scale(1.5);
                    }
                    "}
                </style>
            </div>
        </div>
    }
}

/// Node component for Reconnect Edge example
#[component]
fn ReconnectNode(
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
    let node_type = node.data.get("node_type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

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

    // Determine node styling based on type
    let (bg_color, border_color) = match node_type.as_str() {
        "source" => ("#6ede87", "#4cb864"),
        "target" => ("#6865A5", "#4a4782"),
        _ => ("#64748b", "#475569"), // default
    };

    // Determine which handles to show
    let is_source = node_type == "source";
    let is_target = node_type == "target";
    let has_source_handle = is_source || node_type == "default";
    let has_target_handle = is_target || node_type == "default";

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
                class="xyflow__node-default"
                style=format!(
                    "background: {} !important; border-color: {}; padding: 10px 20px; border-radius: 6px; border: 2px solid {}; min-width: 80px; text-align: center;",
                    bg_color, border_color, border_color
                )
            >
                // Target handle
                {has_target_handle.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Left
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div class="xyflow__node-label" style="color: white; font-weight: 500;">
                    {label}
                </div>

                // Source handle
                {has_source_handle.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Right
                        connection_mode=ConnectionMode::Strict
                    />
                })}
            </div>
        </div>
    }
}

/// Edge renderer with reconnect handles
#[component]
fn ReconnectEdgeRenderer<F>(store: FlowStore, add_log: F) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    let reconnect_signal = get_reconnect_signal();

    view! {
        <svg class="xyflow__edges" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            <defs>
                // Gradient for edges
                <linearGradient id="reconnect-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                // Gradient for reconnecting edges
                <linearGradient id="reconnect-edge-active-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#f59e0b;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#f97316;stop-opacity:1" />
                </linearGradient>
                // Arrow marker
                <marker
                    id="reconnect-edge-arrow"
                    markerWidth="10"
                    markerHeight="10"
                    refX="9"
                    refY="5"
                    orient="auto"
                    markerUnits="strokeWidth"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#667eea" />
                </marker>
                // Arrow marker for active reconnection
                <marker
                    id="reconnect-edge-arrow-active"
                    markerWidth="10"
                    markerHeight="10"
                    refX="9"
                    refY="5"
                    orient="auto"
                    markerUnits="strokeWidth"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f59e0b" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let reconnect_state = reconnect_signal.get();

                edges.iter().map(|edge| {
                    let edge_id = edge.id.clone();
                    let edge_id_for_source = edge.id.clone();
                    let edge_id_for_target = edge.id.clone();
                    let source_id = edge.source.clone();
                    let target_id = edge.target.clone();

                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    // Check if this edge is being reconnected
                    let is_reconnecting = reconnect_state.as_ref()
                        .map(|s| s.edge_id == edge_id)
                        .unwrap_or(false);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        // Calculate edge endpoints
                        let source_x = source.position.x + source.width.unwrap_or(120.0);
                        let source_y = source.position.y + source.height.unwrap_or(40.0) / 2.0;
                        let target_x = target.position.x;
                        let target_y = target.position.y + target.height.unwrap_or(40.0) / 2.0;

                        // Generate bezier path
                        let dx = target_x - source_x;
                        let offset = (dx.abs() / 2.0).max(50.0);

                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            source_x, source_y,
                            source_x + offset, source_y,
                            target_x - offset, target_y,
                            target_x, target_y
                        );

                        // Calculate label position (midpoint)
                        let mid_x = (source_x + target_x) / 2.0;
                        let mid_y = (source_y + target_y) / 2.0;

                        let label = edge.label.clone().unwrap_or_default();

                        // Gradient and marker based on state
                        let (gradient, marker) = if is_reconnecting {
                            ("url(#reconnect-edge-active-gradient)", "url(#reconnect-edge-arrow-active)")
                        } else {
                            ("url(#reconnect-edge-gradient)", "url(#reconnect-edge-arrow)")
                        };

                        // Clone add_log for handlers
                        let add_log_for_source = add_log.clone();
                        let add_log_for_target = add_log.clone();

                        // Source reconnect handle mousedown
                        let source_handler = move |ev: leptos::ev::MouseEvent| {
                            ev.stop_propagation();
                            ev.prevent_default();

                            add_log_for_source(format!("Started reconnecting '{}' source", edge_id_for_source));

                            reconnect_signal.set(Some(ReconnectState {
                                edge_id: edge_id_for_source.clone(),
                                end_type: "source".to_string(),
                                original_node: source_id.clone(),
                                current_pos: Position::new(source_x, source_y),
                            }));
                        };

                        // Target reconnect handle mousedown
                        let target_handler = move |ev: leptos::ev::MouseEvent| {
                            ev.stop_propagation();
                            ev.prevent_default();

                            add_log_for_target(format!("Started reconnecting '{}' target", edge_id_for_target));

                            reconnect_signal.set(Some(ReconnectState {
                                edge_id: edge_id_for_target.clone(),
                                end_type: "target".to_string(),
                                original_node: target_id.clone(),
                                current_pos: Position::new(target_x, target_y),
                            }));
                        };

                        view! {
                            <g class="xyflow__edge" data-id=edge.id.clone()>
                                // Shadow/glow
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke=if is_reconnecting { "#f59e0b" } else { "#667eea" }
                                    stroke-width="6"
                                    stroke-opacity="0.2"
                                    stroke-linecap="round"
                                />
                                // Main path
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke=gradient
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-dasharray=if is_reconnecting { "5,5" } else { "" }
                                    marker-end=marker
                                />
                                // Label
                                {(!label.is_empty()).then(|| view! {
                                    <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                        <rect
                                            x="-30"
                                            y="-10"
                                            width="60"
                                            height="20"
                                            rx="4"
                                            fill="white"
                                            stroke=if is_reconnecting { "#f59e0b" } else { "#667eea" }
                                            stroke-width="1"
                                        />
                                        <text
                                            x="0"
                                            y="5"
                                            text-anchor="middle"
                                            fill=if is_reconnecting { "#f59e0b" } else { "#667eea" }
                                            font-size="10"
                                            font-weight="500"
                                        >
                                            {label}
                                        </text>
                                    </g>
                                })}

                                // Source reconnect handle (blue circle)
                                <circle
                                    class="reconnect-handle"
                                    cx=source_x
                                    cy=source_y
                                    r="6"
                                    fill="#3b82f6"
                                    stroke="white"
                                    stroke-width="2"
                                    style="cursor: grab; pointer-events: all;"
                                    on:mousedown=source_handler
                                />

                                // Target reconnect handle (orange circle)
                                <circle
                                    class="reconnect-handle"
                                    cx=target_x
                                    cy=target_y
                                    r="6"
                                    fill="#f59e0b"
                                    stroke="white"
                                    stroke-width="2"
                                    style="cursor: grab; pointer-events: all;"
                                    on:mousedown=target_handler
                                />
                            </g>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }).collect_view()
            }}
        </svg>
    }
}

/// Component to render the reconnection preview line
#[component]
fn ReconnectPreviewLine(store: FlowStore) -> impl IntoView {
    let reconnect_signal = get_reconnect_signal();

    view! {
        {move || {
            let reconnect = reconnect_signal.get();
            if let Some(state) = reconnect {
                // Get the fixed end position from the edge
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                let edge = edges.iter().find(|e| e.id == state.edge_id);

                if let Some(edge) = edge {
                    // Determine fixed position based on which end is being reconnected
                    let (fixed_x, fixed_y) = if state.end_type == "source" {
                        // Target is fixed
                        let target_node = nodes.iter().find(|n| n.id == edge.target);
                        if let Some(target) = target_node {
                            (target.position.x, target.position.y + target.height.unwrap_or(40.0) / 2.0)
                        } else {
                            return view! {}.into_any();
                        }
                    } else {
                        // Source is fixed
                        let source_node = nodes.iter().find(|n| n.id == edge.source);
                        if let Some(source) = source_node {
                            let source_width = source.width.unwrap_or(120.0);
                            (source.position.x + source_width, source.position.y + source.height.unwrap_or(40.0) / 2.0)
                        } else {
                            return view! {}.into_any();
                        }
                    };

                    // Current drag position
                    let drag_x = state.current_pos.x;
                    let drag_y = state.current_pos.y;

                    // Generate bezier path from fixed to drag position
                    let (start_x, start_y, end_x, end_y) = if state.end_type == "source" {
                        // Dragging source, target is fixed
                        (drag_x, drag_y, fixed_x, fixed_y)
                    } else {
                        // Dragging target, source is fixed
                        (fixed_x, fixed_y, drag_x, drag_y)
                    };

                    let dx = end_x - start_x;
                    let offset = (dx.abs() / 2.0).max(30.0);

                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        start_x, start_y,
                        start_x + offset, start_y,
                        end_x - offset, end_y,
                        end_x, end_y
                    );

                    return view! {
                        <svg style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible; z-index: 1000;">
                            // Preview line
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="#f59e0b"
                                stroke-width="3"
                                stroke-dasharray="8,4"
                                stroke-linecap="round"
                                opacity="0.8"
                            />
                            // Drag indicator circle
                            <circle
                                cx=drag_x
                                cy=drag_y
                                r="8"
                                fill="#f59e0b"
                                stroke="white"
                                stroke-width="2"
                                opacity="0.9"
                            />
                        </svg>
                    }.into_any();
                }
            }
            view! {}.into_any()
        }}
    }
}
