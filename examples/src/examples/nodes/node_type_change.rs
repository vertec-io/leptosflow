//! Node Type Change Example
//!
//! Demonstrates how to dynamically change a node's type at runtime.
//! Nodes can switch between input, default, and output types, which
//! changes their handle configuration.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for node type change example
static NODE_TYPE_CHANGE_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_node_type_change_drag_signal() -> RwSignal<Option<DragState>> {
    *NODE_TYPE_CHANGE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Node type change example showing nodes that can switch between types
#[component]
pub fn NodeTypeChangeExample() -> impl IntoView {
    // Create initial nodes with type stored in data
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({
                "label": "Node 1",
                "nodeType": "input"
            }))
            .with_dimensions(180.0, 80.0),
        Node::new("2".to_string(), Position::new(350.0, 100.0))
            .with_data(json!({
                "label": "Node 2",
                "nodeType": "default"
            }))
            .with_dimensions(180.0, 80.0),
        Node::new("3".to_string(), Position::new(600.0, 100.0))
            .with_data(json!({
                "label": "Node 3",
                "nodeType": "output"
            }))
            .with_dimensions(180.0, 80.0),
        Node::new("4".to_string(), Position::new(220.0, 280.0))
            .with_data(json!({
                "label": "Node 4",
                "nodeType": "default"
            }))
            .with_dimensions(180.0, 80.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_node_type_change_drag_signal();

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

    // Selected node for type change
    let selected_node_id = RwSignal::new(Option::<String>::None);

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Edge renderer
                    <NodeTypeChangeEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes with type change capability
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <TypeChangeNode
                                    node=node.clone()
                                    store=store
                                    selected_node_id=selected_node_id
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel with type change controls
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); max-width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Node Type Change"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666; line-height: 1.4;">
                            "Click on a node to select it, then use the buttons below to change its type. Each type has different handle configurations."
                        </p>

                        // Selected node info
                        {move || {
                            let nodes = store.get_nodes();
                            match selected_node_id.get() {
                                Some(id) => {
                                    if let Some(node) = nodes.iter().find(|n| n.id == id) {
                                        let label = node.data.get("label")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Unknown")
                                            .to_string();
                                        let node_type = node.data.get("nodeType")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("default")
                                            .to_string();

                                        let (type_color, type_icon) = match node_type.as_str() {
                                            "input" => ("#4caf50", "-->"),
                                            "output" => ("#f44336", "<--"),
                                            _ => ("#2196f3", "<->"),
                                        };

                                        view! {
                                            <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; margin-bottom: 12px;">
                                                <div style="font-weight: 600; font-size: 13px; margin-bottom: 8px;">
                                                    "Selected: " {label}
                                                </div>
                                                <div style="display: flex; align-items: center; gap: 8px;">
                                                    <span style=format!("
                                                        display: inline-block;
                                                        padding: 4px 8px;
                                                        background: {};
                                                        color: white;
                                                        border-radius: 4px;
                                                        font-size: 11px;
                                                        font-weight: 600;
                                                    ", type_color)>
                                                        {type_icon} " " {node_type.to_uppercase()}
                                                    </span>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; margin-bottom: 12px; color: #999; font-size: 12px;">
                                                "No node selected"
                                            </div>
                                        }.into_any()
                                    }
                                },
                                None => {
                                    view! {
                                        <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; margin-bottom: 12px; color: #999; font-size: 12px;">
                                            "Click a node to select it"
                                        </div>
                                    }.into_any()
                                }
                            }
                        }}

                        // Type change buttons
                        <div style="display: flex; flex-direction: column; gap: 8px;">
                            <TypeChangeButton
                                label="Input Node"
                                description="Source handle only (outputs)"
                                node_type="input"
                                color="#4caf50"
                                icon="-->"
                                store=store
                                selected_node_id=selected_node_id
                            />
                            <TypeChangeButton
                                label="Default Node"
                                description="Both handles (input & output)"
                                node_type="default"
                                color="#2196f3"
                                icon="<->"
                                store=store
                                selected_node_id=selected_node_id
                            />
                            <TypeChangeButton
                                label="Output Node"
                                description="Target handle only (inputs)"
                                node_type="output"
                                color="#f44336"
                                icon="<--"
                                store=store
                                selected_node_id=selected_node_id
                            />
                        </div>

                        // Legend
                        <div style="margin-top: 16px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; color: #888; margin-bottom: 8px; font-weight: 600;">"Handle Types:"</div>
                            <div style="font-size: 10px; color: #666; line-height: 1.5;">
                                <div>"Input: source handle (bottom) only"</div>
                                <div>"Default: both target (top) & source (bottom)"</div>
                                <div>"Output: target handle (top) only"</div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Button component for changing node type
#[component]
fn TypeChangeButton(
    label: &'static str,
    description: &'static str,
    node_type: &'static str,
    color: &'static str,
    icon: &'static str,
    store: FlowStore,
    selected_node_id: RwSignal<Option<String>>,
) -> impl IntoView {
    let on_click = move |_| {
        if let Some(id) = selected_node_id.get() {
            // Update node data with new type
            store.update_node(&id, |n| {
                if let Some(data) = n.data.as_object_mut() {
                    data.insert("nodeType".to_string(), json!(node_type));
                }
            });
        }
    };

    let is_disabled = move || selected_node_id.get().is_none();
    let is_current_type = move || {
        if let Some(id) = selected_node_id.get() {
            let nodes = store.get_nodes();
            if let Some(node) = nodes.iter().find(|n| n.id == id) {
                return node.data.get("nodeType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default") == node_type;
            }
        }
        false
    };

    view! {
        <button
            style=move || format!(
                "display: flex; align-items: center; gap: 10px; width: 100%; padding: 10px 12px; \
                 background: {}; color: white; border: none; border-radius: 6px; cursor: {}; \
                 font-size: 12px; text-align: left; transition: all 0.15s; opacity: {};",
                if is_current_type() { color } else { "#888" },
                if is_disabled() { "not-allowed" } else { "pointer" },
                if is_disabled() { "0.5" } else { "1" }
            )
            on:click=on_click
            disabled=is_disabled
        >
            <span style="font-family: monospace; font-size: 14px; font-weight: bold;">{icon}</span>
            <div>
                <div style="font-weight: 600;">{label}</div>
                <div style="font-size: 10px; opacity: 0.9;">{description}</div>
            </div>
        </button>
    }
}

/// Node with type change capability
#[component]
fn TypeChangeNode(
    node: Node,
    store: FlowStore,
    selected_node_id: RwSignal<Option<String>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_click = node.id.clone();
    let node_id_for_selected = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();

    let drag_signal = get_node_type_change_drag_signal();

    // Clone for selection
    let node_id_for_select = node_id.clone();

    // Mouse down - start dragging and select node
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Select this node
        selected_node_id.set(Some(node_id_for_select.clone()));

        // Get current node position for dragging
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

    // Click handler - select node (backup for click events)
    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        selected_node_id.set(Some(node_id_for_click.clone()));
    };

    // Clone node_id for multiple uses
    let node_id_for_style = node_id.clone();
    let node_id_for_type = node_id.clone();
    let node_id_for_handles = node_id.clone();

    // Check if this node is selected
    let is_selected = move || {
        selected_node_id.get().as_ref() == Some(&node_id_for_selected)
    };

    // Helper to get node type
    let get_node_type = move |nid: &str| {
        let nodes = store.get_nodes();
        nodes.iter()
            .find(|n| n.id == nid)
            .and_then(|n| n.data.get("nodeType"))
            .and_then(|v| v.as_str())
            .unwrap_or("default")
            .to_string()
    };

    view! {
        <div
            class="xyflow__node type-change-node"
            style=move || {
                let nodes = store.get_nodes();
                let (pos, width, height, node_type) = nodes.iter()
                    .find(|n| n.id == node_id_for_style)
                    .map(|n| {
                        let node_type = n.data.get("nodeType")
                            .and_then(|v| v.as_str())
                            .unwrap_or("default")
                            .to_string();
                        (n.position, n.width.unwrap_or(180.0), n.height.unwrap_or(80.0), node_type)
                    })
                    .unwrap_or((Position::new(0.0, 0.0), 180.0, 80.0, "default".to_string()));

                // Color based on node type
                let (bg_color, border_color) = match node_type.as_str() {
                    "input" => ("#e8f5e9", "#4caf50"),
                    "output" => ("#ffebee", "#f44336"),
                    _ => ("#e3f2fd", "#2196f3"),
                };

                // Selection ring
                let box_shadow = if is_selected() {
                    format!("0 0 0 3px {}, 0 2px 8px rgba(0,0,0,0.15)", border_color)
                } else {
                    "0 2px 8px rgba(0,0,0,0.1)".to_string()
                };

                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     box-shadow: {}; cursor: grab; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 12px; box-sizing: border-box; transition: box-shadow 0.15s, background 0.15s, border-color 0.15s;",
                    pos.x, pos.y, width, height, bg_color, border_color, box_shadow
                )
            }
            on:mousedown=on_mousedown
            on:click=on_click
        >
            // Type indicator
            {move || {
                let node_type = get_node_type(&node_id_for_type);
                let (icon, type_label) = match node_type.as_str() {
                    "input" => ("-->", "INPUT"),
                    "output" => ("<--", "OUTPUT"),
                    _ => ("<->", "DEFAULT"),
                };

                view! {
                    <div style="font-size: 10px; color: #888; margin-bottom: 4px; font-weight: 600; letter-spacing: 0.5px;">
                        <span style="font-family: monospace;">{icon}</span>
                        " " {type_label}
                    </div>
                }
            }}

            // Node label
            <div style="font-weight: 600; font-size: 13px; color: #333;">
                {label}
            </div>

            // Handles based on current node type
            {move || {
                let node_type = get_node_type(&node_id_for_handles);
                let has_source = node_type != "output";
                let has_target = node_type != "input";

                let border_color = match node_type.as_str() {
                    "input" => "#4caf50",
                    "output" => "#f44336",
                    _ => "#2196f3",
                };

                view! {
                    <>
                        // Target handle (top)
                        {has_target.then(|| view! {
                            <Handle
                                node_id=node_id.clone()
                                r#type=HandleType::Target
                                position=HandlePosition::Top
                                connection_mode=ConnectionMode::Strict
                                style=format!(
                                    "background: {}; width: 12px; height: 12px; border: 2px solid white; \
                                     box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
                                    border_color
                                )
                            />
                        })}

                        // Source handle (bottom)
                        {has_source.then(|| view! {
                            <Handle
                                node_id=node_id.clone()
                                r#type=HandleType::Source
                                position=HandlePosition::Bottom
                                connection_mode=ConnectionMode::Strict
                                style=format!(
                                    "background: {}; width: 12px; height: 12px; border: 2px solid white; \
                                     box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
                                    border_color
                                )
                            />
                        })}
                    </>
                }
            }}
        </div>
    }
}

/// Edge renderer for the node type change example
#[component]
fn NodeTypeChangeEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="type-change-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#888" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Check if source has source handle and target has target handle
                    let source_type = source_node.data.get("nodeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");
                    let target_type = target_node.data.get("nodeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    // Only draw edge if source can output and target can input
                    let source_can_output = source_type != "output";
                    let target_can_input = target_type != "input";

                    // Calculate edge path (source bottom to target top)
                    let sx = source_node.position.x + source_node.width.unwrap_or(180.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(80.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(180.0) / 2.0;
                    let ty = target_node.position.y;

                    let offset = (ty - sy).abs() * 0.5;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx, sy + offset,
                        tx, ty - offset,
                        tx, ty
                    );

                    // Determine edge style based on validity
                    let (stroke, dasharray, opacity, marker) = if !source_can_output || !target_can_input {
                        // Disconnected edge - dashed and gray
                        ("#ccc", "6,4", "0.5", "")
                    } else {
                        // Valid edge - solid with arrow
                        ("#888", "", "1", "url(#type-change-arrow)")
                    };

                    Some(view! {
                        <g class="xyflow__edge">
                            <path
                                d=path
                                stroke=stroke
                                stroke-width="2"
                                stroke-dasharray=dasharray
                                fill="none"
                                opacity=opacity
                                marker-end=marker
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
