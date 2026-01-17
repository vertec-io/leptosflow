//! Floating Edges Example
//!
//! Demonstrates edges that connect to nodes dynamically rather than to specific handles.
//! The edge endpoints are positioned on the nearest point of the node boundary.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

/// Floating edge strategy
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatingEdgeStrategy {
    /// Connect to nearest point on node border
    NearestBorder,
    /// Connect to center of nearest edge (top, right, bottom, left)
    NearestEdgeCenter,
    /// Smart routing - considers node positions relative to each other
    Smart,
}

impl FloatingEdgeStrategy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NearestBorder => "Nearest Border",
            Self::NearestEdgeCenter => "Nearest Edge Center",
            Self::Smart => "Smart Routing",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::NearestBorder => "Edges connect to the closest point on the node boundary",
            Self::NearestEdgeCenter => "Edges connect to the center of the nearest edge",
            Self::Smart => "Edges choose optimal connection points based on relative positions",
        }
    }
}

/// Floating edges example showing edges that connect to nodes dynamically
#[component]
pub fn FloatingEdgesExample() -> impl IntoView {
    // Current strategy
    let strategy = RwSignal::new(FloatingEdgeStrategy::Smart);

    // Create initial nodes - positioned to show different connection scenarios
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Node A", "color": "#6366f1"}))
            .with_dimensions(120.0, 50.0),
        Node::new("2".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({"label": "Node B", "color": "#ec4899"}))
            .with_dimensions(120.0, 50.0),
        Node::new("3".to_string(), Position::new(150.0, 180.0))
            .with_data(json!({"label": "Node C", "color": "#14b8a6"}))
            .with_dimensions(120.0, 50.0),
        Node::new("4".to_string(), Position::new(350.0, 180.0))
            .with_data(json!({"label": "Node D", "color": "#f59e0b"}))
            .with_dimensions(120.0, 50.0),
        Node::new("5".to_string(), Position::new(200.0, 320.0))
            .with_data(json!({"label": "Node E", "color": "#8b5cf6"}))
            .with_dimensions(120.0, 50.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("A → B".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("A → C".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("B → D".to_string()),
        Edge::new("e3-5".to_string(), "3".to_string(), "5".to_string())
            .with_label("C → E".to_string()),
        Edge::new("e4-5".to_string(), "4".to_string(), "5".to_string())
            .with_label("D → E".to_string()),
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
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Custom floating edge renderer
                    <FloatingEdgeRenderer store=store strategy=strategy />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <FloatingNode node=node.clone() store=store />
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 220px;">
                        <strong style="display: block; margin-bottom: 8px; font-size: 14px;">"Floating Edges"</strong>
                        <p style="margin: 0 0 16px 0; font-size: 12px; color: #666;">
                            "Edges connect dynamically to nodes. Drag nodes to see edges update."
                        </p>

                        // Strategy selector
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px;">"Edge Strategy:"</div>
                            <div style="display: flex; flex-direction: column; gap: 6px;">
                                {[
                                    FloatingEdgeStrategy::Smart,
                                    FloatingEdgeStrategy::NearestBorder,
                                    FloatingEdgeStrategy::NearestEdgeCenter,
                                ].into_iter().map(|s| {
                                    let s_for_click = s;
                                    let s_for_check = s;
                                    let label = s.label().to_string();
                                    view! {
                                        <label style="display: flex; align-items: center; gap: 8px; font-size: 11px; cursor: pointer;">
                                            <input
                                                type="radio"
                                                name="strategy"
                                                checked=move || strategy.get() == s_for_check
                                                on:change=move |_| strategy.set(s_for_click)
                                            />
                                            {label}
                                        </label>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Current strategy description
                        <div style="padding: 10px; background: #f0f9ff; border-radius: 6px; font-size: 11px; color: #0369a1;">
                            <strong style="display: block; margin-bottom: 4px;">"Current: "{move || strategy.get().label()}</strong>
                            {move || strategy.get().description()}
                        </div>

                        // How it works
                        <div style="margin-top: 16px; padding: 10px; background: #f8f9fa; border-radius: 6px; font-size: 10px; color: #666;">
                            <strong style="display: block; margin-bottom: 4px;">"How Floating Edges Work:"</strong>
                            <ul style="margin: 0; padding-left: 16px;">
                                <li>"No fixed handle positions"</li>
                                <li>"Endpoints calculated per-frame"</li>
                                <li>"Based on node positions & sizes"</li>
                                <li>"Drag nodes to see live updates"</li>
                            </ul>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Calculate the floating edge endpoints based on strategy
fn calculate_floating_endpoints(
    source: &Node,
    target: &Node,
    strategy: FloatingEdgeStrategy,
) -> (Position, Position) {
    let source_center = get_node_center(source);
    let target_center = get_node_center(target);

    match strategy {
        FloatingEdgeStrategy::NearestBorder => {
            // Find the nearest point on each node's border to the other node's center
            let source_point = get_nearest_border_point(source, target_center);
            let target_point = get_nearest_border_point(target, source_center);
            (source_point, target_point)
        }
        FloatingEdgeStrategy::NearestEdgeCenter => {
            // Find the nearest edge center (top, right, bottom, left)
            let source_point = get_nearest_edge_center(source, target_center);
            let target_point = get_nearest_edge_center(target, source_center);
            (source_point, target_point)
        }
        FloatingEdgeStrategy::Smart => {
            // Smart routing - choose based on relative positions
            smart_routing(source, target, source_center, target_center)
        }
    }
}

/// Get the center point of a node
fn get_node_center(node: &Node) -> Position {
    let width = node.width.unwrap_or(120.0);
    let height = node.height.unwrap_or(50.0);
    Position::new(
        node.position.x + width / 2.0,
        node.position.y + height / 2.0,
    )
}

/// Get the nearest point on the node's border to a target point
fn get_nearest_border_point(node: &Node, target: Position) -> Position {
    let width = node.width.unwrap_or(120.0);
    let height = node.height.unwrap_or(50.0);
    let center = get_node_center(node);

    // Direction vector from center to target
    let dx = target.x - center.x;
    let dy = target.y - center.y;

    // Normalize and scale to node bounds
    if dx.abs() < 0.001 && dy.abs() < 0.001 {
        // Target is at center, default to right edge
        return Position::new(node.position.x + width, center.y);
    }

    // Calculate intersection with node rectangle
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    // Scale factors for each axis
    let scale_x = if dx.abs() > 0.001 { half_width / dx.abs() } else { f64::INFINITY };
    let scale_y = if dy.abs() > 0.001 { half_height / dy.abs() } else { f64::INFINITY };

    // Use the smaller scale to find the intersection point
    let scale = scale_x.min(scale_y);

    Position::new(
        center.x + dx * scale,
        center.y + dy * scale,
    )
}

/// Get the center of the nearest edge (top, right, bottom, left)
fn get_nearest_edge_center(node: &Node, target: Position) -> Position {
    let width = node.width.unwrap_or(120.0);
    let height = node.height.unwrap_or(50.0);
    let x = node.position.x;
    let y = node.position.y;

    // Calculate edge centers
    let top = Position::new(x + width / 2.0, y);
    let right = Position::new(x + width, y + height / 2.0);
    let bottom = Position::new(x + width / 2.0, y + height);
    let left = Position::new(x, y + height / 2.0);

    // Find nearest edge center
    let edges = [top, right, bottom, left];
    edges
        .into_iter()
        .min_by(|a, b| {
            let dist_a = distance(*a, target);
            let dist_b = distance(*b, target);
            dist_a.partial_cmp(&dist_b).unwrap()
        })
        .unwrap_or(right)
}

/// Smart routing - chooses optimal connection points based on relative positions
fn smart_routing(
    source: &Node,
    target: &Node,
    source_center: Position,
    target_center: Position,
) -> (Position, Position) {
    let source_width = source.width.unwrap_or(120.0);
    let source_height = source.height.unwrap_or(50.0);
    let target_width = target.width.unwrap_or(120.0);
    let target_height = target.height.unwrap_or(50.0);

    let dx = target_center.x - source_center.x;
    let dy = target_center.y - source_center.y;

    // Determine primary direction
    let primarily_horizontal = dx.abs() > dy.abs();

    let (source_point, target_point) = if primarily_horizontal {
        if dx > 0.0 {
            // Target is to the right of source
            (
                Position::new(source.position.x + source_width, source_center.y),
                Position::new(target.position.x, target_center.y),
            )
        } else {
            // Target is to the left of source
            (
                Position::new(source.position.x, source_center.y),
                Position::new(target.position.x + target_width, target_center.y),
            )
        }
    } else if dy > 0.0 {
        // Target is below source
        (
            Position::new(source_center.x, source.position.y + source_height),
            Position::new(target_center.x, target.position.y),
        )
    } else {
        // Target is above source
        (
            Position::new(source_center.x, source.position.y),
            Position::new(target_center.x, target.position.y + target_height),
        )
    };

    (source_point, target_point)
}

/// Calculate distance between two positions
fn distance(a: Position, b: Position) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

/// Generate a smooth bezier path for floating edges
fn generate_floating_bezier_path(from: Position, to: Position) -> String {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    // Calculate control point offset based on distance and direction
    let offset = (dx.abs() + dy.abs()) * 0.3;

    // Determine control point direction based on whether primarily horizontal or vertical
    let (ctrl1, ctrl2) = if dx.abs() > dy.abs() {
        // Primarily horizontal - curve in x direction
        (
            Position::new(from.x + offset.min(dx.abs() * 0.5), from.y),
            Position::new(to.x - offset.min(dx.abs() * 0.5), to.y),
        )
    } else {
        // Primarily vertical - curve in y direction
        (
            Position::new(from.x, from.y + offset.min(dy.abs() * 0.5) * dy.signum()),
            Position::new(to.x, to.y - offset.min(dy.abs() * 0.5) * dy.signum()),
        )
    };

    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y, ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y
    )
}

/// Floating node component - simple node without fixed handles
#[component]
fn FloatingNode(node: Node, store: FlowStore) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6366f1")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            // Mark node as dragging
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

    let width = node.width.unwrap_or(120.0);
    let height = node.height.unwrap_or(50.0);

    view! {
        <div
            class="floating-node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                 background: {}; border-radius: 8px; cursor: grab; user-select: none; \
                 display: flex; align-items: center; justify-content: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.15); color: white; font-weight: 600; \
                 font-size: 13px; border: 2px solid rgba(255,255,255,0.3);",
                pos().x, pos().y, width, height, color
            )
            on:mousedown=on_mousedown
        >
            {label}
        </div>
    }
}

/// Floating edge renderer component
#[component]
fn FloatingEdgeRenderer(
    store: FlowStore,
    strategy: RwSignal<FloatingEdgeStrategy>,
) -> impl IntoView {
    let edges = move || store.get_edges();

    view! {
        <svg class="xyflow__edges floating-edges" style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            // SVG definitions
            <defs>
                // Gradient for edges
                <linearGradient id="floating-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#6366f1;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#a855f7;stop-opacity:1" />
                </linearGradient>

                // Arrow marker
                <marker id="floating-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#a855f7" />
                </marker>

                // Glow filter
                <filter id="floating-edge-glow" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur stdDeviation="2" result="blur" />
                    <feMerge>
                        <feMergeNode in="blur" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>
            </defs>

            <For
                each=edges
                key=|edge| edge.id.clone()
                children=move |edge| {
                    let edge_id = edge.id.clone();
                    let source_id = edge.source.clone();
                    let target_id = edge.target.clone();
                    let label = edge.label.clone();

                    view! {
                        <FloatingEdge
                            edge_id=edge_id
                            source_id=source_id
                            target_id=target_id
                            label=label
                            store=store
                            strategy=strategy
                        />
                    }
                }
            />
        </svg>
    }
}

/// Individual floating edge component
#[component]
fn FloatingEdge(
    edge_id: String,
    source_id: String,
    target_id: String,
    label: Option<String>,
    store: FlowStore,
    strategy: RwSignal<FloatingEdgeStrategy>,
) -> impl IntoView {
    // Create a reactive memo that recalculates when nodes or strategy change
    let path_data = Memo::new({
        let store = store.clone();
        let source_id = source_id.clone();
        let target_id = target_id.clone();
        move |_| {
            let nodes = store.get_nodes();
            let current_strategy = strategy.get();

            let source = nodes.iter().find(|n| n.id == source_id);
            let target = nodes.iter().find(|n| n.id == target_id);

            if let (Some(source), Some(target)) = (source, target) {
                let (source_point, target_point) =
                    calculate_floating_endpoints(source, target, current_strategy);

                let path = generate_floating_bezier_path(source_point, target_point);

                // Calculate label position at midpoint
                let label_x = (source_point.x + target_point.x) / 2.0;
                let label_y = (source_point.y + target_point.y) / 2.0;

                (path, label_x, label_y)
            } else {
                (String::new(), 0.0, 0.0)
            }
        }
    });

    view! {
        <g class="floating-edge-group" data-id=edge_id.clone()>
            // Glow/shadow path (behind main path)
            <path
                class="floating-edge-glow"
                d=move || path_data.get().0
                fill="none"
                stroke="url(#floating-edge-gradient)"
                stroke-width="4"
                stroke-opacity="0.3"
                stroke-linecap="round"
            />

            // Main edge path
            <path
                class="floating-edge-path"
                d=move || path_data.get().0
                fill="none"
                stroke="url(#floating-edge-gradient)"
                stroke-width="2"
                stroke-linecap="round"
                attr:marker-end="url(#floating-arrow)"
            />

            // Edge label
            {move || {
                let (_, label_x, label_y) = path_data.get();
                label.clone().map(|text| {
                    view! {
                        <g transform=format!("translate({}, {})", label_x, label_y)>
                            // Label background
                            <rect
                                x="-28"
                                y="-10"
                                width="56"
                                height="20"
                                rx="10"
                                fill="white"
                                stroke="#a855f7"
                                stroke-width="1"
                            />
                            // Label text
                            <text
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="10"
                                font-weight="500"
                                fill="#6366f1"
                                style="user-select: none;"
                            >
                                {text}
                            </text>
                        </g>
                    }
                })
            }}
        </g>
    }
}
