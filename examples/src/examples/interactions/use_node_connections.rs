//! UseNodeConnections Example
//!
//! Demonstrates how to get all connections for a specific node:
//! - Implement hook to get node's incoming and outgoing edges
//! - Display connection info in node or panel
//! - Update reactively when connections change
//! - Show connection statistics

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for this example
static NODE_CONNECTIONS_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_node_connections_drag_signal() -> RwSignal<Option<DragState>> {
    *NODE_CONNECTIONS_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Connection info for a node
#[derive(Clone, Debug)]
struct NodeConnectionInfo {
    node_id: String,
    incoming: Vec<ConnectionDetail>,
    outgoing: Vec<ConnectionDetail>,
}

/// Details about a connection
#[derive(Clone, Debug)]
struct ConnectionDetail {
    edge_id: String,
    edge_label: Option<String>,
    connected_node_id: String,
    connected_node_label: String,
}

/// UseNodeConnections example
#[component]
pub fn UseNodeConnectionsExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        // Source nodes (left column)
        Node::new("source-1".to_string(), Position::new(50.0, 60.0))
            .with_data(json!({
                "label": "Source A",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("source-2".to_string(), Position::new(50.0, 160.0))
            .with_data(json!({
                "label": "Source B",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),
        // Center node (hub)
        Node::new("hub".to_string(), Position::new(220.0, 110.0))
            .with_data(json!({
                "label": "Hub Node",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(110.0, 50.0),
        // Target nodes (right column)
        Node::new("target-1".to_string(), Position::new(400.0, 40.0))
            .with_data(json!({
                "label": "Target X",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("target-2".to_string(), Position::new(400.0, 130.0))
            .with_data(json!({
                "label": "Target Y",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("target-3".to_string(), Position::new(400.0, 220.0))
            .with_data(json!({
                "label": "Target Z",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(100.0, 50.0),
    ];

    // Create edges
    let initial_edges = vec![
        // Sources to Hub
        Edge::new("e1".to_string(), "source-1".to_string(), "hub".to_string())
            .with_label("e1".to_string()),
        Edge::new("e2".to_string(), "source-2".to_string(), "hub".to_string())
            .with_label("e2".to_string()),
        // Hub to Targets
        Edge::new("e3".to_string(), "hub".to_string(), "target-1".to_string())
            .with_label("e3".to_string()),
        Edge::new("e4".to_string(), "hub".to_string(), "target-2".to_string())
            .with_label("e4".to_string()),
        Edge::new("e5".to_string(), "hub".to_string(), "target-3".to_string())
            .with_label("e5".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Track selected node
    let selected_node_id = RwSignal::new(Some("hub".to_string()));

    // Connection change log
    let connection_log = RwSignal::new(Vec::<String>::new());

    // Node counter for creating new nodes
    let node_counter = RwSignal::new(7_i32);

    // Global drag handlers
    let drag_signal = get_node_connections_drag_signal();

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

    // Add edge handler
    let add_edge = move |_| {
        // Get current selected node
        if let Some(node_id) = selected_node_id.get() {
            let nodes = store.get_nodes();
            let edges = store.get_edges();

            // Find a node that isn't already connected to this one
            let node = nodes.iter().find(|n| n.id == node_id);
            if let Some(selected) = node {
                let node_type = selected.data.get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");

                // Find available nodes to connect
                let connected_nodes: std::collections::HashSet<String> = edges.iter()
                    .filter_map(|e| {
                        if e.source == node_id {
                            Some(e.target.clone())
                        } else if e.target == node_id {
                            Some(e.source.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                // Find an unconnected node
                let available = nodes.iter()
                    .find(|n| n.id != node_id && !connected_nodes.contains(&n.id));

                if let Some(target) = available {
                    let target_type = target.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let edge_count = edges.len();
                    let edge_id = format!("e{}", edge_count + 1);

                    // Determine direction based on node types
                    let (source, target_id) = if node_type == "output" || target_type == "input" {
                        (target.id.clone(), node_id.clone())
                    } else {
                        (node_id.clone(), target.id.clone())
                    };

                    store.add_edge(Edge::new(edge_id.clone(), source, target_id)
                        .with_label(edge_id.clone()));

                    let time = format_timestamp();
                    connection_log.update(|log| {
                        log.insert(0, format!("[{}] Added edge {}", time, edge_id));
                        if log.len() > 10 {
                            log.pop();
                        }
                    });
                }
            }
        }
    };

    // Remove edge handler
    let remove_edge = move |_| {
        if let Some(node_id) = selected_node_id.get() {
            let edges = store.get_edges();

            // Find an edge connected to this node
            let edge_to_remove = edges.iter()
                .find(|e| e.source == node_id || e.target == node_id);

            if let Some(edge) = edge_to_remove {
                let edge_id = edge.id.clone();
                store.remove_edge(&edge_id);

                let time = format_timestamp();
                connection_log.update(|log| {
                    log.insert(0, format!("[{}] Removed edge {}", time, edge_id));
                    if log.len() > 10 {
                        log.pop();
                    }
                });
            }
        }
    };

    // Add node handler
    let add_node = move |_| {
        let count = node_counter.get();
        let node_id = format!("node-{}", count);

        let new_node = Node::new(node_id.clone(), Position::new(250.0, 250.0 + ((count - 7) * 60) as f64))
            .with_data(json!({
                "label": format!("Node {}", count),
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(100.0, 50.0);

        store.add_node(new_node);
        node_counter.set(count + 1);

        let time = format_timestamp();
        connection_log.update(|log| {
            log.insert(0, format!("[{}] Added node {}", time, node_id));
            if log.len() > 10 {
                log.pop();
            }
        });
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <NodeConnectionsEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <ConnectionNode
                                    node=node.clone()
                                    store=store
                                    selected_node_id=selected_node_id
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Use Node Connections"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click a node to select it. The panel shows all incoming and outgoing connections for the selected node."
                        </p>

                        // Selected node info
                        {move || {
                            if let Some(node_id) = selected_node_id.get() {
                                let nodes = store.get_nodes();
                                let edges = store.get_edges();

                                let node = nodes.iter().find(|n| n.id == node_id);
                                if let Some(n) = node {
                                    let label = n.data.get("label")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("Node")
                                        .to_string();
                                    let color = n.data.get("color")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("#6366f1")
                                        .to_string();

                                    // Get incoming edges (where this node is target)
                                    let incoming: Vec<_> = edges.iter()
                                        .filter(|e| e.target == node_id)
                                        .map(|e| {
                                            let source_node = nodes.iter().find(|n| n.id == e.source);
                                            let source_label = source_node
                                                .and_then(|sn| sn.data.get("label"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Unknown")
                                                .to_string();
                                            (e.id.clone(), e.label.clone(), source_label)
                                        })
                                        .collect();

                                    // Get outgoing edges (where this node is source)
                                    let outgoing: Vec<_> = edges.iter()
                                        .filter(|e| e.source == node_id)
                                        .map(|e| {
                                            let target_node = nodes.iter().find(|n| n.id == e.target);
                                            let target_label = target_node
                                                .and_then(|tn| tn.data.get("label"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Unknown")
                                                .to_string();
                                            (e.id.clone(), e.label.clone(), target_label)
                                        })
                                        .collect();

                                    let total_connections = incoming.len() + outgoing.len();

                                    view! {
                                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                                            // Selected node header
                                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px; padding-bottom: 8px; border-bottom: 1px solid #e2e8f0;">
                                                <div style=format!(
                                                    "width: 12px; height: 12px; border-radius: 3px; background: {};",
                                                    color
                                                )></div>
                                                <span style="font-weight: 600; font-size: 13px; color: #333;">{label}</span>
                                                <span style="margin-left: auto; font-size: 10px; color: #64748b; background: #e2e8f0; padding: 2px 6px; border-radius: 10px;">
                                                    {total_connections} " conn"
                                                </span>
                                            </div>

                                            // Connection counts
                                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 10px;">
                                                <div style="background: #dbeafe; padding: 8px; border-radius: 6px; text-align: center;">
                                                    <div style="font-size: 18px; font-weight: 700; color: #2563eb;">{incoming.len()}</div>
                                                    <div style="font-size: 9px; color: #3b82f6; font-weight: 500;">"Incoming"</div>
                                                </div>
                                                <div style="background: #dcfce7; padding: 8px; border-radius: 6px; text-align: center;">
                                                    <div style="font-size: 18px; font-weight: 700; color: #16a34a;">{outgoing.len()}</div>
                                                    <div style="font-size: 9px; color: #22c55e; font-weight: 500;">"Outgoing"</div>
                                                </div>
                                            </div>

                                            // Incoming connections list
                                            {if !incoming.is_empty() {
                                                view! {
                                                    <div style="margin-bottom: 8px;">
                                                        <div style="font-size: 10px; font-weight: 600; color: #2563eb; margin-bottom: 4px; display: flex; align-items: center; gap: 4px;">
                                                            <span style="font-size: 12px;">"←"</span> "Incoming"
                                                        </div>
                                                        {incoming.iter().map(|(edge_id, edge_label, source_label)| {
                                                            view! {
                                                                <div style="display: flex; align-items: center; gap: 6px; padding: 4px 6px; background: #eff6ff; border-radius: 4px; margin-bottom: 2px; font-size: 10px;">
                                                                    <span style="color: #64748b;">{source_label.clone()}</span>
                                                                    <span style="color: #3b82f6;">"→"</span>
                                                                    <span style="color: #94a3b8; font-family: monospace; font-size: 9px;">
                                                                        {edge_label.clone().unwrap_or_else(|| edge_id.clone())}
                                                                    </span>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }}

                                            // Outgoing connections list
                                            {if !outgoing.is_empty() {
                                                view! {
                                                    <div>
                                                        <div style="font-size: 10px; font-weight: 600; color: #16a34a; margin-bottom: 4px; display: flex; align-items: center; gap: 4px;">
                                                            <span style="font-size: 12px;">"→"</span> "Outgoing"
                                                        </div>
                                                        {outgoing.iter().map(|(edge_id, edge_label, target_label)| {
                                                            view! {
                                                                <div style="display: flex; align-items: center; gap: 6px; padding: 4px 6px; background: #f0fdf4; border-radius: 4px; margin-bottom: 2px; font-size: 10px;">
                                                                    <span style="color: #22c55e;">"→"</span>
                                                                    <span style="color: #64748b;">{target_label.clone()}</span>
                                                                    <span style="color: #94a3b8; font-family: monospace; font-size: 9px;">
                                                                        {edge_label.clone().unwrap_or_else(|| edge_id.clone())}
                                                                    </span>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; text-align: center; color: #999; font-size: 11px; margin-bottom: 12px;">
                                            "Click a node to see its connections"
                                        </div>
                                    }.into_any()
                                }
                            } else {
                                view! {
                                    <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; text-align: center; color: #999; font-size: 11px; margin-bottom: 12px;">
                                        "Click a node to see its connections"
                                    </div>
                                }.into_any()
                            }
                        }}

                        // Actions
                        <div style="display: flex; gap: 4px; margin-bottom: 12px;">
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 9px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=add_edge
                            >
                                "+ Edge"
                            </button>
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 9px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=remove_edge
                            >
                                "- Edge"
                            </button>
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 9px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=add_node
                            >
                                "+ Node"
                            </button>
                        </div>

                        // Connection log
                        <div style="border-top: 1px solid #eee; padding-top: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Connection Log"</div>
                            <div style="background: #f8f9fa; border-radius: 6px; padding: 8px; max-height: 100px; overflow-y: auto;">
                                {move || {
                                    let log = connection_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                                "Changes will appear here"
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.into_iter().map(|entry| {
                                            view! {
                                                <div style="font-size: 10px; color: #666; padding: 2px 0; font-family: monospace;">
                                                    {entry}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // Node types legend
                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 10px; font-weight: 600; color: #333; margin-bottom: 6px;">"Node Types"</div>
                            <div style="display: flex; gap: 12px; font-size: 10px; color: #666;">
                                <div style="display: flex; align-items: center; gap: 4px;">
                                    <div style="width: 8px; height: 8px; background: #10b981; border-radius: 2px;"></div>
                                    "Source"
                                </div>
                                <div style="display: flex; align-items: center; gap: 4px;">
                                    <div style="width: 8px; height: 8px; background: #6366f1; border-radius: 2px;"></div>
                                    "Default"
                                </div>
                                <div style="display: flex; align-items: center; gap: 4px;">
                                    <div style="width: 8px; height: 8px; background: #ef4444; border-radius: 2px;"></div>
                                    "Target"
                                </div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Format current timestamp
fn format_timestamp() -> String {
    let date = js_sys::Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}

/// Connection node component showing connection count
#[component]
fn ConnectionNode(
    node: Node,
    store: FlowStore,
    selected_node_id: RwSignal<Option<String>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();
    let node_id_for_handles = node.id.clone();
    let node_id_for_connections = node.id.clone();

    let drag_signal = get_node_connections_drag_signal();

    // Mouse down - start dragging and select
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let current_id = node_id_for_select.clone();

        // Select this node
        selected_node_id.set(Some(current_id.clone()));

        // Start dragging
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

    view! {
        <div
            class="xyflow__node connection-node"
            style=move || {
                let nodes = store.get_nodes();
                let is_selected = selected_node_id.get().as_ref() == Some(&node_id_for_style);
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let border_color = if is_selected { color } else { "#ddd" };
                    let box_shadow = if is_selected {
                        format!("0 0 0 2px {}40, 0 4px 8px rgba(0,0,0,0.15)", color)
                    } else {
                        "0 2px 4px rgba(0,0,0,0.1)".to_string()
                    };
                    let background = if is_selected {
                        format!("{}15", color)
                    } else {
                        "white".to_string()
                    };

                    // Different styling for input/output nodes
                    let border_style = match node_type {
                        "input" => "2px solid",
                        "output" => "2px dashed",
                        _ => "2px solid",
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: {} {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 4px; box-sizing: border-box; transition: all 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(100.0), n.height.unwrap_or(50.0),
                        background, border_style, border_color, box_shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_mousedown
        >
            // Node label
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_label) {
                    let label = n.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1")
                        .to_string();

                    view! {
                        <div style=format!("font-weight: 600; font-size: 11px; color: {};", color)>
                            {label}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Connection count badge
            {move || {
                let edges = store.get_edges();
                let incoming_count = edges.iter().filter(|e| e.target == node_id_for_connections).count();
                let outgoing_count = edges.iter().filter(|e| e.source == node_id_for_connections).count();
                let total = incoming_count + outgoing_count;

                if total > 0 {
                    view! {
                        <div style="display: flex; gap: 4px; margin-top: 2px;">
                            {(incoming_count > 0).then(|| view! {
                                <span style="font-size: 9px; background: #dbeafe; color: #2563eb; padding: 1px 4px; border-radius: 3px;">
                                    "←" {incoming_count}
                                </span>
                            })}
                            {(outgoing_count > 0).then(|| view! {
                                <span style="font-size: 9px; background: #dcfce7; color: #16a34a; padding: 1px 4px; border-radius: 3px;">
                                    {outgoing_count} "→"
                                </span>
                            })}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="font-size: 8px; color: #999; margin-top: 2px;">"no connections"</div>
                    }.into_any()
                }
            }}

            // Handles
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_handles) {
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");
                    let has_source = node_type != "output";
                    let has_target = node_type != "input";

                    view! {
                        <>
                            {has_target.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Target
                                    position=HandlePosition::Top
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Bottom
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                        </>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Edge renderer component
#[component]
fn NodeConnectionsEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="node-connections-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#888" />
                </marker>
                <linearGradient id="node-connections-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#6366f1;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#8b5cf6;stop-opacity:1" />
                </linearGradient>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    let edge_id = edge.id.clone();

                    // Calculate edge path
                    let sx = source_node.position.x + source_node.width.unwrap_or(100.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(100.0) / 2.0;
                    let ty = target_node.position.y;

                    let offset = (ty - sy).abs() * 0.4;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx, sy + offset,
                        tx, ty - offset,
                        tx, ty
                    );

                    // Calculate midpoint for label
                    let mid_x = (sx + tx) / 2.0;
                    let mid_y = (sy + ty) / 2.0;

                    let label = edge.label.clone().unwrap_or_else(|| edge_id.clone());

                    Some(view! {
                        <g class="xyflow__edge">
                            // Edge path
                            <path
                                d=path.clone()
                                stroke="url(#node-connections-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#node-connections-arrow)"
                            />

                            // Edge label
                            <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                <rect
                                    x="-16"
                                    y="-8"
                                    width="32"
                                    height="16"
                                    fill="white"
                                    stroke="#e2e8f0"
                                    stroke-width="1"
                                    rx="4"
                                />
                                <text
                                    x="0"
                                    y="4"
                                    text-anchor="middle"
                                    font-size="9"
                                    fill="#64748b"
                                    font-weight="500"
                                >
                                    {label}
                                </text>
                            </g>
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
