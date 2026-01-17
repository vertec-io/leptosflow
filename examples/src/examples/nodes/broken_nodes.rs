//! Broken Nodes Example
//!
//! Demonstrates how the library handles edge cases:
//! - Nodes without any handles
//! - Nodes with mismatched handle IDs
//! - Edges referencing non-existent handles
//! - Graceful error handling and warnings

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::get_drag_signal;

/// Warning message type
#[derive(Clone, Debug)]
struct Warning {
    message: String,
    severity: WarningSeverity,
}

#[derive(Clone, Debug)]
enum WarningSeverity {
    Error,
    Warning,
    Info,
}

impl WarningSeverity {
    fn color(&self) -> &'static str {
        match self {
            WarningSeverity::Error => "#ff4444",
            WarningSeverity::Warning => "#ffaa00",
            WarningSeverity::Info => "#4488ff",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            WarningSeverity::Error => "✗",
            WarningSeverity::Warning => "⚠",
            WarningSeverity::Info => "ℹ",
        }
    }
}

/// Broken nodes example showing graceful error handling
#[component]
pub fn BrokenNodesExample() -> impl IntoView {
    // Track warnings as they occur
    let warnings = RwSignal::new(Vec::<Warning>::new());

    // Helper to add a warning
    let add_warning = move |msg: String, severity: WarningSeverity| {
        warnings.update(|w| {
            w.push(Warning { message: msg, severity });
        });
    };

    // Create initial nodes with various "broken" configurations
    let initial_nodes = vec![
        // Normal node with handles (for comparison)
        Node::new("normal".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Normal Node",
                "type": "default",
                "nodeStyle": "normal"
            })),

        // Node without any handles
        Node::new("no-handles".to_string(), Position::new(300.0, 50.0))
            .with_data(json!({
                "label": "No Handles",
                "type": "none",
                "nodeStyle": "no-handles"
            })),

        // Node with only mismatched handle ID (custom-source instead of standard)
        Node::new("mismatched-source".to_string(), Position::new(50.0, 200.0))
            .with_data(json!({
                "label": "Custom Source ID",
                "type": "default",
                "nodeStyle": "mismatched",
                "sourceHandleId": "custom-source-handle"
            })),

        // Node with mismatched target handle ID
        Node::new("mismatched-target".to_string(), Position::new(300.0, 200.0))
            .with_data(json!({
                "label": "Custom Target ID",
                "type": "default",
                "nodeStyle": "mismatched",
                "targetHandleId": "custom-target-handle"
            })),

        // Node with both custom handle IDs
        Node::new("custom-handles".to_string(), Position::new(175.0, 350.0))
            .with_data(json!({
                "label": "Both Custom IDs",
                "type": "default",
                "nodeStyle": "custom",
                "sourceHandleId": "my-source",
                "targetHandleId": "my-target"
            })),
    ];

    // Create edges - some will work, some will fail to render properly
    let initial_edges = vec![
        // Edge from normal node to no-handles node (will appear "broken")
        Edge::new("e1".to_string(), "normal".to_string(), "no-handles".to_string())
            .with_label("To No Handles".to_string()),

        // Edge to mismatched handle ID (uses default handle lookup)
        Edge::new("e2".to_string(), "normal".to_string(), "mismatched-target".to_string())
            .with_label("To Custom Target".to_string()),

        // Edge from mismatched handle ID (uses default handle lookup)
        Edge::new("e3".to_string(), "mismatched-source".to_string(), "custom-handles".to_string())
            .with_label("Custom to Custom".to_string()),

        // Edge referencing specific handle IDs (should connect properly)
        Edge::new("e4".to_string(), "custom-handles".to_string(), "mismatched-target".to_string())
            .with_source_handle(Some("my-source".to_string()))
            .with_target_handle(Some("custom-target-handle".to_string()))
            .with_label("Matched IDs".to_string()),

        // Edge with non-existent handle ID
        Edge::new("e5".to_string(), "normal".to_string(), "custom-handles".to_string())
            .with_source_handle(Some("does-not-exist".to_string()))
            .with_label("Invalid Handle".to_string()),
    ];

    // Add initial warnings about the broken configurations
    add_warning("Node 'no-handles' has no handles - edges cannot connect properly".to_string(), WarningSeverity::Warning);
    add_warning("Edge 'e1' targets a node without handles".to_string(), WarningSeverity::Error);
    add_warning("Edge 'e5' references non-existent handle 'does-not-exist'".to_string(), WarningSeverity::Error);
    add_warning("Nodes with custom handle IDs require edges to specify matching sourceHandle/targetHandle".to_string(), WarningSeverity::Info);

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_drag_signal();

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
            <div class="xyflow leptos-flow broken-nodes-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges with broken edge visualization
                    <BrokenEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <BrokenNodeComponent
                                    node=node.clone()
                                    store=store
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel with warnings
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.15); max-width: 280px;">
                        <strong style="display: block; margin-bottom: 8px; font-size: 14px;">
                            "Broken Nodes Demo"
                        </strong>
                        <p style="margin: 0 0 12px; font-size: 12px; color: #666;">
                            "Demonstrates graceful handling of invalid configurations"
                        </p>

                        <div style="border-top: 1px solid #eee; padding-top: 10px;">
                            <strong style="font-size: 12px; color: #333;">"Warnings & Errors"</strong>
                            <div style="max-height: 200px; overflow-y: auto; margin-top: 8px;">
                                {move || warnings.get().iter().map(|w| {
                                    let color = w.severity.color().to_string();
                                    let icon = w.severity.icon();
                                    let msg = w.message.clone();
                                    view! {
                                        <div style=format!(
                                            "padding: 6px 8px; margin: 4px 0; background: {}15; border-left: 3px solid {}; font-size: 11px; border-radius: 0 4px 4px 0;",
                                            color, color
                                        )>
                                            <span style=format!("color: {}; margin-right: 6px;", color)>{icon}</span>
                                            {msg}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        <div style="border-top: 1px solid #eee; margin-top: 12px; padding-top: 10px;">
                            <strong style="font-size: 12px; color: #333;">"Node Types"</strong>
                            <div style="font-size: 11px; margin-top: 6px;">
                                <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                    <div style="width: 12px; height: 12px; background: #10b981; border-radius: 2px;"></div>
                                    <span>"Normal (has handles)"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                    <div style="width: 12px; height: 12px; background: #ef4444; border-radius: 2px;"></div>
                                    <span>"No handles"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                    <div style="width: 12px; height: 12px; background: #f59e0b; border-radius: 2px;"></div>
                                    <span>"Mismatched handle IDs"</span>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px; margin: 4px 0;">
                                    <div style="width: 12px; height: 12px; background: #6366f1; border-radius: 2px;"></div>
                                    <span>"Custom handle IDs"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Node component that renders differently based on "brokenness"
#[component]
fn BrokenNodeComponent(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_handles = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_style = node.data.get("nodeStyle")
        .and_then(|v| v.as_str())
        .unwrap_or("normal")
        .to_string();
    let source_handle_id = node.data.get("sourceHandleId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let target_handle_id = node.data.get("targetHandleId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(crate::shared::DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            // Mark node as dragging
            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
        }
    };

    // Get reactive node position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    // Determine styling based on node style
    let (bg_color, border_color, has_handles) = match node_style.as_str() {
        "no-handles" => ("#fef2f2", "#ef4444", false),
        "mismatched" => ("#fffbeb", "#f59e0b", true),
        "custom" => ("#eef2ff", "#6366f1", true),
        _ => ("#ecfdf5", "#10b981", true), // normal
    };

    // Determine handles to render
    let render_handles = has_handles;
    let source_id = source_handle_id;
    let target_id = target_handle_id;

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
                class="broken-node"
                style=format!(
                    "background: {}; border: 2px solid {}; padding: 12px 16px; border-radius: 8px; min-width: 120px; text-align: center; position: relative;",
                    bg_color, border_color
                )
            >
                // Target handle (if applicable)
                {render_handles.then(|| {
                    let handle_id = target_id.clone();
                    let node_id_clone = node_id_for_handles.clone();
                    if let Some(id) = handle_id {
                        view! {
                            <Handle
                                node_id=node_id_clone
                                r#type=HandleType::Target
                                position=HandlePosition::Top
                                connection_mode=ConnectionMode::Strict
                                id=id
                            />
                        }.into_any()
                    } else {
                        view! {
                            <Handle
                                node_id=node_id_clone
                                r#type=HandleType::Target
                                position=HandlePosition::Top
                                connection_mode=ConnectionMode::Strict
                            />
                        }.into_any()
                    }
                })}

                // Node content
                <div style=format!("font-weight: 600; color: {}; font-size: 13px;", border_color)>
                    {label}
                </div>

                // Show handle IDs if custom
                {(source_id.is_some() || target_id.is_some()).then(|| {
                    let source_display = source_id.clone().unwrap_or_default();
                    let target_display = target_id.clone().unwrap_or_default();
                    view! {
                        <div style="font-size: 9px; color: #888; margin-top: 4px;">
                            {(!source_display.is_empty()).then(|| view! {
                                <div>"src: " {source_display}</div>
                            })}
                            {(!target_display.is_empty()).then(|| view! {
                                <div>"tgt: " {target_display}</div>
                            })}
                        </div>
                    }
                })}

                // Warning icon for no-handles nodes
                {(!render_handles).then(|| view! {
                    <div style="font-size: 10px; color: #ef4444; margin-top: 6px; display: flex; align-items: center; justify-content: center; gap: 4px;">
                        <span>"⚠"</span>
                        <span>"No handles"</span>
                    </div>
                })}

                // Source handle (if applicable)
                {render_handles.then(|| {
                    let handle_id = source_id;
                    let node_id_clone2 = node_id_for_handles.clone();
                    if let Some(id) = handle_id {
                        view! {
                            <Handle
                                node_id=node_id_clone2
                                r#type=HandleType::Source
                                position=HandlePosition::Bottom
                                connection_mode=ConnectionMode::Strict
                                id=id
                            />
                        }.into_any()
                    } else {
                        view! {
                            <Handle
                                node_id=node_id_clone2
                                r#type=HandleType::Source
                                position=HandlePosition::Bottom
                                connection_mode=ConnectionMode::Strict
                            />
                        }.into_any()
                    }
                })}
            </div>
        </div>
    }
}

/// Custom edge renderer that visualizes broken edges
#[component]
fn BrokenEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg class="xyflow__edges" style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;">
            <defs>
                // Normal edge marker
                <marker
                    id="broken-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#888"/>
                </marker>
                // Error edge marker
                <marker
                    id="broken-arrow-error"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#ef4444"/>
                </marker>
                // Warning edge marker
                <marker
                    id="broken-arrow-warning"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f59e0b"/>
                </marker>
            </defs>
            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        // Check for broken conditions
                        let source_has_handles = source.data.get("nodeStyle")
                            .and_then(|v| v.as_str())
                            .map(|s| s != "no-handles")
                            .unwrap_or(true);
                        let target_has_handles = target.data.get("nodeStyle")
                            .and_then(|v| v.as_str())
                            .map(|s| s != "no-handles")
                            .unwrap_or(true);

                        // Check if edge references specific handle IDs
                        let edge_has_source_handle = edge.source_handle.is_some();
                        let edge_has_target_handle = edge.target_handle.is_some();

                        // Check if node has custom handle IDs
                        let source_custom_handle = source.data.get("sourceHandleId").is_some();
                        let target_custom_handle = target.data.get("targetHandleId").is_some();

                        // Determine if this is a "broken" edge
                        let is_broken = !source_has_handles || !target_has_handles;
                        let is_mismatched = (source_custom_handle && !edge_has_source_handle)
                            || (target_custom_handle && !edge_has_target_handle);

                        // Calculate positions
                        let source_x = source.position.x + source.width.unwrap_or(120.0) / 2.0;
                        let source_y = source.position.y + source.height.unwrap_or(60.0);
                        let target_x = target.position.x + target.width.unwrap_or(120.0) / 2.0;
                        let target_y = target.position.y;

                        // Calculate bezier control points
                        let dy = (target_y - source_y).abs() / 2.0;
                        let ctrl1_y = source_y + dy.max(30.0);
                        let ctrl2_y = target_y - dy.max(30.0);

                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            source_x, source_y,
                            source_x, ctrl1_y,
                            target_x, ctrl2_y,
                            target_x, target_y
                        );

                        // Determine styling
                        let (stroke_color, stroke_dash, marker) = if is_broken {
                            ("#ef4444", "5,5", "url(#broken-arrow-error)")
                        } else if is_mismatched {
                            ("#f59e0b", "8,4", "url(#broken-arrow-warning)")
                        } else {
                            ("#888", "none", "url(#broken-arrow)")
                        };

                        // Calculate label position
                        let label_x = (source_x + target_x) / 2.0;
                        let label_y = (source_y + target_y) / 2.0;
                        let label = edge.label.clone().unwrap_or_default();

                        Some(view! {
                            <g class="xyflow__edge">
                                // Edge path
                                <path
                                    d=path.clone()
                                    fill="none"
                                    stroke=stroke_color
                                    stroke-width="2"
                                    stroke-dasharray=stroke_dash
                                    marker-end=marker
                                />

                                // Edge label
                                {(!label.is_empty()).then(|| {
                                    let bg_color = if is_broken {
                                        "#fef2f2"
                                    } else if is_mismatched {
                                        "#fffbeb"
                                    } else {
                                        "#fff"
                                    };
                                    view! {
                                        <g transform=format!("translate({}, {})", label_x, label_y)>
                                            <rect
                                                x="-40"
                                                y="-10"
                                                width="80"
                                                height="20"
                                                fill=bg_color
                                                stroke=stroke_color
                                                stroke-width="1"
                                                rx="4"
                                            />
                                            <text
                                                x="0"
                                                y="4"
                                                text-anchor="middle"
                                                font-size="10"
                                                fill=stroke_color
                                            >
                                                {label}
                                            </text>
                                        </g>
                                    }
                                })}
                            </g>
                        })
                    } else {
                        None
                    }
                }).collect_view()
            }}
        </svg>
    }
}
