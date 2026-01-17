//! Use Update Node Internals Example
//!
//! Demonstrates how to force re-measurement of node internals after dynamic content changes:
//! - Nodes with expandable content that changes their size
//! - Triggering dimension updates when content changes
//! - Edges automatically recalculate connection points based on new dimensions

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for this example
static USE_UPDATE_INTERNALS_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_use_update_internals_drag_signal() -> RwSignal<Option<DragState>> {
    *USE_UPDATE_INTERNALS_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Use Update Node Internals example
#[component]
pub fn UseUpdateNodeInternalsExample() -> impl IntoView {
    // Create initial nodes with expandable content
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 80.0))
            .with_data(json!({
                "label": "Node 1",
                "expanded": false,
                "items": ["Item A", "Item B", "Item C"]
            }))
            .with_dimensions(180.0, 50.0),
        Node::new("2".to_string(), Position::new(350.0, 80.0))
            .with_data(json!({
                "label": "Node 2",
                "expanded": true,
                "items": ["Detail 1", "Detail 2"]
            }))
            .with_dimensions(180.0, 100.0),
        Node::new("3".to_string(), Position::new(225.0, 280.0))
            .with_data(json!({
                "label": "Node 3",
                "expanded": false,
                "items": ["Info X", "Info Y", "Info Z", "Info W"]
            }))
            .with_dimensions(180.0, 50.0),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Track update count to show internals update feedback
    let update_count = RwSignal::new(0);
    let last_updated_node = RwSignal::new(Option::<String>::None);

    // Global drag handlers
    let drag_signal = get_use_update_internals_drag_signal();

    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let current_x = ev.client_x() as f64;
            let current_y = ev.client_y() as f64;
            let (start_x, start_y) = drag_state.start_mouse;
            let (node_start_x, node_start_y) = drag_state.start_pos;

            let viewport = store.get_viewport();
            let dx = (current_x - start_x) / viewport.zoom;
            let dy = (current_y - start_y) / viewport.zoom;

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

    // Update all nodes button
    let update_all_internals = move |_| {
        let nodes = store.get_nodes();
        for node in nodes.iter() {
            // Force recalculate dimensions based on expanded state
            let node_id = node.id.clone();
            let expanded = node.data.get("expanded")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let item_count = node.data.get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            let new_height = if expanded {
                50.0 + (item_count as f64 * 24.0)
            } else {
                50.0
            };

            store.update_node(&node_id, |n| {
                n.height = Some(new_height);
            });
        }
        update_count.update(|c| *c += 1);
        last_updated_node.set(Some("all".to_string()));
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

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <UpdateInternalsEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <ExpandableNode
                                    node=node.clone()
                                    store=store
                                    update_count=update_count
                                    last_updated_node=last_updated_node
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 260px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Update Node Internals"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click on a node to expand/collapse its content. When content changes, the node dimensions update and edges recalculate their connection points."
                        </p>

                        // Update stats
                        <div style="background: #f5f5f5; padding: 10px; border-radius: 6px; margin-bottom: 12px;">
                            <div style="display: flex; justify-content: space-between; font-size: 11px; color: #666;">
                                <span>"Updates triggered:"</span>
                                <span style="font-weight: 600; color: #333;">{move || update_count.get()}</span>
                            </div>
                            {move || {
                                last_updated_node.get().map(|id| {
                                    view! {
                                        <div style="margin-top: 6px; font-size: 10px; color: #888;">
                                            "Last updated: "
                                            <span style="color: #2196f3; font-weight: 500;">
                                                {if id == "all" { "All nodes".to_string() } else { format!("Node {}", id) }}
                                            </span>
                                        </div>
                                    }
                                })
                            }}
                        </div>

                        // Action buttons
                        <div style="display: flex; flex-direction: column; gap: 8px;">
                            <button
                                style="width: 100%; padding: 8px 12px; font-size: 11px; border: 1px solid #2196f3; \
                                       border-radius: 6px; background: #e3f2fd; color: #1976d2; cursor: pointer; \
                                       font-weight: 500; transition: all 0.15s;"
                                on:click=update_all_internals
                            >
                                "Update All Node Internals"
                            </button>
                        </div>

                        // How it works
                        <div style="margin-top: 14px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"How it works"</div>
                            <ul style="margin: 0; padding-left: 16px; font-size: 10px; color: #666; line-height: 1.6;">
                                <li>"Click a node header to expand/collapse"</li>
                                <li>"Expansion updates node.height"</li>
                                <li>"Edge renderer reads new dimensions"</li>
                                <li>"Connection points recalculate automatically"</li>
                            </ul>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Expandable node component
#[component]
fn ExpandableNode(
    node: Node,
    store: FlowStore,
    update_count: RwSignal<i32>,
    last_updated_node: RwSignal<Option<String>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_header = node.id.clone();
    let node_id_for_expand = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_content = node.id.clone();
    let node_id_for_update = node.id.clone();

    let drag_signal = get_use_update_internals_drag_signal();

    // Toggle expanded state
    let toggle_expanded = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id_for_expand) {
            let current_expanded = node.data.get("expanded")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let item_count = node.data.get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            let new_expanded = !current_expanded;
            let new_height = if new_expanded {
                50.0 + (item_count as f64 * 24.0)
            } else {
                50.0
            };

            // Update node data and dimensions (this is the "update internals" action)
            store.update_node(&node_id_for_expand, |n| {
                if let Some(data) = n.data.as_object_mut() {
                    data.insert("expanded".to_string(), json!(new_expanded));
                }
                n.height = Some(new_height);
            });

            // Track the update
            update_count.update(|c| *c += 1);
            last_updated_node.set(Some(node_id_for_update.clone()));
        }
    };

    // Mouse down on header - start dragging
    let on_header_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        // Don't stop propagation so toggle can work

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

    view! {
        <div
            class="xyflow__node expandable-node"
            style=move || {
                let nodes = store.get_nodes();
                let (pos, width, height, expanded) = nodes.iter()
                    .find(|n| n.id == node_id_for_style)
                    .map(|n| {
                        let expanded = n.data.get("expanded")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        (n.position, n.width.unwrap_or(180.0), n.height.unwrap_or(50.0), expanded)
                    })
                    .unwrap_or((Position::new(0.0, 0.0), 180.0, 50.0, false));

                // Highlight color when expanded
                let border_color = if expanded { "#4caf50" } else { "#888" };
                let shadow = if expanded {
                    "0 0 0 2px rgba(76, 175, 80, 0.3), 0 2px 8px rgba(0,0,0,0.1)"
                } else {
                    "0 2px 8px rgba(0,0,0,0.1)"
                };

                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: white; border: 2px solid {}; border-radius: 8px; \
                     box-shadow: {}; overflow: hidden; \
                     display: flex; flex-direction: column; transition: height 0.2s ease, box-shadow 0.15s;",
                    pos.x, pos.y, width, height, border_color, shadow
                )
            }
        >
            // Node header (clickable to expand/collapse)
            <div
                style="height: 48px; padding: 12px; display: flex; align-items: center; justify-content: space-between; \
                       background: linear-gradient(135deg, #667eea, #764ba2); color: white; cursor: pointer; flex-shrink: 0;"
                on:mousedown=on_header_mousedown
                on:click=toggle_expanded
            >
                {move || {
                    let nodes = store.get_nodes();
                    let (label, expanded) = nodes.iter()
                        .find(|n| n.id == node_id_for_header)
                        .map(|n| {
                            let label = n.data.get("label")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Node")
                                .to_string();
                            let expanded = n.data.get("expanded")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            (label, expanded)
                        })
                        .unwrap_or(("Node".to_string(), false));

                    view! {
                        <>
                            <span style="font-weight: 600; font-size: 12px;">{label}</span>
                            <span style="font-size: 16px; transition: transform 0.2s;"
                                  class:rotated=expanded>
                                {if expanded { "▼" } else { "▶" }}
                            </span>
                        </>
                    }
                }}
            </div>

            // Expandable content
            {move || {
                let nodes = store.get_nodes();
                let (expanded, items) = nodes.iter()
                    .find(|n| n.id == node_id_for_content)
                    .map(|n| {
                        let expanded = n.data.get("expanded")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let items: Vec<String> = n.data.get("items")
                            .and_then(|v| v.as_array())
                            .map(|a| a.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect())
                            .unwrap_or_default();
                        (expanded, items)
                    })
                    .unwrap_or((false, vec![]));

                if expanded {
                    view! {
                        <div style="padding: 4px 8px; background: #fafafa; flex: 1; overflow: hidden;">
                            {items.into_iter().map(|item| {
                                view! {
                                    <div style="padding: 4px 8px; font-size: 11px; color: #555; \
                                                border-bottom: 1px solid #eee; display: flex; align-items: center; gap: 6px;">
                                        <span style="width: 6px; height: 6px; background: #4caf50; border-radius: 50%;"></span>
                                        {item}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div></div>
                    }.into_any()
                }
            }}

            // Handles
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Top
                connection_mode=ConnectionMode::Strict
                style="background: #667eea; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Bottom
                connection_mode=ConnectionMode::Strict
                style="background: #764ba2; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
        </div>
    }
}

/// Edge renderer for update node internals example
#[component]
fn UpdateInternalsEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="update-internals-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#667eea" />
                </marker>
                <linearGradient id="update-internals-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#667eea" />
                    <stop offset="100%" stop-color="#764ba2" />
                </linearGradient>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate edge path using CURRENT node dimensions
                    // This is the key - edges recalculate when dimensions change
                    let source_width = source_node.width.unwrap_or(180.0);
                    let source_height = source_node.height.unwrap_or(50.0);
                    let target_width = target_node.width.unwrap_or(180.0);

                    let sx = source_node.position.x + source_width / 2.0;
                    let sy = source_node.position.y + source_height;  // Bottom of source node
                    let tx = target_node.position.x + target_width / 2.0;
                    let ty = target_node.position.y;  // Top of target node

                    let offset = (ty - sy).abs() * 0.5;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx, sy + offset,
                        tx, ty - offset,
                        tx, ty
                    );

                    Some(view! {
                        <g class="xyflow__edge">
                            // Shadow/glow effect
                            <path
                                d=path.clone()
                                stroke="rgba(102, 126, 234, 0.3)"
                                stroke-width="6"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path
                                stroke="url(#update-internals-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#update-internals-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
