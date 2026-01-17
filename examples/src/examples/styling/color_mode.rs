//! Color Mode Example
//!
//! Demonstrates light and dark mode switching for the flow.
//! Uses CSS variables to update colors for nodes, edges, background, controls, and minimap.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for color mode example
static COLOR_MODE_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_color_mode_drag_signal() -> RwSignal<Option<DragState>> {
    *COLOR_MODE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Color mode enum
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorMode {
    Light,
    Dark,
}

impl ColorMode {
    fn name(&self) -> &'static str {
        match self {
            ColorMode::Light => "Light",
            ColorMode::Dark => "Dark",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ColorMode::Light => "☀️",
            ColorMode::Dark => "🌙",
        }
    }

    fn toggle(&self) -> ColorMode {
        match self {
            ColorMode::Light => ColorMode::Dark,
            ColorMode::Dark => ColorMode::Light,
        }
    }

    /// Get the CSS variable overrides for this color mode
    fn get_css_variables(&self) -> String {
        match self {
            ColorMode::Light => {
                "--xy-background-color: #ffffff; \
                 --xy-background-pattern-color: #91919a; \
                 --xy-edge-stroke: #b1b1b7; \
                 --xy-edge-stroke-selected: #555; \
                 --xy-connectionline-stroke: #b1b1b7; \
                 --xy-node-color: inherit; \
                 --xy-node-border: 1px solid #1a192b; \
                 --xy-node-background-color: #fff; \
                 --xy-node-boxshadow-hover: 0 1px 4px 1px rgba(0, 0, 0, 0.08); \
                 --xy-node-boxshadow-selected: 0 0 0 0.5px #1a192b; \
                 --xy-handle-background-color: #1a192b; \
                 --xy-handle-border-color: #fff; \
                 --xy-controls-button-background-color: #fefefe; \
                 --xy-controls-button-background-color-hover: #f4f4f4; \
                 --xy-controls-button-color: inherit; \
                 --xy-controls-button-border-color: #eee; \
                 --xy-minimap-background-color: #fff; \
                 --xy-minimap-mask-background-color: rgba(240, 240, 240, 0.6); \
                 --xy-minimap-node-background-color: #e2e2e2;"
                    .to_string()
            }
            ColorMode::Dark => {
                "--xy-background-color: #1a1a2e; \
                 --xy-background-pattern-color: #4a4a6a; \
                 --xy-edge-stroke: #6a6a8a; \
                 --xy-edge-stroke-selected: #a78bfa; \
                 --xy-connectionline-stroke: #8b5cf6; \
                 --xy-node-color: #e0e0e0; \
                 --xy-node-border: 1px solid #4a4a6a; \
                 --xy-node-background-color: #2a2a4a; \
                 --xy-node-boxshadow-hover: 0 1px 8px 2px rgba(139, 92, 246, 0.2); \
                 --xy-node-boxshadow-selected: 0 0 0 2px #8b5cf6; \
                 --xy-handle-background-color: #8b5cf6; \
                 --xy-handle-border-color: #2a2a4a; \
                 --xy-controls-button-background-color: #2a2a4a; \
                 --xy-controls-button-background-color-hover: #3a3a5a; \
                 --xy-controls-button-color: #e0e0e0; \
                 --xy-controls-button-border-color: #4a4a6a; \
                 --xy-minimap-background-color: #2a2a4a; \
                 --xy-minimap-mask-background-color: rgba(26, 26, 46, 0.6); \
                 --xy-minimap-node-background-color: #4a4a6a;"
                    .to_string()
            }
        }
    }
}

/// Color mode example component
#[component]
pub fn ColorModeExample() -> impl IntoView {
    // Color mode state
    let color_mode = RwSignal::new(ColorMode::Light);

    // Create initial nodes with different types
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 80.0))
            .with_data(json!({
                "label": "Input Node",
                "nodeType": "input"
            }))
            .with_dimensions(160.0, 60.0),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({
                "label": "Process A",
                "nodeType": "default"
            }))
            .with_dimensions(160.0, 60.0),
        Node::new("3".to_string(), Position::new(350.0, 140.0))
            .with_data(json!({
                "label": "Process B",
                "nodeType": "default"
            }))
            .with_dimensions(160.0, 60.0),
        Node::new("4".to_string(), Position::new(350.0, 280.0))
            .with_data(json!({
                "label": "Output Node",
                "nodeType": "output"
            }))
            .with_dimensions(160.0, 60.0),
        Node::new("5".to_string(), Position::new(600.0, 180.0))
            .with_data(json!({
                "label": "Result",
                "nodeType": "output"
            }))
            .with_dimensions(160.0, 60.0),
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

    // Provide the store to child components via context
    provide_context(store);

    // Global drag handlers
    let drag_signal = get_color_mode_drag_signal();

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
            <div
                class="xyflow leptos-flow svelte-flow"
                style=move || format!(
                    "width: 100%; height: 100%; position: relative; transition: all 0.3s ease; {}",
                    color_mode.get().get_css_variables()
                )
                on:mousemove=on_global_mousemove
                on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Custom edge renderer with theme awareness
                    <ColorModeEdgeRenderer store=store color_mode=color_mode />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        let mode = color_mode.get();
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <ColorModeNode
                                    node=node.clone()
                                    store=store
                                    color_mode=mode
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Color Mode Toggle Panel
                <Panel position=PanelPosition::TopRight>
                    <div style=move || {
                        let mode = color_mode.get();
                        let (bg, border, text) = match mode {
                            ColorMode::Light => ("#fff", "#e0e0e0", "#333"),
                            ColorMode::Dark => ("#2a2a4a", "#4a4a6a", "#e0e0e0"),
                        };
                        format!(
                            "background: {}; border: 1px solid {}; color: {}; \
                             padding: 16px; border-radius: 12px; \
                             box-shadow: 0 4px 12px rgba(0,0,0,0.15); \
                             max-width: 280px; transition: all 0.3s ease;",
                            bg, border, text
                        )
                    }>
                        <strong style="display: block; margin-bottom: 12px; font-size: 14px;">
                            "Color Mode"
                        </strong>

                        // Toggle button
                        <button
                            style=move || {
                                let mode = color_mode.get();
                                let (bg, text) = match mode {
                                    ColorMode::Light => ("#1a192b", "#fff"),
                                    ColorMode::Dark => ("#8b5cf6", "#fff"),
                                };
                                format!(
                                    "display: flex; align-items: center; justify-content: center; \
                                     gap: 10px; width: 100%; padding: 12px 16px; \
                                     background: {}; color: {}; border: none; border-radius: 8px; \
                                     cursor: pointer; font-size: 14px; font-weight: 600; \
                                     transition: all 0.2s ease;",
                                    bg, text
                                )
                            }
                            on:click=move |_| color_mode.update(|m| *m = m.toggle())
                        >
                            <span style="font-size: 18px;">{move || color_mode.get().toggle().icon()}</span>
                            <span>"Switch to " {move || color_mode.get().toggle().name()} " Mode"</span>
                        </button>

                        // Current mode indicator
                        <div style=move || {
                            let mode = color_mode.get();
                            let (indicator_bg, indicator_border) = match mode {
                                ColorMode::Light => ("#f5f5f5", "#e0e0e0"),
                                ColorMode::Dark => ("#3a3a5a", "#5a5a7a"),
                            };
                            format!(
                                "margin-top: 12px; padding: 12px; border-radius: 8px; \
                                 background: {}; border: 1px solid {}; \
                                 display: flex; align-items: center; gap: 10px; \
                                 transition: all 0.3s ease;",
                                indicator_bg, indicator_border
                            )
                        }>
                            <span style="font-size: 24px;">{move || color_mode.get().icon()}</span>
                            <div>
                                <div style="font-weight: 600; font-size: 13px;">
                                    "Currently: " {move || color_mode.get().name()} " Mode"
                                </div>
                                <div style=move || {
                                    let mode = color_mode.get();
                                    let secondary = match mode {
                                        ColorMode::Light => "#666",
                                        ColorMode::Dark => "#aaa",
                                    };
                                    format!("font-size: 11px; color: {}; margin-top: 2px;", secondary)
                                }>
                                    {move || match color_mode.get() {
                                        ColorMode::Light => "Bright and clean appearance",
                                        ColorMode::Dark => "Easy on the eyes in low light",
                                    }}
                                </div>
                            </div>
                        </div>

                        // Theme features list
                        <div style=move || {
                            let mode = color_mode.get();
                            let border_color = match mode {
                                ColorMode::Light => "#e0e0e0",
                                ColorMode::Dark => "#4a4a6a",
                            };
                            format!(
                                "margin-top: 16px; padding-top: 12px; \
                                 border-top: 1px solid {}; transition: all 0.3s ease;",
                                border_color
                            )
                        }>
                            <div style=move || {
                                let mode = color_mode.get();
                                let secondary = match mode {
                                    ColorMode::Light => "#888",
                                    ColorMode::Dark => "#888",
                                };
                                format!("font-size: 11px; color: {}; margin-bottom: 8px; font-weight: 600;", secondary)
                            }>
                                "Theme Updates:"
                            </div>
                            <div style=move || {
                                let mode = color_mode.get();
                                let secondary = match mode {
                                    ColorMode::Light => "#666",
                                    ColorMode::Dark => "#aaa",
                                };
                                format!("font-size: 11px; color: {}; line-height: 1.8;", secondary)
                            }>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #22c55e;">"\u{2713}"</span> "Background color"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #22c55e;">"\u{2713}"</span> "Node styling (border, background, text)"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #22c55e;">"\u{2713}"</span> "Edge colors and selection"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #22c55e;">"\u{2713}"</span> "Handle appearance"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #22c55e;">"\u{2713}"</span> "Controls and MiniMap"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #22c55e;">"\u{2713}"</span> "Background pattern"
                                </div>
                            </div>
                        </div>

                        // Implementation note
                        <div style=move || {
                            let mode = color_mode.get();
                            let (bg, border, text) = match mode {
                                ColorMode::Light => ("#e0f2fe", "#bae6fd", "#0369a1"),
                                ColorMode::Dark => ("#1e3a5f", "#2d4a6f", "#93c5fd"),
                            };
                            format!(
                                "margin-top: 12px; padding: 10px; border-radius: 6px; \
                                 background: {}; border: 1px solid {}; color: {}; \
                                 font-size: 10px; line-height: 1.5; transition: all 0.3s ease;",
                                bg, border, text
                            )
                        }>
                            <strong>"Tip: "</strong>
                            "Uses CSS variables (--xy-*) to update all flow elements consistently."
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Color mode aware node component
#[component]
fn ColorModeNode(node: Node, store: FlowStore, color_mode: ColorMode) -> impl IntoView {
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

    let drag_signal = get_color_mode_drag_signal();

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

    // Clone for view - need separate clones for each closure
    let node_type_for_handles = node_type.clone();
    let node_type_for_node_style = node_type.clone();
    let node_type_for_content_style = node_type.clone();
    let node_type_for_type_indicator = node_type.clone();
    let node_type_for_label = node_type.clone();
    let label_clone = label.clone();

    view! {
        <div
            class="xyflow__node"
            style=move || {
                let nodes = store.get_nodes();
                let (pos, width, height) = nodes.iter()
                    .find(|n| n.id == node_id_for_style)
                    .map(|n| (n.position, n.width.unwrap_or(160.0), n.height.unwrap_or(60.0)))
                    .unwrap_or((Position::new(0.0, 0.0), 160.0, 60.0));

                // Get theme-aware colors
                let (bg, border, _text, _type_color) = get_node_colors(color_mode, &node_type_for_node_style);

                format!(
                    "position: absolute; transform: translate({}px, {}px); \
                     width: {}px; height: {}px; \
                     background: {}; border: 2px solid {}; border-radius: 8px; \
                     display: flex; flex-direction: column; justify-content: center; align-items: center; \
                     padding: 10px; box-sizing: border-box; cursor: grab; \
                     transition: background 0.3s ease, border-color 0.3s ease, box-shadow 0.3s ease;",
                    pos.x, pos.y, width, height, bg, border
                )
            }
            on:mousedown=on_mousedown
        >
            // Node content
            <div style=move || {
                let (_, _, text, _type_color) = get_node_colors(color_mode, &node_type_for_content_style);
                format!("text-align: center; color: {}; transition: color 0.3s ease;", text)
            }>
                // Type indicator
                <div style=move || {
                    let (_, _, _, type_color) = get_node_colors(color_mode, &node_type_for_type_indicator);
                    format!(
                        "font-size: 9px; text-transform: uppercase; letter-spacing: 1px; \
                         font-weight: 600; margin-bottom: 4px; color: {};",
                        type_color
                    )
                }>
                    {match node_type_for_label.as_str() {
                        "input" => "Input",
                        "output" => "Output",
                        _ => "Default"
                    }}
                </div>
                // Label
                <div style="font-weight: 600; font-size: 13px;">
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

/// Get node colors based on color mode and node type
fn get_node_colors(mode: ColorMode, node_type: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match mode {
        ColorMode::Light => match node_type {
            "input" => ("#e8f5e9", "#4caf50", "#333", "#4caf50"),
            "output" => ("#ffebee", "#f44336", "#333", "#f44336"),
            _ => ("#e3f2fd", "#2196f3", "#333", "#2196f3"),
        },
        ColorMode::Dark => match node_type {
            "input" => ("#1a3a2a", "#4caf50", "#e0e0e0", "#6ede87"),
            "output" => ("#3a1a2a", "#f44336", "#e0e0e0", "#ff8a80"),
            _ => ("#1a2a4a", "#8b5cf6", "#e0e0e0", "#a78bfa"),
        },
    }
}

/// Color mode aware edge renderer
#[component]
fn ColorModeEdgeRenderer(store: FlowStore, color_mode: RwSignal<ColorMode>) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                // Light mode gradient
                <linearGradient id="edge-gradient-light" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #2196f3; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #9c27b0; stop-opacity: 1" />
                </linearGradient>
                // Dark mode gradient
                <linearGradient id="edge-gradient-dark" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #8b5cf6; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #ec4899; stop-opacity: 1" />
                </linearGradient>
                // Arrow markers for both modes
                <marker id="arrow-light" markerWidth="10" markerHeight="10" refX="9" refY="5" orient="auto">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#9c27b0" />
                </marker>
                <marker id="arrow-dark" markerWidth="10" markerHeight="10" refX="9" refY="5" orient="auto">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#ec4899" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let mode = color_mode.get();

                edges.into_iter().map(|edge| {
                    // Find source and target nodes
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        let source_x = source.position.x + source.width.unwrap_or(160.0) / 2.0;
                        let source_y = source.position.y + source.height.unwrap_or(60.0);
                        let target_x = target.position.x + target.width.unwrap_or(160.0) / 2.0;
                        let target_y = target.position.y;

                        // Calculate bezier path
                        let dy = (target_y - source_y).abs() / 2.0;
                        let control_y1 = source_y + dy.max(50.0);
                        let control_y2 = target_y - dy.max(50.0);

                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            source_x, source_y,
                            source_x, control_y1,
                            target_x, control_y2,
                            target_x, target_y
                        );

                        // Get mode-specific styling
                        let (gradient_id, marker_id, glow_color) = match mode {
                            ColorMode::Light => ("edge-gradient-light", "arrow-light", "rgba(33, 150, 243, 0.3)"),
                            ColorMode::Dark => ("edge-gradient-dark", "arrow-dark", "rgba(139, 92, 246, 0.3)"),
                        };

                        let path_clone = path.clone();

                        view! {
                            <g class="xyflow__edge">
                                // Glow effect
                                <path
                                    d=path_clone.clone()
                                    fill="none"
                                    stroke=glow_color
                                    stroke-width="6"
                                    stroke-linecap="round"
                                    style="transition: stroke 0.3s ease;"
                                />
                                // Main edge path
                                <path
                                    class="xyflow__edge-path"
                                    d=path
                                    fill="none"
                                    stroke=format!("url(#{})", gradient_id)
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    marker-end=format!("url(#{})", marker_id)
                                    style="transition: stroke 0.3s ease;"
                                />
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
