//! Default Edge Overwrite Example
//!
//! Demonstrates how to customize the default edge component by creating
//! a custom edge renderer that replaces the standard edge appearance.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DraggableNode};

/// Default edge overwrite example showing custom edge styling
#[component]
pub fn DefaultEdgeOverwriteExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({"label": "Node A", "type": "default", "class": "light"})),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({"label": "Node B", "type": "default", "class": "light"})),
        Node::new("3".to_string(), Position::new(300.0, 125.0))
            .with_data(json!({"label": "Node C", "type": "default", "class": "light"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("Gradient Edge".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Animated".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string())
            .with_label("Dashed".to_string()),
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
                // Background with lines pattern
                <Background variant=BackgroundVariant::Lines />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Custom edge renderer with overwritten styling
                    <CustomEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <DraggableNode
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
                        <strong style="display: block; margin-bottom: 8px;">"Default Edge Overwrite"</strong>
                        <p style="margin: 0 0 8px 0; font-size: 12px; color: #666;">
                            "Custom styling for default edges"
                        </p>
                        <div style="font-size: 11px; color: #888;">
                            <div style="margin: 4px 0;">"• Gradient stroke colors"</div>
                            <div style="margin: 4px 0;">"• Animated dashes"</div>
                            <div style="margin: 4px 0;">"• Thicker stroke width"</div>
                            <div style="margin: 4px 0;">"• Custom arrow markers"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Get the handle position for an edge endpoint
fn get_handle_position(node: &Node, handle_id: &Option<String>, is_source: bool) -> Position {
    let node_pos = &node.position;
    let node_width = node.width.unwrap_or(150.0);
    let node_height = node.height.unwrap_or(40.0);

    // Try to find handle bounds
    if let Some(ref bounds) = node.internals.handle_bounds {
        let handles = if is_source { &bounds.source } else { &bounds.target };

        // Find matching handle by ID, or use first handle
        let handle = if let Some(id) = handle_id {
            handles.iter().find(|h| h.id.as_ref() == Some(id))
        } else {
            handles.first()
        };

        if let Some(handle) = handle {
            // Handle bounds are relative to node, convert to absolute
            return handle.center_absolute(node_pos);
        }
    }

    // Fallback: use node edge positions based on handle type
    // Source handles are typically at the bottom, target handles at the top
    if is_source {
        // Bottom center
        Position::new(node_pos.x + node_width / 2.0, node_pos.y + node_height)
    } else {
        // Top center
        Position::new(node_pos.x + node_width / 2.0, node_pos.y)
    }
}

/// Generate a bezier curve path
fn generate_bezier_path(from: Position, to: Position) -> String {
    let mid_x = (from.x + to.x) / 2.0;
    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y, mid_x, from.y, mid_x, to.y, to.x, to.y
    )
}

/// Calculate the label position (midpoint) for an edge
fn calculate_label_position(from: Position, to: Position) -> Position {
    Position::new(
        (from.x + to.x) / 2.0,
        (from.y + to.y) / 2.0,
    )
}

/// Custom edge renderer that renders edges with custom styling
#[component]
fn CustomEdgeRenderer(store: FlowStore) -> impl IntoView {
    // Get edges reactively
    let edges = move || store.get_edges();

    view! {
        <svg class="xyflow__edges leptos-flow__edges" style="position: absolute; width: 100%; height: 100%; pointer-events: none;">
            // Custom gradient and marker definitions
            <defs>
                // Gradient for the first edge style
                <linearGradient id="edge-gradient-1" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>

                // Gradient for animated edges
                <linearGradient id="edge-gradient-2" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#f093fb;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#f5576c;stop-opacity:1" />
                </linearGradient>

                // Gradient for dashed edges
                <linearGradient id="edge-gradient-3" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#4facfe;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#00f2fe;stop-opacity:1" />
                </linearGradient>

                // Custom arrow markers for each gradient
                <marker
                    id="custom-arrow-1"
                    viewBox="0 0 10 10"
                    refX="9"
                    refY="5"
                    markerWidth="8"
                    markerHeight="8"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#764ba2" />
                </marker>

                <marker
                    id="custom-arrow-2"
                    viewBox="0 0 10 10"
                    refX="9"
                    refY="5"
                    markerWidth="8"
                    markerHeight="8"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f5576c" />
                </marker>

                <marker
                    id="custom-arrow-3"
                    viewBox="0 0 10 10"
                    refX="9"
                    refY="5"
                    markerWidth="8"
                    markerHeight="8"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#00f2fe" />
                </marker>
            </defs>

            <For
                each=edges
                key=|edge| edge.id.clone()
                children=move |edge| {
                    let edge_id = edge.id.clone();
                    let source_id = edge.source.clone();
                    let target_id = edge.target.clone();
                    let source_handle = edge.source_handle.clone();
                    let target_handle = edge.target_handle.clone();
                    let label = edge.label.clone();

                    // Determine edge style based on edge ID
                    let (gradient_id, marker_id, stroke_dasharray, animation_class) = match edge_id.as_str() {
                        "e1-2" => ("url(#edge-gradient-1)", "url(#custom-arrow-1)", "none", ""),
                        "e1-3" => ("url(#edge-gradient-2)", "url(#custom-arrow-2)", "10,5", "animated-edge"),
                        _ => ("url(#edge-gradient-3)", "url(#custom-arrow-3)", "5,5", ""),
                    };

                    view! {
                        <CustomEdgeComponent
                            edge_id=edge_id
                            source_id=source_id
                            target_id=target_id
                            source_handle=source_handle
                            target_handle=target_handle
                            label=label
                            gradient_id=gradient_id.to_string()
                            marker_id=marker_id.to_string()
                            stroke_dasharray=stroke_dasharray.to_string()
                            animation_class=animation_class.to_string()
                            store=store
                        />
                    }
                }
            />
        </svg>
    }
}

/// Custom edge component with overwritten styling
#[component]
fn CustomEdgeComponent(
    edge_id: String,
    source_id: String,
    target_id: String,
    source_handle: Option<String>,
    target_handle: Option<String>,
    label: Option<String>,
    gradient_id: String,
    marker_id: String,
    stroke_dasharray: String,
    animation_class: String,
    store: FlowStore,
) -> impl IntoView {
    // Create a reactive memo that recalculates when nodes change
    let path_data = Memo::new({
        let store = store.clone();
        let source_id = source_id.clone();
        let target_id = target_id.clone();
        let source_handle = source_handle.clone();
        let target_handle = target_handle.clone();
        move |_| {
            // This will track the nodes signal
            let nodes = store.get_nodes();

            // Find source and target nodes
            let source = nodes.iter().find(|n| n.id == source_id);
            let target = nodes.iter().find(|n| n.id == target_id);

            if let (Some(source), Some(target)) = (source, target) {
                let source_pos = get_handle_position(source, &source_handle, true);
                let target_pos = get_handle_position(target, &target_handle, false);

                let path = generate_bezier_path(source_pos, target_pos);
                let label_pos = calculate_label_position(source_pos, target_pos);

                (path, label_pos.x, label_pos.y)
            } else {
                (String::new(), 0.0, 0.0)
            }
        }
    });

    // Click handler for selection
    let store_click = store.clone();
    let edge_id_click = edge_id.clone();
    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        let multi_select = ev.ctrl_key() || ev.meta_key();
        store_click.select_edge(&edge_id_click, multi_select);
    };

    // Edge class (reactive for selection state)
    let edge_class = Memo::new({
        let store = store.clone();
        let edge_id = edge_id.clone();
        let animation_class = animation_class.clone();
        move |_| {
            let edges = store.get_edges();
            let edge = edges.iter().find(|e| e.id == edge_id);

            let mut classes = vec!["xyflow__edge", "leptos-flow__edge", "custom-edge"];
            if !animation_class.is_empty() {
                classes.push(&animation_class);
            }
            if let Some(edge) = edge {
                if edge.selected {
                    classes.push("selected");
                }
            }
            classes.join(" ")
        }
    });

    view! {
        <g class=move || edge_class.get() data-id=edge_id.clone() on:click=on_click>
            // Shadow/glow effect behind the edge
            <path
                class="custom-edge-shadow"
                d=move || path_data.get().0
                fill="none"
                stroke="rgba(102, 126, 234, 0.3)"
                stroke-width="8"
                stroke-linecap="round"
            />

            // Main edge path with custom styling
            <path
                class="custom-edge-path"
                d=move || path_data.get().0
                fill="none"
                stroke=gradient_id.clone()
                stroke-width="3"
                stroke-linecap="round"
                stroke-dasharray=stroke_dasharray.clone()
                attr:marker-end=marker_id.clone()
                style="pointer-events: stroke; cursor: pointer;"
            />

            // Edge label with custom styling
            {move || {
                let (_, label_x, label_y) = path_data.get();
                label.clone().map(|label_text| {
                    view! {
                        <g transform=format!("translate({}, {})", label_x, label_y)>
                            // Label background
                            <rect
                                x="-35"
                                y="-12"
                                width="70"
                                height="24"
                                rx="12"
                                fill="white"
                                stroke="rgba(102, 126, 234, 0.3)"
                                stroke-width="1"
                            />
                            // Label text
                            <text
                                class="custom-edge-label"
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="11"
                                font-weight="500"
                                fill="#667eea"
                            >
                                {label_text}
                            </text>
                        </g>
                    }
                })
            }}
        </g>
    }
}
