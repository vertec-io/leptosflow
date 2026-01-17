//! Stress Test Example
//!
//! Demonstrates performance with a large number of nodes:
//! - 625 nodes (25x25 grid)
//! - Corresponding edges connecting adjacent nodes
//! - FPS counter to measure performance
//! - Pan/zoom performance testing
//! - Optional virtualization for off-screen nodes

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Constants
// ============================================================================

const GRID_SIZE: usize = 25;
const NODE_WIDTH: f64 = 80.0;
const NODE_HEIGHT: f64 = 40.0;
const NODE_SPACING_X: f64 = 120.0;
const NODE_SPACING_Y: f64 = 80.0;

// ============================================================================
// Global State
// ============================================================================

/// Global drag state for stress example
static STRESS_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_stress_drag_signal() -> RwSignal<Option<DragState>> {
    *STRESS_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Performance Stats
// ============================================================================

#[derive(Clone, Debug)]
struct PerformanceStats {
    fps: f64,
    frame_count: u32,
    last_timestamp: f64,
    min_fps: f64,
    max_fps: f64,
    avg_fps: f64,
    fps_samples: Vec<f64>,
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            fps: 0.0,
            frame_count: 0,
            last_timestamp: 0.0,
            min_fps: f64::MAX,
            max_fps: 0.0,
            avg_fps: 0.0,
            fps_samples: Vec::new(),
        }
    }
}

// ============================================================================
// Node Generation
// ============================================================================

fn generate_grid_nodes() -> Vec<Node> {
    let mut nodes = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let id = format!("node-{}-{}", row, col);
            let x = col as f64 * NODE_SPACING_X;
            let y = row as f64 * NODE_SPACING_Y;

            // Alternate colors for visual interest
            let color = match (row + col) % 4 {
                0 => "#6366f1", // indigo
                1 => "#10b981", // emerald
                2 => "#f59e0b", // amber
                _ => "#ec4899", // pink
            };

            let node_type = if row == 0 {
                "input"
            } else if row == GRID_SIZE - 1 {
                "output"
            } else {
                "default"
            };

            let node = Node::new(id, Position::new(x, y))
                .with_data(json!({
                    "label": format!("{},{}", row, col),
                    "type": node_type,
                    "color": color,
                    "row": row,
                    "col": col
                }))
                .with_dimensions(NODE_WIDTH, NODE_HEIGHT);

            nodes.push(node);
        }
    }

    nodes
}

fn generate_grid_edges() -> Vec<Edge> {
    let mut edges = Vec::new();

    // Connect horizontally adjacent nodes
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE - 1 {
            let source_id = format!("node-{}-{}", row, col);
            let target_id = format!("node-{}-{}", row, col + 1);
            let edge_id = format!("e-h-{}-{}", row, col);

            edges.push(Edge::new(edge_id, source_id, target_id));
        }
    }

    // Connect vertically adjacent nodes
    for row in 0..GRID_SIZE - 1 {
        for col in 0..GRID_SIZE {
            let source_id = format!("node-{}-{}", row, col);
            let target_id = format!("node-{}-{}", row + 1, col);
            let edge_id = format!("e-v-{}-{}", row, col);

            edges.push(Edge::new(edge_id, source_id, target_id));
        }
    }

    edges
}

// ============================================================================
// Stress Node Component
// ============================================================================

/// Minimal node component for performance
#[component]
fn StressNode(
    node: Node,
    store: FlowStore,
    virtualization_enabled: RwSignal<bool>,
    viewport_bounds: RwSignal<(f64, f64, f64, f64)>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_drag = node.id.clone();

    let drag_signal = get_stress_drag_signal();

    // Extract color from data
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6366f1")
        .to_string();

    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

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

    // Get reactive node position - create two closures to avoid borrow issues
    let node_id_for_visibility = node.id.clone();
    let get_pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    // Check if node is visible in viewport (for virtualization)
    let is_visible = move || {
        if !virtualization_enabled.get() {
            return true;
        }

        let node_pos = store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_visibility)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0));

        let bounds = viewport_bounds.get();
        let (min_x, min_y, max_x, max_y) = bounds;

        // Add padding for nodes near edges
        let padding = 100.0;
        node_pos.x + NODE_WIDTH >= min_x - padding &&
        node_pos.x <= max_x + padding &&
        node_pos.y + NODE_HEIGHT >= min_y - padding &&
        node_pos.y <= max_y + padding
    };

    // Use CSS display:none for virtualization instead of removing from DOM
    view! {
        <div
            class="stress-node"
            style=move || {
                let display = if is_visible() { "flex" } else { "none" };
                let pos = get_pos();
                format!(
                    "position: absolute; transform: translate({}px, {}px); \
                     width: {}px; height: {}px; \
                     background: {}; border-radius: 4px; \
                     display: {}; align-items: center; justify-content: center; \
                     font-size: 10px; color: white; cursor: grab; \
                     box-shadow: 0 1px 2px rgba(0,0,0,0.2); \
                     user-select: none;",
                    pos.x, pos.y, NODE_WIDTH, NODE_HEIGHT, color, display
                )
            }
            on:mousedown=on_mousedown
        >
            {label}
        </div>
    }
}

// ============================================================================
// Stress Edge Renderer
// ============================================================================

#[component]
fn StressEdgeRenderer(
    store: FlowStore,
    virtualization_enabled: RwSignal<bool>,
    viewport_bounds: RwSignal<(f64, f64, f64, f64)>,
) -> impl IntoView {
    view! {
        <svg
            class="stress-edges"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let bounds = viewport_bounds.get();
                let virtualize = virtualization_enabled.get();

                edges.iter().filter_map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Check if edge is visible (if virtualization enabled)
                    if virtualize {
                        let (min_x, min_y, max_x, max_y) = bounds;
                        let padding = 100.0;

                        let edge_min_x = source_node.position.x.min(target_node.position.x);
                        let edge_max_x = source_node.position.x.max(target_node.position.x) + NODE_WIDTH;
                        let edge_min_y = source_node.position.y.min(target_node.position.y);
                        let edge_max_y = source_node.position.y.max(target_node.position.y) + NODE_HEIGHT;

                        // Skip if edge is completely outside viewport
                        if edge_max_x < min_x - padding || edge_min_x > max_x + padding ||
                           edge_max_y < min_y - padding || edge_min_y > max_y + padding {
                            return None;
                        }
                    }

                    // Calculate edge path (simple straight line for performance)
                    let start_x = source_node.position.x + NODE_WIDTH / 2.0;
                    let start_y = source_node.position.y + NODE_HEIGHT / 2.0;
                    let end_x = target_node.position.x + NODE_WIDTH / 2.0;
                    let end_y = target_node.position.y + NODE_HEIGHT / 2.0;

                    Some(view! {
                        <line
                            x1=start_x
                            y1=start_y
                            x2=end_x
                            y2=end_y
                            stroke="#94a3b8"
                            stroke-width="1"
                            opacity="0.5"
                        />
                    })
                }).collect_view()
            }}
        </svg>
    }
}

// ============================================================================
// FPS Counter Component
// ============================================================================

#[component]
fn FpsCounter(stats: RwSignal<PerformanceStats>) -> impl IntoView {
    view! {
        <div class="fps-counter" style="
            position: fixed;
            top: 80px;
            right: 20px;
            background: rgba(0, 0, 0, 0.8);
            color: white;
            padding: 12px 16px;
            border-radius: 8px;
            font-family: monospace;
            font-size: 14px;
            z-index: 1000;
            min-width: 150px;
        ">
            <div style="font-size: 24px; font-weight: bold; color: #10b981;">
                {move || format!("{:.1}", stats.get().fps)} " FPS"
            </div>
            <div style="margin-top: 8px; font-size: 11px; color: #94a3b8;">
                <div>"Min: " {move || format!("{:.1}", if stats.get().min_fps == f64::MAX { 0.0 } else { stats.get().min_fps })}</div>
                <div>"Max: " {move || format!("{:.1}", stats.get().max_fps)}</div>
                <div>"Avg: " {move || format!("{:.1}", stats.get().avg_fps)}</div>
            </div>
        </div>
    }
}

// ============================================================================
// Stress Example Component
// ============================================================================

/// Stress test example - performance with many nodes
#[component]
pub fn StressExample() -> impl IntoView {
    // Generate nodes and edges
    let initial_nodes = generate_grid_nodes();
    let initial_edges = generate_grid_edges();

    let node_count = initial_nodes.len();
    let edge_count = initial_edges.len();

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Performance stats
    let stats = RwSignal::new(PerformanceStats::default());

    // Virtualization toggle
    let virtualization_enabled = RwSignal::new(false);

    // Viewport bounds for virtualization
    let viewport_bounds = RwSignal::new((0.0, 0.0, 1000.0, 800.0));

    // Visible counts for stats
    let visible_nodes = RwSignal::new(node_count);
    let visible_edges = RwSignal::new(edge_count);

    // Get drag signal
    let drag_signal = get_stress_drag_signal();

    // FPS tracking with requestAnimationFrame using Rc<RefCell<>> for self-reference
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use std::rc::Rc;
        use std::cell::RefCell;

        let stats_signal = stats;

        // Use Rc<RefCell<>> to store the closure for self-reference
        let callback_ref: Rc<RefCell<Option<Closure<dyn Fn(f64)>>>> = Rc::new(RefCell::new(None));
        let callback_ref_clone = callback_ref.clone();

        // Create the animation frame callback
        let callback = Closure::new(move |timestamp: f64| {
            stats_signal.update(|s| {
                s.frame_count += 1;

                if s.last_timestamp > 0.0 {
                    let delta = timestamp - s.last_timestamp;
                    if delta > 0.0 {
                        let current_fps = 1000.0 / delta;
                        s.fps = current_fps;

                        // Track min/max
                        if current_fps < s.min_fps {
                            s.min_fps = current_fps;
                        }
                        if current_fps > s.max_fps {
                            s.max_fps = current_fps;
                        }

                        // Calculate rolling average
                        s.fps_samples.push(current_fps);
                        if s.fps_samples.len() > 60 {
                            s.fps_samples.remove(0);
                        }
                        s.avg_fps = s.fps_samples.iter().sum::<f64>() / s.fps_samples.len() as f64;
                    }
                }

                s.last_timestamp = timestamp;
            });

            // Request next frame using the stored callback reference
            if let Some(window) = leptos::web_sys::window() {
                if let Some(ref cb) = *callback_ref_clone.borrow() {
                    let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
                }
            }
        });

        // Store the callback and start the animation loop
        *callback_ref.borrow_mut() = Some(callback);

        if let Some(window) = leptos::web_sys::window() {
            if let Some(ref cb) = *callback_ref.borrow() {
                let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
            }
        }
    });

    // Update viewport bounds for virtualization
    Effect::new(move |_| {
        let viewport = store.get_viewport();
        let window = leptos::web_sys::window();

        if let Some(win) = window {
            let width = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1000.0);
            let height = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);

            // Calculate visible bounds in flow coordinates
            let min_x = -viewport.x / viewport.zoom;
            let min_y = -viewport.y / viewport.zoom;
            let max_x = (width - viewport.x) / viewport.zoom;
            let max_y = (height - viewport.y) / viewport.zoom;

            viewport_bounds.set((min_x, min_y, max_x, max_y));

            // Count visible nodes and edges when virtualization is enabled
            if virtualization_enabled.get() {
                let nodes = store.get_nodes();
                let edges = store.get_edges();
                let padding = 100.0;

                let vis_nodes = nodes.iter().filter(|n| {
                    n.position.x + NODE_WIDTH >= min_x - padding &&
                    n.position.x <= max_x + padding &&
                    n.position.y + NODE_HEIGHT >= min_y - padding &&
                    n.position.y <= max_y + padding
                }).count();

                let vis_edges = edges.iter().filter(|e| {
                    let source = nodes.iter().find(|n| n.id == e.source);
                    let target = nodes.iter().find(|n| n.id == e.target);

                    if let (Some(s), Some(t)) = (source, target) {
                        let edge_min_x = s.position.x.min(t.position.x);
                        let edge_max_x = s.position.x.max(t.position.x) + NODE_WIDTH;
                        let edge_min_y = s.position.y.min(t.position.y);
                        let edge_max_y = s.position.y.max(t.position.y) + NODE_HEIGHT;

                        edge_max_x >= min_x - padding && edge_min_x <= max_x + padding &&
                        edge_max_y >= min_y - padding && edge_min_y <= max_y + padding
                    } else {
                        false
                    }
                }).count();

                visible_nodes.set(vis_nodes);
                visible_edges.set(vis_edges);
            } else {
                visible_nodes.set(node_count);
                visible_edges.set(edge_count);
            }
        }
    });

    // Global mouse move handler for dragging
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;

        let handler = Closure::<dyn Fn(leptos::web_sys::MouseEvent)>::new(move |ev: leptos::web_sys::MouseEvent| {
            if let Some(drag) = drag_signal.get() {
                let viewport = store.get_viewport();
                let delta_x = (ev.client_x() as f64 - drag.start_mouse.0) / viewport.zoom;
                let delta_y = (ev.client_y() as f64 - drag.start_mouse.1) / viewport.zoom;

                store.update_node(&drag.node_id, |n| {
                    n.position.x = drag.start_pos.0 + delta_x;
                    n.position.y = drag.start_pos.1 + delta_y;
                });
            }
        });

        if let Some(window) = leptos::web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document.add_event_listener_with_callback(
                    "mousemove",
                    handler.as_ref().unchecked_ref()
                );
            }
        }

        handler.forget();
    });

    // Global mouse up handler to end dragging
    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;

        let handler = Closure::<dyn Fn(leptos::web_sys::MouseEvent)>::new(move |_ev: leptos::web_sys::MouseEvent| {
            if let Some(drag) = drag_signal.get() {
                store.update_node(&drag.node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
            }
        });

        if let Some(window) = leptos::web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document.add_event_listener_with_callback(
                    "mouseup",
                    handler.as_ref().unchecked_ref()
                );
            }
        }

        handler.forget();
    });

    // Handle scroll wheel for zoom
    let on_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();

        let viewport = store.get_viewport();
        let delta = if ev.delta_y() > 0.0 { -0.1 } else { 0.1 };
        let new_zoom = (viewport.zoom + delta).max(0.1).min(2.0);

        store.set_viewport(Viewport {
            x: viewport.x,
            y: viewport.y,
            zoom: new_zoom,
        });
    };

    // Handle middle click drag for panning
    let is_panning = RwSignal::new(false);
    let pan_start = RwSignal::new((0.0, 0.0));
    let viewport_start = RwSignal::new((0.0, 0.0));

    let on_mousedown_pan = move |ev: leptos::ev::MouseEvent| {
        if ev.button() == 1 { // Middle click
            ev.prevent_default();
            is_panning.set(true);
            pan_start.set((ev.client_x() as f64, ev.client_y() as f64));
            let viewport = store.get_viewport();
            viewport_start.set((viewport.x, viewport.y));
        }
    };

    let on_mousemove_pan = move |ev: leptos::ev::MouseEvent| {
        if is_panning.get() {
            let start = pan_start.get();
            let vp_start = viewport_start.get();
            let dx = ev.client_x() as f64 - start.0;
            let dy = ev.client_y() as f64 - start.1;
            let viewport = store.get_viewport();
            store.set_viewport(Viewport {
                x: vp_start.0 + dx,
                y: vp_start.1 + dy,
                zoom: viewport.zoom,
            });
        }
    };

    let on_mouseup_pan = move |_ev: leptos::ev::MouseEvent| {
        is_panning.set(false);
    };

    // Reset stats
    let reset_stats = move |_| {
        stats.set(PerformanceStats::default());
    };

    // Fit view
    let fit_view = move |_| {
        let total_width = GRID_SIZE as f64 * NODE_SPACING_X;
        let total_height = GRID_SIZE as f64 * NODE_SPACING_Y;

        if let Some(window) = leptos::web_sys::window() {
            let vp_width = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1000.0);
            let vp_height = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);

            // Account for sidebar
            let available_width = vp_width - 280.0;
            let available_height = vp_height - 100.0;

            let zoom_x = available_width / total_width;
            let zoom_y = available_height / total_height;
            let zoom = zoom_x.min(zoom_y).min(1.0).max(0.1);

            let x = (available_width - total_width * zoom) / 2.0;
            let y = (available_height - total_height * zoom) / 2.0 + 50.0;

            store.set_viewport(Viewport { x, y, zoom });
        }
    };

    // Center view
    let center_view = move |_| {
        store.set_viewport(Viewport { x: 100.0, y: 100.0, zoom: 0.5 });
    };

    view! {
        <div
            class="example-container stress-example"
            style="position: relative; width: 100%; height: 100%; overflow: hidden;"
        >
            // Info panel
            <div class="stress-info-panel" style="
                position: absolute;
                top: 10px;
                left: 10px;
                background: white;
                border: 1px solid #e2e8f0;
                border-radius: 8px;
                padding: 16px;
                z-index: 100;
                width: 240px;
                box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1);
            ">
                <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #1e293b;">"Stress Test"</h3>

                <div style="font-size: 12px; color: #64748b; margin-bottom: 16px;">
                    <div style="margin-bottom: 4px;">
                        <strong>{node_count}</strong> " nodes (" {GRID_SIZE} "x" {GRID_SIZE} " grid)"
                    </div>
                    <div style="margin-bottom: 4px;">
                        <strong>{edge_count}</strong> " edges"
                    </div>
                    <div>
                        "Visible: " <strong>{move || visible_nodes.get()}</strong> " nodes, "
                        <strong>{move || visible_edges.get()}</strong> " edges"
                    </div>
                </div>

                <div style="margin-bottom: 16px;">
                    <label style="display: flex; align-items: center; cursor: pointer; font-size: 13px;">
                        <input
                            type="checkbox"
                            checked=virtualization_enabled
                            on:change=move |ev| {
                                virtualization_enabled.set(event_target_checked(&ev));
                            }
                            style="margin-right: 8px;"
                        />
                        "Enable Virtualization"
                    </label>
                    <div style="font-size: 11px; color: #94a3b8; margin-top: 4px;">
                        "Only render nodes visible in viewport"
                    </div>
                </div>

                <div style="display: flex; flex-direction: column; gap: 8px;">
                    <button
                        on:click=fit_view
                        style="padding: 8px 12px; background: #6366f1; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px;"
                    >
                        "Fit View"
                    </button>
                    <button
                        on:click=center_view
                        style="padding: 8px 12px; background: #64748b; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px;"
                    >
                        "Reset View"
                    </button>
                    <button
                        on:click=reset_stats
                        style="padding: 8px 12px; background: #10b981; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px;"
                    >
                        "Reset FPS Stats"
                    </button>
                </div>

                <div style="margin-top: 16px; padding-top: 16px; border-top: 1px solid #e2e8f0;">
                    <div style="font-size: 11px; color: #94a3b8;">
                        <div>"Scroll: Zoom in/out"</div>
                        <div>"Middle-click drag: Pan"</div>
                        <div>"Click & drag: Move node"</div>
                    </div>
                </div>
            </div>

            // FPS Counter
            <FpsCounter stats=stats />

            // Flow canvas wrapper with event handlers
            <div
                class="xyflow leptos-flow stress-flow"
                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;"
                on:wheel=on_wheel
                on:mousedown=on_mousedown_pan
                on:mousemove=on_mousemove_pan
                on:mouseup=on_mouseup_pan
                on:mouseleave=move |_| is_panning.set(false)
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Edges layer
                    <StressEdgeRenderer
                        store=store
                        virtualization_enabled=virtualization_enabled
                        viewport_bounds=viewport_bounds
                    />

                    // Nodes layer
                    {move || {
                        let nodes = store.get_nodes();
                        nodes.into_iter().map(|node| {
                            view! {
                                <StressNode
                                    node=node
                                    store=store
                                    virtualization_enabled=virtualization_enabled
                                    viewport_bounds=viewport_bounds
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>
            </div>
        </div>
    }
}
