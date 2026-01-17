//! Hidden Example
//!
//! Demonstrates how to hide and show the flow and individual elements.
//! Shows toggles for: entire flow visibility, specific nodes, and specific edges.

use leptos::prelude::*;
use leptos::serde_json::json;
use std::collections::HashSet;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for hidden example
static HIDDEN_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_hidden_drag_signal() -> RwSignal<Option<DragState>> {
    *HIDDEN_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Hidden example component
#[component]
pub fn HiddenExample() -> impl IntoView {
    // Flow visibility state
    let flow_visible = RwSignal::new(true);

    // Hidden nodes and edges sets
    let hidden_nodes = RwSignal::new(HashSet::<String>::new());
    let hidden_edges = RwSignal::new(HashSet::<String>::new());

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({
                "label": "Input Node",
                "nodeType": "input"
            }))
            .with_dimensions(150.0, 50.0),
        Node::new("2".to_string(), Position::new(100.0, 150.0))
            .with_data(json!({
                "label": "Process A",
                "nodeType": "default"
            }))
            .with_dimensions(150.0, 50.0),
        Node::new("3".to_string(), Position::new(300.0, 150.0))
            .with_data(json!({
                "label": "Process B",
                "nodeType": "default"
            }))
            .with_dimensions(150.0, 50.0),
        Node::new("4".to_string(), Position::new(200.0, 270.0))
            .with_data(json!({
                "label": "Output Node",
                "nodeType": "output"
            }))
            .with_dimensions(150.0, 50.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("Edge 1".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Edge 2".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("Edge 3".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("Edge 4".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_hidden_drag_signal();

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

    // Helper to toggle node visibility
    let toggle_node = move |node_id: String| {
        hidden_nodes.update(|set| {
            if set.contains(&node_id) {
                set.remove(&node_id);
            } else {
                set.insert(node_id);
            }
        });
    };

    // Helper to toggle edge visibility
    let toggle_edge = move |edge_id: String| {
        hidden_edges.update(|set| {
            if set.contains(&edge_id) {
                set.remove(&edge_id);
            } else {
                set.insert(edge_id);
            }
        });
    };

    // Show all elements
    let show_all = move |_| {
        hidden_nodes.set(HashSet::new());
        hidden_edges.set(HashSet::new());
        flow_visible.set(true);
    };

    // Hide all elements
    let hide_all = move |_| {
        flow_visible.set(false);
    };

    view! {
        <div class="example-container">
            <div
                class="xyflow leptos-flow svelte-flow"
                style="width: 100%; height: 100%; position: relative;"
                on:mousemove=on_global_mousemove
                on:mouseup=on_global_mouseup
            >
                // Flow container with animated visibility
                <div
                    style=move || format!(
                        "width: 100%; height: 100%; position: absolute; \
                         transition: opacity 0.3s ease, transform 0.3s ease; \
                         opacity: {}; transform: scale({}); pointer-events: {};",
                        if flow_visible.get() { "1" } else { "0" },
                        if flow_visible.get() { "1" } else { "0.95" },
                        if flow_visible.get() { "auto" } else { "none" }
                    )
                >
                    // Background
                    <Background variant=BackgroundVariant::Dots />

                    // Main flow container with pan/zoom
                    <FlowViewport store=store>
                        // Edge renderer with visibility filtering
                        <HiddenEdgeRenderer store=store hidden_edges=hidden_edges hidden_nodes=hidden_nodes />

                        // Render connection line while dragging
                        <ConnectionLine />

                        // Render visible nodes only
                        {move || {
                            let hidden = hidden_nodes.get();
                            store.get_nodes().into_iter()
                                .filter(|node| !hidden.contains(&node.id))
                                .map(|node| {
                                    view! {
                                        <HiddenNode
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
                </div>

                // Flow hidden overlay
                {move || (!flow_visible.get()).then(|| view! {
                    <div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); \
                                text-align: center; color: #666; font-size: 18px;">
                        <div style="font-size: 48px; margin-bottom: 16px; opacity: 0.5;">"👁️"</div>
                        <div style="font-weight: 600; margin-bottom: 8px;">"Flow Hidden"</div>
                        <div style="font-size: 14px; color: #888;">"Toggle visibility in the control panel"</div>
                    </div>
                })}

                // Control Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: #fff; border: 1px solid #e0e0e0; padding: 16px; \
                                border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.1); \
                                max-width: 280px; max-height: 80vh; overflow-y: auto;">
                        <strong style="display: block; margin-bottom: 16px; font-size: 14px; color: #333;">
                            "Visibility Controls"
                        </strong>

                        // Flow toggle section
                        <div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #e0e0e0;">
                            <div style="font-size: 12px; color: #666; margin-bottom: 8px; font-weight: 600;">
                                "Entire Flow"
                            </div>
                            <button
                                style=move || format!(
                                    "width: 100%; padding: 10px 16px; border: none; border-radius: 8px; \
                                     cursor: pointer; font-size: 13px; font-weight: 600; \
                                     display: flex; align-items: center; justify-content: center; gap: 8px; \
                                     transition: all 0.2s ease; \
                                     background: {}; color: {};",
                                    if flow_visible.get() { "#10b981" } else { "#ef4444" },
                                    "#fff"
                                )
                                on:click=move |_| flow_visible.update(|v| *v = !*v)
                            >
                                <span>{move || if flow_visible.get() { "👁️" } else { "👁️‍🗨️" }}</span>
                                <span>{move || if flow_visible.get() { "Hide Flow" } else { "Show Flow" }}</span>
                            </button>
                        </div>

                        // Node visibility section
                        <div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #e0e0e0;">
                            <div style="font-size: 12px; color: #666; margin-bottom: 8px; font-weight: 600;">
                                "Individual Nodes"
                            </div>
                            <div style="display: flex; flex-direction: column; gap: 6px;">
                                {move || {
                                    let hidden = hidden_nodes.get();
                                    store.get_nodes().into_iter().map(|node| {
                                        let node_id = node.id.clone();
                                        let node_id_for_click = node.id.clone();
                                        let label = node.data.get("label")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Node")
                                            .to_string();
                                        let is_hidden = hidden.contains(&node_id);

                                        view! {
                                            <button
                                                style=format!(
                                                    "display: flex; align-items: center; gap: 8px; \
                                                     padding: 8px 12px; border: 1px solid {}; border-radius: 6px; \
                                                     background: {}; cursor: pointer; font-size: 12px; \
                                                     transition: all 0.2s ease;",
                                                    if is_hidden { "#fecaca" } else { "#d1fae5" },
                                                    if is_hidden { "#fef2f2" } else { "#f0fdf4" }
                                                )
                                                on:click=move |_| toggle_node(node_id_for_click.clone())
                                            >
                                                <span style=format!(
                                                    "width: 8px; height: 8px; border-radius: 50%; \
                                                     background: {};",
                                                    if is_hidden { "#ef4444" } else { "#10b981" }
                                                )></span>
                                                <span style="flex: 1; text-align: left; font-weight: 500;">
                                                    {label}
                                                </span>
                                                <span style="font-size: 10px; color: #888;">
                                                    {if is_hidden { "Hidden" } else { "Visible" }}
                                                </span>
                                            </button>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        </div>

                        // Edge visibility section
                        <div style="margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid #e0e0e0;">
                            <div style="font-size: 12px; color: #666; margin-bottom: 8px; font-weight: 600;">
                                "Individual Edges"
                            </div>
                            <div style="display: flex; flex-direction: column; gap: 6px;">
                                {move || {
                                    let hidden = hidden_edges.get();
                                    store.get_edges().into_iter().map(|edge| {
                                        let edge_id = edge.id.clone();
                                        let edge_id_for_click = edge.id.clone();
                                        let label = edge.label.clone().unwrap_or_else(|| edge.id.clone());
                                        let is_hidden = hidden.contains(&edge_id);

                                        view! {
                                            <button
                                                style=format!(
                                                    "display: flex; align-items: center; gap: 8px; \
                                                     padding: 8px 12px; border: 1px solid {}; border-radius: 6px; \
                                                     background: {}; cursor: pointer; font-size: 12px; \
                                                     transition: all 0.2s ease;",
                                                    if is_hidden { "#fecaca" } else { "#ddd6fe" },
                                                    if is_hidden { "#fef2f2" } else { "#f5f3ff" }
                                                )
                                                on:click=move |_| toggle_edge(edge_id_for_click.clone())
                                            >
                                                <span style=format!(
                                                    "width: 16px; height: 2px; border-radius: 1px; \
                                                     background: {};",
                                                    if is_hidden { "#ef4444" } else { "#8b5cf6" }
                                                )></span>
                                                <span style="flex: 1; text-align: left; font-weight: 500;">
                                                    {label}
                                                </span>
                                                <span style="font-size: 10px; color: #888;">
                                                    {if is_hidden { "Hidden" } else { "Visible" }}
                                                </span>
                                            </button>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        </div>

                        // Quick actions
                        <div style="display: flex; gap: 8px;">
                            <button
                                style="flex: 1; padding: 8px 12px; border: 1px solid #d1fae5; \
                                       background: #f0fdf4; border-radius: 6px; cursor: pointer; \
                                       font-size: 12px; font-weight: 500; color: #059669; \
                                       transition: all 0.2s ease;"
                                on:click=show_all
                            >
                                "Show All"
                            </button>
                            <button
                                style="flex: 1; padding: 8px 12px; border: 1px solid #fecaca; \
                                       background: #fef2f2; border-radius: 6px; cursor: pointer; \
                                       font-size: 12px; font-weight: 500; color: #dc2626; \
                                       transition: all 0.2s ease;"
                                on:click=hide_all
                            >
                                "Hide All"
                            </button>
                        </div>

                        // Stats
                        <div style="margin-top: 16px; padding: 12px; background: #f8fafc; \
                                    border-radius: 8px; font-size: 11px; color: #666;">
                            <div style="display: flex; justify-content: space-between; margin-bottom: 4px;">
                                <span>"Visible Nodes:"</span>
                                <span style="font-weight: 600; color: #10b981;">
                                    {move || {
                                        let total = store.get_nodes().len();
                                        let hidden = hidden_nodes.get().len();
                                        format!("{}/{}", total - hidden, total)
                                    }}
                                </span>
                            </div>
                            <div style="display: flex; justify-content: space-between;">
                                <span>"Visible Edges:"</span>
                                <span style="font-weight: 600; color: #8b5cf6;">
                                    {move || {
                                        let total = store.get_edges().len();
                                        let hidden = hidden_edges.get().len();
                                        format!("{}/{}", total - hidden, total)
                                    }}
                                </span>
                            </div>
                        </div>

                        // Tip
                        <div style="margin-top: 12px; padding: 10px; background: #eff6ff; \
                                    border: 1px solid #bfdbfe; border-radius: 6px; \
                                    font-size: 10px; color: #1d4ed8; line-height: 1.5;">
                            <strong>"Tip: "</strong>
                            "Hidden nodes also hide their connected edges. Use this to simplify complex flows."
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Node component for hidden example
#[component]
fn HiddenNode(node: Node, store: FlowStore) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();

    // Extract node data
    let label = node
        .data
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();

    let node_type = node
        .data
        .get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_hidden_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position for dragging
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

    // Clone for view
    let node_type_for_handles = node_type.clone();
    let node_type_for_style = node_type.clone();
    let label_clone = label.clone();

    // Get node color based on type
    let get_node_style = move |node_type: &str| -> (&'static str, &'static str) {
        match node_type {
            "input" => ("#dcfce7", "#22c55e"),
            "output" => ("#fee2e2", "#ef4444"),
            _ => ("#ddd6fe", "#8b5cf6"),
        }
    };

    view! {
        <div
            class="xyflow__node"
            style=move || {
                let nodes = store.get_nodes();
                let (pos, width, height) = nodes.iter()
                    .find(|n| n.id == node_id_for_style)
                    .map(|n| (n.position, n.width.unwrap_or(150.0), n.height.unwrap_or(50.0)))
                    .unwrap_or((Position::new(0.0, 0.0), 150.0, 50.0));

                let (bg, border) = get_node_style(&node_type_for_style);

                format!(
                    "position: absolute; transform: translate({}px, {}px); \
                     width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 10px; box-sizing: border-box; cursor: grab; \
                     box-shadow: 0 2px 8px rgba(0,0,0,0.1); \
                     transition: box-shadow 0.2s ease;",
                    pos.x, pos.y, width, height, bg, border
                )
            }
            on:mousedown=on_mousedown
        >
            // Node content
            <div style="text-align: center;">
                // Type indicator
                <div style=move || {
                    let (_, border) = get_node_style(&node_type);
                    format!(
                        "font-size: 9px; text-transform: uppercase; letter-spacing: 1px; \
                         font-weight: 600; margin-bottom: 4px; color: {};",
                        border
                    )
                }>
                    {match node_type.as_str() {
                        "input" => "Input",
                        "output" => "Output",
                        _ => "Process"
                    }}
                </div>
                // Label
                <div style="font-weight: 600; font-size: 13px; color: #333;">
                    {label_clone.clone()}
                </div>
            </div>

            // Handles based on node type
            {
                let has_source = node_type_for_handles != "output";
                let has_target = node_type_for_handles != "input";

                view! {
                    <>
                        {has_target.then(|| view! {
                            <Handle
                                node_id=node_id.clone()
                                r#type=HandleType::Target
                                position=HandlePosition::Top
                                connection_mode=ConnectionMode::Strict
                            />
                        })}
                        {has_source.then(|| view! {
                            <Handle
                                node_id=node_id.clone()
                                r#type=HandleType::Source
                                position=HandlePosition::Bottom
                                connection_mode=ConnectionMode::Strict
                            />
                        })}
                    </>
                }
            }
        </div>
    }
}

/// Edge renderer with visibility filtering
#[component]
fn HiddenEdgeRenderer(
    store: FlowStore,
    hidden_edges: RwSignal<HashSet<String>>,
    hidden_nodes: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="edge-gradient-hidden" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #8b5cf6; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #ec4899; stop-opacity: 1" />
                </linearGradient>
                <marker id="arrow-hidden" markerWidth="10" markerHeight="10" refX="9" refY="5" orient="auto">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#ec4899" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let hidden_e = hidden_edges.get();
                let hidden_n = hidden_nodes.get();

                edges.into_iter()
                    // Filter out hidden edges and edges connected to hidden nodes
                    .filter(|edge| {
                        !hidden_e.contains(&edge.id) &&
                        !hidden_n.contains(&edge.source) &&
                        !hidden_n.contains(&edge.target)
                    })
                    .map(|edge| {
                        // Find source and target nodes
                        let source_node = nodes.iter().find(|n| n.id == edge.source);
                        let target_node = nodes.iter().find(|n| n.id == edge.target);

                        if let (Some(source), Some(target)) = (source_node, target_node) {
                            let source_x = source.position.x + source.width.unwrap_or(150.0) / 2.0;
                            let source_y = source.position.y + source.height.unwrap_or(50.0);
                            let target_x = target.position.x + target.width.unwrap_or(150.0) / 2.0;
                            let target_y = target.position.y;

                            // Calculate bezier path
                            let dy = (target_y - source_y).abs() / 2.0;
                            let control_y1 = source_y + dy.max(40.0);
                            let control_y2 = target_y - dy.max(40.0);

                            let path = format!(
                                "M {} {} C {} {}, {} {}, {} {}",
                                source_x, source_y,
                                source_x, control_y1,
                                target_x, control_y2,
                                target_x, target_y
                            );

                            // Label position
                            let label_x = (source_x + target_x) / 2.0;
                            let label_y = (source_y + target_y) / 2.0;
                            let label_text = edge.label.clone().unwrap_or_default();

                            let path_for_glow = path.clone();

                            view! {
                                <g class="xyflow__edge">
                                    // Glow effect
                                    <path
                                        d=path_for_glow
                                        fill="none"
                                        stroke="rgba(139, 92, 246, 0.2)"
                                        stroke-width="6"
                                        stroke-linecap="round"
                                    />
                                    // Main edge path
                                    <path
                                        class="xyflow__edge-path"
                                        d=path
                                        fill="none"
                                        stroke="url(#edge-gradient-hidden)"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        marker-end="url(#arrow-hidden)"
                                    />
                                    // Edge label
                                    {(!label_text.is_empty()).then(|| view! {
                                        <g transform=format!("translate({}, {})", label_x, label_y)>
                                            <rect
                                                x="-30"
                                                y="-10"
                                                width="60"
                                                height="20"
                                                rx="10"
                                                fill="#fff"
                                                stroke="#e0e0e0"
                                                stroke-width="1"
                                            />
                                            <text
                                                x="0"
                                                y="5"
                                                text-anchor="middle"
                                                font-size="10"
                                                fill="#666"
                                            >
                                                {label_text}
                                            </text>
                                        </g>
                                    })}
                                </g>
                            }.into_any()
                        } else {
                            view! { <g></g> }.into_any()
                        }
                    }).collect_view()
            }}
        </svg>
    }
}
