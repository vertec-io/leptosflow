//! Update Node Example
//!
//! Demonstrates how to programmatically update node properties:
//! - Position (x, y coordinates)
//! - Data/Label
//! - Style/Class

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for update node example
static UPDATE_NODE_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_update_node_drag_signal() -> RwSignal<Option<DragState>> {
    *UPDATE_NODE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Available node styles
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeStyle {
    Default,
    Success,
    Warning,
    Error,
    Info,
}

impl NodeStyle {
    fn as_str(&self) -> &'static str {
        match self {
            NodeStyle::Default => "default",
            NodeStyle::Success => "success",
            NodeStyle::Warning => "warning",
            NodeStyle::Error => "error",
            NodeStyle::Info => "info",
        }
    }

    fn colors(&self) -> (&'static str, &'static str) {
        match self {
            NodeStyle::Default => ("#f5f5f5", "#888"),
            NodeStyle::Success => ("#e8f5e9", "#4caf50"),
            NodeStyle::Warning => ("#fff8e1", "#ff9800"),
            NodeStyle::Error => ("#ffebee", "#f44336"),
            NodeStyle::Info => ("#e3f2fd", "#2196f3"),
        }
    }
}

/// Update Node example
#[component]
pub fn UpdateNodeExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({
                "label": "Node 1",
                "style": "default"
            }))
            .with_dimensions(160.0, 60.0),
        Node::new("2".to_string(), Position::new(350.0, 100.0))
            .with_data(json!({
                "label": "Node 2",
                "style": "success"
            }))
            .with_dimensions(160.0, 60.0),
        Node::new("3".to_string(), Position::new(220.0, 250.0))
            .with_data(json!({
                "label": "Node 3",
                "style": "info"
            }))
            .with_dimensions(160.0, 60.0),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Selected node for editing
    let selected_node_id = RwSignal::new(Option::<String>::None);

    // Form state signals
    let position_x = RwSignal::new(String::from("0"));
    let position_y = RwSignal::new(String::from("0"));
    let node_label = RwSignal::new(String::from(""));
    let node_style = RwSignal::new(NodeStyle::Default);

    // Update form when selection changes
    Effect::new(move |_| {
        if let Some(id) = selected_node_id.get() {
            let nodes = store.get_nodes();
            if let Some(node) = nodes.iter().find(|n| n.id == id) {
                position_x.set(format!("{:.0}", node.position.x));
                position_y.set(format!("{:.0}", node.position.y));

                let label = node.data.get("label")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Node")
                    .to_string();
                node_label.set(label);

                let style_str = node.data.get("style")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");
                let style = match style_str {
                    "success" => NodeStyle::Success,
                    "warning" => NodeStyle::Warning,
                    "error" => NodeStyle::Error,
                    "info" => NodeStyle::Info,
                    _ => NodeStyle::Default,
                };
                node_style.set(style);
            }
        }
    });

    // Global drag handlers
    let drag_signal = get_update_node_drag_signal();

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

            // Update form position if this node is selected
            if selected_node_id.get().as_ref() == Some(&drag_state.node_id) {
                position_x.set(format!("{:.0}", node_start_x + dx));
                position_y.set(format!("{:.0}", node_start_y + dy));
            }
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

    // Apply position changes
    let apply_position = move |_| {
        if let Some(id) = selected_node_id.get() {
            let x = position_x.get().parse::<f64>().unwrap_or(0.0);
            let y = position_y.get().parse::<f64>().unwrap_or(0.0);
            store.update_node(&id, |n| {
                n.position = Position::new(x, y);
            });
        }
    };

    // Apply label changes
    let apply_label = move |_| {
        if let Some(id) = selected_node_id.get() {
            let label = node_label.get();
            store.update_node(&id, |n| {
                if let Some(data) = n.data.as_object_mut() {
                    data.insert("label".to_string(), json!(label));
                }
            });
        }
    };

    // Apply style changes
    let apply_style = move |style: NodeStyle| {
        if let Some(id) = selected_node_id.get() {
            node_style.set(style);
            store.update_node(&id, |n| {
                if let Some(data) = n.data.as_object_mut() {
                    data.insert("style".to_string(), json!(style.as_str()));
                }
            });
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

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <UpdateNodeEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <UpdatableNode
                                    node=node.clone()
                                    store=store
                                    selected_node_id=selected_node_id
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Control Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 260px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Update Node"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click a node to select it, then use the controls below to update its properties. Changes are applied immediately."
                        </p>

                        // Selected node indicator
                        {move || {
                            match selected_node_id.get() {
                                Some(id) => {
                                    let nodes = store.get_nodes();
                                    if let Some(node) = nodes.iter().find(|n| n.id == id) {
                                        let label = node.data.get("label")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Unknown")
                                            .to_string();
                                        view! {
                                            <div style="background: #e3f2fd; padding: 10px; border-radius: 6px; margin-bottom: 12px; border: 1px solid #2196f3;">
                                                <div style="font-weight: 600; font-size: 12px; color: #1976d2;">
                                                    "Selected: " {label} " (#{id})"
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div style="background: #f5f5f5; padding: 10px; border-radius: 6px; margin-bottom: 12px; color: #999; font-size: 12px;">
                                                "No node selected"
                                            </div>
                                        }.into_any()
                                    }
                                },
                                None => {
                                    view! {
                                        <div style="background: #f5f5f5; padding: 10px; border-radius: 6px; margin-bottom: 12px; color: #999; font-size: 12px;">
                                            "Click a node to select it"
                                        </div>
                                    }.into_any()
                                }
                            }
                        }}

                        // Position controls
                        <div style="margin-bottom: 14px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Position"</div>
                            <div style="display: flex; gap: 8px; align-items: center;">
                                <div style="flex: 1;">
                                    <label style="font-size: 10px; color: #666;">"X"</label>
                                    <input
                                        type="number"
                                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px; box-sizing: border-box;"
                                        prop:value=move || position_x.get()
                                        on:input=move |ev| position_x.set(event_target_value(&ev))
                                        on:change=apply_position
                                        disabled=move || selected_node_id.get().is_none()
                                    />
                                </div>
                                <div style="flex: 1;">
                                    <label style="font-size: 10px; color: #666;">"Y"</label>
                                    <input
                                        type="number"
                                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px; box-sizing: border-box;"
                                        prop:value=move || position_y.get()
                                        on:input=move |ev| position_y.set(event_target_value(&ev))
                                        on:change=apply_position
                                        disabled=move || selected_node_id.get().is_none()
                                    />
                                </div>
                            </div>
                        </div>

                        // Label control
                        <div style="margin-bottom: 14px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Label"</div>
                            <input
                                type="text"
                                style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px; font-size: 12px; box-sizing: border-box;"
                                prop:value=move || node_label.get()
                                on:input=move |ev| {
                                    node_label.set(event_target_value(&ev));
                                    apply_label(());
                                }
                                disabled=move || selected_node_id.get().is_none()
                                placeholder="Enter node label"
                            />
                        </div>

                        // Style controls
                        <div style="margin-bottom: 8px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Style"</div>
                            <div style="display: grid; grid-template-columns: repeat(5, 1fr); gap: 4px;">
                                <StyleButton
                                    style=NodeStyle::Default
                                    current_style=node_style
                                    on_click=apply_style
                                    disabled=move || selected_node_id.get().is_none()
                                />
                                <StyleButton
                                    style=NodeStyle::Success
                                    current_style=node_style
                                    on_click=apply_style
                                    disabled=move || selected_node_id.get().is_none()
                                />
                                <StyleButton
                                    style=NodeStyle::Warning
                                    current_style=node_style
                                    on_click=apply_style
                                    disabled=move || selected_node_id.get().is_none()
                                />
                                <StyleButton
                                    style=NodeStyle::Error
                                    current_style=node_style
                                    on_click=apply_style
                                    disabled=move || selected_node_id.get().is_none()
                                />
                                <StyleButton
                                    style=NodeStyle::Info
                                    current_style=node_style
                                    on_click=apply_style
                                    disabled=move || selected_node_id.get().is_none()
                                />
                            </div>
                        </div>

                        // Quick actions
                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Quick Actions"</div>
                            <div style="display: flex; gap: 6px;">
                                <button
                                    style=move || format!(
                                        "flex: 1; padding: 6px; font-size: 10px; border: 1px solid #ddd; border-radius: 4px; \
                                         background: {}; cursor: {}; opacity: {};",
                                        if selected_node_id.get().is_some() { "#f5f5f5" } else { "#fafafa" },
                                        if selected_node_id.get().is_some() { "pointer" } else { "not-allowed" },
                                        if selected_node_id.get().is_some() { "1" } else { "0.5" }
                                    )
                                    disabled=move || selected_node_id.get().is_none()
                                    on:click=move |_| {
                                        if let Some(id) = selected_node_id.get() {
                                            // Center the node
                                            store.update_node(&id, |n| {
                                                n.position = Position::new(200.0, 150.0);
                                            });
                                            position_x.set("200".to_string());
                                            position_y.set("150".to_string());
                                        }
                                    }
                                >
                                    "Center"
                                </button>
                                <button
                                    style=move || format!(
                                        "flex: 1; padding: 6px; font-size: 10px; border: 1px solid #ddd; border-radius: 4px; \
                                         background: {}; cursor: {}; opacity: {};",
                                        if selected_node_id.get().is_some() { "#f5f5f5" } else { "#fafafa" },
                                        if selected_node_id.get().is_some() { "pointer" } else { "not-allowed" },
                                        if selected_node_id.get().is_some() { "1" } else { "0.5" }
                                    )
                                    disabled=move || selected_node_id.get().is_none()
                                    on:click=move |_| {
                                        if let Some(id) = selected_node_id.get() {
                                            // Randomize position
                                            let x = (js_sys::Math::random() * 400.0) + 50.0;
                                            let y = (js_sys::Math::random() * 250.0) + 50.0;
                                            store.update_node(&id, |n| {
                                                n.position = Position::new(x, y);
                                            });
                                            position_x.set(format!("{:.0}", x));
                                            position_y.set(format!("{:.0}", y));
                                        }
                                    }
                                >
                                    "Random Pos"
                                </button>
                                <button
                                    style=move || format!(
                                        "flex: 1; padding: 6px; font-size: 10px; border: 1px solid #ddd; border-radius: 4px; \
                                         background: {}; cursor: {}; opacity: {};",
                                        if selected_node_id.get().is_some() { "#f5f5f5" } else { "#fafafa" },
                                        if selected_node_id.get().is_some() { "pointer" } else { "not-allowed" },
                                        if selected_node_id.get().is_some() { "1" } else { "0.5" }
                                    )
                                    disabled=move || selected_node_id.get().is_none()
                                    on:click=move |_| {
                                        if let Some(id) = selected_node_id.get() {
                                            // Reset to default label
                                            let default_label = format!("Node {}", id);
                                            store.update_node(&id, |n| {
                                                if let Some(data) = n.data.as_object_mut() {
                                                    data.insert("label".to_string(), json!(default_label.clone()));
                                                }
                                            });
                                            node_label.set(default_label);
                                        }
                                    }
                                >
                                    "Reset Label"
                                </button>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Style button component
#[component]
fn StyleButton<F>(
    style: NodeStyle,
    current_style: RwSignal<NodeStyle>,
    on_click: F,
    disabled: impl Fn() -> bool + Copy + Send + Sync + 'static,
) -> impl IntoView
where
    F: Fn(NodeStyle) + Copy + 'static,
{
    let (bg_color, border_color) = style.colors();
    let style_clone = style;

    view! {
        <button
            title=style.as_str()
            style=move || {
                let is_selected = current_style.get() == style_clone;
                format!(
                    "width: 100%; aspect-ratio: 1; border: 2px solid {}; border-radius: 4px; \
                     background: {}; cursor: {}; opacity: {}; transition: all 0.15s; \
                     box-shadow: {};",
                    if is_selected { border_color } else { "#ddd" },
                    bg_color,
                    if disabled() { "not-allowed" } else { "pointer" },
                    if disabled() { "0.5" } else { "1" },
                    if is_selected { format!("0 0 0 2px {}", border_color) } else { "none".to_string() }
                )
            }
            disabled=disabled
            on:click=move |_| on_click(style)
        />
    }
}

/// Updatable node component
#[component]
fn UpdatableNode(
    node: Node,
    store: FlowStore,
    selected_node_id: RwSignal<Option<String>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_selected = node.id.clone();

    let drag_signal = get_update_node_drag_signal();

    // Mouse down - start dragging and select
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Select this node
        selected_node_id.set(Some(node_id_for_select.clone()));

        // Start dragging
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

    // Check if selected
    let is_selected = move || {
        selected_node_id.get().as_ref() == Some(&node_id_for_selected)
    };

    view! {
        <div
            class="xyflow__node updatable-node"
            style=move || {
                let nodes = store.get_nodes();
                let (pos, width, height, _label, style_str) = nodes.iter()
                    .find(|n| n.id == node_id_for_style)
                    .map(|n| {
                        let label = n.data.get("label")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Node")
                            .to_string();
                        let style_str = n.data.get("style")
                            .and_then(|v| v.as_str())
                            .unwrap_or("default")
                            .to_string();
                        (n.position, n.width.unwrap_or(160.0), n.height.unwrap_or(60.0), label, style_str)
                    })
                    .unwrap_or((Position::new(0.0, 0.0), 160.0, 60.0, "Node".to_string(), "default".to_string()));

                // Get colors based on style
                let (bg_color, border_color) = match style_str.as_str() {
                    "success" => ("#e8f5e9", "#4caf50"),
                    "warning" => ("#fff8e1", "#ff9800"),
                    "error" => ("#ffebee", "#f44336"),
                    "info" => ("#e3f2fd", "#2196f3"),
                    _ => ("#f5f5f5", "#888"),
                };

                // Selection ring
                let box_shadow = if is_selected() {
                    format!("0 0 0 3px {}, 0 2px 8px rgba(0,0,0,0.15)", border_color)
                } else {
                    "0 2px 8px rgba(0,0,0,0.1)".to_string()
                };

                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     box-shadow: {}; cursor: grab; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 8px; box-sizing: border-box; transition: box-shadow 0.15s, background 0.2s, border-color 0.2s;",
                    pos.x, pos.y, width, height, bg_color, border_color, box_shadow
                )
            }
            on:mousedown=on_mousedown
        >
            // Node label - reactive
            {move || {
                let nodes = store.get_nodes();
                let label = nodes.iter()
                    .find(|n| n.id == node_id)
                    .and_then(|n| n.data.get("label"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Node")
                    .to_string();

                view! {
                    <div style="font-weight: 600; font-size: 13px; color: #333; text-align: center;">
                        {label}
                    </div>
                }
            }}

            // Handles
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Top
                connection_mode=ConnectionMode::Strict
                style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Bottom
                connection_mode=ConnectionMode::Strict
                style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
        </div>
    }
}

/// Edge renderer for update node example
#[component]
fn UpdateNodeEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="update-node-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#888" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate edge path
                    let sx = source_node.position.x + source_node.width.unwrap_or(160.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(60.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(160.0) / 2.0;
                    let ty = target_node.position.y;

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
                            <path
                                d=path
                                stroke="#888"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#update-node-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
