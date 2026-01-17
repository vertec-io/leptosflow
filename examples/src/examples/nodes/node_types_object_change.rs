//! Node Types Object Change Example
//!
//! Demonstrates how to dynamically change the node type definitions (component mappings)
//! at runtime. This allows swapping out which component renders each node type.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for node types object change example
static NODE_TYPES_OBJECT_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_node_types_object_drag_signal() -> RwSignal<Option<DragState>> {
    *NODE_TYPES_OBJECT_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Available theme options for node rendering
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeTheme {
    /// Default theme - standard appearance
    Default,
    /// Gradient theme - colorful gradients
    Gradient,
    /// Minimal theme - simplified appearance
    Minimal,
    /// Neon theme - glowing neon style
    Neon,
}

impl NodeTheme {
    fn name(&self) -> &'static str {
        match self {
            NodeTheme::Default => "Default",
            NodeTheme::Gradient => "Gradient",
            NodeTheme::Minimal => "Minimal",
            NodeTheme::Neon => "Neon",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            NodeTheme::Default => "Standard node appearance with solid colors",
            NodeTheme::Gradient => "Colorful gradient backgrounds",
            NodeTheme::Minimal => "Clean, simplified appearance",
            NodeTheme::Neon => "Glowing neon style with shadows",
        }
    }

    fn all() -> Vec<NodeTheme> {
        vec![NodeTheme::Default, NodeTheme::Gradient, NodeTheme::Minimal, NodeTheme::Neon]
    }
}

/// Node types object change example
#[component]
pub fn NodeTypesObjectChangeExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 80.0))
            .with_data(json!({
                "label": "Input Node",
                "nodeType": "input"
            }))
            .with_dimensions(160.0, 70.0),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({
                "label": "Default A",
                "nodeType": "default"
            }))
            .with_dimensions(160.0, 70.0),
        Node::new("3".to_string(), Position::new(350.0, 140.0))
            .with_data(json!({
                "label": "Default B",
                "nodeType": "default"
            }))
            .with_dimensions(160.0, 70.0),
        Node::new("4".to_string(), Position::new(350.0, 280.0))
            .with_data(json!({
                "label": "Output Node",
                "nodeType": "output"
            }))
            .with_dimensions(160.0, 70.0),
        Node::new("5".to_string(), Position::new(600.0, 180.0))
            .with_data(json!({
                "label": "Custom Type",
                "nodeType": "custom"
            }))
            .with_dimensions(160.0, 70.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
        Edge::new("e3-5".to_string(), "3".to_string(), "5".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Current theme signal
    let current_theme = RwSignal::new(NodeTheme::Default);

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_node_types_object_drag_signal();

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
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Edge renderer
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes with theme-based component
                    {move || {
                        let theme = current_theme.get();
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <ThemedNode
                                    node=node.clone()
                                    store=store
                                    theme=theme
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Theme Selection Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); max-width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Node Types Object Change"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666; line-height: 1.4;">
                            "Change the theme to swap which component renders each node type. All nodes update to use the new component definitions."
                        </p>

                        // Current theme display
                        <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; margin-bottom: 12px;">
                            <div style="font-weight: 600; font-size: 13px; margin-bottom: 4px;">
                                "Current: " {move || current_theme.get().name()}
                            </div>
                            <div style="font-size: 11px; color: #666;">
                                {move || current_theme.get().description()}
                            </div>
                        </div>

                        // Theme buttons
                        <div style="display: flex; flex-direction: column; gap: 8px;">
                            {NodeTheme::all().into_iter().map(|theme| {
                                let theme_for_click = theme;
                                let theme_for_style = theme;
                                let theme_for_name = theme;
                                let theme_for_desc = theme;
                                view! {
                                    <button
                                        style=move || {
                                            let is_active = current_theme.get() == theme_for_style;
                                            let bg = if is_active { get_theme_color(theme_for_style) } else { "#e0e0e0" };
                                            let color = if is_active { "white" } else { "#333" };
                                            format!(
                                                "display: flex; flex-direction: column; align-items: flex-start; \
                                                 width: 100%; padding: 10px 12px; background: {}; color: {}; \
                                                 border: none; border-radius: 6px; cursor: pointer; \
                                                 font-size: 12px; text-align: left; transition: all 0.15s;",
                                                bg, color
                                            )
                                        }
                                        on:click=move |_| current_theme.set(theme_for_click)
                                    >
                                        <div style="font-weight: 600;">{theme_for_name.name()}</div>
                                        <div style="font-size: 10px; opacity: 0.9; margin-top: 2px;">{theme_for_desc.description()}</div>
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Node types legend
                        <div style="margin-top: 16px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; color: #888; margin-bottom: 8px; font-weight: 600;">"Node Types:"</div>
                            <div style="font-size: 10px; color: #666; line-height: 1.6;">
                                <div>"Input - source handle only"</div>
                                <div>"Default - both handles"</div>
                                <div>"Output - target handle only"</div>
                                <div>"Custom - special appearance"</div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Get theme accent color
fn get_theme_color(theme: NodeTheme) -> &'static str {
    match theme {
        NodeTheme::Default => "#2196f3",
        NodeTheme::Gradient => "#9c27b0",
        NodeTheme::Minimal => "#607d8b",
        NodeTheme::Neon => "#00e676",
    }
}

/// Themed node component that renders differently based on theme
#[component]
fn ThemedNode(
    node: Node,
    store: FlowStore,
    theme: NodeTheme,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();

    let node_type = node.data.get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_node_types_object_drag_signal();

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
    let node_type_for_content = node_type.clone();
    let label_clone = label.clone();

    view! {
        <div
            class="xyflow__node themed-node"
            style=move || {
                let nodes = store.get_nodes();
                let (pos, width, height) = nodes.iter()
                    .find(|n| n.id == node_id_for_style)
                    .map(|n| (n.position, n.width.unwrap_or(160.0), n.height.unwrap_or(70.0)))
                    .unwrap_or((Position::new(0.0, 0.0), 160.0, 70.0));

                let styles = get_node_styles(theme, &node_type_for_style);
                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     {}; cursor: grab; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 12px; box-sizing: border-box; transition: all 0.3s ease;",
                    pos.x, pos.y, width, height, styles
                )
            }
            on:mousedown=on_mousedown
        >
            // Theme-specific content
            {move || get_node_content(theme, &label_clone, &node_type_for_content)}

            // Handles based on node type
            {
                let has_source = node_type_for_handles != "output";
                let has_target = node_type_for_handles != "input";
                let handle_style = get_handle_style(theme);

                view! {
                    <>
                        {has_target.then(|| view! {
                            <Handle
                                node_id=node_id.clone()
                                r#type=HandleType::Target
                                position=HandlePosition::Top
                                connection_mode=ConnectionMode::Strict
                                style=handle_style.to_string()
                            />
                        })}
                        {has_source.then(|| view! {
                            <Handle
                                node_id=node_id.clone()
                                r#type=HandleType::Source
                                position=HandlePosition::Bottom
                                connection_mode=ConnectionMode::Strict
                                style=handle_style.to_string()
                            />
                        })}
                    </>
                }
            }
        </div>
    }
}

/// Get CSS styles for a node based on theme and node type
fn get_node_styles(theme: NodeTheme, node_type: &str) -> String {
    match theme {
        NodeTheme::Default => {
            let (bg, border) = match node_type {
                "input" => ("#e8f5e9", "#4caf50"),
                "output" => ("#ffebee", "#f44336"),
                "custom" => ("#fff3e0", "#ff9800"),
                _ => ("#e3f2fd", "#2196f3"),
            };
            format!(
                "background: {}; border: 2px solid {}; border-radius: 8px; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.1)",
                bg, border
            )
        }
        NodeTheme::Gradient => {
            let gradient = match node_type {
                "input" => "linear-gradient(135deg, #11998e 0%, #38ef7d 100%)",
                "output" => "linear-gradient(135deg, #eb3349 0%, #f45c43 100%)",
                "custom" => "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)",
                _ => "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            };
            format!(
                "background: {}; border: none; border-radius: 12px; \
                 box-shadow: 0 4px 15px rgba(0,0,0,0.2)",
                gradient
            )
        }
        NodeTheme::Minimal => {
            let border_color = match node_type {
                "input" => "#4caf50",
                "output" => "#f44336",
                "custom" => "#ff9800",
                _ => "#9e9e9e",
            };
            format!(
                "background: white; border: 1px solid {}; border-radius: 4px; \
                 box-shadow: none",
                border_color
            )
        }
        NodeTheme::Neon => {
            let (color, glow) = match node_type {
                "input" => ("#00ff88", "rgba(0,255,136,0.5)"),
                "output" => ("#ff0055", "rgba(255,0,85,0.5)"),
                "custom" => ("#ffaa00", "rgba(255,170,0,0.5)"),
                _ => ("#00d4ff", "rgba(0,212,255,0.5)"),
            };
            format!(
                "background: #1a1a2e; border: 2px solid {}; border-radius: 8px; \
                 box-shadow: 0 0 20px {}, 0 0 40px {}, inset 0 0 10px rgba(255,255,255,0.1)",
                color, glow, glow
            )
        }
    }
}

/// Get node content based on theme
fn get_node_content(theme: NodeTheme, label: &str, node_type: &str) -> impl IntoView {
    match theme {
        NodeTheme::Default => {
            let (icon, type_color) = match node_type {
                "input" => ("-->", "#4caf50"),
                "output" => ("<--", "#f44336"),
                "custom" => ("*", "#ff9800"),
                _ => ("<->", "#2196f3"),
            };
            view! {
                <div style="text-align: center;">
                    <div style=format!("font-size: 10px; color: {}; margin-bottom: 4px; font-weight: 600;", type_color)>
                        <span style="font-family: monospace;">{icon}</span>
                        " " {node_type.to_uppercase()}
                    </div>
                    <div style="font-weight: 600; font-size: 13px; color: #333;">
                        {label.to_string()}
                    </div>
                </div>
            }.into_any()
        }
        NodeTheme::Gradient => {
            view! {
                <div style="text-align: center; color: white;">
                    <div style="font-size: 16px; font-weight: 700; text-shadow: 0 1px 2px rgba(0,0,0,0.2);">
                        {label.to_string()}
                    </div>
                    <div style="font-size: 10px; opacity: 0.9; margin-top: 4px; text-transform: uppercase; letter-spacing: 1px;">
                        {node_type.to_string()}
                    </div>
                </div>
            }.into_any()
        }
        NodeTheme::Minimal => {
            let accent = match node_type {
                "input" => "#4caf50",
                "output" => "#f44336",
                "custom" => "#ff9800",
                _ => "#607d8b",
            };
            view! {
                <div style="text-align: center;">
                    <div style="font-weight: 500; font-size: 13px; color: #333;">
                        {label.to_string()}
                    </div>
                    <div style=format!("font-size: 9px; color: {}; margin-top: 4px; text-transform: lowercase;", accent)>
                        {node_type.to_string()}
                    </div>
                </div>
            }.into_any()
        }
        NodeTheme::Neon => {
            let glow_color = match node_type {
                "input" => "#00ff88",
                "output" => "#ff0055",
                "custom" => "#ffaa00",
                _ => "#00d4ff",
            };
            view! {
                <div style="text-align: center;">
                    <div style=format!(
                        "font-weight: 700; font-size: 14px; color: {}; \
                         text-shadow: 0 0 10px {}, 0 0 20px {};",
                        glow_color, glow_color, glow_color
                    )>
                        {label.to_string()}
                    </div>
                    <div style="font-size: 9px; color: #888; margin-top: 4px; text-transform: uppercase; letter-spacing: 2px;">
                        {node_type.to_string()}
                    </div>
                </div>
            }.into_any()
        }
    }
}

/// Get handle style based on theme
fn get_handle_style(theme: NodeTheme) -> &'static str {
    match theme {
        NodeTheme::Default => "background: #2196f3; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
        NodeTheme::Gradient => "background: white; width: 12px; height: 12px; border: none; box-shadow: 0 2px 6px rgba(0,0,0,0.3);",
        NodeTheme::Minimal => "background: #607d8b; width: 8px; height: 8px; border: 1px solid white;",
        NodeTheme::Neon => "background: #00d4ff; width: 10px; height: 10px; border: 2px solid #1a1a2e; box-shadow: 0 0 10px #00d4ff;",
    }
}
