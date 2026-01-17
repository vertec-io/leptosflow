//! Intersection Example
//!
//! Demonstrates how to detect when nodes enter/exit the viewport:
//! - Use viewport bounds checking to determine visibility
//! - Highlight nodes currently visible in viewport
//! - Show visibility status in panel

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static INTERSECTION_DRAG_STATE: OnceLock<RwSignal<Option<IntersectionDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct IntersectionDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_drag_signal() -> RwSignal<Option<IntersectionDragState>> {
    *INTERSECTION_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Visibility Event Log
// ============================================================================

#[derive(Clone, Debug)]
struct VisibilityEvent {
    timestamp: f64,
    node_id: String,
    node_label: String,
    event_type: String, // "entered" or "exited"
}

// ============================================================================
// Intersection checking utilities
// ============================================================================

/// Check if a node intersects with the visible viewport
fn is_node_in_viewport(
    node: &Node,
    viewport: &Viewport,
    container_width: f64,
    container_height: f64,
) -> bool {
    let node_width = node.width.unwrap_or(120.0);
    let node_height = node.height.unwrap_or(60.0);

    // Calculate the viewport bounds in flow coordinates
    // The viewport transform is: screen_pos = (flow_pos * zoom) + offset
    // So flow_pos = (screen_pos - offset) / zoom
    let viewport_left = -viewport.x / viewport.zoom;
    let viewport_top = -viewport.y / viewport.zoom;
    let viewport_right = (container_width - viewport.x) / viewport.zoom;
    let viewport_bottom = (container_height - viewport.y) / viewport.zoom;

    // Check if node bounds intersect with viewport bounds
    let node_left = node.position.x;
    let node_top = node.position.y;
    let node_right = node.position.x + node_width;
    let node_bottom = node.position.y + node_height;

    // Rectangles intersect if they overlap on both axes
    node_right > viewport_left
        && node_left < viewport_right
        && node_bottom > viewport_top
        && node_top < viewport_bottom
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Intersection Example
#[component]
pub fn IntersectionExample() -> impl IntoView {
    // Viewport state
    let viewport_x = RwSignal::new(0.0_f64);
    let viewport_y = RwSignal::new(0.0_f64);
    let viewport_zoom = RwSignal::new(1.0_f64);

    // Container dimensions (will be updated on mount)
    let container_width = RwSignal::new(800.0_f64);
    let container_height = RwSignal::new(600.0_f64);

    // Visibility events log
    let visibility_events = RwSignal::new(Vec::<VisibilityEvent>::new());

    // Previous visibility state for change detection
    let prev_visible_nodes = RwSignal::new(std::collections::HashSet::<String>::new());

    // Helper to add log entry
    let add_visibility_event = move |node_id: String, node_label: String, event_type: String| {
        visibility_events.update(|events| {
            events.push(VisibilityEvent {
                timestamp: js_sys::Date::now(),
                node_id,
                node_label,
                event_type,
            });
            // Keep last 20 entries
            if events.len() > 20 {
                events.remove(0);
            }
        });
    };

    // Create initial nodes - spread them out so some are outside initial viewport
    let initial_nodes = vec![
        // Nodes in initial viewport
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Node A", "color": "#6ede87"})),
        Node::new("2".to_string(), Position::new(250.0, 100.0))
            .with_data(json!({"label": "Node B", "color": "#6865A5"})),
        Node::new("3".to_string(), Position::new(100.0, 250.0))
            .with_data(json!({"label": "Node C", "color": "#6865A5"})),
        Node::new("4".to_string(), Position::new(250.0, 250.0))
            .with_data(json!({"label": "Node D", "color": "#ff6b6b"})),

        // Nodes outside initial viewport (to the right)
        Node::new("5".to_string(), Position::new(700.0, 100.0))
            .with_data(json!({"label": "Node E", "color": "#f0ad4e"})),
        Node::new("6".to_string(), Position::new(850.0, 200.0))
            .with_data(json!({"label": "Node F", "color": "#f0ad4e"})),

        // Nodes outside initial viewport (below)
        Node::new("7".to_string(), Position::new(100.0, 500.0))
            .with_data(json!({"label": "Node G", "color": "#5bc0de"})),
        Node::new("8".to_string(), Position::new(250.0, 550.0))
            .with_data(json!({"label": "Node H", "color": "#5bc0de"})),

        // Nodes far outside (bottom-right corner)
        Node::new("9".to_string(), Position::new(700.0, 500.0))
            .with_data(json!({"label": "Node I", "color": "#d9534f"})),

        // Node far to the left (negative position)
        Node::new("10".to_string(), Position::new(-200.0, 200.0))
            .with_data(json!({"label": "Node J", "color": "#337ab7"})),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
        Edge::new("e4-5".to_string(), "4".to_string(), "5".to_string()),
        Edge::new("e5-6".to_string(), "5".to_string(), "6".to_string()),
        Edge::new("e3-7".to_string(), "3".to_string(), "7".to_string()),
        Edge::new("e7-8".to_string(), "7".to_string(), "8".to_string()),
        Edge::new("e6-9".to_string(), "6".to_string(), "9".to_string()),
        Edge::new("e8-9".to_string(), "8".to_string(), "9".to_string()),
        Edge::new("e10-1".to_string(), "10".to_string(), "1".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Sync viewport state to store
    Effect::new(move |_| {
        let x = viewport_x.get();
        let y = viewport_y.get();
        let zoom = viewport_zoom.get();
        store.set_viewport(Viewport { x, y, zoom });
    });

    // Track visibility changes
    let add_event = add_visibility_event.clone();
    Effect::new(move |_| {
        let nodes = store.get_nodes();
        let viewport = Viewport {
            x: viewport_x.get(),
            y: viewport_y.get(),
            zoom: viewport_zoom.get(),
        };
        let width = container_width.get();
        let height = container_height.get();

        // Calculate currently visible nodes
        let mut current_visible = std::collections::HashSet::<String>::new();
        for node in &nodes {
            if is_node_in_viewport(node, &viewport, width, height) {
                current_visible.insert(node.id.clone());
            }
        }

        // Get previous state
        let previous = prev_visible_nodes.get();

        // Find newly visible nodes (entered)
        for node_id in &current_visible {
            if !previous.contains(node_id) {
                if let Some(node) = nodes.iter().find(|n| &n.id == node_id) {
                    let label = node.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();
                    add_event(node_id.clone(), label, "entered".to_string());
                }
            }
        }

        // Find nodes that left viewport (exited)
        for node_id in &previous {
            if !current_visible.contains(node_id) {
                if let Some(node) = nodes.iter().find(|n| &n.id == node_id) {
                    let label = node.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();
                    add_event(node_id.clone(), label, "exited".to_string());
                }
            }
        }

        // Update previous state
        prev_visible_nodes.set(current_visible);
    });

    // Drag signal
    let drag_signal = get_drag_signal();

    // Pan state for viewport dragging
    let is_panning = RwSignal::new(false);
    let pan_start = RwSignal::new((0.0_f64, 0.0_f64));
    let pan_start_viewport = RwSignal::new((0.0_f64, 0.0_f64));

    // Mouse down handler for canvas (start panning)
    let on_canvas_mousedown = move |ev: leptos::ev::MouseEvent| {
        // Pan on middle-click or when clicking empty space (left-click with shift)
        if ev.button() == 1 || (ev.button() == 0 && ev.shift_key()) {
            ev.prevent_default();
            is_panning.set(true);
            pan_start.set((ev.client_x() as f64, ev.client_y() as f64));
            pan_start_viewport.set((viewport_x.get(), viewport_y.get()));
        }
    };

    // Mouse move handler
    let on_canvas_mousemove = move |ev: leptos::ev::MouseEvent| {
        // Handle node drag
        if let Some(drag_state) = drag_signal.get() {
            let zoom = viewport_zoom.get();
            let dx = (ev.client_x() as f64 - drag_state.start_mouse.0) / zoom;
            let dy = (ev.client_y() as f64 - drag_state.start_mouse.1) / zoom;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(drag_state.start_pos.0 + dx, drag_state.start_pos.1 + dy);
            });
        }

        // Handle panning
        if is_panning.get() {
            let (start_x, start_y) = pan_start.get();
            let (vp_start_x, vp_start_y) = pan_start_viewport.get();

            let dx = ev.client_x() as f64 - start_x;
            let dy = ev.client_y() as f64 - start_y;

            viewport_x.set(vp_start_x + dx);
            viewport_y.set(vp_start_y + dy);
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

        if is_panning.get() {
            is_panning.set(false);
        }
    };

    // Mouse wheel handler for zoom
    let on_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();

        let delta = if ev.delta_y() > 0.0 { -0.1 } else { 0.1 };
        let new_zoom = (viewport_zoom.get() + delta).clamp(0.1, 4.0);
        viewport_zoom.set(new_zoom);
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow intersection-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousedown=on_canvas_mousedown
                 on:mousemove=on_canvas_mousemove
                 on:mouseup=on_canvas_mouseup
                 on:mouseleave=move |_| {
                     if is_panning.get() {
                         is_panning.set(false);
                     }
                     if drag_signal.get().is_some() {
                         if let Some(ds) = drag_signal.get() {
                             store.update_node(&ds.node_id, |n| n.dragging = false);
                         }
                         drag_signal.set(None);
                     }
                 }
                 on:wheel=on_wheel
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Render edges
                    <IntersectionEdgeRenderer store=store />

                    // Render nodes
                    {move || {
                        let nodes = store.get_nodes();
                        let viewport = Viewport {
                            x: viewport_x.get(),
                            y: viewport_y.get(),
                            zoom: viewport_zoom.get(),
                        };
                        let width = container_width.get();
                        let height = container_height.get();

                        nodes.into_iter().map(|node| {
                            let is_visible = is_node_in_viewport(&node, &viewport, width, height);
                            view! {
                                <IntersectionNode
                                    node=node.clone()
                                    store=store
                                    is_visible=is_visible
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
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333; display: flex; align-items: center; gap: 8px;">
                            <span style="display: inline-block; width: 8px; height: 8px; background: #667eea; border-radius: 50%;"></span>
                            "Viewport Intersection"
                        </h3>

                        // Current viewport state
                        <div style="margin-bottom: 16px; padding: 12px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px; color: white;">
                            <div style="font-size: 11px; opacity: 0.8; margin-bottom: 6px;">"Current Viewport"</div>
                            <div style="font-family: monospace; font-size: 13px; display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                                <div>
                                    <div style="font-size: 10px; opacity: 0.7;">"X"</div>
                                    <div style="font-weight: 600;">{move || format!("{:.0}", viewport_x.get())}</div>
                                </div>
                                <div>
                                    <div style="font-size: 10px; opacity: 0.7;">"Y"</div>
                                    <div style="font-weight: 600;">{move || format!("{:.0}", viewport_y.get())}</div>
                                </div>
                                <div>
                                    <div style="font-size: 10px; opacity: 0.7;">"Zoom"</div>
                                    <div style="font-weight: 600;">{move || format!("{:.2}x", viewport_zoom.get())}</div>
                                </div>
                            </div>
                        </div>

                        // Visibility Summary
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Visibility Status"</div>
                            {move || {
                                let nodes = store.get_nodes();
                                let viewport = Viewport {
                                    x: viewport_x.get(),
                                    y: viewport_y.get(),
                                    zoom: viewport_zoom.get(),
                                };
                                let width = container_width.get();
                                let height = container_height.get();

                                let visible_count = nodes.iter()
                                    .filter(|n| is_node_in_viewport(n, &viewport, width, height))
                                    .count();
                                let total_count = nodes.len();

                                view! {
                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 8px;">
                                        <div style="background: #e8f5e9; padding: 8px 12px; border-radius: 6px; text-align: center;">
                                            <div style="font-size: 20px; font-weight: 700; color: #2e7d32;">
                                                {visible_count}
                                            </div>
                                            <div style="font-size: 10px; color: #4caf50;">"Visible"</div>
                                        </div>
                                        <div style="background: #fafafa; padding: 8px 12px; border-radius: 6px; text-align: center;">
                                            <div style="font-size: 20px; font-weight: 700; color: #666;">
                                                {total_count - visible_count}
                                            </div>
                                            <div style="font-size: 10px; color: #999;">"Hidden"</div>
                                        </div>
                                    </div>
                                }
                            }}
                        </div>

                        // Node list with visibility status
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Node Status"</div>
                            <div style="max-height: 150px; overflow-y: auto; border: 1px solid #eee; border-radius: 6px;">
                                {move || {
                                    let nodes = store.get_nodes();
                                    let viewport = Viewport {
                                        x: viewport_x.get(),
                                        y: viewport_y.get(),
                                        zoom: viewport_zoom.get(),
                                    };
                                    let width = container_width.get();
                                    let height = container_height.get();

                                    nodes.into_iter().map(|node| {
                                        let is_visible = is_node_in_viewport(&node, &viewport, width, height);
                                        let label = node.data.get("label")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Node")
                                            .to_string();
                                        let color = node.data.get("color")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("#666")
                                            .to_string();

                                        view! {
                                            <div style=format!(
                                                "display: flex; align-items: center; justify-content: space-between; padding: 6px 10px; border-bottom: 1px solid #f0f0f0; background: {};",
                                                if is_visible { "#f0fff0" } else { "white" }
                                            )>
                                                <div style="display: flex; align-items: center; gap: 6px;">
                                                    <span style=format!(
                                                        "display: inline-block; width: 8px; height: 8px; background: {}; border-radius: 2px;",
                                                        color
                                                    )></span>
                                                    <span style="font-size: 12px; color: #333;">{label}</span>
                                                </div>
                                                <span style=format!(
                                                    "font-size: 10px; padding: 2px 6px; border-radius: 10px; background: {}; color: {};",
                                                    if is_visible { "#c8e6c9" } else { "#e0e0e0" },
                                                    if is_visible { "#2e7d32" } else { "#757575" }
                                                )>
                                                    {if is_visible { "visible" } else { "hidden" }}
                                                </span>
                                            </div>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        </div>

                        // Visibility Event Log
                        <div>
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Event Log"</div>
                            <div style="max-height: 120px; overflow-y: auto; font-size: 11px; font-family: monospace; background: #fafafa; border-radius: 4px; padding: 8px;">
                                {move || {
                                    let events = visibility_events.get();
                                    if events.is_empty() {
                                        view! {
                                            <div style="color: #999; text-align: center; padding: 12px 0;">
                                                "Pan or zoom to see events..."
                                            </div>
                                        }.into_any()
                                    } else {
                                        events.iter().rev().map(|event| {
                                            let (icon, bg, color) = if event.event_type == "entered" {
                                                ("→", "#e8f5e9", "#2e7d32")
                                            } else {
                                                ("←", "#ffebee", "#c62828")
                                            };
                                            let node_label = event.node_label.clone();
                                            let event_type = event.event_type.clone();
                                            view! {
                                                <div style=format!(
                                                    "margin-bottom: 4px; padding: 4px 6px; background: {}; border-radius: 3px; display: flex; align-items: center; gap: 6px;",
                                                    bg
                                                )>
                                                    <span style=format!("color: {}; font-weight: 600;", color)>
                                                        {icon}
                                                    </span>
                                                    <span style="color: #333; font-weight: 500;">
                                                        {node_label}
                                                    </span>
                                                    <span style=format!("color: {}; font-size: 10px;", color)>
                                                        {event_type}
                                                    </span>
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
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 10px 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <div style="color: white; font-size: 11px; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 4px;">"Viewport Intersection"</div>
                            <div style="opacity: 0.9;">"• Visible nodes have green glow"</div>
                            <div style="opacity: 0.9;">"• Scroll to zoom"</div>
                            <div style="opacity: 0.9;">"• Middle-click or Shift+drag to pan"</div>
                            <div style="opacity: 0.9;">"• Drag nodes to move them"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Intersection Node Component
// ============================================================================

#[component]
fn IntersectionNode(
    node: Node,
    store: FlowStore,
    is_visible: bool,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6865A5")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down handler
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(IntersectionDragState {
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

    let color_for_border = color.clone();
    let color_for_shadow = color.clone();

    // Visibility glow effect
    let glow_style = if is_visible {
        format!(
            "box-shadow: 0 0 12px 4px {}66, 0 2px 8px rgba(0,0,0,0.15);",
            color_for_shadow
        )
    } else {
        "box-shadow: 0 2px 4px rgba(0,0,0,0.1); opacity: 0.7;".to_string()
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab; transition: box-shadow 0.2s ease, opacity 0.2s ease;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
        >
            <div
                class="xyflow__node-default light"
                style=format!(
                    "background: {}; border: 2px solid {}; border-radius: 8px; padding: 12px 18px; min-width: 80px; text-align: center; {}",
                    color, color_for_border, glow_style
                )
            >
                // Target handle
                <Handle
                    node_id=node.id.clone()
                    r#type=HandleType::Target
                    position=HandlePosition::Top
                    connection_mode=ConnectionMode::Strict
                />

                <div style="display: flex; align-items: center; gap: 6px; justify-content: center;">
                    // Visibility indicator
                    <span style=format!(
                        "display: inline-block; width: 6px; height: 6px; border-radius: 50%; background: {};",
                        if is_visible { "#4caf50" } else { "#bdbdbd" }
                    )></span>
                    <span style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 13px;">
                        {label}
                    </span>
                </div>

                // Source handle
                {
                    let node_id_for_source = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id_for_source
                            r#type=HandleType::Source
                            position=HandlePosition::Bottom
                            connection_mode=ConnectionMode::Strict
                        />
                    }
                }
            </div>
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

#[component]
fn IntersectionEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="intersection-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="intersection-arrow"
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
                        let source_width = source.width.unwrap_or(120.0);
                        let source_height = source.height.unwrap_or(60.0);
                        let target_width = target.width.unwrap_or(120.0);

                        let source_x = source.position.x + source_width / 2.0;
                        let source_y = source.position.y + source_height;
                        let target_x = target.position.x + target_width / 2.0;
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
                                stroke="url(#intersection-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#intersection-arrow)"
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
