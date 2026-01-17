//! Moving Handles Example
//!
//! Demonstrates how to create handles that can dynamically change position.
//! A toggle button moves handles between different positions (e.g., Top <-> Bottom).

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for moving handles example
static MOVING_HANDLES_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_moving_handles_drag_signal() -> RwSignal<Option<DragState>> {
    *MOVING_HANDLES_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Moving handles example showing nodes with dynamically repositioning handles
#[component]
pub fn MovingHandlesExample() -> impl IntoView {
    // Signal to track handle positions - true = original positions, false = swapped
    let handle_position_mode = RwSignal::new(true);

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 80.0))
            .with_data(json!({
                "label": "Source Node",
                "description": "Handles move from Top/Bottom to Left/Right",
                "type": "source"
            }))
            .with_dimensions(180.0, 100.0),
        Node::new("2".to_string(), Position::new(350.0, 80.0))
            .with_data(json!({
                "label": "Target Node",
                "description": "Watch the handles reposition!",
                "type": "target"
            }))
            .with_dimensions(180.0, 100.0),
        Node::new("3".to_string(), Position::new(220.0, 250.0))
            .with_data(json!({
                "label": "Middle Node",
                "description": "Has both source and target handles",
                "type": "default"
            }))
            .with_dimensions(180.0, 100.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e3-2".to_string(), "3".to_string(), "2".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_moving_handles_drag_signal();

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

    // Toggle handle positions
    let toggle_positions = move |_| {
        handle_position_mode.update(|v| *v = !*v);
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
                    // Custom edge renderer that recalculates paths based on handle positions
                    <MovingEdgeRenderer store=store handle_position_mode=handle_position_mode />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes with moving handles
                    {move || {
                        let mode = handle_position_mode.get();
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <MovingHandleNode
                                    node=node.clone()
                                    store=store
                                    original_positions=mode
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel with toggle button
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); max-width: 260px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Moving Handles"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666; line-height: 1.4;">
                            "Click the button below to toggle handle positions. Edges will update to follow the new handle locations."
                        </p>

                        <button
                            style="
                                width: 100%;
                                padding: 10px 16px;
                                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                                color: white;
                                border: none;
                                border-radius: 6px;
                                cursor: pointer;
                                font-size: 13px;
                                font-weight: 600;
                                transition: transform 0.1s;
                            "
                            on:click=toggle_positions
                        >
                            {move || {
                                if handle_position_mode.get() {
                                    "Move Handles to Sides"
                                } else {
                                    "Move Handles to Top/Bottom"
                                }
                            }}
                        </button>

                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; color: #888;">
                                <strong style="display: block; margin-bottom: 6px;">"Current Mode:"</strong>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="
                                        display: inline-block;
                                        width: 8px;
                                        height: 8px;
                                        border-radius: 50%;
                                        background: #667eea;
                                    "></span>
                                    {move || {
                                        if handle_position_mode.get() {
                                            "Top/Bottom (Original)"
                                        } else {
                                            "Left/Right (Swapped)"
                                        }
                                    }}
                                </div>
                            </div>
                        </div>

                        <div style="margin-top: 12px; font-size: 10px; color: #999;">
                            <div style="margin: 4px 0;">"Source Node: green handles"</div>
                            <div style="margin: 4px 0;">"Target Node: blue handles"</div>
                            <div style="margin: 4px 0;">"Middle Node: both handles"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Node with handles that can move between positions
#[component]
fn MovingHandleNode(
    node: Node,
    store: FlowStore,
    /// True = Top/Bottom positions, False = Left/Right positions
    original_positions: bool,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_drag = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let description = node.data.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_moving_handles_drag_signal();

    // Determine handle visibility based on node type
    let has_source = node_type != "target";
    let has_target = node_type != "source";

    // Determine handle positions based on mode
    let (source_pos, target_pos) = if original_positions {
        // Original: Target on top, Source on bottom
        (HandlePosition::Bottom, HandlePosition::Top)
    } else {
        // Swapped: Target on left, Source on right
        (HandlePosition::Right, HandlePosition::Left)
    };

    // Node color based on type
    let (bg_color, border_color) = match node_type.as_str() {
        "source" => ("#e8f5e9", "#4caf50"),
        "target" => ("#e3f2fd", "#2196f3"),
        _ => ("#f3e5f5", "#9c27b0"),
    };

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
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

    // Get reactive node state
    let node_state = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| (n.position, n.width.unwrap_or(180.0), n.height.unwrap_or(100.0)))
            .unwrap_or((Position::new(0.0, 0.0), 180.0, 100.0))
    };

    view! {
        <div
            class="xyflow__node moving-handle-node"
            style=move || {
                let (pos, width, height) = node_state();
                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     box-shadow: 0 2px 8px rgba(0,0,0,0.1); cursor: grab; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 12px; box-sizing: border-box;",
                    pos.x, pos.y, width, height, bg_color, border_color
                )
            }
            on:mousedown=on_mousedown
        >
            // Node label
            <div style=format!("font-weight: 600; font-size: 13px; color: {}; margin-bottom: 4px;", border_color)>
                {label}
            </div>

            // Node description
            <div style="font-size: 10px; color: #666; text-align: center; line-height: 1.3;">
                {description}
            </div>

            // Target handle (position changes based on mode)
            {has_target.then(|| view! {
                <Handle
                    node_id=node_id.clone()
                    r#type=HandleType::Target
                    position=target_pos
                    connection_mode=ConnectionMode::Strict
                    style=format!(
                        "background: {}; width: 12px; height: 12px; border: 2px solid white; \
                         box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
                        border_color
                    )
                />
            })}

            // Source handle (position changes based on mode)
            {has_source.then(|| view! {
                <Handle
                    node_id=node_id.clone()
                    r#type=HandleType::Source
                    position=source_pos
                    connection_mode=ConnectionMode::Strict
                    style=format!(
                        "background: {}; width: 12px; height: 12px; border: 2px solid white; \
                         box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
                        border_color
                    )
                />
            })}
        </div>
    }
}

/// Custom edge renderer that recalculates edge paths based on handle positions
#[component]
fn MovingEdgeRenderer(
    store: FlowStore,
    /// True = Top/Bottom positions, False = Left/Right positions
    handle_position_mode: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                // Arrow marker for edges
                <marker
                    id="moving-edge-arrow"
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
                let original_mode = handle_position_mode.get();

                edges.into_iter().filter_map(move |edge| {
                    // Find source and target nodes
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate handle positions based on mode
                    let (source_x, source_y, target_x, target_y) = if original_mode {
                        // Original: Source on bottom, Target on top
                        let sx = source_node.position.x + source_node.width.unwrap_or(180.0) / 2.0;
                        let sy = source_node.position.y + source_node.height.unwrap_or(100.0);
                        let tx = target_node.position.x + target_node.width.unwrap_or(180.0) / 2.0;
                        let ty = target_node.position.y;
                        (sx, sy, tx, ty)
                    } else {
                        // Swapped: Source on right, Target on left
                        let sx = source_node.position.x + source_node.width.unwrap_or(180.0);
                        let sy = source_node.position.y + source_node.height.unwrap_or(100.0) / 2.0;
                        let tx = target_node.position.x;
                        let ty = target_node.position.y + target_node.height.unwrap_or(100.0) / 2.0;
                        (sx, sy, tx, ty)
                    };

                    // Calculate bezier control points
                    let (c1x, c1y, c2x, c2y) = if original_mode {
                        // Vertical bezier
                        let offset = (target_y - source_y).abs() * 0.5;
                        (source_x, source_y + offset, target_x, target_y - offset)
                    } else {
                        // Horizontal bezier
                        let offset = (target_x - source_x).abs() * 0.5;
                        (source_x + offset, source_y, target_x - offset, target_y)
                    };

                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        source_x, source_y,
                        c1x, c1y,
                        c2x, c2y,
                        target_x, target_y
                    );

                    Some(view! {
                        <g class="xyflow__edge">
                            // Edge path
                            <path
                                d=path.clone()
                                stroke="#888"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#moving-edge-arrow)"
                                style="pointer-events: stroke; transition: d 0.3s ease-in-out;"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
