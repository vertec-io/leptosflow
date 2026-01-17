//! Node Toolbar Example
//!
//! Demonstrates how to add a context toolbar to nodes:
//! - Toolbar appears on node selection
//! - Contains action buttons (delete, duplicate, change color)
//! - Positioned relative to node (above, below, etc.)

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for node toolbar example
static NODE_TOOLBAR_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_node_toolbar_drag_signal() -> RwSignal<Option<DragState>> {
    *NODE_TOOLBAR_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Toolbar position relative to node
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ToolbarPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl ToolbarPosition {
    fn as_str(&self) -> &'static str {
        match self {
            ToolbarPosition::Top => "top",
            ToolbarPosition::Bottom => "bottom",
            ToolbarPosition::Left => "left",
            ToolbarPosition::Right => "right",
        }
    }
}

/// Node Toolbar example
#[component]
pub fn NodeToolbarExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 120.0))
            .with_data(json!({
                "label": "Select Me",
                "color": "#6366f1"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("2".to_string(), Position::new(300.0, 80.0))
            .with_data(json!({
                "label": "Or Me",
                "color": "#10b981"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("3".to_string(), Position::new(200.0, 220.0))
            .with_data(json!({
                "label": "Try Me Too",
                "color": "#f59e0b"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("4".to_string(), Position::new(400.0, 200.0))
            .with_data(json!({
                "label": "Click Me",
                "color": "#ef4444"
            }))
            .with_dimensions(140.0, 50.0),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Selected node for toolbar
    let selected_node_id = RwSignal::new(Option::<String>::None);

    // Toolbar position setting
    let toolbar_position = RwSignal::new(ToolbarPosition::Top);

    // Action log for feedback
    let action_log = RwSignal::new(Vec::<String>::new());

    // Counter for duplicated nodes
    let node_counter = RwSignal::new(5);

    // Add action to log
    let add_action = move |action: String| {
        action_log.update(|log| {
            log.insert(0, action);
            if log.len() > 5 {
                log.pop();
            }
        });
    };

    // Global drag handlers
    let drag_signal = get_node_toolbar_drag_signal();

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

    // Click on background to deselect
    let on_background_click = move |ev: leptos::ev::MouseEvent| {
        // Check if the click was directly on the background (not bubbled from node)
        let target = ev.target();
        if let Some(el) = target {
            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                // Check if it's the flow container or background
                let class_list = html_el.class_list();
                if class_list.contains("xyflow__viewport") ||
                   class_list.contains("leptos-flow") ||
                   class_list.contains("xyflow__background") {
                    selected_node_id.set(None);
                }
            }
        }
    };

    // Available colors defined in ToolbarNode component

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
                 on:click=on_background_click
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <NodeToolbarEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes with toolbars
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            let node_id = node.id.clone();
                            let add_action_clone = add_action.clone();

                            view! {
                                <ToolbarNode
                                    node=node.clone()
                                    store=store
                                    selected_node_id=selected_node_id
                                    toolbar_position=toolbar_position
                                    node_counter=node_counter
                                    on_action=move |action: String| add_action_clone(format!("{}: {}", node_id, action))
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 240px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Node Toolbar"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click a node to select it and show its toolbar. The toolbar provides actions like delete, duplicate, and change color."
                        </p>

                        // Toolbar position selector
                        <div style="margin-bottom: 14px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Toolbar Position"</div>
                            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 4px;">
                                {vec![ToolbarPosition::Top, ToolbarPosition::Bottom, ToolbarPosition::Left, ToolbarPosition::Right]
                                    .into_iter()
                                    .map(|pos| {
                                        let pos_clone = pos;
                                        view! {
                                            <button
                                                style=move || format!(
                                                    "padding: 6px 8px; font-size: 10px; border: 1px solid {}; \
                                                     border-radius: 4px; background: {}; cursor: pointer; \
                                                     color: {}; text-transform: capitalize;",
                                                    if toolbar_position.get() == pos_clone { "#6366f1" } else { "#ddd" },
                                                    if toolbar_position.get() == pos_clone { "#eef2ff" } else { "#fff" },
                                                    if toolbar_position.get() == pos_clone { "#6366f1" } else { "#666" }
                                                )
                                                on:click=move |_| toolbar_position.set(pos_clone)
                                            >
                                                {pos.as_str()}
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </div>

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
                                        let color = node.data.get("color")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("#6366f1")
                                            .to_string();
                                        view! {
                                            <div style=format!(
                                                "background: {}20; padding: 10px; border-radius: 6px; margin-bottom: 12px; \
                                                 border: 1px solid {};",
                                                color, color
                                            )>
                                                <div style=format!("font-weight: 600; font-size: 12px; color: {};", color)>
                                                    "Selected: " {label}
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

                        // Action log
                        <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Action Log"</div>
                            <div style="background: #f8f9fa; border-radius: 4px; padding: 8px; max-height: 120px; overflow-y: auto;">
                                {move || {
                                    let log = action_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic;">
                                                "No actions yet"
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.into_iter().map(|entry| {
                                            view! {
                                                <div style="font-size: 10px; color: #666; padding: 2px 0; border-bottom: 1px solid #eee;">
                                                    {entry}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Node component with toolbar
#[component]
fn ToolbarNode<F>(
    node: Node,
    store: FlowStore,
    selected_node_id: RwSignal<Option<String>>,
    toolbar_position: RwSignal<ToolbarPosition>,
    node_counter: RwSignal<i32>,
    on_action: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_delete = node.id.clone();
    let node_id_for_duplicate = node.id.clone();
    let node_id_for_color = node.id.clone();
    let node_id_for_toolbar = node.id.clone();

    let on_action_delete = on_action.clone();
    let on_action_duplicate = on_action.clone();
    let on_action_color = on_action.clone();

    let drag_signal = get_node_toolbar_drag_signal();

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

    // Node IDs for closures checking if selected
    let node_id_for_selected_style = node.id.clone();
    let node_id_for_selected_toolbar = node.id.clone();

    // Delete action
    let on_delete = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        // Remove node from store
        store.set_nodes(
            store.get_nodes().into_iter()
                .filter(|n| n.id != node_id_for_delete)
                .collect()
        );
        // Remove edges connected to this node
        store.set_edges(
            store.get_edges().into_iter()
                .filter(|e| e.source != node_id_for_delete && e.target != node_id_for_delete)
                .collect()
        );
        selected_node_id.set(None);
        on_action_delete("Deleted".to_string());
    };

    // Duplicate action
    let on_duplicate = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        let nodes = store.get_nodes();
        if let Some(source_node) = nodes.iter().find(|n| n.id == node_id_for_duplicate) {
            let new_id = format!("{}", node_counter.get());
            node_counter.update(|c| *c += 1);

            let new_node = Node::new(
                new_id.clone(),
                Position::new(source_node.position.x + 30.0, source_node.position.y + 30.0)
            )
            .with_data(source_node.data.clone())
            .with_dimensions(
                source_node.width.unwrap_or(140.0),
                source_node.height.unwrap_or(50.0)
            );

            let mut new_nodes = store.get_nodes();
            new_nodes.push(new_node);
            store.set_nodes(new_nodes);

            // Select the new node
            selected_node_id.set(Some(new_id));
            on_action_duplicate("Duplicated".to_string());
        }
    };

    // Available colors
    let colors = vec![
        ("#6366f1", "Indigo"),
        ("#10b981", "Green"),
        ("#f59e0b", "Amber"),
        ("#ef4444", "Red"),
        ("#8b5cf6", "Purple"),
        ("#06b6d4", "Cyan"),
    ];

    view! {
        <div
            class="xyflow__node toolbar-node"
            style=move || {
                let nodes = store.get_nodes();
                let is_selected = selected_node_id.get().as_ref() == Some(&node_id_for_selected_style);
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");

                    let box_shadow = if is_selected {
                        format!("0 0 0 3px {}80, 0 4px 12px rgba(0,0,0,0.15)", color)
                    } else {
                        "0 2px 8px rgba(0,0,0,0.1)".to_string()
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: white; border: 2px solid {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: box-shadow 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(140.0), n.height.unwrap_or(50.0),
                        color, box_shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_mousedown
        >
            // Node label - reactive
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
                    let label = n.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1")
                        .to_string();

                    view! {
                        <div style=format!("font-weight: 600; font-size: 12px; color: {};", color)>
                            {label}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Toolbar - only visible when selected
            {move || {
                let is_selected = selected_node_id.get().as_ref() == Some(&node_id_for_selected_toolbar);
                if is_selected {
                    let nodes = store.get_nodes();
                    let node_data = nodes.iter().find(|n| n.id == node_id_for_toolbar).cloned();

                    if let Some(n) = node_data {
                        let pos = toolbar_position.get();
                        let color = n.data.get("color")
                            .and_then(|v| v.as_str())
                            .unwrap_or("#6366f1");

                        // Calculate toolbar position based on setting
                        let toolbar_style = match pos {
                            ToolbarPosition::Top => format!(
                                "position: absolute; bottom: calc(100% + 8px); left: 50%; \
                                 transform: translateX(-50%); display: flex; gap: 4px; \
                                 background: white; padding: 6px 8px; border-radius: 8px; \
                                 box-shadow: 0 2px 10px rgba(0,0,0,0.2); border: 1px solid {}; \
                                 z-index: 1000;",
                                color
                            ),
                            ToolbarPosition::Bottom => format!(
                                "position: absolute; top: calc(100% + 8px); left: 50%; \
                                 transform: translateX(-50%); display: flex; gap: 4px; \
                                 background: white; padding: 6px 8px; border-radius: 8px; \
                                 box-shadow: 0 2px 10px rgba(0,0,0,0.2); border: 1px solid {}; \
                                 z-index: 1000;",
                                color
                            ),
                            ToolbarPosition::Left => format!(
                                "position: absolute; right: calc(100% + 8px); top: 50%; \
                                 transform: translateY(-50%); display: flex; flex-direction: column; gap: 4px; \
                                 background: white; padding: 6px 8px; border-radius: 8px; \
                                 box-shadow: 0 2px 10px rgba(0,0,0,0.2); border: 1px solid {}; \
                                 z-index: 1000;",
                                color
                            ),
                            ToolbarPosition::Right => format!(
                                "position: absolute; left: calc(100% + 8px); top: 50%; \
                                 transform: translateY(-50%); display: flex; flex-direction: column; gap: 4px; \
                                 background: white; padding: 6px 8px; border-radius: 8px; \
                                 box-shadow: 0 2px 10px rgba(0,0,0,0.2); border: 1px solid {}; \
                                 z-index: 1000;",
                                color
                            ),
                        };

                        let on_delete_clone = on_delete.clone();
                        let on_duplicate_clone = on_duplicate.clone();
                        let colors_clone = colors.clone();
                        let on_action_color_clone = on_action_color.clone();
                        let node_id_for_color_inner = node_id_for_color.clone();

                        view! {
                            <div
                                class="node-toolbar"
                                style=toolbar_style
                                on:mousedown=|ev: leptos::ev::MouseEvent| ev.stop_propagation()
                            >
                                // Delete button
                                <button
                                    title="Delete node"
                                    style="width: 28px; height: 28px; border: none; border-radius: 6px; \
                                           background: #fee2e2; color: #dc2626; cursor: pointer; \
                                           display: flex; align-items: center; justify-content: center; \
                                           font-size: 14px; transition: all 0.15s;"
                                    on:click=on_delete_clone.clone()
                                >
                                    "🗑"
                                </button>

                                // Duplicate button
                                <button
                                    title="Duplicate node"
                                    style="width: 28px; height: 28px; border: none; border-radius: 6px; \
                                           background: #dbeafe; color: #2563eb; cursor: pointer; \
                                           display: flex; align-items: center; justify-content: center; \
                                           font-size: 14px; transition: all 0.15s;"
                                    on:click=on_duplicate_clone.clone()
                                >
                                    "📋"
                                </button>

                                // Color picker buttons
                                {colors_clone.clone().into_iter().map(|(c, name)| {
                                    let color = c.to_string();
                                    let color_clone = color.clone();
                                    let node_id_color = node_id_for_color_inner.clone();
                                    let on_action_c = on_action_color_clone.clone();

                                    view! {
                                        <button
                                            title=format!("Change to {}", name)
                                            style=format!(
                                                "width: 20px; height: 20px; border: 2px solid white; border-radius: 50%; \
                                                 background: {}; cursor: pointer; box-shadow: 0 1px 3px rgba(0,0,0,0.2);",
                                                color
                                            )
                                            on:click=move |ev: leptos::ev::MouseEvent| {
                                                ev.stop_propagation();
                                                store.update_node(&node_id_color, |n| {
                                                    if let Some(data) = n.data.as_object_mut() {
                                                        data.insert("color".to_string(), json!(color_clone.clone()));
                                                    }
                                                });
                                                on_action_c(format!("Color -> {}", name));
                                            }
                                        />
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Handles
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Top
                connection_mode=ConnectionMode::Strict
                style="background: #888; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Bottom
                connection_mode=ConnectionMode::Strict
                style="background: #888; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
        </div>
    }
}

/// Edge renderer for node toolbar example
#[component]
fn NodeToolbarEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="node-toolbar-arrow"
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

                    // Get colors
                    let source_color = source_node.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#888");
                    let target_color = target_node.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#888");

                    // Calculate edge path
                    let sx = source_node.position.x + source_node.width.unwrap_or(140.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(140.0) / 2.0;
                    let ty = target_node.position.y;

                    let offset = (ty - sy).abs() * 0.5;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx, sy + offset,
                        tx, ty - offset,
                        tx, ty
                    );

                    // Create unique gradient ID
                    let grad_id = format!("edge-grad-{}", edge.id);
                    let grad_id_ref = format!("url(#{})", grad_id);

                    Some(view! {
                        <g class="xyflow__edge">
                            <defs>
                                <linearGradient id=grad_id.clone() x1="0%" y1="0%" x2="100%" y2="0%">
                                    <stop offset="0%" stop-color=source_color.to_string() />
                                    <stop offset="100%" stop-color=target_color.to_string() />
                                </linearGradient>
                            </defs>
                            <path
                                d=path
                                stroke=grad_id_ref
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#node-toolbar-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
