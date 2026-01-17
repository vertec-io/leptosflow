//! Undirectional Example
//!
//! Demonstrates how to restrict connection direction in a flow graph:
//! - Enforce one-way connections only (source → target)
//! - Prevent backward connections
//! - Visual feedback on invalid connection attempts
//!
//! This is useful for DAGs (Directed Acyclic Graphs) where data flows
//! in one direction only.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

// ============================================================================
// Connection State for Validation
// ============================================================================

/// Tracks active connection attempt for visual feedback
#[derive(Clone, Debug, Default)]
struct ConnectionAttemptState {
    /// Is a connection currently being attempted
    is_connecting: bool,
    /// Source node ID
    from_node: Option<String>,
    /// Target node ID (while hovering)
    to_node: Option<String>,
    /// Current mouse position (for connection preview)
    mouse_x: f64,
    mouse_y: f64,
    /// Whether the current target is valid
    is_valid: bool,
    /// Validation message
    message: String,
}

static CONNECTION_ATTEMPT_STATE: std::sync::OnceLock<RwSignal<ConnectionAttemptState>> = std::sync::OnceLock::new();

fn get_connection_attempt_signal() -> RwSignal<ConnectionAttemptState> {
    *CONNECTION_ATTEMPT_STATE.get_or_init(|| RwSignal::new(ConnectionAttemptState::default()))
}

// ============================================================================
// Action Log
// ============================================================================

#[derive(Clone, Debug)]
struct ConnectionEvent {
    timestamp: f64,
    event_type: String,
    message: String,
    is_error: bool,
}

// ============================================================================
// Node Level Tracking (for directional validation)
// ============================================================================

/// Determines if a connection from source to target would create a backward connection
/// In a unidirectional flow, nodes have implicit "levels" based on their position
/// and we only allow connections from left to right (lower level to higher level)
fn is_valid_direction(source_x: f64, target_x: f64) -> bool {
    // Allow connections that flow from left to right (source.x < target.x)
    // or are roughly at the same x level (within tolerance)
    source_x < target_x + 50.0
}

/// Check if adding this edge would create a cycle (backward reference)
fn would_create_cycle(
    edges: &[Edge],
    _nodes: &[Node],
    source_id: &str,
    target_id: &str,
) -> bool {
    // Simple check: can we reach source from target by following edges?
    // If yes, adding this edge would create a cycle
    let mut visited = std::collections::HashSet::new();
    let mut to_visit = vec![target_id.to_string()];

    while let Some(current) = to_visit.pop() {
        if current == source_id {
            return true; // Would create a cycle!
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        // Find all nodes reachable from current
        for edge in edges {
            if edge.source == current && !visited.contains(&edge.target) {
                to_visit.push(edge.target.clone());
            }
        }
    }

    false
}

// ============================================================================
// Undirectional Example Component
// ============================================================================

/// Undirectional example - restricting connection direction
#[component]
pub fn UndirectionalExample() -> impl IntoView {
    // Create initial nodes arranged in a left-to-right flow
    let initial_nodes = vec![
        // Level 1 - Input nodes (leftmost)
        Node::new("input1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Input A",
                "node_type": "input",
                "level": 1,
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("input2".to_string(), Position::new(50.0, 150.0))
            .with_data(json!({
                "label": "Input B",
                "node_type": "input",
                "level": 1,
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),

        // Level 2 - Processing nodes (middle)
        Node::new("process1".to_string(), Position::new(220.0, 50.0))
            .with_data(json!({
                "label": "Process 1",
                "node_type": "process",
                "level": 2,
                "color": "#6366f1"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("process2".to_string(), Position::new(220.0, 150.0))
            .with_data(json!({
                "label": "Process 2",
                "node_type": "process",
                "level": 2,
                "color": "#6366f1"
            }))
            .with_dimensions(100.0, 50.0),

        // Level 3 - Processing nodes (middle-right)
        Node::new("process3".to_string(), Position::new(390.0, 100.0))
            .with_data(json!({
                "label": "Process 3",
                "node_type": "process",
                "level": 3,
                "color": "#8b5cf6"
            }))
            .with_dimensions(100.0, 50.0),

        // Level 4 - Output node (rightmost)
        Node::new("output1".to_string(), Position::new(560.0, 100.0))
            .with_data(json!({
                "label": "Output",
                "node_type": "output",
                "level": 4,
                "color": "#ef4444"
            }))
            .with_dimensions(100.0, 50.0),
    ];

    // Create initial edges (left-to-right flow)
    let initial_edges = vec![
        Edge::new("e1".to_string(), "input1".to_string(), "process1".to_string()),
        Edge::new("e2".to_string(), "input2".to_string(), "process2".to_string()),
        Edge::new("e3".to_string(), "process1".to_string(), "process3".to_string()),
        Edge::new("e4".to_string(), "process2".to_string(), "process3".to_string()),
        Edge::new("e5".to_string(), "process3".to_string(), "output1".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Event log
    let event_log = RwSignal::new(Vec::<ConnectionEvent>::new());

    // Add event to log
    let add_event = move |event_type: &str, message: &str, is_error: bool| {
        event_log.update(|log| {
            log.insert(0, ConnectionEvent {
                timestamp: js_sys::Date::now(),
                event_type: event_type.to_string(),
                message: message.to_string(),
                is_error,
            });
            if log.len() > 15 {
                log.pop();
            }
        });
    };

    // Get signals
    let drag_signal = get_drag_signal();
    let connection_attempt = get_connection_attempt_signal();

    // Global mouse move handler
    let on_global_mousemove = {
        move |ev: leptos::ev::MouseEvent| {
            // Handle node drag
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

            // Handle connection attempt mouse tracking
            let attempt = connection_attempt.get();
            if attempt.is_connecting {
                let viewport = store.get_viewport();
                let flow_x = (ev.offset_x() as f64 - viewport.x) / viewport.zoom;
                let flow_y = (ev.offset_y() as f64 - viewport.y) / viewport.zoom;

                connection_attempt.update(|state| {
                    state.mouse_x = flow_x;
                    state.mouse_y = flow_y;
                });
            }
        }
    };

    // Global mouse up handler
    let on_global_mouseup = {
        let add_event = add_event.clone();
        move |_ev: leptos::ev::MouseEvent| {
            // End node drag
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
            }

            // End connection attempt
            let attempt = connection_attempt.get();
            if attempt.is_connecting {
                if let (Some(from), Some(to)) = (attempt.from_node.clone(), attempt.to_node.clone()) {
                    if attempt.is_valid {
                        // Create the connection
                        let edge_id = format!("e{}", js_sys::Date::now() as u64);
                        store.add_edge(Edge::new(edge_id, from.clone(), to.clone()));
                        add_event("Connected", &format!("{} → {}", from, to), false);
                    } else {
                        add_event("Rejected", &attempt.message, true);
                    }
                } else if attempt.from_node.is_some() {
                    add_event("Cancelled", "Connection dropped on empty space", false);
                }

                connection_attempt.set(ConnectionAttemptState::default());
            }
        }
    };

    // Clear log handler
    let clear_log = move |_| {
        event_log.set(vec![]);
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow undirectional-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Connection preview line
                {move || {
                    let attempt = connection_attempt.get();
                    if attempt.is_connecting {
                        if let Some(from_id) = &attempt.from_node {
                            let nodes = store.get_nodes();
                            if let Some(from_node) = nodes.iter().find(|n| &n.id == from_id) {
                                let viewport = store.get_viewport();

                                // Source position (right side of node)
                                let sx = from_node.position.x + from_node.width.unwrap_or(100.0);
                                let sy = from_node.position.y + from_node.height.unwrap_or(50.0) / 2.0;

                                // Target position
                                let (tx, ty) = if let Some(to_id) = &attempt.to_node {
                                    // Snap to target node
                                    if let Some(to_node) = nodes.iter().find(|n| &n.id == to_id) {
                                        (to_node.position.x, to_node.position.y + to_node.height.unwrap_or(50.0) / 2.0)
                                    } else {
                                        (attempt.mouse_x, attempt.mouse_y)
                                    }
                                } else {
                                    (attempt.mouse_x, attempt.mouse_y)
                                };

                                // Bezier control points
                                let offset = (tx - sx).abs() * 0.4;
                                let path = format!(
                                    "M {} {} C {} {}, {} {}, {} {}",
                                    sx, sy,
                                    sx + offset, sy,
                                    tx - offset, ty,
                                    tx, ty
                                );

                                let stroke_color = if attempt.is_valid || attempt.to_node.is_none() {
                                    "#10b981" // Green for valid
                                } else {
                                    "#ef4444" // Red for invalid
                                };

                                return Some(view! {
                                    <svg
                                        style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none; z-index: 1000;"
                                    >
                                        <defs>
                                            <marker
                                                id="connection-arrow"
                                                viewBox="0 0 10 10"
                                                refX="8"
                                                refY="5"
                                                markerWidth="6"
                                                markerHeight="6"
                                                orient="auto-start-reverse"
                                            >
                                                <path d="M 0 0 L 10 5 L 0 10 z" fill=stroke_color />
                                            </marker>
                                        </defs>
                                        <g transform=format!("translate({}, {}) scale({})", viewport.x, viewport.y, viewport.zoom)>
                                            // Shadow
                                            <path
                                                d=path.clone()
                                                stroke=format!("{}40", stroke_color)
                                                stroke-width="8"
                                                fill="none"
                                            />
                                            // Main line
                                            <path
                                                d=path.clone()
                                                stroke=stroke_color
                                                stroke-width="2"
                                                stroke-dasharray="8 4"
                                                fill="none"
                                                marker-end="url(#connection-arrow)"
                                            >
                                                <animate
                                                    attributeName="stroke-dashoffset"
                                                    values="12;0"
                                                    dur="0.5s"
                                                    repeatCount="indefinite"
                                                />
                                            </path>
                                        </g>
                                    </svg>
                                });
                            }
                        }
                    }
                    None
                }}

                // Validation feedback overlay
                {move || {
                    let attempt = connection_attempt.get();
                    if attempt.is_connecting && attempt.to_node.is_some() && !attempt.is_valid {
                        Some(view! {
                            <div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); \
                                        background: rgba(239, 68, 68, 0.95); color: white; padding: 12px 20px; \
                                        border-radius: 8px; font-size: 13px; font-weight: 600; \
                                        box-shadow: 0 4px 12px rgba(0,0,0,0.3); z-index: 1001; \
                                        pointer-events: none; text-align: center;">
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <span style="font-size: 18px;">"⛔"</span>
                                    <span>{attempt.message.clone()}</span>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <UndirectionalEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        let add_event = add_event.clone();
                        store.get_nodes().into_iter().map(move |node| {
                            let add_event = add_event.clone();
                            view! {
                                <UndirectionalNode
                                    node=node.clone()
                                    store=store
                                    add_event=add_event
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Direction indicator badge
                <div style="position: absolute; top: 10px; left: 10px; background: linear-gradient(135deg, #10b981 0%, #6366f1 100%); color: white; padding: 8px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; box-shadow: 0 2px 8px rgba(0,0,0,0.2); display: flex; align-items: center; gap: 8px;">
                    <span>"Unidirectional Flow"</span>
                    <span style="font-size: 14px;">"→"</span>
                </div>

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Unidirectional Connections"</strong>

                        // Rules explanation
                        <div style="background: #f0f9ff; padding: 10px; border-radius: 6px; margin-bottom: 12px; font-size: 11px; color: #0369a1; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 6px;">"Connection Rules:"</div>
                            <ul style="margin: 0; padding-left: 16px;">
                                <li>"Connections flow left → right only"</li>
                                <li>"Cannot connect to nodes on the left"</li>
                                <li>"Cannot create cycles (feedback loops)"</li>
                                <li>"Invalid attempts show red feedback"</li>
                            </ul>
                        </div>

                        // Node legend
                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Node Types"</div>
                            <div style="display: flex; flex-direction: column; gap: 6px; font-size: 11px;">
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 16px; height: 16px; background: #10b981; border-radius: 3px;"></div>
                                    <span>"Input (source only)"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 16px; height: 16px; background: #6366f1; border-radius: 3px;"></div>
                                    <span>"Process (source + target)"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 16px; height: 16px; background: #8b5cf6; border-radius: 3px;"></div>
                                    <span>"Process (level 3)"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 16px; height: 16px; background: #ef4444; border-radius: 3px;"></div>
                                    <span>"Output (target only)"</span>
                                </div>
                            </div>
                        </div>

                        // Connection status
                        {move || {
                            let attempt = connection_attempt.get();
                            if attempt.is_connecting {
                                let status_color = if attempt.is_valid || attempt.to_node.is_none() { "#10b981" } else { "#ef4444" };
                                let status_bg = if attempt.is_valid || attempt.to_node.is_none() { "#f0fdf4" } else { "#fef2f2" };
                                let from_label = attempt.from_node.as_ref().map(|id| {
                                    store.get_nodes().iter()
                                        .find(|n| &n.id == id)
                                        .and_then(|n| n.data.get("label"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("?")
                                        .to_string()
                                }).unwrap_or_default();

                                view! {
                                    <div style=format!("background: {}; padding: 10px; border-radius: 6px; margin-bottom: 12px; border: 1px solid {}30;", status_bg, status_color)>
                                        <div style=format!("font-size: 11px; font-weight: 600; color: {};", status_color)>
                                            "🔗 Connecting..."
                                        </div>
                                        <div style="font-size: 10px; color: #666; margin-top: 4px;">
                                            {format!("From: {}", from_label)}
                                        </div>
                                        {attempt.to_node.as_ref().map(|to_id| {
                                            let to_label = store.get_nodes().iter()
                                                .find(|n| &n.id == to_id)
                                                .and_then(|n| n.data.get("label"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("?")
                                                .to_string();
                                            view! {
                                                <div style="font-size: 10px; color: #666; margin-top: 2px;">
                                                    {format!("To: {}", to_label)}
                                                </div>
                                            }
                                        })}
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="background: #f5f5f5; padding: 10px; border-radius: 6px; text-align: center; margin-bottom: 12px;">
                                        <div style="font-size: 11px; color: #999;">
                                            "Drag from a source handle to connect"
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }}

                        // Event log
                        <div style="border-top: 1px solid #eee; padding-top: 12px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                <div style="font-size: 11px; font-weight: 600; color: #333;">"Connection Log"</div>
                                <button
                                    style="font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                                    on:click=clear_log
                                >
                                    "Clear"
                                </button>
                            </div>
                            <div style="background: #f8f9fa; border-radius: 6px; padding: 8px; max-height: 150px; overflow-y: auto;">
                                {move || {
                                    let log = event_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                                "Events will appear here"
                                            </div>
                                        }.into_any()
                                    } else {
                                        let log_len = log.len();
                                        log.into_iter().enumerate().map(|(idx, event)| {
                                            let date = js_sys::Date::new(&leptos::wasm_bindgen::JsValue::from_f64(event.timestamp));
                                            let time = format!(
                                                "{:02}:{:02}:{:02}",
                                                date.get_hours(),
                                                date.get_minutes(),
                                                date.get_seconds()
                                            );

                                            let bg_color = if idx == 0 {
                                                if event.is_error { "#fef2f2" } else { "#f0fdf4" }
                                            } else {
                                                "transparent"
                                            };
                                            let border = if idx < log_len - 1 { "1px solid #eee" } else { "none" };
                                            let event_color = if event.is_error { "#ef4444" } else { "#10b981" };
                                            let icon = if event.is_error { "❌" } else { "✅" };

                                            view! {
                                                <div style=format!(
                                                    "padding: 6px; background: {}; border-bottom: {}; font-size: 10px;",
                                                    bg_color, border
                                                )>
                                                    <div style="display: flex; justify-content: space-between; align-items: center;">
                                                        <span style=format!(
                                                            "font-weight: 600; color: {}; font-size: 10px;",
                                                            event_color
                                                        )>
                                                            {icon} " " {event.event_type.clone()}
                                                        </span>
                                                        <span style="color: #999; font-family: monospace; font-size: 9px;">{time}</span>
                                                    </div>
                                                    <div style="color: #666; font-size: 9px; margin-top: 2px;">
                                                        {event.message.clone()}
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Undirectional Node Component
// ============================================================================

/// Node component for unidirectional flow
#[component]
fn UndirectionalNode<F>(
    node: Node,
    store: FlowStore,
    add_event: F,
) -> impl IntoView
where
    F: Fn(&str, &str, bool) + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_handle = node.id.clone();
    let node_id_for_target = node.id.clone();

    let drag_signal = get_drag_signal();
    let connection_attempt = get_connection_attempt_signal();

    // Extract node data
    let node_type = node.data.get("node_type")
        .and_then(|v| v.as_str())
        .unwrap_or("process")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6366f1")
        .to_string();
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();

    let is_input = node_type == "input";
    let is_output = node_type == "output";

    // Mouse down on node body - start drag
    let on_node_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id_for_drag) {
            drag_signal.set(Some(DragState {
                node_id: node_id_for_drag.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            store.update_node(&node_id_for_drag, |n| {
                n.dragging = true;
            });
        }
    };

    // Start connection from source handle
    let on_source_mousedown = {
        let node_id = node_id.clone();
        let add_event = add_event.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();

            let viewport = store.get_viewport();
            let flow_x = (ev.offset_x() as f64 - viewport.x) / viewport.zoom;
            let flow_y = (ev.offset_y() as f64 - viewport.y) / viewport.zoom;

            connection_attempt.set(ConnectionAttemptState {
                is_connecting: true,
                from_node: Some(node_id.clone()),
                to_node: None,
                mouse_x: flow_x,
                mouse_y: flow_y,
                is_valid: true,
                message: String::new(),
            });

            add_event("Started", &format!("Connection from {}", node_id), false);
        }
    };

    // Mouse enter on target handle - validate connection
    let on_target_mouseenter = {
        let node_id = node_id_for_target.clone();
        move |_ev: leptos::ev::MouseEvent| {
            let attempt = connection_attempt.get();
            if attempt.is_connecting {
                if let Some(from_id) = &attempt.from_node {
                    let nodes = store.get_nodes();
                    let edges = store.get_edges();

                    // Get source and target nodes
                    let from_node = nodes.iter().find(|n| &n.id == from_id);
                    let to_node = nodes.iter().find(|n| n.id == node_id);

                    if let (Some(source), Some(target)) = (from_node, to_node) {
                        // Validate connection direction
                        let direction_valid = is_valid_direction(source.position.x, target.position.x);
                        let cycle_valid = !would_create_cycle(&edges, &nodes, &source.id, &target.id);
                        let same_node = source.id == target.id;

                        let (is_valid, message) = if same_node {
                            (false, "Cannot connect to self".to_string())
                        } else if !direction_valid {
                            (false, "Cannot connect backward (must flow left → right)".to_string())
                        } else if !cycle_valid {
                            (false, "Would create a cycle (feedback loop)".to_string())
                        } else {
                            (true, "Valid connection".to_string())
                        };

                        connection_attempt.update(|state| {
                            state.to_node = Some(node_id.clone());
                            state.is_valid = is_valid;
                            state.message = message;
                        });
                    }
                }
            }
        }
    };

    // Mouse leave target handle
    let on_target_mouseleave = move |_ev: leptos::ev::MouseEvent| {
        let attempt = connection_attempt.get();
        if attempt.is_connecting {
            connection_attempt.update(|state| {
                state.to_node = None;
                state.is_valid = true;
                state.message = String::new();
            });
        }
    };

    view! {
        <div
            class="xyflow__node undirectional-node"
            style=move || {
                let nodes = store.get_nodes();
                let attempt = connection_attempt.get();
                let is_connecting_from = attempt.from_node.as_ref().map(|id| id == &node_id_for_style).unwrap_or(false);
                let is_connecting_to = attempt.to_node.as_ref().map(|id| id == &node_id_for_style).unwrap_or(false);

                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let border = if is_connecting_from {
                        format!("2px solid {}", color)
                    } else if is_connecting_to {
                        if attempt.is_valid {
                            "2px solid #10b981".to_string()
                        } else {
                            "2px solid #ef4444".to_string()
                        }
                    } else {
                        format!("2px solid {}60", color)
                    };

                    let box_shadow = if is_connecting_to {
                        if attempt.is_valid {
                            "0 0 0 3px rgba(16, 185, 129, 0.3), 0 4px 12px rgba(0,0,0,0.15)"
                        } else {
                            "0 0 0 3px rgba(239, 68, 68, 0.3), 0 4px 12px rgba(0,0,0,0.15)"
                        }
                    } else {
                        "0 2px 6px rgba(0,0,0,0.1)"
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: linear-gradient(135deg, {}15 0%, {}30 100%); \
                         border: {}; border-radius: 8px; box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: box-shadow 0.15s, border 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(100.0), n.height.unwrap_or(50.0),
                        color, color, border, box_shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_node_mousedown
        >
            // Node label
            <div style=format!("font-weight: 600; font-size: 11px; color: {}; pointer-events: none;", color)>
                {label}
            </div>

            // Direction arrow indicator
            <div style="font-size: 10px; color: #999; margin-top: 2px; pointer-events: none;">
                {if is_input { "●→" } else if is_output { "→●" } else { "→" }}
            </div>

            // Target handle (left side) - for process and output nodes
            {(!is_input).then(|| {
                let node_id = node_id_for_handle.clone();
                view! {
                    <div
                        class="xyflow__handle xyflow__handle-left"
                        style=move || {
                            let attempt = connection_attempt.get();
                            let is_potential_target = attempt.is_connecting &&
                                attempt.from_node.as_ref().map(|id| id != &node_id).unwrap_or(false);

                            let bg_color = if is_potential_target {
                                if attempt.to_node.as_ref().map(|id| id == &node_id).unwrap_or(false) {
                                    if attempt.is_valid { "#10b981" } else { "#ef4444" }
                                } else {
                                    "#f59e0b" // Orange for potential target
                                }
                            } else {
                                "#888"
                            };

                            let scale = if is_potential_target { "1.3" } else { "1" };

                            format!(
                                "position: absolute; left: -6px; top: 50%; transform: translateY(-50%) scale({}); \
                                 width: 12px; height: 12px; background: {}; border: 2px solid white; \
                                 border-radius: 50%; cursor: crosshair; box-shadow: 0 1px 4px rgba(0,0,0,0.2); \
                                 transition: transform 0.15s, background 0.15s;",
                                scale, bg_color
                            )
                        }
                        on:mouseenter=on_target_mouseenter
                        on:mouseleave=on_target_mouseleave
                    />
                }
            })}

            // Source handle (right side) - for input and process nodes
            {(!is_output).then(|| {
                view! {
                    <div
                        class="xyflow__handle xyflow__handle-right"
                        style="position: absolute; right: -6px; top: 50%; transform: translateY(-50%); \
                               width: 12px; height: 12px; background: #10b981; border: 2px solid white; \
                               border-radius: 50%; cursor: crosshair; box-shadow: 0 1px 4px rgba(0,0,0,0.2);"
                        on:mousedown=on_source_mousedown
                    />
                }
            })}
        </div>
    }
}

// ============================================================================
// Undirectional Edge Renderer
// ============================================================================

/// Edge renderer for unidirectional flow
#[component]
fn UndirectionalEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="unidirectional-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#10b981" />
                    <stop offset="100%" stop-color="#6366f1" />
                </linearGradient>
                <marker
                    id="unidirectional-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#6366f1" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate edge path (source right to target left)
                    let sx = source_node.position.x + source_node.width.unwrap_or(100.0);
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0) / 2.0;
                    let tx = target_node.position.x;
                    let ty = target_node.position.y + target_node.height.unwrap_or(50.0) / 2.0;

                    // Bezier control points
                    let offset = (tx - sx).abs() * 0.4;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx + offset, sy,
                        tx - offset, ty,
                        tx, ty
                    );

                    Some(view! {
                        <g class="xyflow__edge">
                            // Shadow
                            <path
                                d=path.clone()
                                stroke="#10b98130"
                                stroke-width="6"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path.clone()
                                stroke="url(#unidirectional-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#unidirectional-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
