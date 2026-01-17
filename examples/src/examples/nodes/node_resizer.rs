//! Node Resizer Example
//!
//! Demonstrates how to create resizable nodes with drag handles on corners
//! and edges, supporting min/max dimensions and different resize strategies.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::get_drag_signal;

/// Resize handle position
#[derive(Clone, Copy, Debug, PartialEq)]
enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

/// Global resize state
static RESIZING_NODE: std::sync::OnceLock<RwSignal<Option<ResizeState>>> = std::sync::OnceLock::new();

#[derive(Clone, Debug)]
struct ResizeState {
    node_id: String,
    handle: ResizeHandle,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
    start_size: (f64, f64),
}

/// Get or initialize the global resize state signal
fn get_resize_signal() -> RwSignal<Option<ResizeState>> {
    *RESIZING_NODE.get_or_init(|| RwSignal::new(None))
}

/// Node resizer example showing resizable nodes
#[component]
pub fn NodeResizerExample() -> impl IntoView {
    // Create initial nodes with explicit dimensions
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({
                "label": "Free Resize",
                "type": "resizable",
                "minWidth": 100.0,
                "minHeight": 60.0,
                "maxWidth": 400.0,
                "maxHeight": 300.0
            }))
            .with_dimensions(180.0, 80.0),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({
                "label": "Aspect Locked",
                "type": "resizable-aspect",
                "minWidth": 80.0,
                "minHeight": 80.0,
                "maxWidth": 300.0,
                "maxHeight": 300.0,
                "aspectRatio": 1.0
            }))
            .with_dimensions(120.0, 120.0),
        Node::new("3".to_string(), Position::new(350.0, 100.0))
            .with_data(json!({
                "label": "Horizontal Only",
                "type": "resizable-horizontal",
                "minWidth": 100.0,
                "maxWidth": 350.0
            }))
            .with_dimensions(150.0, 60.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_drag_signal();
    let resize_signal = get_resize_signal();

    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
        let current_x = ev.client_x() as f64;
        let current_y = ev.client_y() as f64;
        let viewport = store.get_viewport();

        // Handle resize
        if let Some(resize_state) = resize_signal.get() {
            let (start_x, start_y) = resize_state.start_mouse;
            let (node_start_x, node_start_y) = resize_state.start_pos;
            let (start_width, start_height) = resize_state.start_size;

            // Calculate delta accounting for zoom
            let dx = (current_x - start_x) / viewport.zoom;
            let dy = (current_y - start_y) / viewport.zoom;

            // Get node constraints from data
            let nodes = store.get_nodes();
            if let Some(node) = nodes.iter().find(|n| n.id == resize_state.node_id) {
                let min_width = node.data.get("minWidth").and_then(|v| v.as_f64()).unwrap_or(50.0);
                let min_height = node.data.get("minHeight").and_then(|v| v.as_f64()).unwrap_or(30.0);
                let max_width = node.data.get("maxWidth").and_then(|v| v.as_f64()).unwrap_or(500.0);
                let max_height = node.data.get("maxHeight").and_then(|v| v.as_f64()).unwrap_or(500.0);
                let aspect_ratio = node.data.get("aspectRatio").and_then(|v| v.as_f64());
                let node_type = node.data.get("type").and_then(|v| v.as_str()).unwrap_or("");

                let (mut new_width, mut new_height, mut new_x, mut new_y) =
                    (start_width, start_height, node_start_x, node_start_y);

                match resize_state.handle {
                    ResizeHandle::Right => {
                        new_width = (start_width + dx).clamp(min_width, max_width);
                    }
                    ResizeHandle::Left => {
                        let potential_width = start_width - dx;
                        new_width = potential_width.clamp(min_width, max_width);
                        new_x = node_start_x + (start_width - new_width);
                    }
                    ResizeHandle::Bottom => {
                        if node_type != "resizable-horizontal" {
                            new_height = (start_height + dy).clamp(min_height, max_height);
                        }
                    }
                    ResizeHandle::Top => {
                        if node_type != "resizable-horizontal" {
                            let potential_height = start_height - dy;
                            new_height = potential_height.clamp(min_height, max_height);
                            new_y = node_start_y + (start_height - new_height);
                        }
                    }
                    ResizeHandle::BottomRight => {
                        new_width = (start_width + dx).clamp(min_width, max_width);
                        if node_type != "resizable-horizontal" {
                            new_height = (start_height + dy).clamp(min_height, max_height);
                        }
                    }
                    ResizeHandle::BottomLeft => {
                        let potential_width = start_width - dx;
                        new_width = potential_width.clamp(min_width, max_width);
                        new_x = node_start_x + (start_width - new_width);
                        if node_type != "resizable-horizontal" {
                            new_height = (start_height + dy).clamp(min_height, max_height);
                        }
                    }
                    ResizeHandle::TopRight => {
                        new_width = (start_width + dx).clamp(min_width, max_width);
                        if node_type != "resizable-horizontal" {
                            let potential_height = start_height - dy;
                            new_height = potential_height.clamp(min_height, max_height);
                            new_y = node_start_y + (start_height - new_height);
                        }
                    }
                    ResizeHandle::TopLeft => {
                        let potential_width = start_width - dx;
                        new_width = potential_width.clamp(min_width, max_width);
                        new_x = node_start_x + (start_width - new_width);
                        if node_type != "resizable-horizontal" {
                            let potential_height = start_height - dy;
                            new_height = potential_height.clamp(min_height, max_height);
                            new_y = node_start_y + (start_height - new_height);
                        }
                    }
                }

                // Apply aspect ratio lock if needed
                if let Some(ratio) = aspect_ratio {
                    if new_width / ratio <= max_height && new_width / ratio >= min_height {
                        new_height = new_width / ratio;
                    } else {
                        new_width = new_height * ratio;
                    }
                }

                // Update node
                let node_id = resize_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.position = Position::new(new_x, new_y);
                    n.width = Some(new_width);
                    n.height = Some(new_height);
                });
            }
            return;
        }

        // Handle drag
        if let Some(drag_state) = drag_signal.get() {
            let (start_x, start_y) = drag_state.start_mouse;
            let (node_start_x, node_start_y) = drag_state.start_pos;

            let dx = (current_x - start_x) / viewport.zoom;
            let dy = (current_y - start_y) / viewport.zoom;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(node_start_x + dx, node_start_y + dy);
            });
        }
    };

    let on_global_mouseup = move |_ev: leptos::ev::MouseEvent| {
        // End resize
        if resize_signal.get().is_some() {
            resize_signal.set(None);
            return;
        }

        // End drag
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
                    // Render edges
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render resizable nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <ResizableNode
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
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); max-width: 220px;">
                        <strong style="display: block; margin-bottom: 8px;">"Node Resizer"</strong>
                        <p style="margin: 0 0 8px 0; font-size: 12px; color: #666;">
                            "Drag the corners or edges to resize nodes"
                        </p>
                        <div style="font-size: 11px; color: #888;">
                            <div style="margin: 4px 0; padding: 4px; background: #f5f5f5; border-radius: 4px;">
                                <strong>"Free Resize:"</strong>" Resize in any direction"
                            </div>
                            <div style="margin: 4px 0; padding: 4px; background: #f5f5f5; border-radius: 4px;">
                                <strong>"Aspect Locked:"</strong>" Maintains 1:1 ratio"
                            </div>
                            <div style="margin: 4px 0; padding: 4px; background: #f5f5f5; border-radius: 4px;">
                                <strong>"Horizontal:"</strong>" Width only"
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Resizable node component with resize handles
#[component]
fn ResizableNode(
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
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("resizable")
        .to_string();
    let is_horizontal_only = node_type == "resizable-horizontal";
    let is_aspect_locked = node_type == "resizable-aspect";

    let drag_signal = get_drag_signal();
    let resize_signal = get_resize_signal();

    // Mouse down on node body - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id_for_drag) {
            drag_signal.set(Some(crate::shared::DragState {
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
            .map(|n| (n.position, n.width.unwrap_or(150.0), n.height.unwrap_or(60.0)))
            .unwrap_or((Position::new(0.0, 0.0), 150.0, 60.0))
    };

    // Determine background color based on type
    let bg_color = if is_aspect_locked {
        "#e8f4ea"
    } else if is_horizontal_only {
        "#f4e8f4"
    } else {
        "#e8f0f4"
    };

    let border_color = if is_aspect_locked {
        "#4caf50"
    } else if is_horizontal_only {
        "#9c27b0"
    } else {
        "#2196f3"
    };

    view! {
        <div
            class="xyflow__node resizable-node"
            style=move || {
                let (pos, width, height) = node_state();
                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     display: flex; align-items: center; justify-content: center; cursor: grab;",
                    pos.x, pos.y, width, height, bg_color, border_color
                )
            }
            on:mousedown=on_mousedown
        >
            // Node content
            <div style="font-size: 12px; font-weight: 500; color: #333; text-align: center; padding: 8px; user-select: none;">
                {label}
            </div>

            // Target handle (top)
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Top
                connection_mode=ConnectionMode::Strict
            />

            // Source handle (bottom)
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Bottom
                connection_mode=ConnectionMode::Strict
            />

            // Resize handles - corners
            <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::TopLeft store=store />
            <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::TopRight store=store />
            <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::BottomLeft store=store />
            <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::BottomRight store=store />

            // Resize handles - edges (only for non-horizontal-only mode)
            {(!is_horizontal_only).then(|| view! {
                <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::Top store=store />
                <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::Bottom store=store />
            })}

            <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::Left store=store />
            <ResizeHandleComponent node_id=node_id.clone() handle=ResizeHandle::Right store=store />
        </div>
    }
}

/// Individual resize handle component
#[component]
fn ResizeHandleComponent(
    node_id: String,
    handle: ResizeHandle,
    store: FlowStore,
) -> impl IntoView {
    let resize_signal = get_resize_signal();
    let node_id_clone = node_id.clone();

    // Handle styles based on position
    let (position_style, cursor) = match handle {
        ResizeHandle::TopLeft => ("top: -4px; left: -4px;", "nwse-resize"),
        ResizeHandle::TopRight => ("top: -4px; right: -4px;", "nesw-resize"),
        ResizeHandle::BottomLeft => ("bottom: -4px; left: -4px;", "nesw-resize"),
        ResizeHandle::BottomRight => ("bottom: -4px; right: -4px;", "nwse-resize"),
        ResizeHandle::Top => ("top: -4px; left: 50%; transform: translateX(-50%);", "ns-resize"),
        ResizeHandle::Bottom => ("bottom: -4px; left: 50%; transform: translateX(-50%);", "ns-resize"),
        ResizeHandle::Left => ("left: -4px; top: 50%; transform: translateY(-50%);", "ew-resize"),
        ResizeHandle::Right => ("right: -4px; top: 50%; transform: translateY(-50%);", "ew-resize"),
    };

    let is_corner = matches!(
        handle,
        ResizeHandle::TopLeft | ResizeHandle::TopRight | ResizeHandle::BottomLeft | ResizeHandle::BottomRight
    );

    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id_clone) {
            resize_signal.set(Some(ResizeState {
                node_id: node_id_clone.clone(),
                handle,
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
                start_size: (node.width.unwrap_or(150.0), node.height.unwrap_or(60.0)),
            }));
        }
    };

    view! {
        <div
            class="resize-handle"
            style=format!(
                "position: absolute; {}; width: {}px; height: {}px; \
                 background: white; border: 2px solid #666; border-radius: {}; \
                 cursor: {}; z-index: 10;",
                position_style,
                if is_corner { 10 } else { 8 },
                if is_corner { 10 } else { 8 },
                if is_corner { "2px" } else { "1px" },
                cursor
            )
            on:mousedown=on_mousedown
        />
    }
}
