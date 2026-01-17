//! MultiSetNodes Example
//!
//! Demonstrates handling multiple disconnected node groups:
//! - Display multiple separate node groups
//! - Each group independently selectable
//! - Show group selection behavior
//! - Visual feedback for group membership

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use std::collections::HashSet;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for multi-set nodes example
static MULTI_SET_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_multi_set_drag_signal() -> RwSignal<Option<DragState>> {
    *MULTI_SET_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Node group information
#[derive(Clone, Debug, PartialEq)]
struct NodeGroup {
    id: &'static str,
    name: &'static str,
    color: &'static str,
    description: &'static str,
}

/// Get all groups
fn get_groups() -> Vec<NodeGroup> {
    vec![
        NodeGroup {
            id: "data",
            name: "Data Processing",
            color: "#10b981",
            description: "Data input and transformation nodes",
        },
        NodeGroup {
            id: "compute",
            name: "Compute",
            color: "#6366f1",
            description: "Processing and computation nodes",
        },
        NodeGroup {
            id: "output",
            name: "Output",
            color: "#f59e0b",
            description: "Output and export nodes",
        },
    ]
}

/// MultiSetNodes example
#[component]
pub fn MultiSetNodesExample() -> impl IntoView {
    // Create initial nodes organized in groups
    let initial_nodes = vec![
        // Group 1: Data Processing (green) - top left
        Node::new("data-1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Data Source",
                "group": "data",
                "nodeType": "input"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("data-2".to_string(), Position::new(50.0, 140.0))
            .with_data(json!({
                "label": "Transform",
                "group": "data",
                "nodeType": "default"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("data-3".to_string(), Position::new(50.0, 230.0))
            .with_data(json!({
                "label": "Filter",
                "group": "data",
                "nodeType": "output"
            }))
            .with_dimensions(120.0, 50.0),

        // Group 2: Compute (indigo) - center
        Node::new("compute-1".to_string(), Position::new(250.0, 80.0))
            .with_data(json!({
                "label": "Process A",
                "group": "compute",
                "nodeType": "input"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("compute-2".to_string(), Position::new(250.0, 170.0))
            .with_data(json!({
                "label": "Process B",
                "group": "compute",
                "nodeType": "default"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("compute-3".to_string(), Position::new(250.0, 260.0))
            .with_data(json!({
                "label": "Merge",
                "group": "compute",
                "nodeType": "output"
            }))
            .with_dimensions(120.0, 50.0),

        // Group 3: Output (amber) - right
        Node::new("output-1".to_string(), Position::new(450.0, 100.0))
            .with_data(json!({
                "label": "Export CSV",
                "group": "output",
                "nodeType": "input"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("output-2".to_string(), Position::new(450.0, 200.0))
            .with_data(json!({
                "label": "Export JSON",
                "group": "output",
                "nodeType": "output"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    // Create edges within each group
    let initial_edges = vec![
        // Data group edges
        Edge::new("e-data-1-2".to_string(), "data-1".to_string(), "data-2".to_string())
            .with_data(json!({"group": "data"})),
        Edge::new("e-data-2-3".to_string(), "data-2".to_string(), "data-3".to_string())
            .with_data(json!({"group": "data"})),

        // Compute group edges
        Edge::new("e-compute-1-2".to_string(), "compute-1".to_string(), "compute-2".to_string())
            .with_data(json!({"group": "compute"})),
        Edge::new("e-compute-2-3".to_string(), "compute-2".to_string(), "compute-3".to_string())
            .with_data(json!({"group": "compute"})),

        // Output group edges
        Edge::new("e-output-1-2".to_string(), "output-1".to_string(), "output-2".to_string())
            .with_data(json!({"group": "output"})),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Track selected nodes
    let selected_nodes = RwSignal::new(HashSet::<String>::new());

    // Track selected group for quick actions
    let selected_group = RwSignal::new(Option::<String>::None);

    // Action log
    let action_log = RwSignal::new(Vec::<(f64, String)>::new());

    // Helper to log actions
    let add_log = move |message: String| {
        let timestamp = js_sys::Date::now();
        action_log.update(|log| {
            log.insert(0, (timestamp, message));
            if log.len() > 15 {
                log.pop();
            }
        });
    };

    // Global drag handlers
    let drag_signal = get_multi_set_drag_signal();

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

    let add_log_for_mouseup = add_log.clone();
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
    let add_log_for_bg = add_log.clone();
    let on_background_click = move |ev: leptos::ev::MouseEvent| {
        let target = ev.target();
        if let Some(el) = target {
            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                let class_list = html_el.class_list();
                if class_list.contains("xyflow__viewport") ||
                   class_list.contains("leptos-flow") ||
                   class_list.contains("xyflow__background") {
                    let prev_count = selected_nodes.get().len();
                    if prev_count > 0 {
                        selected_nodes.set(HashSet::new());
                        selected_group.set(None);
                        add_log_for_bg(format!("Cleared {} selected nodes", prev_count));
                    }
                }
            }
        }
    };

    // Select all in group
    let add_log_for_select_group = add_log.clone();
    let select_group = move |group_id: String| {
        let nodes = store.get_nodes();
        let group_nodes: HashSet<String> = nodes.iter()
            .filter(|n| {
                n.data.get("group")
                    .and_then(|v| v.as_str())
                    .map(|g| g == group_id)
                    .unwrap_or(false)
            })
            .map(|n| n.id.clone())
            .collect();
        let count = group_nodes.len();
        selected_nodes.set(group_nodes);
        selected_group.set(Some(group_id.clone()));
        add_log_for_select_group(format!("Selected {} group ({} nodes)", group_id, count));
    };

    // Clear selection
    let add_log_for_clear = add_log.clone();
    let clear_selection = move |_| {
        let prev_count = selected_nodes.get().len();
        selected_nodes.set(HashSet::new());
        selected_group.set(None);
        if prev_count > 0 {
            add_log_for_clear(format!("Cleared {} selected nodes", prev_count));
        }
    };

    // Select all nodes
    let add_log_for_all = add_log.clone();
    let select_all = move |_| {
        let nodes = store.get_nodes();
        let all_ids: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();
        let count = all_ids.len();
        selected_nodes.set(all_ids);
        selected_group.set(None);
        add_log_for_all(format!("Selected all {} nodes", count));
    };

    let groups = get_groups();

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
                    <MultiSetEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        let add_log_inner = add_log.clone();
                        store.get_nodes().into_iter().map(|node| {
                            let add_log_for_node = add_log_inner.clone();
                            view! {
                                <MultiSetNode
                                    node=node.clone()
                                    store=store
                                    selected_nodes=selected_nodes
                                    selected_group=selected_group
                                    on_select=move |node_id, group, shift| {
                                        let prev_selected = selected_nodes.get();
                                        if shift {
                                            // Shift+click: toggle in selection
                                            selected_nodes.update(|selected| {
                                                if selected.contains(&node_id) {
                                                    selected.remove(&node_id);
                                                    add_log_for_node(format!("Removed {} from selection", node_id));
                                                } else {
                                                    selected.insert(node_id.clone());
                                                    add_log_for_node(format!("Added {} to selection", node_id));
                                                }
                                            });
                                            // Check if all selected nodes are same group
                                            let current = selected_nodes.get();
                                            let nodes = store.get_nodes();
                                            let groups_in_selection: HashSet<_> = current.iter()
                                                .filter_map(|id| {
                                                    nodes.iter()
                                                        .find(|n| &n.id == id)
                                                        .and_then(|n| n.data.get("group"))
                                                        .and_then(|v| v.as_str())
                                                        .map(|s| s.to_string())
                                                })
                                                .collect();
                                            if groups_in_selection.len() == 1 {
                                                selected_group.set(groups_in_selection.into_iter().next());
                                            } else {
                                                selected_group.set(None);
                                            }
                                        } else {
                                            // Regular click: select only this node
                                            let mut new_selection = HashSet::new();
                                            new_selection.insert(node_id.clone());
                                            selected_nodes.set(new_selection);
                                            selected_group.set(Some(group.clone()));
                                            add_log_for_node(format!("Selected {} ({})", node_id, group));
                                        }
                                    }
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
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Multi-Set Nodes"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click nodes to select. Hold Shift to multi-select. Click group buttons to select entire groups."
                        </p>

                        // Selection summary
                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Current Selection"</div>
                            <div style="display: flex; gap: 8px; align-items: center; margin-bottom: 8px;">
                                <div style="background: #dbeafe; padding: 8px 12px; border-radius: 6px; text-align: center; flex: 1;">
                                    <div style="font-size: 24px; font-weight: 700; color: #2563eb;">
                                        {move || selected_nodes.get().len()}
                                    </div>
                                    <div style="font-size: 10px; color: #3b82f6; font-weight: 500;">"Nodes"</div>
                                </div>
                                {move || {
                                    if let Some(group_id) = selected_group.get() {
                                        let group = get_groups().into_iter().find(|g| g.id == group_id);
                                        if let Some(g) = group {
                                            view! {
                                                <div style=format!(
                                                    "background: {}20; padding: 8px 12px; border-radius: 6px; text-align: center; flex: 1; border: 2px solid {};",
                                                    g.color, g.color
                                                )>
                                                    <div style=format!("font-size: 12px; font-weight: 700; color: {};", g.color)>
                                                        {g.name}
                                                    </div>
                                                    <div style="font-size: 10px; color: #666; font-weight: 500;">"Group"</div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    } else {
                                        view! {
                                            <div style="background: #f5f5f5; padding: 8px 12px; border-radius: 6px; text-align: center; flex: 1;">
                                                <div style="font-size: 12px; font-weight: 600; color: #999;">"Mixed"</div>
                                                <div style="font-size: 10px; color: #999; font-weight: 500;">"Group"</div>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // Group selection buttons
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Select by Group"</div>
                            <div style="display: flex; flex-direction: column; gap: 6px;">
                                {groups.clone().into_iter().map(|group| {
                                    let group_id = group.id.to_string();
                                    let select_group = select_group.clone();
                                    view! {
                                        <button
                                            style=format!(
                                                "display: flex; align-items: center; gap: 8px; padding: 8px 10px; \
                                                 border: 2px solid {}; border-radius: 6px; background: {}10; \
                                                 cursor: pointer; transition: all 0.15s; text-align: left;",
                                                group.color, group.color
                                            )
                                            on:click=move |_| select_group(group_id.clone())
                                        >
                                            <div style=format!(
                                                "width: 12px; height: 12px; border-radius: 3px; background: {};",
                                                group.color
                                            )></div>
                                            <div style="flex: 1;">
                                                <div style=format!("font-size: 11px; font-weight: 600; color: {};", group.color)>
                                                    {group.name}
                                                </div>
                                                <div style="font-size: 9px; color: #666;">{group.description}</div>
                                            </div>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Quick actions
                        <div style="display: flex; gap: 4px; margin-bottom: 12px;">
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 10px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=select_all
                            >
                                "Select All"
                            </button>
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 10px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=clear_selection
                            >
                                "Clear"
                            </button>
                        </div>

                        // Action log
                        <div style="border-top: 1px solid #eee; padding-top: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Action Log"</div>
                            <div style="background: #f8f9fa; border-radius: 6px; padding: 8px; max-height: 120px; overflow-y: auto;">
                                {move || {
                                    let log = action_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                                "Actions will appear here"
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.into_iter().enumerate().map(|(idx, (timestamp, message))| {
                                            let date = js_sys::Date::new(&leptos::wasm_bindgen::JsValue::from_f64(timestamp));
                                            let time = format!(
                                                "{:02}:{:02}:{:02}",
                                                date.get_hours(),
                                                date.get_minutes(),
                                                date.get_seconds()
                                            );
                                            let bg = if idx == 0 { "#eef2ff" } else { "transparent" };
                                            view! {
                                                <div style=format!(
                                                    "padding: 4px 6px; background: {}; font-size: 10px; \
                                                     border-bottom: 1px solid #eee;",
                                                    bg
                                                )>
                                                    <span style="color: #999; font-family: monospace; font-size: 9px; margin-right: 6px;">
                                                        {time}
                                                    </span>
                                                    <span style="color: #333;">{message}</span>
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // Node groups legend
                        <div style="border-top: 1px solid #eee; padding-top: 12px; margin-top: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Node Groups"</div>
                            <div style="display: flex; flex-direction: column; gap: 4px;">
                                {groups.into_iter().map(|group| {
                                    view! {
                                        <div style="display: flex; align-items: center; gap: 6px; font-size: 10px;">
                                            <div style=format!(
                                                "width: 10px; height: 10px; border-radius: 2px; background: {};",
                                                group.color
                                            )></div>
                                            <span style="color: #333; font-weight: 500;">{group.name}</span>
                                            <span style="color: #999;">"-"</span>
                                            <span style="color: #666;">{group.description}</span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Multi-set node component
#[component]
fn MultiSetNode<F>(
    node: Node,
    store: FlowStore,
    selected_nodes: RwSignal<HashSet<String>>,
    selected_group: RwSignal<Option<String>>,
    on_select: F,
) -> impl IntoView
where
    F: Fn(String, String, bool) + Clone + Send + Sync + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    let drag_signal = get_multi_set_drag_signal();

    // Get node group
    let group_id = node.data.get("group")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let group_id_for_select = group_id.clone();
    let group_id_for_style = group_id.clone();

    // Get group color
    let groups = get_groups();
    let group = groups.iter().find(|g| g.id == group_id);
    let group_color = group.map(|g| g.color).unwrap_or("#888");
    let group_color_string = group_color.to_string();

    // Mouse down - start dragging and select
    let on_select_clone = on_select.clone();
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let shift_key = ev.shift_key();
        let current_id = node_id_for_select.clone();
        let current_group = group_id_for_select.clone();

        // Call selection handler
        on_select_clone(current_id.clone(), current_group.clone(), shift_key);

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

    view! {
        <div
            class="xyflow__node multi-set-node"
            style=move || {
                let nodes = store.get_nodes();
                let is_selected = selected_nodes.get().contains(&node_id_for_style);
                let current_selected_group = selected_group.get();
                let is_group_selected = current_selected_group.as_ref() == Some(&group_id_for_style);

                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let node_type = n.data.get("nodeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let border_color = if is_selected { &group_color_string } else { "#ddd" };
                    let border_width = if is_selected { "3px" } else { "2px" };
                    let box_shadow = if is_selected {
                        format!("0 0 0 3px {}40, 0 4px 12px rgba(0,0,0,0.15)", group_color_string)
                    } else if is_group_selected {
                        format!("0 0 0 1px {}30, 0 2px 6px rgba(0,0,0,0.1)", group_color_string)
                    } else {
                        "0 2px 4px rgba(0,0,0,0.1)".to_string()
                    };
                    let background = if is_selected {
                        format!("{}15", group_color_string)
                    } else {
                        "white".to_string()
                    };

                    // Different border styles for node types
                    let border_style = match node_type {
                        "input" => "solid",
                        "output" => "dashed",
                        _ => "solid",
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: {} {} {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: all 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(120.0), n.height.unwrap_or(50.0),
                        background, border_width, border_style, border_color, box_shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_mousedown
        >
            // Group indicator dot
            <div style=format!(
                "position: absolute; top: -4px; right: -4px; width: 12px; height: 12px; \
                 border-radius: 50%; background: {}; border: 2px solid white; \
                 box-shadow: 0 1px 3px rgba(0,0,0,0.2);",
                group_color
            )></div>

            // Node label
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_label) {
                    let label = n.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();

                    view! {
                        <div style=format!("font-weight: 600; font-size: 11px; color: {}; text-align: center;", group_color)>
                            {label}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Handles
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
                    let node_type = n.data.get("nodeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");
                    let has_source = node_type != "output";
                    let has_target = node_type != "input";

                    view! {
                        <>
                            {has_target.then(|| view! {
                                <Handle
                                    node_id=node_id.clone()
                                    r#type=HandleType::Target
                                    position=HandlePosition::Top
                                    connection_mode=ConnectionMode::Strict
                                    style=format!(
                                        "background: {}; width: 10px; height: 10px; border: 2px solid white; \
                                         box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
                                        group_color
                                    )
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node_id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Bottom
                                    connection_mode=ConnectionMode::Strict
                                    style=format!(
                                        "background: {}; width: 10px; height: 10px; border: 2px solid white; \
                                         box-shadow: 0 1px 4px rgba(0,0,0,0.2);",
                                        group_color
                                    )
                                />
                            })}
                        </>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Edge renderer for multi-set nodes
#[component]
fn MultiSetEdgeRenderer(store: FlowStore) -> impl IntoView {
    let groups = get_groups();

    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                // Create arrow markers for each group
                {groups.iter().map(|group| {
                    let marker_id = format!("multi-set-arrow-{}", group.id);
                    let color = group.color;
                    view! {
                        <marker
                            id=marker_id
                            viewBox="0 0 10 10"
                            refX="8"
                            refY="5"
                            markerWidth="6"
                            markerHeight="6"
                            orient="auto-start-reverse"
                        >
                            <path d="M 0 0 L 10 5 L 0 10 z" fill=color />
                        </marker>
                    }
                }).collect_view()}

                // Default arrow
                <marker
                    id="multi-set-arrow-default"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#888" />
                </marker>

                // Gradients for each group
                {groups.iter().map(|group| {
                    let gradient_id = format!("multi-set-gradient-{}", group.id);
                    let color = group.color;
                    view! {
                        <linearGradient id=gradient_id x1="0%" y1="0%" x2="100%" y2="100%">
                            <stop offset="0%" style=format!("stop-color:{};stop-opacity:0.8", color) />
                            <stop offset="100%" style=format!("stop-color:{};stop-opacity:1", color) />
                        </linearGradient>
                    }
                }).collect_view()}
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Get edge group from edge data or source node
                    let edge_group = edge.data.get("group")
                        .and_then(|v| v.as_str())
                        .or_else(|| source_node.data.get("group").and_then(|v| v.as_str()))
                        .unwrap_or("default");

                    let groups = get_groups();
                    let group = groups.iter().find(|g| g.id == edge_group);
                    let color = group.map(|g| g.color).unwrap_or("#888");
                    let gradient_id = format!("url(#multi-set-gradient-{})", edge_group);
                    let marker_id = format!("url(#multi-set-arrow-{})", edge_group);

                    // Calculate edge path
                    let sx = source_node.position.x + source_node.width.unwrap_or(120.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(120.0) / 2.0;
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
                            // Glow effect
                            <path
                                d=path.clone()
                                stroke=format!("{}30", color)
                                stroke-width="8"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path
                                stroke=gradient_id
                                stroke-width="2.5"
                                fill="none"
                                marker-end=marker_id
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
