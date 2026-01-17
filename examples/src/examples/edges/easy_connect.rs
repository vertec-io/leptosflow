//! Easy Connect Example
//!
//! Demonstrates click-based connection creation as an alternative to drag-based connections.
//! Click on a source handle to start a connection, then click on a target handle to complete it.
//!
//! This example shows:
//! - Click-to-connect workflow (no dragging required)
//! - Visual feedback during connection mode (pulsing source handle, highlighted target handles)
//! - Connection preview line from selected handle to cursor
//! - Click anywhere else to cancel connection

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

/// Connection state for click-to-connect
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ClickConnectionState {
    /// Source node ID
    source_node: String,
    /// Source handle ID (optional, for future use with multiple handles)
    source_handle: Option<String>,
    /// Position of the source handle
    source_position: Position,
}

/// Global click connection state
static CLICK_CONNECTION: std::sync::OnceLock<RwSignal<Option<ClickConnectionState>>> = std::sync::OnceLock::new();

fn get_click_connection_signal() -> RwSignal<Option<ClickConnectionState>> {
    *CLICK_CONNECTION.get_or_init(|| RwSignal::new(None))
}

/// Easy Connect example
#[component]
pub fn EasyConnectExample() -> impl IntoView {
    // Create initial nodes
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
        Edge::new("e1".to_string(), "source1".to_string(), "target1".to_string())
            .with_label("Existing".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components
    provide_context(store);

    // Click connection state
    let click_conn = get_click_connection_signal();

    // Mouse position for connection preview
    let mouse_pos = RwSignal::new(Position::new(0.0, 0.0));

    // Action log
    let action_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        action_log.update(|log| {
            log.insert(0, msg);
            if log.len() > 10 {
                log.pop();
            }
        });
    };

    // Global drag handlers (for node dragging)
    let drag_signal = get_drag_signal();

    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
        // Update mouse position for connection preview
        // Get the flow container to calculate relative position
        let viewport = store.get_viewport();

        // Get the mouse position relative to the viewport
        // We need to account for the sidebar width and any offsets
        let client_x = ev.client_x() as f64;
        let client_y = ev.client_y() as f64;

        // Convert to flow coordinates (accounting for zoom and pan)
        let flow_x = (client_x - viewport.x) / viewport.zoom;
        let flow_y = (client_y - viewport.y) / viewport.zoom;

        mouse_pos.set(Position::new(flow_x, flow_y));

        // Handle node dragging
        if let Some(drag_state) = drag_signal.get() {
            let current_x = ev.client_x() as f64;
            let current_y = ev.client_y() as f64;
            let (start_x, start_y) = drag_state.start_mouse;
            let (node_start_x, node_start_y) = drag_state.start_pos;

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

    // Click on background cancels connection
    let add_log_for_cancel = add_log.clone();
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        if click_conn.get().is_some() {
            click_conn.set(None);
            add_log_for_cancel("Connection cancelled".to_string());
        }
        store.clear_node_selection();
        store.clear_edge_selection();
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow easy-connect-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
                 on:click=on_background_click
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Custom edge renderer with connection preview
                    <EasyConnectEdgeRenderer store=store />

                    // Connection preview line
                    <ConnectionPreviewLine
                        click_conn=click_conn
                        mouse_pos=mouse_pos
                    />

                    // Render nodes
                    {move || {
                        let add_log = add_log.clone();
                        store.get_nodes().into_iter().map(move |node| {
                            let add_log = add_log.clone();
                            view! {
                                <EasyConnectNode
                                    node=node.clone()
                                    store=store
                                    click_conn=click_conn
                                    add_log=add_log
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 240px;">
                        <strong style="display: block; margin-bottom: 8px;">"Easy Connect"</strong>
                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666;">
                            "Click-based connection creation"
                        </p>

                        // Connection mode status
                        <div style="margin-bottom: 12px; padding: 8px; border-radius: 4px; font-size: 12px;"
                             class=move || if click_conn.get().is_some() { "connection-active" } else { "connection-idle" }
                             style:background=move || if click_conn.get().is_some() { "#e3f2fd" } else { "#f5f5f5" }
                        >
                            {move || {
                                if let Some(conn) = click_conn.get() {
                                    view! {
                                        <div>
                                            <span style="color: #1976d2; font-weight: 600;">"Connection Mode"</span>
                                            <div style="font-size: 11px; margin-top: 4px; color: #666;">
                                                "Source: " {conn.source_node}
                                            </div>
                                            <div style="font-size: 11px; color: #999;">
                                                "Click a target handle or anywhere to cancel"
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div style="color: #666;">
                                            "Click a source handle to start connecting"
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>

                        // Instructions
                        <div style="margin-bottom: 12px; padding: 8px; background: #fff3e0; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px; color: #e65100;">"How to use:"</div>
                            <ol style="font-size: 11px; margin: 0; padding-left: 18px; color: #666;">
                                <li>"Click a source handle (right side of source nodes)"</li>
                                <li>"The handle will pulse to show it's selected"</li>
                                <li>"Click a target handle to create connection"</li>
                                <li>"Click elsewhere to cancel"</li>
                            </ol>
                        </div>

                        // Node type legend
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Node Types:"</div>
                            <div style="font-size: 11px; color: #666;">
                                <div style="margin-bottom: 4px;">
                                    <span style="display: inline-block; width: 12px; height: 12px; background: #4ade80; border-radius: 2px; margin-right: 6px; vertical-align: middle;"></span>
                                    "Source (output handle only)"
                                </div>
                                <div style="margin-bottom: 4px;">
                                    <span style="display: inline-block; width: 12px; height: 12px; background: #60a5fa; border-radius: 2px; margin-right: 6px; vertical-align: middle;"></span>
                                    "Default (both handles)"
                                </div>
                                <div>
                                    <span style="display: inline-block; width: 12px; height: 12px; background: #f87171; border-radius: 2px; margin-right: 6px; vertical-align: middle;"></span>
                                    "Target (input handle only)"
                                </div>
                            </div>
                        </div>

                        // Action log
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"Action Log:"</div>
                        <div style="max-height: 100px; overflow-y: auto; font-size: 10px;">
                            {move || {
                                let log = action_log.get();
                                if log.is_empty() {
                                    view! { <div style="color: #999;">"Click a handle to start..."</div> }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        let entry = entry.clone();
                                        let (icon, color) = if entry.contains("created") {
                                            ("✓", "#4ade80")
                                        } else if entry.contains("cancelled") {
                                            ("✗", "#f87171")
                                        } else if entry.contains("started") {
                                            ("→", "#60a5fa")
                                        } else {
                                            ("•", "#666")
                                        };
                                        view! {
                                            <div style="padding: 2px 0; border-bottom: 1px solid #eee;">
                                                <span style=format!("color: {}; margin-right: 4px;", color)>{icon}</span>
                                                {entry}
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </Panel>

                // CSS for pulsing animation
                <style>
                    {"
                    @keyframes pulse-handle {
                        0%, 100% { transform: scale(1); box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.7); }
                        50% { transform: scale(1.3); box-shadow: 0 0 0 6px rgba(59, 130, 246, 0); }
                    }
                    .handle-connecting {
                        animation: pulse-handle 1s ease-in-out infinite;
                        background: #3b82f6 !important;
                        z-index: 100;
                    }
                    .handle-target-available {
                        background: #4ade80 !important;
                        transform: scale(1.2);
                        transition: all 0.2s ease;
                    }
                    .handle-target-available:hover {
                        transform: scale(1.5);
                        background: #22c55e !important;
                    }
                    "}
                </style>
            </div>
        </div>
    }
}

/// Connection preview line component
#[component]
fn ConnectionPreviewLine(
    click_conn: RwSignal<Option<ClickConnectionState>>,
    mouse_pos: RwSignal<Position>,
) -> impl IntoView {
    view! {
        {move || {
            if let Some(conn) = click_conn.get() {
                let from = conn.source_position;
                let to = mouse_pos.get();

                // Generate bezier path
                let dx = to.x - from.x;
                let _dy = to.y - from.y;
                let offset = (dx.abs() / 2.0).max(50.0).min(150.0);

                let path = format!(
                    "M {} {} C {} {}, {} {}, {} {}",
                    from.x, from.y,
                    from.x + offset, from.y,
                    to.x - offset, to.y,
                    to.x, to.y
                );

                view! {
                    <svg class="easy-connect-preview" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000; overflow: visible;">
                        <defs>
                            <linearGradient id="preview-line-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                                <stop offset="0%" style="stop-color:#3b82f6;stop-opacity:1" />
                                <stop offset="100%" style="stop-color:#8b5cf6;stop-opacity:1" />
                            </linearGradient>
                        </defs>
                        // Shadow/glow layer
                        <path
                            d=path.clone()
                            fill="none"
                            stroke="#3b82f6"
                            stroke-width="8"
                            stroke-opacity="0.2"
                            stroke-linecap="round"
                        />
                        // Main line
                        <path
                            d=path.clone()
                            fill="none"
                            stroke="url(#preview-line-gradient)"
                            stroke-width="2"
                            stroke-dasharray="8,4"
                            stroke-linecap="round"
                            class="animated-preview-line"
                        />
                        // Cursor indicator
                        <circle
                            cx=to.x
                            cy=to.y
                            r="6"
                            fill="#3b82f6"
                            stroke="white"
                            stroke-width="2"
                        />
                        // Animation style
                        <style>
                            {"
                            @keyframes dash-flow {
                                from { stroke-dashoffset: 24; }
                                to { stroke-dashoffset: 0; }
                            }
                            .animated-preview-line {
                                animation: dash-flow 0.5s linear infinite;
                            }
                            "}
                        </style>
                    </svg>
                }.into_any()
            } else {
                view! {}.into_any()
            }
        }}
    }
}

/// Easy Connect node component with click-to-connect handles
#[component]
fn EasyConnectNode<F>(
    node: Node,
    store: FlowStore,
    click_conn: RwSignal<Option<ClickConnectionState>>,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_source = node.id.clone();
    let node_id_for_source_handler = node.id.clone();
    let node_id_for_target = node.id.clone();
    let node_id_for_target_handler = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("node_type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let has_source = node_type != "target";
    let has_target = node_type != "source";

    // Node colors based on type
    let (bg_color, border_color) = match node_type.as_str() {
        "source" => ("#4ade80", "#22c55e"),
        "target" => ("#f87171", "#ef4444"),
        _ => ("#60a5fa", "#3b82f6"),
    };

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging the node
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

    // Source handle click - start connection
    let add_log_for_source = add_log.clone();
    let on_source_click = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get the node's current position to calculate handle position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id_for_source_handler) {
            // Source handle is on the right side of the node
            // Node width is approximately 120px, height 40px
            let handle_pos = Position::new(
                node.position.x + 120.0,  // Right side
                node.position.y + 20.0,   // Middle height
            );

            click_conn.set(Some(ClickConnectionState {
                source_node: node_id_for_source_handler.clone(),
                source_handle: None,
                source_position: handle_pos,
            }));

            add_log_for_source(format!("Connection started from {}", node_id_for_source_handler));
        }
    };

    // Target handle click - complete connection
    let add_log_for_target = add_log.clone();
    let on_target_click = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        if let Some(conn) = click_conn.get() {
            // Create new edge
            let edge_id = format!("e{}-{}", conn.source_node, node_id_for_target_handler);
            let new_edge = Edge::new(
                edge_id,
                conn.source_node.clone(),
                node_id_for_target_handler.clone(),
            );

            store.add_edge(new_edge);
            add_log_for_target(format!("Connection created: {} → {}", conn.source_node, node_id_for_target_handler));

            // Clear connection state
            click_conn.set(None);
        }
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
            <div
                class="easy-connect-node"
                style=format!(
                    "background: {}; border: 2px solid {}; padding: 10px 20px; border-radius: 8px; min-width: 80px; text-align: center; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                    bg_color, border_color
                )
            >
                // Target handle (left side)
                {has_target.then(|| {
                    let node_id_for_target_handle = node_id_for_target.clone();
                    view! {
                        <div
                            class=move || {
                                let is_connecting = click_conn.get().is_some();
                                let is_same_node = click_conn.get()
                                    .map(|c| c.source_node == node_id_for_target_handle)
                                    .unwrap_or(false);
                                if is_connecting && !is_same_node {
                                    "easy-connect-handle target-handle handle-target-available"
                                } else {
                                    "easy-connect-handle target-handle"
                                }
                            }
                            style="position: absolute; left: -6px; top: 50%; transform: translateY(-50%); width: 12px; height: 12px; background: #999; border-radius: 50%; cursor: crosshair; border: 2px solid white;"
                            on:click=on_target_click
                        />
                    }
                })}

                <div style="color: white; font-weight: 500; font-size: 13px; pointer-events: none;">
                    {label}
                </div>

                // Source handle (right side)
                {has_source.then(|| {
                    let node_id_for_source_handle = node_id_for_source.clone();
                    view! {
                        <div
                            class=move || {
                                let is_this_source = click_conn.get()
                                    .map(|c| c.source_node == node_id_for_source_handle)
                                    .unwrap_or(false);
                                if is_this_source {
                                    "easy-connect-handle source-handle handle-connecting"
                                } else {
                                    "easy-connect-handle source-handle"
                                }
                            }
                            style="position: absolute; right: -6px; top: 50%; transform: translateY(-50%); width: 12px; height: 12px; background: #999; border-radius: 50%; cursor: crosshair; border: 2px solid white;"
                            on:click=on_source_click
                        />
                    }
                })}
            </div>
        </div>
    }
}

/// Custom edge renderer for Easy Connect example
#[component]
fn EasyConnectEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg class="easy-connect-edges" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            <defs>
                <linearGradient id="easy-connect-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#4ade80;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#60a5fa;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="easy-connect-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#60a5fa" />
                </marker>
            </defs>
            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().map(|edge| {
                    // Find source and target nodes
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        // Calculate edge endpoints (right side of source, left side of target)
                        let source_x = source.position.x + 120.0;  // Right side
                        let source_y = source.position.y + 20.0;   // Middle
                        let target_x = target.position.x;          // Left side
                        let target_y = target.position.y + 20.0;   // Middle

                        // Generate bezier path
                        let dx = target_x - source_x;
                        let offset = (dx.abs() / 2.0).max(50.0).min(150.0);

                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            source_x, source_y,
                            source_x + offset, source_y,
                            target_x - offset, target_y,
                            target_x, target_y
                        );

                        // Calculate label position (middle of bezier)
                        let label_x = (source_x + target_x) / 2.0;
                        let label_y = (source_y + target_y) / 2.0;

                        let edge_label = edge.label.clone().unwrap_or_default();
                        let has_label = !edge_label.is_empty();

                        view! {
                            <g class="easy-connect-edge">
                                // Shadow layer
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke="rgba(0,0,0,0.1)"
                                    stroke-width="6"
                                    stroke-linecap="round"
                                />
                                // Main edge
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke="url(#easy-connect-edge-gradient)"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    marker-end="url(#easy-connect-arrow)"
                                />
                                // Label (if present)
                                {has_label.then(|| {
                                    view! {
                                        <g transform=format!("translate({}, {})", label_x, label_y)>
                                            <rect
                                                x="-30"
                                                y="-10"
                                                width="60"
                                                height="20"
                                                rx="4"
                                                fill="white"
                                                stroke="#ddd"
                                                stroke-width="1"
                                            />
                                            <text
                                                text-anchor="middle"
                                                dominant-baseline="middle"
                                                font-size="11"
                                                fill="#666"
                                            >
                                                {edge_label}
                                            </text>
                                        </g>
                                    }
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
