//! Overview Example
//!
//! Demonstrates how to toggle minimap visibility:
//! - Toggle button to show/hide minimap
//! - Smooth show/hide animation with CSS transitions
//! - Session preference memory using leptos_use storage

use leptos::prelude::*;
use leptos::prelude::window;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static OVERVIEW_DRAG_STATE: OnceLock<RwSignal<Option<OverviewDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct OverviewDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_drag_signal() -> RwSignal<Option<OverviewDragState>> {
    *OVERVIEW_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Session storage for minimap visibility preference
// ============================================================================

static MINIMAP_VISIBLE_STORAGE: OnceLock<RwSignal<bool>> = OnceLock::new();

fn get_minimap_visible_signal() -> RwSignal<bool> {
    *MINIMAP_VISIBLE_STORAGE.get_or_init(|| {
        // Try to read from localStorage on init (defaults to true if not set)
        let initial = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item("minimap_visible").ok())
            .flatten()
            .map(|v| v != "false")
            .unwrap_or(true);
        RwSignal::new(initial)
    })
}

/// Save minimap visibility to localStorage
fn save_minimap_preference(visible: bool) {
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.set_item("minimap_visible", if visible { "true" } else { "false" });
    }
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Overview Example - Toggle minimap visibility with animation
#[component]
pub fn OverviewExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("a".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Node A", "type": "input"})),
        Node::new("b".to_string(), Position::new(300.0, 50.0))
            .with_data(json!({"label": "Node B", "type": "default"})),
        Node::new("c".to_string(), Position::new(300.0, 200.0))
            .with_data(json!({"label": "Node C", "type": "default"})),
        Node::new("d".to_string(), Position::new(500.0, 100.0))
            .with_data(json!({"label": "Node D", "type": "default"})),
        Node::new("e".to_string(), Position::new(500.0, 250.0))
            .with_data(json!({"label": "Node E", "type": "default"})),
        Node::new("f".to_string(), Position::new(700.0, 175.0))
            .with_data(json!({"label": "Node F", "type": "output"})),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e-ab".to_string(), "a".to_string(), "b".to_string()),
        Edge::new("e-ac".to_string(), "a".to_string(), "c".to_string()),
        Edge::new("e-bd".to_string(), "b".to_string(), "d".to_string()),
        Edge::new("e-ce".to_string(), "c".to_string(), "e".to_string()),
        Edge::new("e-df".to_string(), "d".to_string(), "f".to_string()),
        Edge::new("e-ef".to_string(), "e".to_string(), "f".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Minimap visibility signal
    let minimap_visible = get_minimap_visible_signal();

    // Toggle count for logging
    let toggle_count = RwSignal::new(0);

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

    // Toggle handler
    let toggle_minimap = move |_| {
        let new_value = !minimap_visible.get();
        minimap_visible.set(new_value);
        save_minimap_preference(new_value);
        toggle_count.update(|c| *c += 1);
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow overview-example"
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
                    <OverviewEdgeRenderer store=store />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <OverviewNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap with animated visibility
                <OverviewMiniMap store=store visible=minimap_visible />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; max-width: 320px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333; display: flex; align-items: center; gap: 8px;">
                            <span style="display: inline-block; width: 8px; height: 8px; background: #667eea; border-radius: 50%;"></span>
                            "MiniMap Overview"
                        </h3>

                        // Toggle button section
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Visibility Control"</div>
                            <button
                                style=move || format!(
                                    "width: 100%; padding: 12px 16px; border: none; border-radius: 8px; cursor: pointer; font-weight: 600; font-size: 14px; display: flex; align-items: center; justify-content: center; gap: 8px; transition: all 0.2s ease; {}",
                                    if minimap_visible.get() {
                                        "background: linear-gradient(135deg, #6ede87 0%, #4caf50 100%); color: white; box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);"
                                    } else {
                                        "background: linear-gradient(135deg, #ff6b6b 0%, #ee5a5a 100%); color: white; box-shadow: 0 2px 8px rgba(238, 90, 90, 0.3);"
                                    }
                                )
                                on:click=toggle_minimap
                            >
                                {move || if minimap_visible.get() {
                                    view! {
                                        <>
                                            <span style="font-size: 18px;">"👁"</span>
                                            "Hide MiniMap"
                                        </>
                                    }.into_any()
                                } else {
                                    view! {
                                        <>
                                            <span style="font-size: 18px;">"👁‍🗨"</span>
                                            "Show MiniMap"
                                        </>
                                    }.into_any()
                                }}
                            </button>
                        </div>

                        // Status display
                        <div style="margin-bottom: 16px; padding: 12px; background: #f5f5f5; border-radius: 8px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                <span style="font-size: 12px; color: #666;">"Status:"</span>
                                <span style=move || format!(
                                    "padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 600; {}",
                                    if minimap_visible.get() {
                                        "background: #e8f5e9; color: #2e7d32;"
                                    } else {
                                        "background: #ffebee; color: #c62828;"
                                    }
                                )>
                                    {move || if minimap_visible.get() { "Visible" } else { "Hidden" }}
                                </span>
                            </div>
                            <div style="display: flex; justify-content: space-between; align-items: center;">
                                <span style="font-size: 12px; color: #666;">"Toggle Count:"</span>
                                <span style="font-size: 12px; font-weight: 600; color: #333;">
                                    {move || toggle_count.get()}
                                </span>
                            </div>
                        </div>

                        // How it works section
                        <div style="padding: 12px; background: #f0f4ff; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; color: #666; line-height: 1.5;">
                                <strong style="color: #4a5568;">"How it works:"</strong>
                                <br />
                                "• Click the button to toggle MiniMap"
                                <br />
                                "• CSS transition animates show/hide"
                                <br />
                                "• Preference saved to localStorage"
                                <br />
                                "• Refreshing the page remembers choice"
                            </div>
                        </div>

                        // Keyboard shortcut hint
                        <div style="padding: 10px; background: linear-gradient(135deg, #667eea22 0%, #764ba222 100%); border-radius: 6px; border: 1px dashed #667eea44;">
                            <div style="font-size: 11px; color: #555; display: flex; align-items: center; gap: 6px;">
                                <span style="background: #667eea; color: white; padding: 2px 6px; border-radius: 4px; font-size: 10px; font-weight: 600;">"TIP"</span>
                                "Try refreshing the page - your preference persists!"
                            </div>
                        </div>
                    </div>
                </Panel>

                // Instructions badge
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 10px 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <div style="color: white; font-size: 11px; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 4px;">"MiniMap Overview"</div>
                            <div style="opacity: 0.9;">"• Toggle visibility with button"</div>
                            <div style="opacity: 0.9;">"• Smooth CSS animation"</div>
                            <div style="opacity: 0.9;">"• Session preference saved"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// MiniMap Component with Animated Visibility
// ============================================================================

#[component]
fn OverviewMiniMap(store: FlowStore, visible: RwSignal<bool>) -> impl IntoView {
    // Minimap dimensions
    let width: u32 = 200;
    let height: u32 = 140;

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
        let padding = 60.0;
        (min_x - padding, min_y - padding, max_x + padding, max_y + padding)
    };

    view! {
        <div
            class="xyflow__minimap xyflow__panel bottom right"
            style=move || format!(
                "width: {}px; height: {}px; background: white; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); overflow: hidden; padding: 8px; transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); {}",
                width + 16, height + 16,
                if visible.get() {
                    "opacity: 1; transform: scale(1) translateY(0); pointer-events: auto;"
                } else {
                    "opacity: 0; transform: scale(0.8) translateY(10px); pointer-events: none;"
                }
            )
        >
            // MiniMap title
            <div style="font-size: 10px; font-weight: 600; color: #888; margin-bottom: 6px; text-transform: uppercase; letter-spacing: 0.5px; display: flex; justify-content: space-between; align-items: center;">
                <span>"Overview"</span>
                <span style="font-size: 8px; background: #e0e0e0; padding: 2px 6px; border-radius: 4px; color: #666;">
                    {move || format!("{} nodes", store.get_nodes().len())}
                </span>
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

                        let color = match node_type {
                            "input" => "#6ede87",
                            "output" => "#ff6b6b",
                            _ => "#667eea",
                        };

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
                            />
                        }
                    }).collect_view()
                }}

                // Render minimap edges
                {move || {
                    let nodes = store.get_nodes();
                    store.get_edges().into_iter().map(|edge| {
                        let source = nodes.iter().find(|n| n.id == edge.source);
                        let target = nodes.iter().find(|n| n.id == edge.target);

                        if let (Some(s), Some(t)) = (source, target) {
                            let sw = s.width.unwrap_or(150.0);
                            let sh = s.height.unwrap_or(60.0);
                            let tw = t.width.unwrap_or(150.0);

                            let sx = s.position.x + sw / 2.0;
                            let sy = s.position.y + sh;
                            let tx = t.position.x + tw / 2.0;
                            let ty = t.position.y;

                            view! {
                                <line
                                    x1=sx
                                    y1=sy
                                    x2=tx
                                    y2=ty
                                    stroke="#999"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                />
                            }.into_any()
                        } else {
                            view! { <g></g> }.into_any()
                        }
                    }).collect_view()
                }}

                // Render viewport indicator
                {move || {
                    let viewport = store.get_viewport();
                    let (min_x, min_y, _, _) = bounds();

                    // Calculate visible area in flow coordinates
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
// Node Component
// ============================================================================

#[component]
fn OverviewNode(node: Node, store: FlowStore) -> impl IntoView {
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

    let drag_signal = get_drag_signal();

    // Mouse down handler
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(OverviewDragState {
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

    // Get node color
    let color = match node_type.as_str() {
        "input" => "#6ede87",
        "output" => "#ff6b6b",
        _ => "#667eea",
    };

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
fn OverviewEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="overview-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="overview-arrow"
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
                                stroke="url(#overview-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#overview-arrow)"
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
