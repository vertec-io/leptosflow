//! Edge Toolbar Example
//!
//! Demonstrates how to add toolbars to edges:
//! - Toolbar appears on edge selection or hover
//! - Positioned at edge midpoint or label position
//! - Contains action buttons (delete, change type, change color)

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DraggableNode};

/// Toolbar position options for edges
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EdgeToolbarPosition {
    Top,
    Bottom,
    Center,
}

impl EdgeToolbarPosition {
    fn as_str(&self) -> &'static str {
        match self {
            EdgeToolbarPosition::Top => "top",
            EdgeToolbarPosition::Bottom => "bottom",
            EdgeToolbarPosition::Center => "center",
        }
    }

    fn y_offset(&self) -> f64 {
        match self {
            EdgeToolbarPosition::Top => -35.0,
            EdgeToolbarPosition::Bottom => 35.0,
            EdgeToolbarPosition::Center => 0.0,
        }
    }
}

/// Edge Toolbar example
#[component]
pub fn EdgeToolbarExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(80.0, 100.0))
            .with_data(json!({"label": "Start", "type": "input", "class": "light"}))
            .with_dimensions(120.0, 50.0),
        Node::new("2".to_string(), Position::new(280.0, 50.0))
            .with_data(json!({"label": "Process A", "type": "default", "class": "light"}))
            .with_dimensions(120.0, 50.0),
        Node::new("3".to_string(), Position::new(280.0, 180.0))
            .with_data(json!({"label": "Process B", "type": "default", "class": "light"}))
            .with_dimensions(120.0, 50.0),
        Node::new("4".to_string(), Position::new(480.0, 115.0))
            .with_data(json!({"label": "End", "type": "output", "class": "light"}))
            .with_dimensions(120.0, 50.0),
    ];

    // Create edges with different types and colors
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("Flow 1".to_string())
            .with_data(json!({"edgeType": "bezier", "color": "#6366f1"})),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Flow 2".to_string())
            .with_data(json!({"edgeType": "bezier", "color": "#10b981"})),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("Flow 3".to_string())
            .with_data(json!({"edgeType": "step", "color": "#f59e0b"})),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("Flow 4".to_string())
            .with_data(json!({"edgeType": "straight", "color": "#ef4444"})),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Selected edge for toolbar
    let selected_edge_id = RwSignal::new(Option::<String>::None);

    // Hovered edge (for hover-based toolbar)
    let hovered_edge_id = RwSignal::new(Option::<String>::None);

    // Toolbar position setting
    let toolbar_position = RwSignal::new(EdgeToolbarPosition::Top);

    // Action log for feedback
    let action_log = RwSignal::new(Vec::<String>::new());

    // Add action to log
    let add_action = move |action: String| {
        action_log.update(|log| {
            log.insert(0, action);
            if log.len() > 6 {
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

    // Click on background to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        selected_edge_id.set(None);
        store.clear_edge_selection();
        store.clear_node_selection();
    };

    // Available colors for edges
    let edge_colors = vec![
        ("#6366f1", "Indigo"),
        ("#10b981", "Green"),
        ("#f59e0b", "Amber"),
        ("#ef4444", "Red"),
        ("#8b5cf6", "Purple"),
        ("#06b6d4", "Cyan"),
    ];

    // Edge types available
    let edge_types = vec![
        ("bezier", "Bezier"),
        ("step", "Step"),
        ("straight", "Straight"),
    ];

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

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer with toolbar support
                    <EdgeToolbarRenderer
                        store=store
                        selected_edge_id=selected_edge_id
                        hovered_edge_id=hovered_edge_id
                        toolbar_position=toolbar_position
                        edge_colors=edge_colors.clone()
                        edge_types=edge_types.clone()
                        add_action=add_action
                    />

                    // Connection line
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

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 250px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Edge Toolbar"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click or hover over an edge to show its toolbar. The toolbar provides actions like delete, change type, and change color."
                        </p>

                        // Toolbar position selector
                        <div style="margin-bottom: 14px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Toolbar Position"</div>
                            <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 4px;">
                                {vec![EdgeToolbarPosition::Top, EdgeToolbarPosition::Center, EdgeToolbarPosition::Bottom]
                                    .into_iter()
                                    .map(|pos| {
                                        let pos_clone = pos;
                                        view! {
                                            <button
                                                style=move || format!(
                                                    "padding: 6px 8px; font-size: 10px; border: 1px solid {}; \
                                                     border-radius: 4px; background: {}; cursor: pointer; \
                                                     color: {}; text-transform: capitalize;",
                                                    if toolbar_position.get() == pos_clone { "#6366f1" } else { "#ddd" },
                                                    if toolbar_position.get() == pos_clone { "#eef2ff" } else { "#fff" },
                                                    if toolbar_position.get() == pos_clone { "#6366f1" } else { "#666" }
                                                )
                                                on:click=move |_| toolbar_position.set(pos_clone)
                                            >
                                                {pos.as_str()}
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </div>

                        // Selected edge indicator
                        {move || {
                            let sel_edge_id = selected_edge_id.get();
                            match sel_edge_id {
                                Some(ref id) => {
                                    let edges = store.get_edges();
                                    if let Some(edge) = edges.iter().find(|e| &e.id == id) {
                                        let label = edge.label.clone().unwrap_or_else(|| "Unnamed".to_string());
                                        let color = edge.data.get("color")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("#6366f1")
                                            .to_string();
                                        let edge_type = edge.data.get("edgeType")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("bezier")
                                            .to_string();
                                        view! {
                                            <div style=format!(
                                                "background: {}20; padding: 10px; border-radius: 6px; margin-bottom: 12px; \
                                                 border: 1px solid {};",
                                                color, color
                                            )>
                                                <div style=format!("font-weight: 600; font-size: 12px; color: {};", color)>
                                                    "Selected: " {label}
                                                </div>
                                                <div style="font-size: 10px; color: #666; margin-top: 4px;">
                                                    "Type: " {edge_type} " | " "ID: " {id.clone()}
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div style="background: #f5f5f5; padding: 10px; border-radius: 6px; margin-bottom: 12px; color: #999; font-size: 12px;">
                                                "No edge selected"
                                            </div>
                                        }.into_any()
                                    }
                                },
                                None => {
                                    view! {
                                        <div style="background: #f5f5f5; padding: 10px; border-radius: 6px; margin-bottom: 12px; color: #999; font-size: 12px;">
                                            "Click or hover an edge"
                                        </div>
                                    }.into_any()
                                }
                            }
                        }}

                        // Action log
                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Action Log"</div>
                            <div style="background: #f8f9fa; border-radius: 4px; padding: 8px; max-height: 120px; overflow-y: auto;">
                                {move || {
                                    let log = action_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic;">
                                                "No actions yet"
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.into_iter().map(|entry| {
                                            view! {
                                                <div style="font-size: 10px; color: #666; padding: 2px 0; border-bottom: 1px solid #eee;">
                                                    {entry}
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

/// Calculate edge path midpoint
fn calculate_midpoint(from: Position, to: Position) -> Position {
    Position::new(
        (from.x + to.x) / 2.0,
        (from.y + to.y) / 2.0,
    )
}

/// Generate a bezier curve path
fn generate_bezier_path(from: Position, to: Position) -> String {
    let dy = (to.y - from.y).abs();
    let offset = dy.max(50.0) * 0.5;
    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y,
        from.x, from.y + offset,
        to.x, to.y - offset,
        to.x, to.y
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

/// Edge renderer with toolbar support
#[component]
fn EdgeToolbarRenderer<F>(
    store: FlowStore,
    selected_edge_id: RwSignal<Option<String>>,
    hovered_edge_id: RwSignal<Option<String>>,
    toolbar_position: RwSignal<EdgeToolbarPosition>,
    edge_colors: Vec<(&'static str, &'static str)>,
    edge_types: Vec<(&'static str, &'static str)>,
    add_action: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    // Clone before the view! macro to avoid closure ownership issues
    let edge_colors_outer = edge_colors.clone();
    let edge_types_outer = edge_types.clone();
    let add_action_outer = add_action.clone();

    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                // Arrow markers for each color
                <marker id="edge-toolbar-arrow-indigo" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#6366f1" />
                </marker>
                <marker id="edge-toolbar-arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#10b981" />
                </marker>
                <marker id="edge-toolbar-arrow-amber" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f59e0b" />
                </marker>
                <marker id="edge-toolbar-arrow-red" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#ef4444" />
                </marker>
                <marker id="edge-toolbar-arrow-purple" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#8b5cf6" />
                </marker>
                <marker id="edge-toolbar-arrow-cyan" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#06b6d4" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                // Clone these inside the outer closure so they can be used in the inner loop
                let edge_colors_for_iter = edge_colors_outer.clone();
                let edge_types_for_iter = edge_types_outer.clone();
                let add_action_for_iter = add_action_outer.clone();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    let edge_id = edge.id.clone();
                    let edge_id_for_click = edge.id.clone();
                    let edge_id_for_hover = edge.id.clone();
                    let edge_id_for_leave = edge.id.clone();
                    let edge_id_for_style = edge.id.clone();
                    let edge_id_for_toolbar = edge.id.clone();
                    let edge_id_for_delete = edge.id.clone();

                    // Get edge properties
                    let color = edge.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1")
                        .to_string();
                    let edge_type = edge.data.get("edgeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("bezier")
                        .to_string();
                    let label = edge.label.clone();

                    // Calculate path
                    let sx = source_node.position.x + source_node.width.unwrap_or(120.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(120.0) / 2.0;
                    let ty = target_node.position.y;

                    let from_pos = Position::new(sx, sy);
                    let to_pos = Position::new(tx, ty);

                    let path = match edge_type.as_str() {
                        "step" => generate_step_path(from_pos, to_pos),
                        "straight" => generate_straight_path(from_pos, to_pos),
                        _ => generate_bezier_path(from_pos, to_pos),
                    };

                    let midpoint = calculate_midpoint(from_pos, to_pos);

                    // Arrow marker based on color
                    let marker_id = match color.as_str() {
                        "#10b981" => "url(#edge-toolbar-arrow-green)",
                        "#f59e0b" => "url(#edge-toolbar-arrow-amber)",
                        "#ef4444" => "url(#edge-toolbar-arrow-red)",
                        "#8b5cf6" => "url(#edge-toolbar-arrow-purple)",
                        "#06b6d4" => "url(#edge-toolbar-arrow-cyan)",
                        _ => "url(#edge-toolbar-arrow-indigo)",
                    };

                    let color_for_path = color.clone();
                    let color_for_highlight = color.clone();
                    let color_for_toolbar = color.clone();

                    let add_action_click = add_action_for_iter.clone();
                    let add_action_delete = add_action_for_iter.clone();
                    let add_action_type = add_action_for_iter.clone();
                    let add_action_color = add_action_for_iter.clone();

                    let edge_colors_clone = edge_colors_for_iter.clone();
                    let edge_types_clone = edge_types_for_iter.clone();

                    // Click handler
                    let on_click = move |ev: leptos::ev::MouseEvent| {
                        ev.stop_propagation();
                        selected_edge_id.set(Some(edge_id_for_click.clone()));
                        add_action_click(format!("Selected: {}", edge_id_for_click));
                    };

                    // Hover handlers
                    let on_mouseenter = move |_ev: leptos::ev::MouseEvent| {
                        hovered_edge_id.set(Some(edge_id_for_hover.clone()));
                    };

                    let on_mouseleave = move |_ev: leptos::ev::MouseEvent| {
                        if hovered_edge_id.get() == Some(edge_id_for_leave.clone()) {
                            hovered_edge_id.set(None);
                        }
                    };

                    // Delete handler
                    let on_delete = {
                        let edge_id_delete = edge_id_for_delete.clone();
                        move |ev: leptos::ev::MouseEvent| {
                            ev.stop_propagation();
                            store.remove_edge(&edge_id_delete);
                            selected_edge_id.set(None);
                            hovered_edge_id.set(None);
                            add_action_delete(format!("Deleted: {}", edge_id_delete));
                        }
                    };

                    // Type change handler creator
                    let create_type_handler = {
                        let edge_id_type = edge_id.clone();
                        let add_action_type = add_action_type.clone();
                        move |new_type: &'static str, type_name: &'static str| {
                            let edge_id_inner = edge_id_type.clone();
                            let add_action_inner = add_action_type.clone();
                            move |ev: leptos::ev::MouseEvent| {
                                ev.stop_propagation();
                                store.state.edges.update(|edges| {
                                    if let Some(edge) = edges.iter_mut().find(|e| e.id == edge_id_inner) {
                                        if let Some(data) = edge.data.as_object_mut() {
                                            data.insert("edgeType".to_string(), json!(new_type));
                                        }
                                    }
                                });
                                add_action_inner(format!("Type -> {}", type_name));
                            }
                        }
                    };

                    // Color change handler creator
                    let create_color_handler = {
                        let edge_id_color = edge_id.clone();
                        let add_action_color = add_action_color.clone();
                        move |new_color: &'static str, color_name: &'static str| {
                            let edge_id_inner = edge_id_color.clone();
                            let add_action_inner = add_action_color.clone();
                            move |ev: leptos::ev::MouseEvent| {
                                ev.stop_propagation();
                                store.state.edges.update(|edges| {
                                    if let Some(edge) = edges.iter_mut().find(|e| e.id == edge_id_inner) {
                                        if let Some(data) = edge.data.as_object_mut() {
                                            data.insert("color".to_string(), json!(new_color));
                                        }
                                    }
                                });
                                add_action_inner(format!("Color -> {}", color_name));
                            }
                        }
                    };

                    // Clone path for use in closures and outside closures
                    let path_for_hitbox = path.clone();
                    let path_for_highlight = path.clone();
                    let path_for_main = path.clone();

                    Some(view! {
                        <g class="xyflow__edge">
                            // Invisible hitbox for easier clicking
                            <path
                                d=path_for_hitbox
                                stroke="transparent"
                                stroke-width="20"
                                fill="none"
                                style="pointer-events: stroke; cursor: pointer;"
                                on:click=on_click.clone()
                                on:mouseenter=on_mouseenter.clone()
                                on:mouseleave=on_mouseleave.clone()
                            />

                            // Selection highlight
                            {move || {
                                let is_selected = selected_edge_id.get() == Some(edge_id_for_style.clone());
                                let is_hovered = hovered_edge_id.get() == Some(edge_id_for_style.clone());
                                if is_selected || is_hovered {
                                    Some(view! {
                                        <path
                                            d=path_for_highlight.clone()
                                            stroke=color_for_highlight.clone()
                                            stroke-width="6"
                                            stroke-opacity="0.3"
                                            fill="none"
                                            stroke-linecap="round"
                                        />
                                    })
                                } else {
                                    None
                                }
                            }}

                            // Main edge path
                            <path
                                d=path_for_main
                                stroke=color_for_path.clone()
                                stroke-width="2"
                                fill="none"
                                marker-end=marker_id
                            />

                            // Edge label
                            {label.as_ref().map(|label_text| {
                                let label_display = label_text.clone();
                                view! {
                                    <g transform=format!("translate({}, {})", midpoint.x, midpoint.y)>
                                        <rect
                                            x="-30"
                                            y="-10"
                                            width="60"
                                            height="20"
                                            rx="10"
                                            fill="white"
                                            stroke=color.clone()
                                            stroke-width="1"
                                        />
                                        <text
                                            text-anchor="middle"
                                            dominant-baseline="middle"
                                            font-size="10"
                                            font-weight="500"
                                            fill=color.clone()
                                            style="user-select: none;"
                                        >
                                            {label_display}
                                        </text>
                                    </g>
                                }
                            })}

                            // Toolbar (shows when selected or hovered)
                            {move || {
                                let is_selected = selected_edge_id.get() == Some(edge_id_for_toolbar.clone());
                                let is_hovered = hovered_edge_id.get() == Some(edge_id_for_toolbar.clone());

                                if is_selected || is_hovered {
                                    let pos = toolbar_position.get();
                                    let y_offset = pos.y_offset();

                                    let on_delete_clone = on_delete.clone();
                                    let edge_types_inner = edge_types_clone.clone();
                                    let edge_colors_inner = edge_colors_clone.clone();

                                    Some(view! {
                                        <g
                                            transform=format!("translate({}, {})", midpoint.x, midpoint.y + y_offset)
                                            style="pointer-events: all;"
                                        >
                                            // Toolbar background
                                            <rect
                                                x="-95"
                                                y="-16"
                                                width="190"
                                                height="32"
                                                rx="16"
                                                fill="white"
                                                stroke=color_for_toolbar.clone()
                                                stroke-width="1"
                                                filter="drop-shadow(0 2px 4px rgba(0,0,0,0.15))"
                                            />

                                            // Delete button
                                            <g
                                                transform="translate(-78, 0)"
                                                style="cursor: pointer;"
                                                on:click=on_delete_clone.clone()
                                            >
                                                <circle cx="0" cy="0" r="11" fill="#fee2e2" />
                                                <text
                                                    text-anchor="middle"
                                                    dominant-baseline="middle"
                                                    font-size="12"
                                                    fill="#dc2626"
                                                    style="user-select: none;"
                                                >
                                                    "\u{1F5D1}"
                                                </text>
                                            </g>

                                            // Separator
                                            <line x1="-60" y1="-10" x2="-60" y2="10" stroke="#e5e7eb" stroke-width="1" />

                                            // Edge type buttons
                                            {edge_types_inner.iter().enumerate().map(|(i, (type_id, type_name))| {
                                                let x_pos = -45.0 + (i as f64 * 22.0);
                                                let handler = create_type_handler(*type_id, *type_name);
                                                let icon = match *type_id {
                                                    "bezier" => "~",
                                                    "step" => "\u{2514}",
                                                    "straight" => "/",
                                                    _ => "?",
                                                };
                                                view! {
                                                    <g
                                                        transform=format!("translate({}, 0)", x_pos)
                                                        style="cursor: pointer;"
                                                        on:click=handler
                                                    >
                                                        <rect
                                                            x="-9"
                                                            y="-9"
                                                            width="18"
                                                            height="18"
                                                            rx="4"
                                                            fill="#f3f4f6"
                                                        />
                                                        <text
                                                            text-anchor="middle"
                                                            dominant-baseline="middle"
                                                            font-size="11"
                                                            font-weight="600"
                                                            fill="#374151"
                                                            style="user-select: none;"
                                                        >
                                                            {icon}
                                                        </text>
                                                    </g>
                                                }
                                            }).collect_view()}

                                            // Separator
                                            <line x1="25" y1="-10" x2="25" y2="10" stroke="#e5e7eb" stroke-width="1" />

                                            // Color picker dots
                                            {edge_colors_inner.iter().enumerate().map(|(i, (color_hex, color_name))| {
                                                let x_pos = 38.0 + (i as f64 * 16.0);
                                                let handler = create_color_handler(*color_hex, *color_name);
                                                view! {
                                                    <circle
                                                        cx=x_pos.to_string()
                                                        cy="0"
                                                        r="6"
                                                        fill=*color_hex
                                                        stroke="white"
                                                        stroke-width="1.5"
                                                        style="cursor: pointer;"
                                                        on:click=handler
                                                    />
                                                }
                                            }).collect_view()}
                                        </g>
                                    })
                                } else {
                                    None
                                }
                            }}
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
