//! Custom MiniMap Node Example
//!
//! Demonstrates how to customize minimap node appearance:
//! - Different colors based on node type
//! - Different shapes (rectangles, circles, diamonds)
//! - Custom minimap node component rendering
//! - Size and style based on node data

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static CUSTOM_MINIMAP_DRAG_STATE: OnceLock<RwSignal<Option<CustomMiniMapDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct CustomMiniMapDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_drag_signal() -> RwSignal<Option<CustomMiniMapDragState>> {
    *CUSTOM_MINIMAP_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Node Shape Enum
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeShape {
    Rectangle,
    RoundedRect,
    Circle,
    Diamond,
}

impl NodeShape {
    fn label(&self) -> &'static str {
        match self {
            Self::Rectangle => "Rectangle",
            Self::RoundedRect => "Rounded",
            Self::Circle => "Circle",
            Self::Diamond => "Diamond",
        }
    }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Get color for node type
fn get_node_color(node_type: &str) -> &'static str {
    match node_type {
        "input" => "#6ede87",  // Green
        "output" => "#ff6b6b", // Red
        "process" => "#6865A5", // Purple
        "storage" => "#f0ad4e", // Orange
        _ => "#5bc0de",        // Blue (default)
    }
}

/// Get shape for node type
fn get_node_shape(node_type: &str) -> NodeShape {
    match node_type {
        "input" => NodeShape::RoundedRect,
        "output" => NodeShape::RoundedRect,
        "process" => NodeShape::Rectangle,
        "storage" => NodeShape::Diamond,
        _ => NodeShape::Circle,
    }
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Custom MiniMap Node Example - Different colors/shapes based on node type
#[component]
pub fn CustomMiniMapNodeExample() -> impl IntoView {
    // Create initial nodes with different types
    let initial_nodes = vec![
        Node::new("source".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Data Source", "type": "input"})),
        Node::new("process1".to_string(), Position::new(300.0, 50.0))
            .with_data(json!({"label": "Process A", "type": "process"})),
        Node::new("process2".to_string(), Position::new(300.0, 180.0))
            .with_data(json!({"label": "Process B", "type": "process"})),
        Node::new("storage".to_string(), Position::new(500.0, 120.0))
            .with_data(json!({"label": "Database", "type": "storage"})),
        Node::new("default1".to_string(), Position::new(500.0, 250.0))
            .with_data(json!({"label": "Transform", "type": "default"})),
        Node::new("output".to_string(), Position::new(700.0, 150.0))
            .with_data(json!({"label": "Export", "type": "output"})),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e-sp1".to_string(), "source".to_string(), "process1".to_string()),
        Edge::new("e-sp2".to_string(), "source".to_string(), "process2".to_string()),
        Edge::new("e-p1s".to_string(), "process1".to_string(), "storage".to_string()),
        Edge::new("e-p2d".to_string(), "process2".to_string(), "default1".to_string()),
        Edge::new("e-so".to_string(), "storage".to_string(), "output".to_string()),
        Edge::new("e-do".to_string(), "default1".to_string(), "output".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Drag signal
    let drag_signal = get_drag_signal();

    // Mouse move handler
    let on_canvas_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let dx = ev.client_x() as f64 - drag_state.start_mouse.0;
            let dy = ev.client_y() as f64 - drag_state.start_mouse.1;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(drag_state.start_pos.0 + dx, drag_state.start_pos.1 + dy);
            });
        }
    };

    // Mouse up handler
    let on_canvas_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            store.update_node(&drag_state.node_id, |n| {
                n.dragging = false;
            });
            drag_signal.set(None);
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow custom-minimap-example"
                 style="width: 100%; height: 100%; position: relative; background: #fafafa;"
                 on:mousemove=on_canvas_mousemove
                 on:mouseup=on_canvas_mouseup
                 on:mouseleave=move |_| {
                     if let Some(ds) = drag_signal.get() {
                         store.update_node(&ds.node_id, |n| n.dragging = false);
                         drag_signal.set(None);
                     }
                 }
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Render edges
                    <CustomMiniMapEdgeRenderer store=store />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <CustomMiniMapMainNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // Custom MiniMap with custom node rendering
                <CustomMiniMap store=store />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; max-width: 300px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333; display: flex; align-items: center; gap: 8px;">
                            <span style="display: inline-block; width: 8px; height: 8px; background: #667eea; border-radius: 50%;"></span>
                            "Custom MiniMap Nodes"
                        </h3>

                        // Node Types Legend
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Node Types & Shapes"</div>
                            <div style="display: flex; flex-direction: column; gap: 6px;">
                                {[
                                    ("input", "Input/Source", NodeShape::RoundedRect),
                                    ("process", "Process", NodeShape::Rectangle),
                                    ("storage", "Storage", NodeShape::Diamond),
                                    ("default", "Default", NodeShape::Circle),
                                    ("output", "Output", NodeShape::RoundedRect),
                                ].into_iter().map(|(node_type, label, shape)| {
                                    let color = get_node_color(node_type);
                                    view! {
                                        <div style="display: flex; align-items: center; gap: 10px; padding: 6px 10px; background: #f9f9f9; border-radius: 4px;">
                                            <div style=format!(
                                                "width: 20px; height: 16px; background: {}; border-radius: {};",
                                                color,
                                                match shape {
                                                    NodeShape::Circle => "50%",
                                                    NodeShape::RoundedRect => "4px",
                                                    _ => "2px",
                                                }
                                            )></div>
                                            <div style="flex: 1;">
                                                <div style="font-size: 12px; font-weight: 500; color: #333;">{label}</div>
                                                <div style="font-size: 10px; color: #888;">{shape.label()}</div>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // MiniMap description
                        <div style="padding: 12px; background: #f0f0f0; border-radius: 6px; margin-bottom: 12px;">
                            <div style="font-size: 11px; color: #666; line-height: 1.5;">
                                <strong>"Custom MiniMap:"</strong>
                                <br />
                                "Check the bottom-right corner for the minimap. Node shapes and colors match their types."
                            </div>
                        </div>

                        // Node count
                        <div style="display: flex; gap: 8px; font-size: 12px;">
                            <div style="padding: 8px 12px; background: #f5f5f5; border-radius: 4px; flex: 1; text-align: center;">
                                <div style="color: #888; font-size: 10px;">"Nodes"</div>
                                <div style="font-weight: 600; color: #333;">{move || store.get_nodes().len()}</div>
                            </div>
                            <div style="padding: 8px 12px; background: #f5f5f5; border-radius: 4px; flex: 1; text-align: center;">
                                <div style="color: #888; font-size: 10px;">"Edges"</div>
                                <div style="font-weight: 600; color: #333;">{move || store.get_edges().len()}</div>
                            </div>
                        </div>
                    </div>
                </Panel>

                // Instructions badge
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 10px 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <div style="color: white; font-size: 11px; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 4px;">"Custom MiniMap"</div>
                            <div style="opacity: 0.9;">"• Drag nodes to see updates"</div>
                            <div style="opacity: 0.9;">"• Colors match node types"</div>
                            <div style="opacity: 0.9;">"• Shapes indicate category"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Custom MiniMap Component
// ============================================================================

#[component]
fn CustomMiniMap(store: FlowStore) -> impl IntoView {
    // Minimap dimensions
    let width: u32 = 220;
    let height: u32 = 150;

    // Calculate bounds of all nodes with padding
    let bounds = move || {
        let nodes = store.get_nodes();
        if nodes.is_empty() {
            return (0.0, 0.0, 500.0, 500.0);
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for node in &nodes {
            let x = node.position.x;
            let y = node.position.y;
            let node_width = node.width.unwrap_or(150.0);
            let node_height = node.height.unwrap_or(60.0);

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + node_width);
            max_y = max_y.max(y + node_height);
        }

        // Add padding
        let padding = 80.0;
        (min_x - padding, min_y - padding, max_x + padding, max_y + padding)
    };

    view! {
        <div
            class="xyflow__minimap xyflow__panel bottom right"
            style=format!(
                "width: {}px; height: {}px; background: white; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); overflow: hidden; padding: 8px;",
                width + 16, height + 16
            )
        >
            // MiniMap title
            <div style="font-size: 10px; font-weight: 600; color: #888; margin-bottom: 6px; text-transform: uppercase; letter-spacing: 0.5px;">
                "MiniMap"
            </div>

            <svg
                width=width
                height=height
                style="background: #fafafa; border-radius: 4px;"
                viewBox=move || {
                    let (min_x, min_y, max_x, max_y) = bounds();
                    format!("{} {} {} {}", min_x, min_y, max_x - min_x, max_y - min_y)
                }
            >
                // Render custom minimap nodes
                {move || {
                    store.get_nodes().into_iter().map(|node| {
                        let x = node.position.x;
                        let y = node.position.y;
                        let w = node.width.unwrap_or(150.0);
                        let h = node.height.unwrap_or(60.0);

                        // Get node type from data
                        let node_type = node.data.get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("default");

                        let color = get_node_color(node_type);
                        let shape = get_node_shape(node_type);

                        // Calculate center for circle/diamond
                        let cx = x + w / 2.0;
                        let cy = y + h / 2.0;
                        let r = w.min(h) / 2.0;

                        match shape {
                            NodeShape::Rectangle => {
                                view! {
                                    <rect
                                        x=x
                                        y=y
                                        width=w
                                        height=h
                                        fill=color
                                        stroke="white"
                                        stroke-width="2"
                                    />
                                }.into_any()
                            }
                            NodeShape::RoundedRect => {
                                view! {
                                    <rect
                                        x=x
                                        y=y
                                        width=w
                                        height=h
                                        rx="8"
                                        ry="8"
                                        fill=color
                                        stroke="white"
                                        stroke-width="2"
                                    />
                                }.into_any()
                            }
                            NodeShape::Circle => {
                                view! {
                                    <ellipse
                                        cx=cx
                                        cy=cy
                                        rx=w / 2.0
                                        ry=h / 2.0
                                        fill=color
                                        stroke="white"
                                        stroke-width="2"
                                    />
                                }.into_any()
                            }
                            NodeShape::Diamond => {
                                // Diamond path: top -> right -> bottom -> left
                                let path = format!(
                                    "M {} {} L {} {} L {} {} L {} {} Z",
                                    cx, y,             // top
                                    x + w, cy,         // right
                                    cx, y + h,         // bottom
                                    x, cy              // left
                                );
                                view! {
                                    <path
                                        d=path
                                        fill=color
                                        stroke="white"
                                        stroke-width="2"
                                    />
                                }.into_any()
                            }
                        }
                    }).collect_view()
                }}

                // Render viewport indicator
                {move || {
                    let viewport = store.get_viewport();
                    let (min_x, min_y, _, _) = bounds();

                    // Calculate visible area in flow coordinates
                    // Assuming a viewport size of 800x600
                    let container_width = 800.0;
                    let container_height = 600.0;

                    let visible_x = -viewport.x / viewport.zoom + min_x;
                    let visible_y = -viewport.y / viewport.zoom + min_y;
                    let visible_width = container_width / viewport.zoom;
                    let visible_height = container_height / viewport.zoom;

                    view! {
                        <rect
                            x=visible_x
                            y=visible_y
                            width=visible_width
                            height=visible_height
                            fill="rgba(102, 126, 234, 0.1)"
                            stroke="#667eea"
                            stroke-width="3"
                            rx="4"
                        />
                    }
                }}
            </svg>
        </div>
    }
}

// ============================================================================
// Main Flow Node Component
// ============================================================================

#[component]
fn CustomMiniMapMainNode(node: Node, store: FlowStore) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let color = get_node_color(&node_type);
    let drag_signal = get_drag_signal();

    // Mouse down handler
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(CustomMiniMapDragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (n.position.x, n.position.y),
            }));

            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
        }
    };

    // Get reactive position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    // Determine if input or output
    let has_source = node_type != "output";
    let has_target = node_type != "input";

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
                class="xyflow__node-default light"
                style=format!(
                    "background: {}; border: 2px solid {}; border-radius: 8px; padding: 12px 18px; min-width: 100px; text-align: center; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",
                    color, color
                )
            >
                // Target handle
                {has_target.then(|| {
                    let node_id = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id
                            r#type=HandleType::Target
                            position=HandlePosition::Top
                            connection_mode=ConnectionMode::Strict
                        />
                    }
                })}

                <span style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 13px;">
                    {label}
                </span>

                // Source handle
                {has_source.then(|| {
                    let node_id = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id
                            r#type=HandleType::Source
                            position=HandlePosition::Bottom
                            connection_mode=ConnectionMode::Strict
                        />
                    }
                })}
            </div>
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

#[component]
fn CustomMiniMapEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="custom-minimap-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="custom-minimap-arrow"
                    markerWidth="12"
                    markerHeight="12"
                    refX="10"
                    refY="6"
                    orient="auto"
                    markerUnits="userSpaceOnUse"
                >
                    <path d="M2,2 L10,6 L2,10 L4,6 Z" fill="#764ba2" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.iter().map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        let source_width = source.width.unwrap_or(150.0);
                        let source_height = source.height.unwrap_or(60.0);
                        let target_width = target.width.unwrap_or(150.0);

                        // Calculate connection points
                        let source_x = source.position.x + source_width / 2.0;
                        let source_y = source.position.y + source_height;
                        let target_x = target.position.x + target_width / 2.0;
                        let target_y = target.position.y;

                        // Generate bezier path
                        let ctrl_offset = (target_y - source_y).abs() * 0.4;
                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            source_x, source_y,
                            source_x, source_y + ctrl_offset,
                            target_x, target_y - ctrl_offset,
                            target_x, target_y
                        );

                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke="url(#custom-minimap-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#custom-minimap-arrow)"
                            />
                        }.into_any()
                    } else {
                        view! { <g></g> }.into_any()
                    }
                }).collect_view()
            }}
        </svg>
    }
}
