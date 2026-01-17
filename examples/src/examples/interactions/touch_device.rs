//! Touch Device Example
//!
//! Demonstrates touch-optimized interactions for mobile devices:
//! - Touch dragging for nodes
//! - Pinch-to-zoom
//! - Two-finger pan
//! - Larger touch targets for handles

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Touch State (global for this example)
// ============================================================================

static TOUCH_DRAG_STATE: OnceLock<RwSignal<Option<TouchDragState>>> = OnceLock::new();
static TOUCH_GESTURE_STATE: OnceLock<RwSignal<Option<TouchGestureState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct TouchDragState {
    node_id: String,
    start_touch: (f64, f64),
    start_pos: (f64, f64),
    touch_id: i32,
}

#[derive(Clone, Debug)]
struct TouchGestureState {
    gesture_type: GestureType,
    initial_distance: f64,
    initial_zoom: f64,
    initial_center: (f64, f64),
    initial_viewport: (f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
enum GestureType {
    Pinch,
    Pan,
}

fn get_touch_drag_signal() -> RwSignal<Option<TouchDragState>> {
    *TOUCH_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

fn get_touch_gesture_signal() -> RwSignal<Option<TouchGestureState>> {
    *TOUCH_GESTURE_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Touch Event Log
// ============================================================================

#[derive(Clone, Debug)]
struct TouchEvent {
    timestamp: f64,
    event_type: String,
    details: String,
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Touch Device Example
#[component]
pub fn TouchDeviceExample() -> impl IntoView {
    // Touch event log
    let touch_log = RwSignal::new(Vec::<TouchEvent>::new());

    // Touch statistics
    let drag_count = RwSignal::new(0_i32);
    let pinch_count = RwSignal::new(0_i32);
    let pan_count = RwSignal::new(0_i32);

    // Handle size for larger touch targets
    let handle_size = RwSignal::new(24_i32);

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(80.0, 50.0))
            .with_data(json!({"label": "Input", "type": "input", "color": "#6ede87"})),
        Node::new("2".to_string(), Position::new(80.0, 200.0))
            .with_data(json!({"label": "Process A", "type": "default", "color": "#6865A5"})),
        Node::new("3".to_string(), Position::new(280.0, 125.0))
            .with_data(json!({"label": "Process B", "type": "default", "color": "#6865A5"})),
        Node::new("4".to_string(), Position::new(280.0, 275.0))
            .with_data(json!({"label": "Output", "type": "output", "color": "#ff6b6b"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Drag signal
    let drag_signal = get_touch_drag_signal();
    let gesture_signal = get_touch_gesture_signal();

    // Helper to add log entry
    let add_log = move |event_type: String, details: String| {
        touch_log.update(|logs| {
            logs.push(TouchEvent {
                timestamp: js_sys::Date::now(),
                event_type,
                details,
            });
            // Keep last 12 entries
            if logs.len() > 12 {
                logs.remove(0);
            }
        });
    };

    // Helper to calculate distance between two touches
    let calculate_distance = |t1: (f64, f64), t2: (f64, f64)| -> f64 {
        let dx = t2.0 - t1.0;
        let dy = t2.1 - t1.1;
        (dx * dx + dy * dy).sqrt()
    };

    // Helper to calculate center of two touches
    let calculate_center = |t1: (f64, f64), t2: (f64, f64)| -> (f64, f64) {
        ((t1.0 + t2.0) / 2.0, (t1.1 + t2.1) / 2.0)
    };

    // Touch start handler for canvas (two-finger gestures)
    let add_log_for_canvas_start = add_log.clone();
    let on_canvas_touchstart = move |ev: leptos::ev::TouchEvent| {
        let touches = ev.touches();

        if touches.length() == 2 {
            // Two-finger gesture (pinch or pan)
            ev.prevent_default();

            let touch1 = touches.get(0).unwrap();
            let touch2 = touches.get(1).unwrap();

            let t1 = (touch1.client_x() as f64, touch1.client_y() as f64);
            let t2 = (touch2.client_x() as f64, touch2.client_y() as f64);

            let distance = calculate_distance(t1, t2);
            let center = calculate_center(t1, t2);
            let viewport = store.get_viewport();

            gesture_signal.set(Some(TouchGestureState {
                gesture_type: GestureType::Pinch, // Will determine actual type on move
                initial_distance: distance,
                initial_zoom: viewport.zoom,
                initial_center: center,
                initial_viewport: (viewport.x, viewport.y),
            }));

            add_log_for_canvas_start(
                "gesture_start".to_string(),
                format!("2 fingers at distance {:.0}px", distance),
            );
        }
    };

    // Touch move handler for canvas
    let add_log_for_canvas_move = add_log.clone();
    let on_canvas_touchmove = move |ev: leptos::ev::TouchEvent| {
        let touches = ev.touches();

        // Handle node drag
        if let Some(drag_state) = drag_signal.get() {
            if let Some(touch) = (0..touches.length()).find_map(|i| {
                let t = touches.get(i)?;
                if t.identifier() == drag_state.touch_id {
                    Some(t)
                } else {
                    None
                }
            }) {
                ev.prevent_default();

                let current_x = touch.client_x() as f64;
                let current_y = touch.client_y() as f64;
                let (start_x, start_y) = drag_state.start_touch;
                let (node_start_x, node_start_y) = drag_state.start_pos;

                let viewport = store.get_viewport();
                let dx = (current_x - start_x) / viewport.zoom;
                let dy = (current_y - start_y) / viewport.zoom;

                store.update_node(&drag_state.node_id, |n| {
                    n.position = Position::new(node_start_x + dx, node_start_y + dy);
                });
            }
        }

        // Handle two-finger gesture
        if touches.length() == 2 {
            if let Some(gesture_state) = gesture_signal.get() {
                ev.prevent_default();

                let touch1 = touches.get(0).unwrap();
                let touch2 = touches.get(1).unwrap();

                let t1 = (touch1.client_x() as f64, touch1.client_y() as f64);
                let t2 = (touch2.client_x() as f64, touch2.client_y() as f64);

                let current_distance = calculate_distance(t1, t2);
                let current_center = calculate_center(t1, t2);

                // Calculate zoom change
                let zoom_ratio = current_distance / gesture_state.initial_distance;
                let new_zoom = (gesture_state.initial_zoom * zoom_ratio).clamp(0.1, 4.0);

                // Calculate pan (center movement)
                let pan_dx = current_center.0 - gesture_state.initial_center.0;
                let pan_dy = current_center.1 - gesture_state.initial_center.1;

                // Update viewport
                let new_x = gesture_state.initial_viewport.0 + pan_dx;
                let new_y = gesture_state.initial_viewport.1 + pan_dy;

                store.set_viewport(Viewport {
                    x: new_x,
                    y: new_y,
                    zoom: new_zoom,
                });

                // Determine gesture type for logging
                let distance_change = (current_distance - gesture_state.initial_distance).abs();
                let pan_distance = (pan_dx * pan_dx + pan_dy * pan_dy).sqrt();

                if distance_change > 20.0 && distance_change > pan_distance {
                    // Primarily pinch
                    if gesture_state.gesture_type != GestureType::Pinch {
                        gesture_signal.update(|g| {
                            if let Some(gs) = g {
                                gs.gesture_type = GestureType::Pinch;
                            }
                        });
                    }
                } else if pan_distance > 20.0 {
                    // Primarily pan
                    if gesture_state.gesture_type != GestureType::Pan {
                        gesture_signal.update(|g| {
                            if let Some(gs) = g {
                                gs.gesture_type = GestureType::Pan;
                            }
                        });
                    }
                }
            }
        }
    };

    // Touch end handler for canvas
    let add_log_for_canvas_end = add_log.clone();
    let on_canvas_touchend = move |ev: leptos::ev::TouchEvent| {
        // Check if the drag touch ended
        if let Some(drag_state) = drag_signal.get() {
            let touches = ev.touches();
            let drag_touch_active = (0..touches.length()).any(|i| {
                touches.get(i).map(|t| t.identifier() == drag_state.touch_id).unwrap_or(false)
            });

            if !drag_touch_active {
                store.update_node(&drag_state.node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
                drag_count.update(|c| *c += 1);
                add_log_for_canvas_end(
                    "drag_end".to_string(),
                    format!("Node: {}", drag_state.node_id),
                );
            }
        }

        // Check if gesture ended
        if ev.touches().length() < 2 {
            if let Some(gesture_state) = gesture_signal.get() {
                match gesture_state.gesture_type {
                    GestureType::Pinch => {
                        pinch_count.update(|c| *c += 1);
                        let viewport = store.get_viewport();
                        add_log_for_canvas_end(
                            "pinch_end".to_string(),
                            format!("Zoom: {:.2}x", viewport.zoom),
                        );
                    }
                    GestureType::Pan => {
                        pan_count.update(|c| *c += 1);
                        let viewport = store.get_viewport();
                        add_log_for_canvas_end(
                            "pan_end".to_string(),
                            format!("Pos: ({:.0}, {:.0})", viewport.x, viewport.y),
                        );
                    }
                }
                gesture_signal.set(None);
            }
        }
    };

    // Node touch start handler
    let on_node_touchstart = move |node_id: String, ev: leptos::ev::TouchEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let touches = ev.touches();
        if touches.length() == 1 {
            if let Some(touch) = touches.get(0) {
                let nodes = store.get_nodes();
                if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                    drag_signal.set(Some(TouchDragState {
                        node_id: node_id.clone(),
                        start_touch: (touch.client_x() as f64, touch.client_y() as f64),
                        start_pos: (node.position.x, node.position.y),
                        touch_id: touch.identifier(),
                    }));

                    store.update_node(&node_id, |n| {
                        n.dragging = true;
                    });

                    add_log(
                        "drag_start".to_string(),
                        format!("Node: {}", node_id),
                    );
                }
            }
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow touch-device-example"
                 style="width: 100%; height: 100%; position: relative; touch-action: none;"
                 on:touchstart=on_canvas_touchstart
                 on:touchmove=on_canvas_touchmove
                 on:touchend=on_canvas_touchend
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Render edges
                    <TouchDeviceEdgeRenderer store=store />

                    // Render nodes with touch handlers
                    {move || {
                        let on_touchstart = on_node_touchstart.clone();
                        let hs = handle_size.get();
                        store.get_nodes().into_iter().map(move |node| {
                            let node_id = node.id.clone();
                            let on_touchstart = on_touchstart.clone();
                            view! {
                                <TouchDeviceNode
                                    node=node.clone()
                                    store=store
                                    handle_size=hs
                                    on_touchstart=move |ev| on_touchstart(node_id.clone(), ev)
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
                    <div style="background: white; padding: 16px; border-radius: 8px; max-width: 320px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333;">"Touch Device"</h3>

                        // Touch instructions
                        <div style="margin-bottom: 16px; padding: 10px; background: #e3f2fd; border-radius: 6px;">
                            <div style="font-size: 12px; color: #1565c0; line-height: 1.6;">
                                <div style="margin-bottom: 4px;"><strong>"1 finger"</strong>": Drag node"</div>
                                <div style="margin-bottom: 4px;"><strong>"2 fingers"</strong>": Pinch zoom or pan"</div>
                                <div><strong>"Handles"</strong>": Large touch targets"</div>
                            </div>
                        </div>

                        // Handle size control
                        <div style="margin-bottom: 16px; padding: 12px; background: #f8f9fa; border-radius: 6px;">
                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;">
                                <label style="font-size: 13px; font-weight: 500; color: #555;">"Handle Size (px)"</label>
                                <span style="font-size: 14px; font-weight: 600; color: #6865A5; background: #eef; padding: 2px 8px; border-radius: 4px;">
                                    {move || handle_size.get()}
                                </span>
                            </div>
                            <input
                                type="range"
                                min="12"
                                max="40"
                                step="2"
                                style="width: 100%; cursor: pointer;"
                                prop:value=move || handle_size.get()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev).parse::<i32>().unwrap_or(24);
                                    handle_size.set(value);
                                }
                            />
                            <div style="display: flex; justify-content: space-between; font-size: 10px; color: #999; margin-top: 4px;">
                                <span>"12px (small)"</span>
                                <span>"40px (large)"</span>
                            </div>
                        </div>

                        // Statistics
                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 6px; margin-bottom: 16px;">
                            <div style="padding: 10px 6px; background: linear-gradient(135deg, #6ede87 0%, #4CAF50 100%); border-radius: 6px; text-align: center;">
                                <div style="font-size: 20px; font-weight: 700; color: white;">
                                    {move || drag_count.get()}
                                </div>
                                <div style="font-size: 10px; color: rgba(255,255,255,0.9);">"Drags"</div>
                            </div>
                            <div style="padding: 10px 6px; background: linear-gradient(135deg, #6865A5 0%, #5a4f9a 100%); border-radius: 6px; text-align: center;">
                                <div style="font-size: 20px; font-weight: 700; color: white;">
                                    {move || pinch_count.get()}
                                </div>
                                <div style="font-size: 10px; color: rgba(255,255,255,0.9);">"Pinches"</div>
                            </div>
                            <div style="padding: 10px 6px; background: linear-gradient(135deg, #ff6b6b 0%, #e53935 100%); border-radius: 6px; text-align: center;">
                                <div style="font-size: 20px; font-weight: 700; color: white;">
                                    {move || pan_count.get()}
                                </div>
                                <div style="font-size: 10px; color: rgba(255,255,255,0.9);">"Pans"</div>
                            </div>
                        </div>

                        // Current viewport
                        <div style="margin-bottom: 16px; padding: 10px; background: #f3e5f5; border-radius: 6px;">
                            <div style="font-size: 11px; color: #7b1fa2; font-weight: 600; margin-bottom: 4px;">"Viewport"</div>
                            <div style="font-size: 11px; color: #666; font-family: monospace;">
                                {move || {
                                    let vp = store.get_viewport();
                                    format!("x: {:.0}, y: {:.0}, zoom: {:.2}x", vp.x, vp.y, vp.zoom)
                                }}
                            </div>
                        </div>

                        // Touch Event Log
                        <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Touch Events"</div>
                        <div style="max-height: 150px; overflow-y: auto; font-size: 11px; font-family: monospace; background: #fafafa; border-radius: 4px; padding: 8px;">
                            {move || {
                                let logs = touch_log.get();
                                if logs.is_empty() {
                                    view! {
                                        <div style="color: #999; text-align: center; padding: 20px 0;">
                                            "Touch to interact..."
                                        </div>
                                    }.into_any()
                                } else {
                                    logs.iter().rev().map(|event| {
                                        let (bg, color) = match event.event_type.as_str() {
                                            "drag_start" => ("#e8f5e9", "#2e7d32"),
                                            "drag_end" => ("#c8e6c9", "#1b5e20"),
                                            "gesture_start" => ("#e3f2fd", "#1565c0"),
                                            "pinch_end" => ("#ede7f6", "#5e35b1"),
                                            "pan_end" => ("#ffebee", "#c62828"),
                                            _ => ("#fafafa", "#666"),
                                        };
                                        let event_type = event.event_type.clone();
                                        let details = event.details.clone();
                                        view! {
                                            <div style=format!(
                                                "margin-bottom: 4px; padding: 6px 8px; background: {}; border-radius: 4px;",
                                                bg
                                            )>
                                                <div style=format!("display: flex; justify-content: space-between; align-items: center;")>
                                                    <span style=format!("font-weight: 600; color: {}; text-transform: uppercase; font-size: 9px;", color)>
                                                        {event_type}
                                                    </span>
                                                </div>
                                                <div style="color: #666; font-size: 10px; margin-top: 2px;">
                                                    {details}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>

                        // Reset button
                        <button
                            style="width: 100%; margin-top: 12px; padding: 8px; font-size: 12px; cursor: pointer; background: #f5f5f5; border: 1px solid #ddd; border-radius: 4px;"
                            on:click=move |_| {
                                drag_count.set(0);
                                pinch_count.set(0);
                                pan_count.set(0);
                                touch_log.set(vec![]);
                            }
                        >
                            "Reset Statistics"
                        </button>
                    </div>
                </Panel>

                // Touch device indicator
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 8px 16px; border-radius: 20px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <span style="color: white; font-size: 12px; font-weight: 600;">"Touch Optimized"</span>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Touch Device Node Component
// ============================================================================

#[component]
fn TouchDeviceNode<F>(
    node: Node,
    store: FlowStore,
    handle_size: i32,
    on_touchstart: F,
) -> impl IntoView
where
    F: Fn(leptos::ev::TouchEvent) + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_inner = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6865A5")
        .to_string();

    let has_source = node_type != "output";
    let has_target = node_type != "input";

    // Get drag signal to check if this node is being dragged
    let drag_signal = get_touch_drag_signal();

    // Get reactive node position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    let color_for_style = color.clone();
    let color_for_inner = color.clone();

    // Handle style based on size
    let handle_style_source = format!(
        "position: absolute; bottom: -{}px; left: 50%; transform: translateX(-50%); width: {}px; height: {}px; background: {}; border: 2px solid white; border-radius: 50%; cursor: pointer; box-shadow: 0 2px 4px rgba(0,0,0,0.2);",
        handle_size / 2,
        handle_size,
        handle_size,
        color
    );
    let handle_style_target = format!(
        "position: absolute; top: -{}px; left: 50%; transform: translateX(-50%); width: {}px; height: {}px; background: {}; border: 2px solid white; border-radius: 50%; cursor: pointer; box-shadow: 0 2px 4px rgba(0,0,0,0.2);",
        handle_size / 2,
        handle_size,
        handle_size,
        color
    );

    view! {
        <div
            class="xyflow__node"
            style=move || {
                let dragging = drag_signal.get()
                    .map(|s| s.node_id == node_id)
                    .unwrap_or(false);
                format!(
                    "position: absolute; transform: translate({}px, {}px); touch-action: none;{}",
                    pos().x, pos().y,
                    if dragging { " z-index: 1000;" } else { "" }
                )
            }
            on:touchstart=on_touchstart
        >
            <div
                class="xyflow__node-default light"
                style=move || {
                    let dragging = drag_signal.get()
                        .map(|s| s.node_id == node_id_for_inner)
                        .unwrap_or(false);
                    format!(
                        "background: {}; border: 2px solid {}; border-radius: 8px; padding: 14px 20px; min-width: 100px; text-align: center; transition: box-shadow 0.2s, transform 0.2s;{}",
                        color_for_inner,
                        if dragging { "#333" } else { &color_for_style },
                        if dragging { " box-shadow: 0 8px 20px rgba(0,0,0,0.3); transform: scale(1.05);" } else { "" }
                    )
                }
            >
                // Target handle (top) - larger touch target
                {has_target.then(|| {
                    let style = handle_style_target.clone();
                    view! {
                        <div style=style>
                            <Handle
                                node_id=node.id.clone()
                                r#type=HandleType::Target
                                position=HandlePosition::Top
                                connection_mode=ConnectionMode::Strict
                                style=format!("width: 100%; height: 100%; opacity: 0;")
                            />
                        </div>
                    }
                })}

                <div style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 14px;">
                    {label}
                </div>

                // Touch indicator
                {
                    let node_id_for_indicator = node.id.clone();
                    view! {
                        <div style="font-size: 10px; color: rgba(255,255,255,0.7); margin-top: 4px;">
                            {move || {
                                let dragging = drag_signal.get()
                                    .map(|s| s.node_id == node_id_for_indicator)
                                    .unwrap_or(false);
                                if dragging { "Dragging..." } else { "Touch to drag" }
                            }}
                        </div>
                    }
                }

                // Source handle (bottom) - larger touch target
                {
                    let node_id_for_source = node.id.clone();
                    has_source.then(|| {
                        let style = handle_style_source.clone();
                        view! {
                            <div style=style>
                                <Handle
                                    node_id=node_id_for_source.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Bottom
                                    connection_mode=ConnectionMode::Strict
                                    style=format!("width: 100%; height: 100%; opacity: 0;")
                                />
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

#[component]
fn TouchDeviceEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="touch-device-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#6ede87;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#6865A5;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="touch-device-arrow"
                    markerWidth="12"
                    markerHeight="12"
                    refX="10"
                    refY="6"
                    orient="auto"
                    markerUnits="userSpaceOnUse"
                >
                    <path d="M2,2 L10,6 L2,10 L4,6 Z" fill="#6865A5" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.iter().map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        let source_x = source.position.x + 60.0;
                        let source_y = source.position.y + 55.0;
                        let target_x = target.position.x + 60.0;
                        let target_y = target.position.y;

                        let ctrl_offset = (target_y - source_y).abs() * 0.5;
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
                                stroke="url(#touch-device-edge-gradient)"
                                stroke-width="3"
                                marker-end="url(#touch-device-arrow)"
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
