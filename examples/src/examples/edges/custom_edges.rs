//! Custom Edges Example
//!
//! Demonstrates how to create fully custom edge components with:
//! - Different path rendering (bezier, step, straight, curved)
//! - Interactive elements (delete button, label with click)
//! - Edge selection styling
//! - Animated edges

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DraggableNode};

/// Custom edges example showing fully custom edge components
#[component]
pub fn CustomEdgesExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Start", "type": "input", "class": "light"})),
        Node::new("2".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({"label": "Process A", "type": "default", "class": "light"})),
        Node::new("3".to_string(), Position::new(250.0, 200.0))
            .with_data(json!({"label": "Process B", "type": "default", "class": "light"})),
        Node::new("4".to_string(), Position::new(450.0, 125.0))
            .with_data(json!({"label": "End", "type": "output", "class": "light"})),
    ];

    // Create initial edges with different custom types
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("Bezier".to_string())
            .with_data(json!({"edgeType": "bezier", "color": "#667eea"})),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Animated".to_string())
            .with_data(json!({"edgeType": "animated", "color": "#f093fb"})),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("Step".to_string())
            .with_data(json!({"edgeType": "step", "color": "#4facfe"})),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("Straight".to_string())
            .with_data(json!({"edgeType": "straight", "color": "#43e97b"})),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Selected edge for info panel
    let selected_edge_id = RwSignal::new(Option::<String>::None);

    // Action log
    let action_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        action_log.update(|log| {
            log.insert(0, msg);
            if log.len() > 8 {
                log.pop();
            }
        });
    };

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

    // Handle background click to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        selected_edge_id.set(None);
        store.clear_node_selection();
        store.clear_edge_selection();
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
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
                    <CustomEdgesRenderer
                        store=store
                        selected_edge_id=selected_edge_id
                        add_log=add_log
                    />

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
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 200px;">
                        <strong style="display: block; margin-bottom: 8px;">"Custom Edges"</strong>
                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666;">
                            "Click edges to select, use buttons to interact"
                        </p>

                        // Edge type legend
                        <div style="margin-bottom: 12px; padding: 8px; background: #f8f9fa; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Edge Types:"</div>
                            <div style="font-size: 10px; display: flex; flex-direction: column; gap: 4px;">
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="width: 20px; height: 2px; background: #667eea;"></span>
                                    <span>"Bezier - smooth curve"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="width: 20px; height: 2px; background: #f093fb; background-image: repeating-linear-gradient(90deg, #f093fb 0, #f093fb 4px, transparent 4px, transparent 8px);"></span>
                                    <span>"Animated - moving dashes"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="width: 20px; height: 2px; background: #4facfe;"></span>
                                    <span>"Step - orthogonal path"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="width: 20px; height: 2px; background: #43e97b;"></span>
                                    <span>"Straight - direct line"</span>
                                </div>
                            </div>
                        </div>

                        // Selected edge info
                        {move || {
                            selected_edge_id.get().and_then(|edge_id| {
                                let edges = store.get_edges();
                                edges.iter().find(|e| e.id == edge_id).map(|edge| {
                                    let edge_type = edge.data.get("edgeType")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("bezier")
                                        .to_string();
                                    let color = edge.data.get("color")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("#667eea")
                                        .to_string();
                                    let edge_id_display = edge_id.clone();
                                    let color_display = color.clone();
                                    (edge_id_display, edge_type, color, color_display)
                                })
                            }).map(|(edge_id_display, edge_type, color, color_display)| {
                                view! {
                                    <div style="padding: 8px; background: #e3f2fd; border-radius: 4px; margin-bottom: 12px;">
                                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"Selected Edge:"</div>
                                        <div style="font-size: 10px;">
                                            <div>"ID: "{edge_id_display}</div>
                                            <div>"Type: "{edge_type}</div>
                                            <div style="display: flex; align-items: center; gap: 4px;">
                                                "Color: "
                                                <span style=format!("width: 12px; height: 12px; background: {}; border-radius: 2px;", color)></span>
                                                {color_display}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                        }}

                        // Action log
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"Action Log:"</div>
                        <div style="max-height: 100px; overflow-y: auto; font-size: 10px; color: #666;">
                            {move || {
                                let log = action_log.get();
                                if log.is_empty() {
                                    view! { <div style="color: #999;">"No actions yet"</div> }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        view! { <div style="padding: 2px 0; border-bottom: 1px solid #eee;">{entry.clone()}</div> }
                                    }).collect_view().into_any()
                                }
                            }}
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
            return handle.center_absolute(node_pos);
        }
    }

    // Fallback: use node edge positions based on handle type
    if is_source {
        Position::new(node_pos.x + node_width / 2.0, node_pos.y + node_height)
    } else {
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

/// Generate a step (orthogonal) path
fn generate_step_path(from: Position, to: Position) -> String {
    let mid_y = (from.y + to.y) / 2.0;
    format!(
        "M {} {} L {} {} L {} {} L {} {}",
        from.x, from.y,
        from.x, mid_y,
        to.x, mid_y,
        to.x, to.y
    )
}

/// Generate a straight line path
fn generate_straight_path(from: Position, to: Position) -> String {
    format!("M {} {} L {} {}", from.x, from.y, to.x, to.y)
}

/// Calculate the label position (midpoint) for an edge
fn calculate_label_position(from: Position, to: Position) -> Position {
    Position::new(
        (from.x + to.x) / 2.0,
        (from.y + to.y) / 2.0,
    )
}

/// Custom edges renderer component
#[component]
fn CustomEdgesRenderer<F>(
    store: FlowStore,
    selected_edge_id: RwSignal<Option<String>>,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    let edges = move || store.get_edges();

    view! {
        <svg class="xyflow__edges leptos-flow__edges" style="position: absolute; width: 100%; height: 100%; pointer-events: none;">
            // SVG definitions for gradients, markers, and animations
            <defs>
                // Gradients for each edge color
                <linearGradient id="custom-gradient-bezier" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <linearGradient id="custom-gradient-animated" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#f093fb;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#f5576c;stop-opacity:1" />
                </linearGradient>
                <linearGradient id="custom-gradient-step" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#4facfe;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#00f2fe;stop-opacity:1" />
                </linearGradient>
                <linearGradient id="custom-gradient-straight" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#43e97b;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#38f9d7;stop-opacity:1" />
                </linearGradient>

                // Arrow markers
                <marker id="custom-arrow-bezier" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#764ba2" />
                </marker>
                <marker id="custom-arrow-animated" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f5576c" />
                </marker>
                <marker id="custom-arrow-step" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#00f2fe" />
                </marker>
                <marker id="custom-arrow-straight" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#38f9d7" />
                </marker>
            </defs>

            // CSS for animation
            <style>
                {"
                @keyframes dash-animation {
                    from { stroke-dashoffset: 24; }
                    to { stroke-dashoffset: 0; }
                }
                .animated-edge-path {
                    animation: dash-animation 1s linear infinite;
                }
                "}
            </style>

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
                    let edge_type = edge.data.get("edgeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("bezier")
                        .to_string();
                    let color = edge.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#667eea")
                        .to_string();

                    let add_log = add_log.clone();

                    view! {
                        <CustomEdgeComponent
                            edge_id=edge_id
                            source_id=source_id
                            target_id=target_id
                            source_handle=source_handle
                            target_handle=target_handle
                            label=label
                            edge_type=edge_type
                            color=color
                            store=store
                            selected_edge_id=selected_edge_id
                            add_log=add_log
                        />
                    }
                }
            />
        </svg>
    }
}

/// Custom edge component with interactive elements
#[component]
fn CustomEdgeComponent<F>(
    edge_id: String,
    source_id: String,
    target_id: String,
    source_handle: Option<String>,
    target_handle: Option<String>,
    label: Option<String>,
    edge_type: String,
    color: String,
    store: FlowStore,
    selected_edge_id: RwSignal<Option<String>>,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    // Create a reactive memo that recalculates when nodes change
    let edge_type_for_path = edge_type.clone();
    let path_data = Memo::new({
        let store = store.clone();
        let source_id = source_id.clone();
        let target_id = target_id.clone();
        let source_handle = source_handle.clone();
        let target_handle = target_handle.clone();
        move |_| {
            let nodes = store.get_nodes();
            let source = nodes.iter().find(|n| n.id == source_id);
            let target = nodes.iter().find(|n| n.id == target_id);

            if let (Some(source), Some(target)) = (source, target) {
                let source_pos = get_handle_position(source, &source_handle, true);
                let target_pos = get_handle_position(target, &target_handle, false);

                let path = match edge_type_for_path.as_str() {
                    "step" => generate_step_path(source_pos, target_pos),
                    "straight" => generate_straight_path(source_pos, target_pos),
                    _ => generate_bezier_path(source_pos, target_pos),
                };
                let label_pos = calculate_label_position(source_pos, target_pos);

                (path, label_pos.x, label_pos.y)
            } else {
                (String::new(), 0.0, 0.0)
            }
        }
    });

    // Get gradient and marker IDs based on edge type
    let (gradient_id, marker_id) = match edge_type.as_str() {
        "animated" => ("url(#custom-gradient-animated)", "url(#custom-arrow-animated)"),
        "step" => ("url(#custom-gradient-step)", "url(#custom-arrow-step)"),
        "straight" => ("url(#custom-gradient-straight)", "url(#custom-arrow-straight)"),
        _ => ("url(#custom-gradient-bezier)", "url(#custom-arrow-bezier)"),
    };

    // Determine if this is an animated edge
    let is_animated = edge_type == "animated";

    // Click handler for selection
    let edge_id_click = edge_id.clone();
    let add_log_click = add_log.clone();
    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        selected_edge_id.set(Some(edge_id_click.clone()));
        add_log_click(format!("Selected edge: {}", edge_id_click));
    };

    // Delete button handler
    let edge_id_delete = edge_id.clone();
    let edge_id_delete_log = edge_id.clone();
    let store_delete = store.clone();
    let add_log_delete = add_log.clone();
    let on_delete = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        // Remove the edge
        store_delete.remove_edge(&edge_id_delete);
        selected_edge_id.set(None);
        add_log_delete(format!("Deleted edge: {}", edge_id_delete_log));
    };

    // Label click handler
    let edge_id_label = edge_id.clone();
    let label_text = label.clone().unwrap_or_default();
    let add_log_label = add_log.clone();
    let on_label_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        add_log_label(format!("Clicked label '{}' on edge {}", label_text, edge_id_label));
    };

    // Edge group class
    let edge_id_for_class = edge_id.clone();
    let edge_class = move || {
        let mut classes = vec!["custom-edge-group"];
        if selected_edge_id.get() == Some(edge_id_for_class.clone()) {
            classes.push("selected");
        }
        classes.join(" ")
    };

    // For selection highlight
    let edge_id_for_highlight = edge_id.clone();
    let color_for_highlight = color.clone();

    // For delete button
    let edge_id_for_delete_btn = edge_id.clone();

    view! {
        <g class=edge_class data-id=edge_id.clone()>
            // Invisible wider path for easier clicking
            <path
                class="custom-edge-hitbox"
                d=move || path_data.get().0
                fill="none"
                stroke="transparent"
                stroke-width="20"
                style="pointer-events: stroke; cursor: pointer;"
                on:click=on_click.clone()
            />

            // Selection highlight (wider stroke behind main path)
            {move || {
                if selected_edge_id.get() == Some(edge_id_for_highlight.clone()) {
                    Some(view! {
                        <path
                            class="custom-edge-selection"
                            d=move || path_data.get().0
                            fill="none"
                            stroke=color_for_highlight.clone()
                            stroke-width="6"
                            stroke-opacity="0.3"
                            stroke-linecap="round"
                        />
                    })
                } else {
                    None
                }
            }}

            // Main edge path
            <path
                class=if is_animated { "custom-edge-path animated-edge-path" } else { "custom-edge-path" }
                d=move || path_data.get().0
                fill="none"
                stroke=gradient_id.to_string()
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-dasharray=if is_animated { "8,4" } else { "" }
                attr:marker-end=marker_id.to_string()
                style="pointer-events: none;"
            />

            // Edge label with interactive elements
            <CustomEdgeLabelGroup
                path_data=path_data
                label=label
                color=color
                edge_id=edge_id_for_delete_btn
                selected_edge_id=selected_edge_id
                on_label_click=on_label_click
                on_delete=on_delete
            />
        </g>
    }
}

/// Component for edge label and delete button
#[component]
fn CustomEdgeLabelGroup<F1, F2>(
    path_data: Memo<(String, f64, f64)>,
    label: Option<String>,
    color: String,
    edge_id: String,
    selected_edge_id: RwSignal<Option<String>>,
    on_label_click: F1,
    on_delete: F2,
) -> impl IntoView
where
    F1: Fn(leptos::ev::MouseEvent) + Clone + Send + Sync + 'static,
    F2: Fn(leptos::ev::MouseEvent) + Clone + Send + Sync + 'static,
{
    let edge_id_for_selected = edge_id.clone();

    view! {
        {move || {
            let (_, label_x, label_y) = path_data.get();
            let is_selected = selected_edge_id.get() == Some(edge_id_for_selected.clone());
            let color_clone = color.clone();
            let color_clone2 = color.clone();
            let on_label_click_clone = on_label_click.clone();
            let on_label_click_clone2 = on_label_click.clone();
            let on_delete_clone = on_delete.clone();
            let on_delete_clone2 = on_delete.clone();

            label.clone().map(move |label_text| {
                let label_text_display = label_text.clone();
                view! {
                    <g transform=format!("translate({}, {})", label_x, label_y) style="pointer-events: all;">
                        // Label background
                        <rect
                            x="-40"
                            y="-14"
                            width="80"
                            height="28"
                            rx="14"
                            fill="white"
                            stroke=color_clone.clone()
                            stroke-width="1.5"
                            style="cursor: pointer;"
                            on:click=on_label_click_clone.clone()
                        />
                        // Label text
                        <text
                            class="custom-edge-label"
                            text-anchor="middle"
                            dominant-baseline="middle"
                            font-size="11"
                            font-weight="600"
                            fill=color_clone2.clone()
                            style="cursor: pointer; user-select: none;"
                            on:click=on_label_click_clone2.clone()
                        >
                            {label_text_display}
                        </text>

                        // Delete button (shows on selection)
                        {if is_selected {
                            Some(view! {
                                <g transform="translate(50, 0)">
                                    <circle
                                        cx="0"
                                        cy="0"
                                        r="10"
                                        fill="#ff6b6b"
                                        style="cursor: pointer;"
                                        on:click=on_delete_clone.clone()
                                    />
                                    <text
                                        text-anchor="middle"
                                        dominant-baseline="middle"
                                        font-size="12"
                                        font-weight="bold"
                                        fill="white"
                                        style="cursor: pointer; user-select: none;"
                                        on:click=on_delete_clone2.clone()
                                    >
                                        "\u{00D7}"
                                    </text>
                                </g>
                            })
                        } else {
                            None
                        }}
                    </g>
                }
            })
        }}
    }
}
