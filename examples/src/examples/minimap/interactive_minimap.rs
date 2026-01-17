//! Interactive MiniMap Example
//!
//! Demonstrates an interactive minimap with:
//! - Click on minimap to pan viewport
//! - Drag viewport indicator to pan
//! - Scroll to zoom on minimap (optional)
//! - Visual feedback during interactions

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static INTERACTIVE_MINIMAP_DRAG_STATE: OnceLock<RwSignal<Option<InteractiveMinimapDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct InteractiveMinimapDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_drag_signal() -> RwSignal<Option<InteractiveMinimapDragState>> {
    *INTERACTIVE_MINIMAP_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Minimap Interaction State
// ============================================================================

static MINIMAP_INTERACTION_STATE: OnceLock<RwSignal<Option<MinimapInteractionState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct MinimapInteractionState {
    interaction_type: MinimapInteractionType,
    start_mouse: (f64, f64),
    start_viewport: (f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
enum MinimapInteractionType {
    ViewportDrag,
    MinimapClick,
}

fn get_minimap_interaction_signal() -> RwSignal<Option<MinimapInteractionState>> {
    *MINIMAP_INTERACTION_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Action Log State
// ============================================================================

static INTERACTIVE_MINIMAP_LOG: OnceLock<RwSignal<Vec<String>>> = OnceLock::new();

fn get_log_signal() -> RwSignal<Vec<String>> {
    *INTERACTIVE_MINIMAP_LOG.get_or_init(|| RwSignal::new(Vec::new()))
}

fn add_log(message: &str) {
    let log = get_log_signal();
    let now = js_sys::Date::new_0();
    let timestamp = format!(
        "{:02}:{:02}:{:02}",
        now.get_hours(),
        now.get_minutes(),
        now.get_seconds()
    );
    log.update(|logs| {
        logs.insert(0, format!("[{}] {}", timestamp, message));
        if logs.len() > 10 {
            logs.pop();
        }
    });
}

// ============================================================================
// Statistics State
// ============================================================================

static MINIMAP_STATS: OnceLock<RwSignal<MinimapStats>> = OnceLock::new();

#[derive(Clone, Debug, Default)]
struct MinimapStats {
    click_count: i32,
    drag_count: i32,
    scroll_zoom_count: i32,
}

fn get_stats_signal() -> RwSignal<MinimapStats> {
    *MINIMAP_STATS.get_or_init(|| RwSignal::new(MinimapStats::default()))
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
        _ => "#5bc0de",        // Blue (default)
    }
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Interactive MiniMap Example - Click/drag to pan viewport
#[component]
pub fn InteractiveMinimapExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("node-1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Node A", "type": "input"})),
        Node::new("node-2".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({"label": "Node B", "type": "process"})),
        Node::new("node-3".to_string(), Position::new(450.0, 50.0))
            .with_data(json!({"label": "Node C", "type": "default"})),
        Node::new("node-4".to_string(), Position::new(150.0, 200.0))
            .with_data(json!({"label": "Node D", "type": "process"})),
        Node::new("node-5".to_string(), Position::new(350.0, 200.0))
            .with_data(json!({"label": "Node E", "type": "default"})),
        Node::new("node-6".to_string(), Position::new(550.0, 200.0))
            .with_data(json!({"label": "Node F", "type": "output"})),
        // Some nodes further away
        Node::new("node-7".to_string(), Position::new(-150.0, 350.0))
            .with_data(json!({"label": "Node G", "type": "input"})),
        Node::new("node-8".to_string(), Position::new(700.0, 350.0))
            .with_data(json!({"label": "Node H", "type": "output"})),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "node-1".to_string(), "node-2".to_string()),
        Edge::new("e2-3".to_string(), "node-2".to_string(), "node-3".to_string()),
        Edge::new("e1-4".to_string(), "node-1".to_string(), "node-4".to_string()),
        Edge::new("e4-5".to_string(), "node-4".to_string(), "node-5".to_string()),
        Edge::new("e5-6".to_string(), "node-5".to_string(), "node-6".to_string()),
        Edge::new("e3-6".to_string(), "node-3".to_string(), "node-6".to_string()),
        Edge::new("e7-4".to_string(), "node-7".to_string(), "node-4".to_string()),
        Edge::new("e6-8".to_string(), "node-6".to_string(), "node-8".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Signals
    let drag_signal = get_drag_signal();
    let log_signal = get_log_signal();
    let stats_signal = get_stats_signal();

    // Container dimensions for viewport calculations
    let container_width = RwSignal::new(800.0_f64);
    let container_height = RwSignal::new(600.0_f64);

    // Mouse move handler for node dragging
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

    // Clear log handler
    let on_clear_log = move |_| {
        log_signal.set(Vec::new());
        add_log("Log cleared");
    };

    // Reset stats handler
    let on_reset_stats = move |_| {
        stats_signal.set(MinimapStats::default());
        add_log("Stats reset");
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow interactive-minimap-example"
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
                    <InteractiveMinimapEdgeRenderer store=store />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <InteractiveMinimapNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // Interactive MiniMap
                <InteractiveMiniMap
                    store=store
                    container_width=container_width
                    container_height=container_height
                />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; max-width: 320px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333; display: flex; align-items: center; gap: 8px;">
                            <span style="display: inline-block; width: 8px; height: 8px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 50%;"></span>
                            "Interactive MiniMap"
                        </h3>

                        // Current Viewport State
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Current Viewport"</div>
                            <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                                {move || {
                                    let viewport = store.get_viewport();
                                    view! {
                                        <div style="background: #f5f5f5; padding: 8px; border-radius: 6px; text-align: center;">
                                            <div style="font-size: 10px; color: #888;">"X"</div>
                                            <div style="font-size: 13px; font-weight: 600; color: #667eea;">{format!("{:.0}", viewport.x)}</div>
                                        </div>
                                        <div style="background: #f5f5f5; padding: 8px; border-radius: 6px; text-align: center;">
                                            <div style="font-size: 10px; color: #888;">"Y"</div>
                                            <div style="font-size: 13px; font-weight: 600; color: #764ba2;">{format!("{:.0}", viewport.y)}</div>
                                        </div>
                                        <div style="background: #f5f5f5; padding: 8px; border-radius: 6px; text-align: center;">
                                            <div style="font-size: 10px; color: #888;">"Zoom"</div>
                                            <div style="font-size: 13px; font-weight: 600; color: #333;">{format!("{:.2}x", viewport.zoom)}</div>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>

                        // Statistics
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"MiniMap Interactions"</div>
                            <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                                <div style="background: #e8f5e9; padding: 8px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 10px; color: #2e7d32;">"Clicks"</div>
                                    <div style="font-size: 16px; font-weight: 600; color: #2e7d32;">{move || stats_signal.get().click_count}</div>
                                </div>
                                <div style="background: #e3f2fd; padding: 8px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 10px; color: #1565c0;">"Drags"</div>
                                    <div style="font-size: 16px; font-weight: 600; color: #1565c0;">{move || stats_signal.get().drag_count}</div>
                                </div>
                                <div style="background: #f3e5f5; padding: 8px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 10px; color: #7b1fa2;">"Zooms"</div>
                                    <div style="font-size: 16px; font-weight: 600; color: #7b1fa2;">{move || stats_signal.get().scroll_zoom_count}</div>
                                </div>
                            </div>
                        </div>

                        // Quick Actions
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Quick Actions"</div>
                            <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                                <button
                                    style="flex: 1; min-width: 80px; padding: 8px; background: #667eea; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 11px; font-weight: 500;"
                                    on:click=move |_| {
                                        store.set_viewport(Viewport { x: 0.0, y: 0.0, zoom: 1.0 });
                                        add_log("Reset viewport to origin");
                                    }
                                >
                                    "Reset View"
                                </button>
                                <button
                                    style="flex: 1; min-width: 80px; padding: 8px; background: #764ba2; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 11px; font-weight: 500;"
                                    on:click=move |_| {
                                        store.set_viewport(Viewport { x: 150.0, y: 100.0, zoom: 0.7 });
                                        add_log("Fit view to see all nodes");
                                    }
                                >
                                    "Fit All"
                                </button>
                                <button
                                    style="flex: 1; min-width: 80px; padding: 8px; background: #f5f5f5; color: #333; border: 1px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 11px; font-weight: 500;"
                                    on:click=on_reset_stats
                                >
                                    "Reset Stats"
                                </button>
                            </div>
                        </div>

                        // Event Log
                        <div style="margin-bottom: 12px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                <div style="font-size: 12px; font-weight: 600; color: #555;">"Event Log"</div>
                                <button
                                    style="padding: 4px 8px; background: #f5f5f5; color: #666; border: 1px solid #ddd; border-radius: 4px; cursor: pointer; font-size: 10px;"
                                    on:click=on_clear_log
                                >
                                    "Clear"
                                </button>
                            </div>
                            <div style="max-height: 120px; overflow-y: auto; font-size: 11px; font-family: monospace; background: #fafafa; padding: 8px; border-radius: 6px; border: 1px solid #eee;">
                                {move || {
                                    let logs = log_signal.get();
                                    if logs.is_empty() {
                                        view! {
                                            <div style="color: #999; font-style: italic;">"No events yet. Try interacting with the minimap!"</div>
                                        }.into_any()
                                    } else {
                                        logs.iter().map(|log| {
                                            let is_click = log.contains("Click");
                                            let is_drag = log.contains("Drag") || log.contains("drag");
                                            let is_zoom = log.contains("Zoom") || log.contains("zoom");
                                            let color = if is_click { "#2e7d32" } else if is_drag { "#1565c0" } else if is_zoom { "#7b1fa2" } else { "#666" };
                                            view! {
                                                <div style=format!("padding: 2px 0; color: {}; border-bottom: 1px solid #eee;", color)>
                                                    {log.clone()}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </Panel>

                // Instructions badge
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 12px 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <div style="color: white; font-size: 12px; line-height: 1.6;">
                            <div style="font-weight: 600; margin-bottom: 6px; display: flex; align-items: center; gap: 6px;">
                                <span style="font-size: 14px;">"🗺️"</span>
                                "Interactive MiniMap"
                            </div>
                            <div style="opacity: 0.95;">
                                <div>"• "<strong>"Click"</strong>" anywhere on minimap to pan"</div>
                                <div>"• "<strong>"Drag"</strong>" the viewport box to pan"</div>
                                <div>"• "<strong>"Scroll"</strong>" on minimap to zoom"</div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Interactive MiniMap Component
// ============================================================================

#[component]
fn InteractiveMiniMap(
    store: FlowStore,
    container_width: RwSignal<f64>,
    container_height: RwSignal<f64>,
) -> impl IntoView {
    // Minimap dimensions
    let width: f64 = 250.0;
    let height: f64 = 170.0;

    // Interaction signals
    let minimap_interaction = get_minimap_interaction_signal();
    let stats = get_stats_signal();

    // Calculate bounds of all nodes with padding
    let bounds = move || {
        let nodes = store.get_nodes();
        if nodes.is_empty() {
            return (-100.0, -100.0, 800.0, 600.0);
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
        let padding = 100.0;
        (min_x - padding, min_y - padding, max_x + padding, max_y + padding)
    };

    // Calculate scale from flow coordinates to minimap coordinates
    let get_scale = move || {
        let (min_x, min_y, max_x, max_y) = bounds();
        let flow_width = max_x - min_x;
        let flow_height = max_y - min_y;
        (width / flow_width, height / flow_height)
    };

    // Convert minimap coordinates to flow coordinates
    let minimap_to_flow = move |minimap_x: f64, minimap_y: f64| -> (f64, f64) {
        let (min_x, min_y, max_x, max_y) = bounds();
        let flow_width = max_x - min_x;
        let flow_height = max_y - min_y;

        let flow_x = min_x + (minimap_x / width) * flow_width;
        let flow_y = min_y + (minimap_y / height) * flow_height;

        (flow_x, flow_y)
    };

    // Click on minimap to pan
    let on_minimap_click = move |ev: leptos::ev::MouseEvent| {
        // Get click position using offset coordinates (relative to target element)
        // This works directly when clicking on the SVG
        let minimap_x = ev.offset_x() as f64;
        let minimap_y = ev.offset_y() as f64;

        // Convert to flow coordinates
        let (flow_x, flow_y) = minimap_to_flow(minimap_x, minimap_y);

        // Calculate viewport offset to center on clicked position
        let viewport = store.get_viewport();
        let cw = container_width.get();
        let ch = container_height.get();

        // We want the clicked point to be at the center of the viewport
        let new_x = -(flow_x - (cw / 2.0) / viewport.zoom);
        let new_y = -(flow_y - (ch / 2.0) / viewport.zoom);

        store.set_viewport(Viewport {
            x: new_x,
            y: new_y,
            zoom: viewport.zoom,
        });

        stats.update(|s| s.click_count += 1);
        add_log(&format!("Click pan to ({:.0}, {:.0})", flow_x, flow_y));
    };

    // Start viewport drag
    let on_viewport_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let viewport = store.get_viewport();
        minimap_interaction.set(Some(MinimapInteractionState {
            interaction_type: MinimapInteractionType::ViewportDrag,
            start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
            start_viewport: (viewport.x, viewport.y),
        }));

        add_log("Started viewport drag");
    };

    // Mouse move for viewport drag
    let on_minimap_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(interaction) = minimap_interaction.get() {
            if interaction.interaction_type == MinimapInteractionType::ViewportDrag {
                let dx = ev.client_x() as f64 - interaction.start_mouse.0;
                let dy = ev.client_y() as f64 - interaction.start_mouse.1;

                // Scale the mouse movement to flow coordinates
                let (scale_x, scale_y) = get_scale();
                let viewport = store.get_viewport();

                // Minimap movement is inverse of viewport movement
                let flow_dx = -dx / scale_x;
                let flow_dy = -dy / scale_y;

                store.set_viewport(Viewport {
                    x: interaction.start_viewport.0 + flow_dx * viewport.zoom,
                    y: interaction.start_viewport.1 + flow_dy * viewport.zoom,
                    zoom: viewport.zoom,
                });
            }
        }
    };

    // Mouse up to end drag
    let on_minimap_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(interaction) = minimap_interaction.get() {
            if interaction.interaction_type == MinimapInteractionType::ViewportDrag {
                stats.update(|s| s.drag_count += 1);
                add_log("Ended viewport drag");
            }
        }
        minimap_interaction.set(None);
    };

    // Scroll to zoom
    let on_minimap_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();

        let viewport = store.get_viewport();
        let delta = ev.delta_y();
        let zoom_factor = if delta > 0.0 { 0.9 } else { 1.1 };
        let new_zoom = (viewport.zoom * zoom_factor).clamp(0.1, 4.0);

        store.set_viewport(Viewport {
            x: viewport.x,
            y: viewport.y,
            zoom: new_zoom,
        });

        stats.update(|s| s.scroll_zoom_count += 1);
        add_log(&format!("Scroll zoom to {:.2}x", new_zoom));
    };

    view! {
        <div
            class="xyflow__minimap xyflow__panel bottom right"
            style=format!(
                "width: {}px; height: {}px; background: white; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); overflow: hidden; padding: 12px;",
                width as i32 + 24, height as i32 + 36
            )
            on:mousemove=on_minimap_mousemove
            on:mouseup=on_minimap_mouseup
            on:mouseleave=move |_| {
                minimap_interaction.set(None);
            }
        >
            // MiniMap title with interaction indicator
            <div style="font-size: 10px; font-weight: 600; color: #888; margin-bottom: 8px; display: flex; justify-content: space-between; align-items: center;">
                <span style="text-transform: uppercase; letter-spacing: 0.5px;">"Interactive MiniMap"</span>
                {move || {
                    if minimap_interaction.get().is_some() {
                        view! {
                            <span style="font-size: 9px; background: #667eea; color: white; padding: 2px 6px; border-radius: 4px;">"Dragging"</span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>

            <svg
                width=width as i32
                height=height as i32
                style="background: #fafafa; border-radius: 6px; cursor: crosshair; border: 1px solid #e0e0e0;"
                viewBox=move || {
                    let (min_x, min_y, max_x, max_y) = bounds();
                    format!("{} {} {} {}", min_x, min_y, max_x - min_x, max_y - min_y)
                }
                on:click=on_minimap_click
                on:wheel=on_minimap_wheel
            >
                // Render minimap nodes
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

                        view! {
                            <rect
                                x=x
                                y=y
                                width=w
                                height=h
                                rx="4"
                                ry="4"
                                fill=color
                                stroke="white"
                                stroke-width="2"
                                style="pointer-events: none;"
                            />
                        }
                    }).collect_view()
                }}

                // Render viewport indicator (draggable)
                {move || {
                    let viewport = store.get_viewport();
                    let (min_x, min_y, _, _) = bounds();
                    let cw = container_width.get();
                    let ch = container_height.get();

                    // Calculate visible area in flow coordinates
                    let visible_x = -viewport.x / viewport.zoom + min_x;
                    let visible_y = -viewport.y / viewport.zoom + min_y;
                    let visible_width = cw / viewport.zoom;
                    let visible_height = ch / viewport.zoom;

                    let is_dragging = minimap_interaction.get()
                        .map(|i| i.interaction_type == MinimapInteractionType::ViewportDrag)
                        .unwrap_or(false);

                    let stroke_color = if is_dragging { "#ff6b6b" } else { "#667eea" };
                    let stroke_width = if is_dragging { 4 } else { 3 };

                    view! {
                        <g>
                            // Shadow/glow effect
                            <rect
                                x=visible_x
                                y=visible_y
                                width=visible_width
                                height=visible_height
                                fill="none"
                                stroke="rgba(102, 126, 234, 0.3)"
                                stroke-width="8"
                                rx="4"
                                style="pointer-events: none;"
                            />
                            // Main viewport indicator
                            <rect
                                x=visible_x
                                y=visible_y
                                width=visible_width
                                height=visible_height
                                fill="rgba(102, 126, 234, 0.1)"
                                stroke=stroke_color
                                stroke-width=stroke_width
                                rx="4"
                                style="cursor: move; pointer-events: all;"
                                on:mousedown=on_viewport_mousedown
                            />
                            // Drag handle in center
                            <circle
                                cx=visible_x + visible_width / 2.0
                                cy=visible_y + visible_height / 2.0
                                r="8"
                                fill=stroke_color
                                stroke="white"
                                stroke-width="2"
                                style="cursor: move; pointer-events: all;"
                                on:mousedown=on_viewport_mousedown
                            />
                        </g>
                    }
                }}
            </svg>
        </div>
    }
}

// ============================================================================
// Node Component
// ============================================================================

#[component]
fn InteractiveMinimapNode(node: Node, store: FlowStore) -> impl IntoView {
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
            drag_signal.set(Some(InteractiveMinimapDragState {
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

    // Determine handles
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
fn InteractiveMinimapEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="interactive-minimap-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="interactive-minimap-arrow"
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
                                stroke="url(#interactive-minimap-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#interactive-minimap-arrow)"
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
