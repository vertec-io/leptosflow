//! Use Connection Example
//!
//! Demonstrates the connection hook for custom connection behavior.
//!
//! This example shows:
//! - How to access connection state (connection in progress)
//! - Display source handle info, target position
//! - Custom behavior based on connection state
//! - Connection event logging

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

/// Use Connection example
#[component]
pub fn UseConnectionExample() -> impl IntoView {
    // Create initial nodes: mix of source and target nodes
    let initial_nodes = vec![
        Node::new("source1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Source 1", "node_type": "source"})),
        Node::new("source2".to_string(), Position::new(50.0, 200.0))
            .with_data(json!({"label": "Source 2", "node_type": "source"})),
        Node::new("middle".to_string(), Position::new(225.0, 125.0))
            .with_data(json!({"label": "Middle", "node_type": "default"})),
        Node::new("target1".to_string(), Position::new(400.0, 50.0))
            .with_data(json!({"label": "Target 1", "node_type": "target"})),
        Node::new("target2".to_string(), Position::new(400.0, 200.0))
            .with_data(json!({"label": "Target 2", "node_type": "target"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1".to_string(), "source1".to_string(), "middle".to_string())
            .with_label("existing".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Connection event log
    let connection_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        let timestamp = js_sys::Date::now();
        let time_str = format!("{:.0}", timestamp % 100000.0);
        connection_log.update(|logs| {
            logs.insert(0, format!("[{}] {}", time_str, msg));
            if logs.len() > 15 {
                logs.pop();
            }
        });
    };

    // Track previous connection state for event detection
    let prev_connection = RwSignal::new(Option::<String>::None);

    // Detect connection state changes
    Effect::new({
        let add_log = add_log.clone();
        move |_| {
            let current = store.state.connection_in_progress.get();
            let prev = prev_connection.get();

            match (&current, &prev) {
                (Some(conn), None) => {
                    // Connection started
                    add_log(format!("Connection started from node '{}'", conn.from_node));
                    prev_connection.set(Some(conn.from_node.clone()));
                }
                (None, Some(node_id)) => {
                    // Connection ended (completed or cancelled)
                    add_log(format!("Connection ended (was from '{}')", node_id));
                    prev_connection.set(None);
                }
                (Some(conn), Some(prev_node)) if conn.from_node != *prev_node => {
                    // Connection changed source (unusual case)
                    add_log(format!("Connection source changed to '{}'", conn.from_node));
                    prev_connection.set(Some(conn.from_node.clone()));
                }
                _ => {}
            }
        }
    });

    // Global drag handlers
    let drag_signal = get_drag_signal();

    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
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

    // Handle background click to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        store.clear_node_selection();
        store.clear_edge_selection();
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow use-connection-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
                 on:click=on_background_click
            >
                // Background with dots
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges
                    <UseConnectionEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <UseConnectionNode
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

                // Connection State Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 280px; max-width: 320px;">
                        <strong style="display: block; margin-bottom: 8px; font-size: 14px;">"🔗 Use Connection Hook"</strong>

                        // Connection state display
                        <div style="margin-bottom: 12px; padding: 10px; background: #f8f9fa; border-radius: 6px; border: 1px solid #e9ecef;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; color: #495057;">"Connection State:"</div>
                            {move || {
                                let conn = store.state.connection_in_progress.get();
                                if let Some(conn_state) = conn {
                                    view! {
                                        <div style="font-size: 12px;">
                                            // Status indicator
                                            <div style="display: flex; align-items: center; margin-bottom: 6px;">
                                                <span style="display: inline-block; width: 8px; height: 8px; background: #4ade80; border-radius: 50%; margin-right: 8px; animation: pulse 1s ease-in-out infinite;"></span>
                                                <span style="color: #16a34a; font-weight: 600;">"Connection in progress"</span>
                                            </div>

                                            // Source node
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; margin-bottom: 4px; border-left: 3px solid #667eea;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"Source Node"</span>
                                                <span style="font-weight: 500; color: #111827;">{conn_state.from_node.clone()}</span>
                                            </div>

                                            // Source handle
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; margin-bottom: 4px; border-left: 3px solid #a855f7;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"Source Handle"</span>
                                                <span style="font-weight: 500; color: #111827;">
                                                    {conn_state.from_handle.clone().unwrap_or_else(|| "(default)".to_string())}
                                                </span>
                                            </div>

                                            // Handle type
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; margin-bottom: 4px; border-left: 3px solid #f59e0b;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"Handle Type"</span>
                                                <span style="font-weight: 500; color: #111827;">
                                                    {format!("{:?}", conn_state.from_handle_type)}
                                                </span>
                                            </div>

                                            // From position
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; margin-bottom: 4px; border-left: 3px solid #06b6d4;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"From Position"</span>
                                                <span style="font-weight: 500; color: #111827; font-family: monospace; font-size: 11px;">
                                                    {format!("({:.1}, {:.1})", conn_state.from_position.x, conn_state.from_position.y)}
                                                </span>
                                            </div>

                                            // To position (cursor)
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; margin-bottom: 4px; border-left: 3px solid #ec4899;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"To Position (cursor)"</span>
                                                <span style="font-weight: 500; color: #111827; font-family: monospace; font-size: 11px;">
                                                    {format!("({:.1}, {:.1})", conn_state.to_position.x, conn_state.to_position.y)}
                                                </span>
                                            </div>

                                            // Validity
                                            <div style="padding: 6px 8px; background: white; border-radius: 4px; border-left: 3px solid #10b981;">
                                                <span style="font-size: 10px; color: #6b7280; display: block;">"Is Valid"</span>
                                                <span style=move || format!(
                                                    "font-weight: 500; {}",
                                                    if conn_state.is_valid { "color: #16a34a;" } else { "color: #dc2626;" }
                                                )>
                                                    {if conn_state.is_valid { "✓ Valid" } else { "✗ Invalid" }}
                                                </span>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div style="display: flex; align-items: center; color: #6b7280; font-size: 12px;">
                                            <span style="display: inline-block; width: 8px; height: 8px; background: #9ca3af; border-radius: 50%; margin-right: 8px;"></span>
                                            "No connection in progress"
                                        </div>
                                        <div style="margin-top: 8px; padding: 8px; background: #e3f2fd; border-radius: 4px; font-size: 11px; color: #1565c0;">
                                            "Drag from a handle to start a connection and see the state update in real-time."
                                        </div>
                                    }.into_any()
                                }
                            }}
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

                        // Connection event log
                        <div>
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Connection Events:"</div>
                            <div style="max-height: 100px; overflow-y: auto; font-size: 10px; font-family: monospace; background: #1e293b; color: #94a3b8; border-radius: 4px; padding: 8px;">
                                {move || {
                                    let logs = connection_log.get();
                                    if logs.is_empty() {
                                        view! { <div style="color: #64748b;">"// Waiting for events..."</div> }.into_any()
                                    } else {
                                        logs.iter().map(|entry| {
                                            view! {
                                                <div style="padding: 2px 0; border-bottom: 1px solid #334155;">
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
                    "}
                </style>
            </div>
        </div>
    }
}

/// Node component for Use Connection example
#[component]
fn UseConnectionNode(
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

/// Edge renderer for Use Connection example
#[component]
fn UseConnectionEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg class="xyflow__edges" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            <defs>
                // Gradient for edges
                <linearGradient id="use-connection-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                // Arrow marker
                <marker
                    id="use-connection-arrow"
                    markerWidth="10"
                    markerHeight="10"
                    refX="9"
                    refY="5"
                    orient="auto"
                    markerUnits="strokeWidth"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#667eea" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.iter().map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

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
                        let edge_id = edge.id.clone();

                        view! {
                            <g class="xyflow__edge" data-id=edge_id>
                                // Shadow/glow
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke="#667eea"
                                    stroke-width="6"
                                    stroke-opacity="0.2"
                                    stroke-linecap="round"
                                />
                                // Main path
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke="url(#use-connection-edge-gradient)"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    marker-end="url(#use-connection-arrow)"
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
                                            stroke="#667eea"
                                            stroke-width="1"
                                        />
                                        <text
                                            x="0"
                                            y="5"
                                            text-anchor="middle"
                                            fill="#667eea"
                                            font-size="10"
                                            font-weight="500"
                                        >
                                            {label}
                                        </text>
                                    </g>
                                })}
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
