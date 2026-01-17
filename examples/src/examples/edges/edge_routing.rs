//! Edge Routing Example
//!
//! Demonstrates advanced edge routing strategies including obstacle avoidance
//! and orthogonal (right-angle) routing.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

// ============================================================================
// Edge Routing Strategy
// ============================================================================

/// Edge routing strategy
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeRoutingStrategy {
    /// Direct bezier curves (no routing)
    Direct,
    /// Orthogonal routing with right angles
    Orthogonal,
    /// Orthogonal routing that avoids obstacles (other nodes)
    OrthogonalAvoidance,
}

impl EdgeRoutingStrategy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Direct => "Direct (Bezier)",
            Self::Orthogonal => "Orthogonal",
            Self::OrthogonalAvoidance => "Orthogonal + Avoidance",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Direct => "Smooth bezier curves directly between nodes",
            Self::Orthogonal => "Right-angle paths for clean diagrams",
            Self::OrthogonalAvoidance => "Right-angle paths that route around other nodes",
        }
    }
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Edge routing example showing different routing strategies
#[component]
pub fn EdgeRoutingExample() -> impl IntoView {
    // Current strategy
    let strategy = RwSignal::new(EdgeRoutingStrategy::OrthogonalAvoidance);

    // Show routing debug info
    let show_routing_debug = RwSignal::new(false);

    // Create initial nodes - positioned to demonstrate obstacle avoidance
    let initial_nodes = vec![
        // Source node (left)
        Node::new("source".to_string(), Position::new(50.0, 150.0))
            .with_data(json!({"label": "Source", "nodeType": "source", "color": "#10b981"}))
            .with_dimensions(100.0, 50.0),
        // Target node (right)
        Node::new("target".to_string(), Position::new(450.0, 150.0))
            .with_data(json!({"label": "Target", "nodeType": "target", "color": "#3b82f6"}))
            .with_dimensions(100.0, 50.0),
        // Obstacle nodes (in the middle)
        Node::new("obstacle1".to_string(), Position::new(220.0, 80.0))
            .with_data(json!({"label": "Obstacle 1", "nodeType": "obstacle", "color": "#f59e0b"}))
            .with_dimensions(120.0, 60.0),
        Node::new("obstacle2".to_string(), Position::new(220.0, 200.0))
            .with_data(json!({"label": "Obstacle 2", "nodeType": "obstacle", "color": "#f59e0b"}))
            .with_dimensions(120.0, 60.0),
        // Additional nodes for more complex routing
        Node::new("source2".to_string(), Position::new(50.0, 350.0))
            .with_data(json!({"label": "Source 2", "nodeType": "source", "color": "#10b981"}))
            .with_dimensions(100.0, 50.0),
        Node::new("target2".to_string(), Position::new(450.0, 350.0))
            .with_data(json!({"label": "Target 2", "nodeType": "target", "color": "#3b82f6"}))
            .with_dimensions(100.0, 50.0),
        Node::new("obstacle3".to_string(), Position::new(250.0, 330.0))
            .with_data(json!({"label": "Obstacle 3", "nodeType": "obstacle", "color": "#f59e0b"}))
            .with_dimensions(100.0, 70.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1".to_string(), "source".to_string(), "target".to_string())
            .with_label("Route 1".to_string()),
        Edge::new("e2".to_string(), "source2".to_string(), "target2".to_string())
            .with_label("Route 2".to_string()),
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
                    // Custom edge router
                    <EdgeRoutingRenderer store=store strategy=strategy show_debug=show_routing_debug />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <RoutingNode node=node.clone() store=store />
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 240px;">
                        <strong style="display: block; margin-bottom: 8px; font-size: 14px;">"Edge Routing"</strong>
                        <p style="margin: 0 0 16px 0; font-size: 12px; color: #666;">
                            "Compare different edge routing strategies. Drag nodes to see routing update."
                        </p>

                        // Strategy selector
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px;">"Routing Strategy:"</div>
                            <div style="display: flex; flex-direction: column; gap: 6px;">
                                {[
                                    EdgeRoutingStrategy::Direct,
                                    EdgeRoutingStrategy::Orthogonal,
                                    EdgeRoutingStrategy::OrthogonalAvoidance,
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

                        // Debug toggle
                        <div style="margin-top: 12px;">
                            <label style="display: flex; align-items: center; gap: 8px; font-size: 11px; cursor: pointer;">
                                <input
                                    type="checkbox"
                                    checked=move || show_routing_debug.get()
                                    on:change=move |_| show_routing_debug.update(|v| *v = !*v)
                                />
                                "Show routing debug"
                            </label>
                        </div>

                        // Legend
                        <div style="margin-top: 16px; padding: 10px; background: #f8f9fa; border-radius: 6px; font-size: 10px;">
                            <strong style="display: block; margin-bottom: 8px;">"Node Types:"</strong>
                            <div style="display: flex; flex-direction: column; gap: 4px;">
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 12px; height: 12px; background: #10b981; border-radius: 2px;"></div>
                                    <span>"Source (edges start)"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 12px; height: 12px; background: #3b82f6; border-radius: 2px;"></div>
                                    <span>"Target (edges end)"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 8px;">
                                    <div style="width: 12px; height: 12px; background: #f59e0b; border-radius: 2px;"></div>
                                    <span>"Obstacle (routes around)"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Routing Node Component
// ============================================================================

/// Node component for the edge routing example
#[component]
fn RoutingNode(node: Node, store: FlowStore) -> impl IntoView {
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
    let node_type = node.data.get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
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

    let width = node.width.unwrap_or(100.0);
    let height = node.height.unwrap_or(50.0);

    // Determine if this node has handles
    let has_source_handle = node_type == "source";
    let has_target_handle = node_type == "target";

    let color_for_style = color.clone();

    view! {
        <div
            class="routing-node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                 background: {}; border-radius: 8px; cursor: grab; user-select: none; \
                 display: flex; align-items: center; justify-content: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.15); color: white; font-weight: 600; \
                 font-size: 12px; border: 2px solid rgba(255,255,255,0.3);",
                pos().x, pos().y, width, height, color_for_style
            )
            on:mousedown=on_mousedown
        >
            // Target handle (left side)
            {has_target_handle.then(|| {
                view! {
                    <div style="position: absolute; left: -6px; top: 50%; transform: translateY(-50%); \
                                width: 12px; height: 12px; background: white; border: 2px solid #3b82f6; \
                                border-radius: 50%;">
                    </div>
                }
            })}

            {label}

            // Source handle (right side)
            {has_source_handle.then(|| {
                view! {
                    <div style="position: absolute; right: -6px; top: 50%; transform: translateY(-50%); \
                                width: 12px; height: 12px; background: white; border: 2px solid #10b981; \
                                border-radius: 50%;">
                    </div>
                }
            })}
        </div>
    }
}

// ============================================================================
// Edge Routing Renderer
// ============================================================================

/// Edge routing renderer component
#[component]
fn EdgeRoutingRenderer(
    store: FlowStore,
    strategy: RwSignal<EdgeRoutingStrategy>,
    show_debug: RwSignal<bool>,
) -> impl IntoView {
    let edges = move || store.get_edges();

    view! {
        <svg class="xyflow__edges edge-routing" style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            // SVG definitions
            <defs>
                // Gradient for edges
                <linearGradient id="routing-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#10b981;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#3b82f6;stop-opacity:1" />
                </linearGradient>

                // Debug gradient (shows routing path)
                <linearGradient id="routing-debug-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#ef4444;stop-opacity:0.3" />
                    <stop offset="100%" style="stop-color:#f59e0b;stop-opacity:0.3" />
                </linearGradient>

                // Arrow marker
                <marker id="routing-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#3b82f6" />
                </marker>

                // Glow filter
                <filter id="routing-edge-glow" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur stdDeviation="2" result="blur" />
                    <feMerge>
                        <feMergeNode in="blur" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>
            </defs>

            // Render obstacle bounding boxes when debug is on
            {move || {
                if show_debug.get() {
                    let nodes = store.get_nodes();
                    nodes.into_iter()
                        .filter(|n| {
                            n.data.get("nodeType")
                                .and_then(|v| v.as_str())
                                .map(|t| t == "obstacle")
                                .unwrap_or(false)
                        })
                        .map(|node| {
                            let x = node.position.x;
                            let y = node.position.y;
                            let w = node.width.unwrap_or(100.0);
                            let h = node.height.unwrap_or(50.0);
                            let padding = 20.0;
                            view! {
                                <rect
                                    x=x - padding
                                    y=y - padding
                                    width=w + padding * 2.0
                                    height=h + padding * 2.0
                                    fill="rgba(245, 158, 11, 0.1)"
                                    stroke="#f59e0b"
                                    stroke-width="1"
                                    stroke-dasharray="4,4"
                                />
                            }
                        })
                        .collect_view()
                        .into_any()
                } else {
                    ().into_any()
                }
            }}

            <For
                each=edges
                key=|edge| edge.id.clone()
                children=move |edge| {
                    let edge_id = edge.id.clone();
                    let source_id = edge.source.clone();
                    let target_id = edge.target.clone();
                    let label = edge.label.clone();

                    view! {
                        <RoutedEdge
                            edge_id=edge_id
                            source_id=source_id
                            target_id=target_id
                            label=label
                            store=store
                            strategy=strategy
                            show_debug=show_debug
                        />
                    }
                }
            />
        </svg>
    }
}

// ============================================================================
// Routed Edge Component
// ============================================================================

/// Individual routed edge component
#[component]
fn RoutedEdge(
    edge_id: String,
    source_id: String,
    target_id: String,
    label: Option<String>,
    store: FlowStore,
    strategy: RwSignal<EdgeRoutingStrategy>,
    show_debug: RwSignal<bool>,
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
                // Get obstacle nodes
                let obstacles: Vec<&Node> = nodes.iter()
                    .filter(|n| {
                        n.data.get("nodeType")
                            .and_then(|v| v.as_str())
                            .map(|t| t == "obstacle")
                            .unwrap_or(false)
                    })
                    .collect();

                // Calculate edge endpoints (right of source, left of target)
                let source_point = Position::new(
                    source.position.x + source.width.unwrap_or(100.0),
                    source.position.y + source.height.unwrap_or(50.0) / 2.0,
                );
                let target_point = Position::new(
                    target.position.x,
                    target.position.y + target.height.unwrap_or(50.0) / 2.0,
                );

                let (path, waypoints) = match current_strategy {
                    EdgeRoutingStrategy::Direct => {
                        let path = generate_bezier_path(source_point, target_point);
                        (path, vec![])
                    }
                    EdgeRoutingStrategy::Orthogonal => {
                        let (path, waypoints) = generate_orthogonal_path(source_point, target_point);
                        (path, waypoints)
                    }
                    EdgeRoutingStrategy::OrthogonalAvoidance => {
                        let (path, waypoints) = generate_avoiding_path(source_point, target_point, &obstacles);
                        (path, waypoints)
                    }
                };

                // Calculate label position at midpoint
                let label_x = (source_point.x + target_point.x) / 2.0;
                let label_y = (source_point.y + target_point.y) / 2.0;

                (path, label_x, label_y, waypoints)
            } else {
                (String::new(), 0.0, 0.0, vec![])
            }
        }
    });

    view! {
        <g class="routed-edge-group" data-id=edge_id.clone()>
            // Debug waypoints
            {move || {
                if show_debug.get() {
                    let (_, _, _, waypoints) = path_data.get();
                    let waypoints_len = waypoints.len();
                    waypoints.into_iter().enumerate().map(move |(i, wp)| {
                        view! {
                            <circle
                                cx=wp.x
                                cy=wp.y
                                r="4"
                                fill=if i == 0 || i == waypoints_len - 1 { "#ef4444" } else { "#f59e0b" }
                                stroke="white"
                                stroke-width="1"
                            />
                        }
                    }).collect_view()
                    .into_any()
                } else {
                    ().into_any()
                }
            }}

            // Glow/shadow path (behind main path)
            <path
                class="routed-edge-glow"
                d=move || path_data.get().0
                fill="none"
                stroke="url(#routing-edge-gradient)"
                stroke-width="4"
                stroke-opacity="0.3"
                stroke-linecap="round"
                stroke-linejoin="round"
            />

            // Main edge path
            <path
                class="routed-edge-path"
                d=move || path_data.get().0
                fill="none"
                stroke="url(#routing-edge-gradient)"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                attr:marker-end="url(#routing-arrow)"
            />

            // Edge label
            {move || {
                let (_, label_x, label_y, _) = path_data.get();
                label.clone().map(|text| {
                    view! {
                        <g transform=format!("translate({}, {})", label_x, label_y)>
                            // Label background
                            <rect
                                x="-32"
                                y="-10"
                                width="64"
                                height="20"
                                rx="10"
                                fill="white"
                                stroke="#3b82f6"
                                stroke-width="1"
                            />
                            // Label text
                            <text
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="10"
                                font-weight="500"
                                fill="#3b82f6"
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

// ============================================================================
// Path Generation Functions
// ============================================================================

/// Generate a smooth bezier path (direct routing)
fn generate_bezier_path(from: Position, to: Position) -> String {
    let dx = to.x - from.x;
    let offset = dx.abs() * 0.4;

    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y,
        from.x + offset, from.y,
        to.x - offset, to.y,
        to.x, to.y
    )
}

/// Generate an orthogonal (right-angle) path
fn generate_orthogonal_path(from: Position, to: Position) -> (String, Vec<Position>) {
    let mid_x = (from.x + to.x) / 2.0;

    // Create waypoints
    let waypoints = vec![
        from,
        Position::new(mid_x, from.y),
        Position::new(mid_x, to.y),
        to,
    ];

    let path = format!(
        "M {} {} L {} {} L {} {} L {} {}",
        from.x, from.y,
        mid_x, from.y,
        mid_x, to.y,
        to.x, to.y
    );

    (path, waypoints)
}

/// Generate a path that avoids obstacles
fn generate_avoiding_path(from: Position, to: Position, obstacles: &[&Node]) -> (String, Vec<Position>) {
    // Simple obstacle avoidance algorithm:
    // 1. Check if direct path intersects any obstacle
    // 2. If yes, route around the obstacle

    let padding = 20.0; // Space around obstacles

    // Check which obstacles are in the way
    let blocking_obstacles: Vec<&Node> = obstacles.iter()
        .filter(|obs| is_obstacle_blocking(from, to, obs, padding))
        .copied()
        .collect();

    if blocking_obstacles.is_empty() {
        // No obstacles, use orthogonal path
        return generate_orthogonal_path(from, to);
    }

    // Calculate bounding box of all blocking obstacles
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for obs in &blocking_obstacles {
        let ox = obs.position.x - padding;
        let oy = obs.position.y - padding;
        let ow = obs.width.unwrap_or(100.0) + padding * 2.0;
        let oh = obs.height.unwrap_or(50.0) + padding * 2.0;

        min_x = min_x.min(ox);
        max_x = max_x.max(ox + ow);
        min_y = min_y.min(oy);
        max_y = max_y.max(oy + oh);
    }

    // Decide whether to go above or below the obstacles
    let from_y_dist_to_top = (from.y - min_y).abs();
    let from_y_dist_to_bottom = (from.y - max_y).abs();
    let to_y_dist_to_top = (to.y - min_y).abs();
    let to_y_dist_to_bottom = (to.y - max_y).abs();

    let go_above = (from_y_dist_to_top + to_y_dist_to_top) < (from_y_dist_to_bottom + to_y_dist_to_bottom);

    let route_y = if go_above {
        min_y - 10.0 // Go above
    } else {
        max_y + 10.0 // Go below
    };

    // Build waypoints
    let waypoints = vec![
        from,
        Position::new(from.x + 20.0, from.y),
        Position::new(from.x + 20.0, route_y),
        Position::new(to.x - 20.0, route_y),
        Position::new(to.x - 20.0, to.y),
        to,
    ];

    // Build path
    let path = format!(
        "M {} {} L {} {} L {} {} L {} {} L {} {} L {} {}",
        waypoints[0].x, waypoints[0].y,
        waypoints[1].x, waypoints[1].y,
        waypoints[2].x, waypoints[2].y,
        waypoints[3].x, waypoints[3].y,
        waypoints[4].x, waypoints[4].y,
        waypoints[5].x, waypoints[5].y,
    );

    (path, waypoints)
}

/// Check if an obstacle is blocking the path between two points
fn is_obstacle_blocking(from: Position, to: Position, obstacle: &Node, padding: f64) -> bool {
    let ox = obstacle.position.x - padding;
    let oy = obstacle.position.y - padding;
    let ow = obstacle.width.unwrap_or(100.0) + padding * 2.0;
    let oh = obstacle.height.unwrap_or(50.0) + padding * 2.0;

    // Check if the horizontal segment of an orthogonal path would intersect
    let mid_x = (from.x + to.x) / 2.0;

    // Check if mid_x is within obstacle's x range
    let x_overlaps = mid_x >= ox && mid_x <= ox + ow;

    // Check if the vertical range of the path overlaps with obstacle
    let path_min_y = from.y.min(to.y);
    let path_max_y = from.y.max(to.y);
    let obs_min_y = oy;
    let obs_max_y = oy + oh;

    let y_overlaps = !(path_max_y < obs_min_y || path_min_y > obs_max_y);

    // Also check if obstacle is between source and target horizontally
    let obs_between = ox <= to.x && ox + ow >= from.x;

    x_overlaps && y_overlaps && obs_between
}
