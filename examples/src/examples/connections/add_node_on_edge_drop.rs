//! Add Node on Edge Drop Example
//!
//! Demonstrates how to create a new node when dropping a connection on empty canvas space.
//!
//! This example shows:
//! - Detecting connection drop on empty canvas area
//! - Showing a node type picker at drop position
//! - Creating new node at drop position
//! - Automatically connecting edge to new node

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

/// State for tracking edge drop and node creation
#[derive(Clone, Debug)]
struct EdgeDropState {
    /// Position where the connection was dropped (flow coordinates)
    drop_position: Position,
    /// Source node ID
    source_node: String,
    /// Source handle ID (optional)
    source_handle: Option<String>,
}

/// Global edge drop state
static EDGE_DROP_STATE: std::sync::OnceLock<RwSignal<Option<EdgeDropState>>> = std::sync::OnceLock::new();

fn get_edge_drop_signal() -> RwSignal<Option<EdgeDropState>> {
    *EDGE_DROP_STATE.get_or_init(|| RwSignal::new(None))
}

/// Node type options for the picker
#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeTypeOption {
    Default,
    Input,
    Output,
    Process,
}

impl NodeTypeOption {
    fn label(&self) -> &'static str {
        match self {
            NodeTypeOption::Default => "Default Node",
            NodeTypeOption::Input => "Input Node",
            NodeTypeOption::Output => "Output Node",
            NodeTypeOption::Process => "Process Node",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            NodeTypeOption::Default => "◇",
            NodeTypeOption::Input => "▶",
            NodeTypeOption::Output => "◀",
            NodeTypeOption::Process => "⚙",
        }
    }

    fn color(&self) -> (&'static str, &'static str) {
        match self {
            NodeTypeOption::Default => ("#60a5fa", "#3b82f6"),
            NodeTypeOption::Input => ("#4ade80", "#22c55e"),
            NodeTypeOption::Output => ("#f87171", "#ef4444"),
            NodeTypeOption::Process => ("#c084fc", "#a855f7"),
        }
    }

    fn node_type(&self) -> &'static str {
        match self {
            NodeTypeOption::Default => "default",
            NodeTypeOption::Input => "source",
            NodeTypeOption::Output => "target",
            NodeTypeOption::Process => "default",
        }
    }
}

/// Add Node on Edge Drop example
#[component]
pub fn AddNodeOnEdgeDropExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("source1".to_string(), Position::new(50.0, 100.0))
            .with_data(json!({"label": "Source 1", "node_type": "source"})),
        Node::new("source2".to_string(), Position::new(50.0, 250.0))
            .with_data(json!({"label": "Source 2", "node_type": "source"})),
        Node::new("target1".to_string(), Position::new(400.0, 175.0))
            .with_data(json!({"label": "Target", "node_type": "target"})),
    ];

    // Create initial edge
    let initial_edges = vec![
        Edge::new("e1".to_string(), "source1".to_string(), "target1".to_string())
            .with_label("Existing".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components
    provide_context(store);

    // Edge drop state for showing node picker
    let edge_drop = get_edge_drop_signal();

    // Node counter for generating unique IDs
    let node_counter = RwSignal::new(4);

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
        // Handle node dragging
        if let Some(drag_state) = drag_signal.get() {
            let viewport = store.get_viewport();
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

    // Close node picker on background click (if no connection active)
    let add_log_for_cancel = add_log.clone();
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        if edge_drop.get().is_some() {
            edge_drop.set(None);
            add_log_for_cancel("Node picker cancelled".to_string());
        }
        store.clear_node_selection();
        store.clear_edge_selection();
    };

    // Handler for creating a new node from the picker
    let add_log_for_create = add_log.clone();
    let create_node = move |node_type: NodeTypeOption| {
        if let Some(drop_state) = edge_drop.get() {
            // Get next node ID
            let counter = node_counter.get();
            node_counter.set(counter + 1);
            let new_node_id = format!("node{}", counter);

            // Get colors for this node type
            let (bg_color, _border_color) = node_type.color();

            // Create the new node at drop position
            let new_node = Node::new(new_node_id.clone(), drop_state.drop_position)
                .with_data(json!({
                    "label": node_type.label(),
                    "node_type": node_type.node_type(),
                    "color": bg_color
                }));

            store.add_node(new_node);

            // Create edge connecting source to new node
            let edge_id = format!("e{}-{}", drop_state.source_node, new_node_id);
            let new_edge = Edge::new(
                edge_id,
                drop_state.source_node.clone(),
                new_node_id.clone(),
            );
            store.add_edge(new_edge);

            add_log_for_create(format!(
                "Created {} at ({:.0}, {:.0})",
                node_type.label(),
                drop_state.drop_position.x,
                drop_state.drop_position.y
            ));

            // Clear the drop state
            edge_drop.set(None);
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow add-node-on-edge-drop-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
                 on:click=on_background_click
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Custom edge renderer
                    <AddNodeEdgeRenderer store=store />

                    // Connection line renderer (for in-progress connections)
                    <ConnectionLineRenderer
                        store=store
                        edge_drop=edge_drop
                        add_log=add_log.clone()
                    />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <AddNodeDropNode
                                    node=node.clone()
                                    store=store
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Node type picker (appears when connection dropped on empty space)
                {move || {
                    let create_node = create_node.clone();
                    if let Some(drop_state) = edge_drop.get() {
                        let viewport = store.get_viewport();

                        // Convert flow coordinates to screen coordinates
                        let screen_x = drop_state.drop_position.x * viewport.zoom + viewport.x;
                        let screen_y = drop_state.drop_position.y * viewport.zoom + viewport.y;

                        view! {
                            <NodeTypePicker
                                x=screen_x
                                y=screen_y
                                on_select=create_node.clone()
                            />
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 240px;">
                        <strong style="display: block; margin-bottom: 8px;">"Add Node on Edge Drop"</strong>
                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666;">
                            "Drop connections on empty space to create new nodes"
                        </p>

                        // Instructions
                        <div style="margin-bottom: 12px; padding: 8px; background: #e3f2fd; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px; color: #1976d2;">"How to use:"</div>
                            <ol style="font-size: 11px; margin: 0; padding-left: 18px; color: #666;">
                                <li>"Click and drag from a source handle (right side)"</li>
                                <li>"Release on empty canvas space"</li>
                                <li>"Choose node type from picker"</li>
                                <li>"New node is created and connected"</li>
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

                        // Statistics
                        <div style="margin-bottom: 12px; padding: 8px; background: #f5f5f5; border-radius: 4px;">
                            <div style="font-size: 11px; display: flex; justify-content: space-between; margin-bottom: 4px;">
                                <span style="color: #666;">"Total Nodes:"</span>
                                <span style="font-weight: 600;">{move || store.get_nodes().len()}</span>
                            </div>
                            <div style="font-size: 11px; display: flex; justify-content: space-between;">
                                <span style="color: #666;">"Total Edges:"</span>
                                <span style="font-weight: 600;">{move || store.get_edges().len()}</span>
                            </div>
                        </div>

                        // Action log
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"Action Log:"</div>
                        <div style="max-height: 100px; overflow-y: auto; font-size: 10px;">
                            {move || {
                                let log = action_log.get();
                                if log.is_empty() {
                                    view! { <div style="color: #999;">"Drag from a source handle..."</div> }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        let entry = entry.clone();
                                        let (icon, color) = if entry.contains("Created") {
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

                // CSS for animations
                <style>
                    {"
                    @keyframes pulse-picker {
                        0%, 100% { transform: scale(1); box-shadow: 0 4px 12px rgba(0,0,0,0.15); }
                        50% { transform: scale(1.02); box-shadow: 0 6px 16px rgba(0,0,0,0.2); }
                    }
                    .node-type-picker {
                        animation: pulse-picker 2s ease-in-out infinite;
                    }
                    .node-type-option:hover {
                        transform: scale(1.05);
                        box-shadow: 0 2px 8px rgba(0,0,0,0.2);
                    }
                    "}
                </style>
            </div>
        </div>
    }
}

/// Node type picker component that appears when dropping a connection
#[component]
fn NodeTypePicker<F>(
    x: f64,
    y: f64,
    on_select: F,
) -> impl IntoView
where
    F: Fn(NodeTypeOption) + Clone + Send + Sync + 'static,
{
    let options = vec![
        NodeTypeOption::Default,
        NodeTypeOption::Input,
        NodeTypeOption::Output,
        NodeTypeOption::Process,
    ];

    view! {
        <div
            class="node-type-picker"
            style=format!(
                "position: fixed; left: {}px; top: {}px; transform: translate(-50%, -50%); z-index: 1000; background: white; border-radius: 12px; padding: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); min-width: 160px;",
                x, y
            )
            on:click=|ev: leptos::ev::MouseEvent| {
                // Prevent click from bubbling to background
                ev.stop_propagation();
            }
        >
            <div style="font-size: 12px; font-weight: 600; margin-bottom: 8px; color: #333; text-align: center;">
                "Select Node Type"
            </div>
            <div style="display: flex; flex-direction: column; gap: 6px;">
                {options.into_iter().map(|option| {
                    let on_select = on_select.clone();
                    let (bg_color, border_color) = option.color();
                    view! {
                        <button
                            class="node-type-option"
                            style=format!(
                                "display: flex; align-items: center; gap: 8px; padding: 8px 12px; border: 2px solid {}; background: {}; border-radius: 8px; cursor: pointer; transition: all 0.2s ease; font-size: 12px;",
                                border_color, bg_color
                            )
                            on:click=move |ev: leptos::ev::MouseEvent| {
                                ev.stop_propagation();
                                on_select(option);
                            }
                        >
                            <span style="font-size: 16px; color: white;">{option.icon()}</span>
                            <span style="color: white; font-weight: 500;">{option.label()}</span>
                        </button>
                    }
                }).collect_view()}
            </div>
            <div style="margin-top: 8px; text-align: center; font-size: 10px; color: #999;">
                "Click elsewhere to cancel"
            </div>
        </div>
    }
}

/// Connection line renderer - shows preview when dragging from handle
#[component]
fn ConnectionLineRenderer<F>(
    store: FlowStore,
    edge_drop: RwSignal<Option<EdgeDropState>>,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    // Track connection in progress
    let connection_source = RwSignal::new(Option::<(String, Position)>::None);
    let mouse_pos = RwSignal::new(Position::new(0.0, 0.0));

    // Watch for connection state changes
    let add_log_for_effect = add_log.clone();
    Effect::new(move || {
        let conn_state = store.state.connection_in_progress.get();

        if let Some(conn) = conn_state {
            // Connection started
            let source_id = conn.from_node.clone();
            let source_pos = conn.from_position;
            connection_source.set(Some((source_id.clone(), source_pos)));

            // Log connection start
            add_log_for_effect(format!("Connection started from {}", source_id));
        } else if connection_source.get().is_some() {
            // Connection ended - check if we should show the picker
            let current_pos = mouse_pos.get();

            // Check if the drop is on empty space (not on a node)
            let nodes = store.get_nodes();
            let node_width = 120.0;
            let node_height = 40.0;

            let is_on_node = nodes.iter().any(|n| {
                current_pos.x >= n.position.x
                    && current_pos.x <= n.position.x + node_width
                    && current_pos.y >= n.position.y
                    && current_pos.y <= n.position.y + node_height
            });

            if !is_on_node {
                // Get source info before clearing
                if let Some((source_id, _source_pos)) = connection_source.get() {
                    // Set the edge drop state to show the picker
                    edge_drop.set(Some(EdgeDropState {
                        drop_position: current_pos,
                        source_node: source_id,
                        source_handle: None,
                    }));
                }
            }

            connection_source.set(None);
        }
    });

    // Track mouse position globally for drop detection
    Effect::new(move || {
        use leptos::wasm_bindgen::JsCast;

        let document = leptos::prelude::document();
        let handler = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move |ev: leptos::web_sys::MouseEvent| {
            let viewport = store.get_viewport();
            let client_x = ev.client_x() as f64;
            let client_y = ev.client_y() as f64;

            // Convert to flow coordinates
            let flow_x = (client_x - viewport.x) / viewport.zoom;
            let flow_y = (client_y - viewport.y) / viewport.zoom;

            mouse_pos.set(Position::new(flow_x, flow_y));
        }) as Box<dyn FnMut(leptos::web_sys::MouseEvent)>);

        let _ = document.add_event_listener_with_callback(
            "mousemove",
            handler.as_ref().unchecked_ref(),
        );

        // Keep the handler alive
        handler.forget();
    });

    // Render connection preview line
    view! {
        {move || {
            if let Some(conn) = store.state.connection_in_progress.get() {
                let from = conn.from_position;
                let to = conn.to_position;

                // Generate bezier path
                let dx = to.x - from.x;
                let offset = (dx.abs() / 2.0).max(50.0).min(150.0);

                let path = format!(
                    "M {} {} C {} {}, {} {}, {} {}",
                    from.x, from.y,
                    from.x + offset, from.y,
                    to.x - offset, to.y,
                    to.x, to.y
                );

                view! {
                    <svg class="connection-preview" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000; overflow: visible;">
                        <defs>
                            <linearGradient id="connection-preview-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                                <stop offset="0%" style="stop-color:#4ade80;stop-opacity:1" />
                                <stop offset="100%" style="stop-color:#60a5fa;stop-opacity:1" />
                            </linearGradient>
                        </defs>
                        // Shadow layer
                        <path
                            d=path.clone()
                            fill="none"
                            stroke="rgba(0,0,0,0.1)"
                            stroke-width="8"
                            stroke-linecap="round"
                        />
                        // Main line
                        <path
                            d=path.clone()
                            fill="none"
                            stroke="url(#connection-preview-gradient)"
                            stroke-width="3"
                            stroke-dasharray="8,4"
                            stroke-linecap="round"
                            class="animated-connection-line"
                        />
                        // Drop indicator circle
                        <circle
                            cx=to.x
                            cy=to.y
                            r="10"
                            fill="#4ade80"
                            fill-opacity="0.3"
                            stroke="#4ade80"
                            stroke-width="2"
                        />
                        <circle
                            cx=to.x
                            cy=to.y
                            r="4"
                            fill="#4ade80"
                        />
                        // Animation
                        <style>
                            {"
                            @keyframes dash-flow {
                                from { stroke-dashoffset: 24; }
                                to { stroke-dashoffset: 0; }
                            }
                            .animated-connection-line {
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

/// Node component for this example
#[component]
fn AddNodeDropNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_drag = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("node_type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let custom_color = node.data.get("color")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let has_source = node_type != "target";
    let has_target = node_type != "source";

    // Node colors based on type (or custom color)
    let (bg_color, border_color) = match (custom_color.as_deref(), node_type.as_str()) {
        (Some(color), _) => (color.to_string(), color.to_string()),
        (None, "source") => ("#4ade80".to_string(), "#22c55e".to_string()),
        (None, "target") => ("#f87171".to_string(), "#ef4444".to_string()),
        _ => ("#60a5fa".to_string(), "#3b82f6".to_string()),
    };

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging the node
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
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
            <div
                class="add-node-drop-node"
                style=format!(
                    "background: {}; border: 2px solid {}; padding: 10px 20px; border-radius: 8px; min-width: 80px; text-align: center; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                    bg_color, border_color
                )
            >
                // Target handle (left side)
                {has_target.then(|| {
                    view! {
                        <Handle
                            node_id=node_id.clone()
                            r#type=HandleType::Target
                            position=HandlePosition::Left
                            connection_mode=ConnectionMode::Strict
                            style="width: 12px; height: 12px; background: #999; border-radius: 50%; border: 2px solid white;".to_string()
                        />
                    }
                })}

                <div style="color: white; font-weight: 500; font-size: 13px; pointer-events: none;">
                    {label}
                </div>

                // Source handle (right side)
                {has_source.then(|| {
                    view! {
                        <Handle
                            node_id=node_id.clone()
                            r#type=HandleType::Source
                            position=HandlePosition::Right
                            connection_mode=ConnectionMode::Strict
                            style="width: 12px; height: 12px; background: #999; border-radius: 50%; border: 2px solid white;".to_string()
                        />
                    }
                })}
            </div>
        </div>
    }
}

/// Custom edge renderer for this example
#[component]
fn AddNodeEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg class="add-node-edges" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            <defs>
                <linearGradient id="add-node-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#4ade80;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#60a5fa;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="add-node-arrow"
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
                        // Calculate edge endpoints
                        let node_width = 120.0;
                        let node_height = 40.0;

                        let source_x = source.position.x + node_width;  // Right side
                        let source_y = source.position.y + node_height / 2.0;
                        let target_x = target.position.x;               // Left side
                        let target_y = target.position.y + node_height / 2.0;

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

                        // Calculate label position
                        let label_x = (source_x + target_x) / 2.0;
                        let label_y = (source_y + target_y) / 2.0;

                        let edge_label = edge.label.clone().unwrap_or_default();
                        let has_label = !edge_label.is_empty();

                        view! {
                            <g class="add-node-edge">
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
                                    stroke="url(#add-node-edge-gradient)"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    marker-end="url(#add-node-arrow)"
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
