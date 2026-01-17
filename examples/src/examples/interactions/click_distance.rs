//! Click Distance Example
//!
//! Demonstrates how to distinguish between clicks and drags using a configurable
//! click distance threshold. If the mouse moves less than the threshold distance,
//! it's considered a click; otherwise, it's a drag.

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static CLICK_DISTANCE_DRAG_STATE: OnceLock<RwSignal<Option<ClickDistanceDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct ClickDistanceDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
    has_dragged: bool, // True if mouse has moved beyond click threshold
}

fn get_click_distance_drag_signal() -> RwSignal<Option<ClickDistanceDragState>> {
    *CLICK_DISTANCE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Interaction Event
// ============================================================================

#[derive(Clone, Debug)]
struct InteractionEvent {
    timestamp: f64,
    event_type: String,
    node_id: String,
    details: String,
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Click Distance Example
#[component]
pub fn ClickDistanceExample() -> impl IntoView {
    // Click distance threshold (in pixels)
    let click_threshold = RwSignal::new(5.0_f64);

    // Interaction statistics
    let click_count = RwSignal::new(0_i32);
    let drag_count = RwSignal::new(0_i32);

    // Interaction log
    let interaction_log = RwSignal::new(Vec::<InteractionEvent>::new());

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({"label": "Node A", "type": "input", "color": "#6ede87"})),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({"label": "Node B", "type": "default", "color": "#6865A5"})),
        Node::new("3".to_string(), Position::new(300.0, 125.0))
            .with_data(json!({"label": "Node C", "type": "default", "color": "#6865A5"})),
        Node::new("4".to_string(), Position::new(300.0, 275.0))
            .with_data(json!({"label": "Node D", "type": "output", "color": "#ff6b6b"})),
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

    // Helper to add log entry
    let add_log = move |event_type: String, node_id: String, details: String| {
        interaction_log.update(|logs| {
            logs.push(InteractionEvent {
                timestamp: js_sys::Date::now(),
                event_type,
                node_id,
                details,
            });
            // Keep last 15 entries
            if logs.len() > 15 {
                logs.remove(0);
            }
        });
    };

    // Drag signal
    let drag_signal = get_click_distance_drag_signal();

    // Global mouse move handler
    let add_log_for_move = add_log.clone();
    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(mut drag_state) = drag_signal.get() {
            let current_x = ev.client_x() as f64;
            let current_y = ev.client_y() as f64;
            let (start_x, start_y) = drag_state.start_mouse;
            let (node_start_x, node_start_y) = drag_state.start_pos;

            // Calculate distance moved
            let dx = current_x - start_x;
            let dy = current_y - start_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let threshold = click_threshold.get();

            // Check if we've crossed the threshold
            if !drag_state.has_dragged && distance >= threshold {
                drag_state.has_dragged = true;
                drag_signal.set(Some(drag_state.clone()));

                // Log drag start
                add_log_for_move(
                    "drag_start".to_string(),
                    drag_state.node_id.clone(),
                    format!("Distance: {:.1}px (threshold: {:.0}px)", distance, threshold),
                );
                drag_count.update(|c| *c += 1);

                // Mark node as dragging
                store.update_node(&drag_state.node_id, |n| {
                    n.dragging = true;
                });
            }

            // If dragging, update position
            if drag_state.has_dragged {
                let viewport = store.get_viewport();
                let scaled_dx = dx / viewport.zoom;
                let scaled_dy = dy / viewport.zoom;

                store.update_node(&drag_state.node_id, |n| {
                    n.position = Position::new(node_start_x + scaled_dx, node_start_y + scaled_dy);
                });
            }
        }
    };

    // Global mouse up handler
    let add_log_for_up = add_log.clone();
    let on_global_mouseup = move |ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let node_id = drag_state.node_id.clone();
            let (start_x, start_y) = drag_state.start_mouse;
            let current_x = ev.client_x() as f64;
            let current_y = ev.client_y() as f64;
            let dx = current_x - start_x;
            let dy = current_y - start_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let threshold = click_threshold.get();

            if drag_state.has_dragged {
                // It was a drag
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                add_log_for_up(
                    "drag_end".to_string(),
                    node_id,
                    format!("Total: {:.1}px", distance),
                );
            } else {
                // It was a click
                click_count.update(|c| *c += 1);
                add_log_for_up(
                    "click".to_string(),
                    node_id,
                    format!("Distance: {:.1}px < {:.0}px", distance, threshold),
                );
            }

            drag_signal.set(None);
        }
    };

    // Node mousedown handler
    let on_node_mousedown = move |node_id: String, ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(ClickDistanceDragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
                has_dragged: false,
            }));

            add_log(
                "mousedown".to_string(),
                node_id,
                format!("at ({}, {})", ev.client_x(), ev.client_y()),
            );
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow click-distance-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Render edges
                    <ClickDistanceEdgeRenderer store=store />

                    // Render nodes
                    {move || {
                        let on_mousedown = on_node_mousedown.clone();
                        store.get_nodes().into_iter().map(move |node| {
                            let node_id = node.id.clone();
                            let on_mousedown = on_mousedown.clone();
                            view! {
                                <ClickDistanceNode
                                    node=node.clone()
                                    store=store
                                    threshold=click_threshold
                                    on_mousedown=move |ev| on_mousedown(node_id.clone(), ev)
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
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333;">"Click Distance"</h3>

                        // Threshold control
                        <div style="margin-bottom: 16px; padding: 12px; background: #f8f9fa; border-radius: 6px;">
                            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;">
                                <label style="font-size: 13px; font-weight: 500; color: #555;">"Threshold (px)"</label>
                                <span style="font-size: 14px; font-weight: 600; color: #6865A5; background: #eef; padding: 2px 8px; border-radius: 4px;">
                                    {move || format!("{:.0}", click_threshold.get())}
                                </span>
                            </div>
                            <input
                                type="range"
                                min="1"
                                max="50"
                                step="1"
                                style="width: 100%; cursor: pointer;"
                                prop:value=move || click_threshold.get()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev).parse::<f64>().unwrap_or(5.0);
                                    click_threshold.set(value);
                                }
                            />
                            <div style="display: flex; justify-content: space-between; font-size: 10px; color: #999; margin-top: 4px;">
                                <span>"1px (sensitive)"</span>
                                <span>"50px (tolerant)"</span>
                            </div>
                        </div>

                        // Statistics
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 16px;">
                            <div style="padding: 12px; background: linear-gradient(135deg, #6ede87 0%, #4CAF50 100%); border-radius: 6px; text-align: center;">
                                <div style="font-size: 24px; font-weight: 700; color: white;">
                                    {move || click_count.get()}
                                </div>
                                <div style="font-size: 11px; color: rgba(255,255,255,0.9);">"Clicks"</div>
                            </div>
                            <div style="padding: 12px; background: linear-gradient(135deg, #6865A5 0%, #5a4f9a 100%); border-radius: 6px; text-align: center;">
                                <div style="font-size: 24px; font-weight: 700; color: white;">
                                    {move || drag_count.get()}
                                </div>
                                <div style="font-size: 11px; color: rgba(255,255,255,0.9);">"Drags"</div>
                            </div>
                        </div>

                        // How it works
                        <div style="margin-bottom: 16px; padding: 10px; background: #fff8e1; border-radius: 6px; border-left: 3px solid #ffc107;">
                            <div style="font-size: 12px; font-weight: 600; color: #f57c00; margin-bottom: 4px;">"How it works"</div>
                            <div style="font-size: 11px; color: #666; line-height: 1.5;">
                                "When you mousedown on a node, movement is tracked. "
                                "If you release before moving " <strong>{move || format!("{}px", click_threshold.get() as i32)}</strong>
                                " it's a " <span style="color: #4CAF50; font-weight: 600;">"click"</span>
                                ". Beyond that, it's a " <span style="color: #6865A5; font-weight: 600;">"drag"</span> "."
                            </div>
                        </div>

                        // Interaction Log
                        <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Interaction Log"</div>
                        <div style="max-height: 180px; overflow-y: auto; font-size: 11px; font-family: monospace; background: #fafafa; border-radius: 4px; padding: 8px;">
                            {move || {
                                let logs = interaction_log.get();
                                if logs.is_empty() {
                                    view! {
                                        <div style="color: #999; text-align: center; padding: 20px 0;">
                                            "Click or drag nodes..."
                                        </div>
                                    }.into_any()
                                } else {
                                    logs.iter().rev().map(|event| {
                                        let (bg, color) = match event.event_type.as_str() {
                                            "click" => ("#e8f5e9", "#2e7d32"),
                                            "drag_start" => ("#ede7f6", "#5e35b1"),
                                            "drag_end" => ("#f3e5f5", "#7b1fa2"),
                                            "mousedown" => ("#e3f2fd", "#1565c0"),
                                            _ => ("#fafafa", "#666"),
                                        };
                                        let event_type = event.event_type.clone();
                                        let node_id = event.node_id.clone();
                                        let details = event.details.clone();
                                        view! {
                                            <div style=format!(
                                                "margin-bottom: 4px; padding: 6px 8px; background: {}; border-radius: 4px;",
                                                bg
                                            )>
                                                <div style=format!("display: flex; justify-content: space-between; align-items: center;")>
                                                    <span style=format!("font-weight: 600; color: {}; text-transform: uppercase; font-size: 10px;", color)>
                                                        {event_type}
                                                    </span>
                                                    <span style="color: #888; font-size: 10px;">
                                                        {node_id}
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
                                click_count.set(0);
                                drag_count.set(0);
                                interaction_log.set(vec![]);
                            }
                        >
                            "Reset Statistics"
                        </button>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Click Distance Node Component
// ============================================================================

#[component]
fn ClickDistanceNode<F>(
    node: Node,
    store: FlowStore,
    threshold: RwSignal<f64>,
    on_mousedown: F,
) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + Clone + 'static,
{
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
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6865A5")
        .to_string();

    let has_source = node_type != "output";
    let has_target = node_type != "input";

    // Get drag signal to check if this node is being dragged
    let drag_signal = get_click_distance_drag_signal();
    let node_id_for_outer = node_id.clone();
    let node_id_for_inner = node_id.clone();

    // Get reactive node position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    let color_for_inner = color.clone();

    view! {
        <div
            class="xyflow__node"
            style=move || {
                let dragging = drag_signal.get()
                    .map(|s| s.node_id == node_id_for_outer && s.has_dragged)
                    .unwrap_or(false);
                format!(
                    "position: absolute; transform: translate({}px, {}px); cursor: {};",
                    pos().x, pos().y,
                    if dragging { "grabbing" } else { "grab" }
                )
            }
            on:mousedown=on_mousedown
        >
            <div
                class="xyflow__node-default light"
                style=move || {
                    let dragging = drag_signal.get()
                        .map(|s| s.node_id == node_id_for_inner && s.has_dragged)
                        .unwrap_or(false);
                    let c = &color_for_inner;
                    format!(
                        "background: {}; border: 2px solid {}; border-radius: 8px; padding: 10px 16px; min-width: 100px; text-align: center; transition: box-shadow 0.2s, transform 0.2s;{}",
                        c,
                        if dragging { "#333" } else { c },
                        if dragging { " box-shadow: 0 8px 20px rgba(0,0,0,0.3); transform: scale(1.02);" } else { "" }
                    )
                }
            >
                {has_target.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2);">
                    {label}
                </div>

                // Threshold indicator
                <div style="font-size: 10px; color: rgba(255,255,255,0.8); margin-top: 4px;">
                    {move || format!("threshold: {}px", threshold.get() as i32)}
                </div>

                {has_source.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Bottom
                        connection_mode=ConnectionMode::Strict
                    />
                })}
            </div>
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

#[component]
fn ClickDistanceEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="click-distance-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#6ede87;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#6865A5;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="click-distance-arrow"
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
                        let source_y = source.position.y + 50.0;
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
                                stroke="url(#click-distance-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#click-distance-arrow)"
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
