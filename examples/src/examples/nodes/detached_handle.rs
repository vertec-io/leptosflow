//! Detached Handle Example
//!
//! Demonstrates handles positioned outside the node body using CSS positioning.
//! The handles are rendered outside the normal node bounds via absolute positioning
//! with additional offset from the node edges.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for detached handle example
static DETACHED_HANDLE_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_detached_handle_drag_signal() -> RwSignal<Option<DragState>> {
    *DETACHED_HANDLE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Detached handle example showing nodes with handles positioned outside the node body
#[component]
pub fn DetachedHandleExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({
                "label": "Source Node",
                "description": "Handles are 20px outside the node",
                "handle_offset": 20,
                "node_style": "source"
            }))
            .with_dimensions(160.0, 80.0),
        Node::new("2".to_string(), Position::new(400.0, 100.0))
            .with_data(json!({
                "label": "Target Node",
                "description": "Target handle floats to the left",
                "handle_offset": 20,
                "node_style": "target"
            }))
            .with_dimensions(160.0, 80.0),
        Node::new("3".to_string(), Position::new(250.0, 280.0))
            .with_data(json!({
                "label": "Both Handles",
                "description": "Handles on both sides",
                "handle_offset": 25,
                "node_style": "default"
            }))
            .with_dimensions(160.0, 80.0),
        Node::new("4".to_string(), Position::new(100.0, 400.0))
            .with_data(json!({
                "label": "Wide Offset",
                "description": "Handles 40px away",
                "handle_offset": 40,
                "node_style": "source"
            }))
            .with_dimensions(160.0, 80.0),
        Node::new("5".to_string(), Position::new(400.0, 400.0))
            .with_data(json!({
                "label": "Minimal Offset",
                "description": "Handles 15px away",
                "handle_offset": 15,
                "node_style": "target"
            }))
            .with_dimensions(160.0, 80.0),
    ];

    // Create initial edges connecting the nodes
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e3-2".to_string(), "3".to_string(), "2".to_string()),
        Edge::new("e4-3".to_string(), "4".to_string(), "3".to_string()),
        Edge::new("e3-5".to_string(), "3".to_string(), "5".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_detached_handle_drag_signal();

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
                    // Edge renderer with custom path calculation for detached handles
                    <DetachedEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes with detached handles
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <DetachedHandleNode
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); max-width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Detached Handles"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666; line-height: 1.4;">
                            "Handles are positioned outside the node bounds using CSS absolute positioning with custom offsets."
                        </p>

                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; color: #666;">
                                <strong style="display: block; margin-bottom: 8px;">"Handle Offsets:"</strong>
                                <div style="margin: 4px 0; display: flex; align-items: center; gap: 6px;">
                                    <span style="display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: #4caf50;"></span>
                                    "Source nodes: right side"
                                </div>
                                <div style="margin: 4px 0; display: flex; align-items: center; gap: 6px;">
                                    <span style="display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: #2196f3;"></span>
                                    "Target nodes: left side"
                                </div>
                                <div style="margin: 4px 0; display: flex; align-items: center; gap: 6px;">
                                    <span style="display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: #9c27b0;"></span>
                                    "Default: both sides"
                                </div>
                            </div>
                        </div>

                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee; font-size: 10px; color: #999;">
                            <p style="margin: 0;">"Each node has a configurable offset (15-40px) that determines how far the handles float from the node body."</p>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Node with handles positioned outside the node body
#[component]
fn DetachedHandleNode(
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
    let description = node.data.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let handle_offset = node.data.get("handle_offset")
        .and_then(|v| v.as_i64())
        .unwrap_or(20) as i32;
    let node_style = node.data.get("node_style")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_detached_handle_drag_signal();

    // Determine handle visibility based on node style
    let has_source = node_style != "target";
    let has_target = node_style != "source";

    // Node color based on style
    let (bg_color, border_color, handle_color) = match node_style.as_str() {
        "source" => ("#e8f5e9", "#4caf50", "#4caf50"),
        "target" => ("#e3f2fd", "#2196f3", "#2196f3"),
        _ => ("#f3e5f5", "#9c27b0", "#9c27b0"),
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
            .map(|n| (n.position, n.width.unwrap_or(160.0), n.height.unwrap_or(80.0)))
            .unwrap_or((Position::new(0.0, 0.0), 160.0, 80.0))
    };

    // Calculate handle styles for detached positioning
    // The handle is positioned relative to the node, then moved further out
    let source_handle_style = format!(
        "position: absolute; right: -{}px; top: 50%; transform: translateY(-50%); \
         background: {}; width: 14px; height: 14px; border: 2px solid white; \
         border-radius: 50%; box-shadow: 0 2px 6px rgba(0,0,0,0.2); cursor: crosshair;",
        handle_offset, handle_color
    );

    let target_handle_style = format!(
        "position: absolute; left: -{}px; top: 50%; transform: translateY(-50%); \
         background: {}; width: 14px; height: 14px; border: 2px solid white; \
         border-radius: 50%; box-shadow: 0 2px 6px rgba(0,0,0,0.2); cursor: crosshair;",
        handle_offset, handle_color
    );

    view! {
        <div
            class="xyflow__node detached-handle-node"
            style=move || {
                let (pos, width, height) = node_state();
                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     box-shadow: 0 4px 12px rgba(0,0,0,0.1); cursor: grab; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 10px; box-sizing: border-box; overflow: visible;",
                    pos.x, pos.y, width, height, bg_color, border_color
                )
            }
            on:mousedown=on_mousedown
        >
            // Node label
            <div style=format!("font-weight: 600; font-size: 12px; color: {}; margin-bottom: 4px;", border_color)>
                {label}
            </div>

            // Node description
            <div style="font-size: 10px; color: #666; text-align: center; line-height: 1.3;">
                {description}
            </div>

            // Detached source handle (right side, outside node)
            {has_source.then(|| {
                let style = source_handle_style.clone();
                view! {
                    <Handle
                        node_id=node_id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Right
                        connection_mode=ConnectionMode::Strict
                        style=style
                    />
                }
            })}

            // Detached target handle (left side, outside node)
            {has_target.then(|| {
                let style = target_handle_style.clone();
                view! {
                    <Handle
                        node_id=node_id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Left
                        connection_mode=ConnectionMode::Strict
                        style=style
                    />
                }
            })}
        </div>
    }
}

/// Custom edge renderer that accounts for detached handle positions
#[component]
fn DetachedEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                // Arrow marker for edges
                <marker
                    id="detached-edge-arrow"
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
                    // Find source and target nodes
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Get handle offsets from node data
                    let source_offset = source_node.data.get("handle_offset")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(20.0);
                    let target_offset = target_node.data.get("handle_offset")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(20.0);

                    // Calculate edge endpoints at detached handle positions
                    // Source handle is on the right side, offset outward
                    let source_x = source_node.position.x + source_node.width.unwrap_or(160.0) + source_offset;
                    let source_y = source_node.position.y + source_node.height.unwrap_or(80.0) / 2.0;

                    // Target handle is on the left side, offset outward
                    let target_x = target_node.position.x - target_offset;
                    let target_y = target_node.position.y + target_node.height.unwrap_or(80.0) / 2.0;

                    // Calculate bezier control points for horizontal flow
                    let dx = (target_x - source_x).abs();
                    let control_offset = dx * 0.4;

                    let c1x = source_x + control_offset;
                    let c1y = source_y;
                    let c2x = target_x - control_offset;
                    let c2y = target_y;

                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        source_x, source_y,
                        c1x, c1y,
                        c2x, c2y,
                        target_x, target_y
                    );

                    Some(view! {
                        <g class="xyflow__edge">
                            // Edge path with glow
                            <path
                                d=path.clone()
                                stroke="rgba(100, 100, 100, 0.2)"
                                stroke-width="4"
                                fill="none"
                            />
                            // Main edge path
                            <path
                                d=path
                                stroke="#888"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#detached-edge-arrow)"
                                style="pointer-events: stroke;"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
