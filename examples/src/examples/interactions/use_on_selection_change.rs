//! UseOnSelectionChange Example
//!
//! Demonstrates how to react to selection changes:
//! - Implement selection change callback/effect
//! - Log selected nodes and edges to panel
//! - Show selection count
//! - Visual feedback for selected elements

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use std::collections::HashSet;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for selection change example
static SELECTION_CHANGE_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_selection_change_drag_signal() -> RwSignal<Option<DragState>> {
    *SELECTION_CHANGE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Selection change event
#[derive(Clone, Debug)]
struct SelectionChangeEvent {
    timestamp: f64,
    node_count: usize,
    edge_count: usize,
    added_nodes: Vec<String>,
    removed_nodes: Vec<String>,
    added_edges: Vec<String>,
    removed_edges: Vec<String>,
}

/// UseOnSelectionChange example
#[component]
pub fn UseOnSelectionChangeExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(80.0, 80.0))
            .with_data(json!({
                "label": "Node 1",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("2".to_string(), Position::new(250.0, 60.0))
            .with_data(json!({
                "label": "Node 2",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("3".to_string(), Position::new(80.0, 200.0))
            .with_data(json!({
                "label": "Node 3",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("4".to_string(), Position::new(250.0, 180.0))
            .with_data(json!({
                "label": "Node 4",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("5".to_string(), Position::new(420.0, 120.0))
            .with_data(json!({
                "label": "Node 5",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("Edge 1-2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Edge 1-3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("Edge 2-4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("Edge 3-4".to_string()),
        Edge::new("e4-5".to_string(), "4".to_string(), "5".to_string())
            .with_label("Edge 4-5".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Track selected nodes and edges
    let selected_nodes = RwSignal::new(HashSet::<String>::new());
    let selected_edges = RwSignal::new(HashSet::<String>::new());

    // Selection change log
    let selection_log = RwSignal::new(Vec::<SelectionChangeEvent>::new());

    // Previous selection for comparison
    let prev_selected_nodes = RwSignal::new(HashSet::<String>::new());
    let prev_selected_edges = RwSignal::new(HashSet::<String>::new());

    // Total selection change count
    let change_count = RwSignal::new(0_i32);

    // Track selection changes using Effect
    Effect::new(move || {
        let current_nodes = selected_nodes.get();
        let current_edges = selected_edges.get();
        let prev_nodes = prev_selected_nodes.get_untracked();
        let prev_edges = prev_selected_edges.get_untracked();

        // Calculate changes
        let added_nodes: Vec<String> = current_nodes.difference(&prev_nodes).cloned().collect();
        let removed_nodes: Vec<String> = prev_nodes.difference(&current_nodes).cloned().collect();
        let added_edges: Vec<String> = current_edges.difference(&prev_edges).cloned().collect();
        let removed_edges: Vec<String> = prev_edges.difference(&current_edges).cloned().collect();

        // Only log if there are changes
        if !added_nodes.is_empty() || !removed_nodes.is_empty() ||
           !added_edges.is_empty() || !removed_edges.is_empty() {
            let event = SelectionChangeEvent {
                timestamp: js_sys::Date::now(),
                node_count: current_nodes.len(),
                edge_count: current_edges.len(),
                added_nodes,
                removed_nodes,
                added_edges,
                removed_edges,
            };

            selection_log.update(|log| {
                log.insert(0, event);
                if log.len() > 10 {
                    log.pop();
                }
            });

            change_count.update(|c| *c += 1);
        }

        // Update previous selection
        prev_selected_nodes.set(current_nodes);
        prev_selected_edges.set(current_edges);
    });

    // Global drag handlers
    let drag_signal = get_selection_change_drag_signal();

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

    // Click on background to deselect all
    let on_background_click = move |ev: leptos::ev::MouseEvent| {
        let target = ev.target();
        if let Some(el) = target {
            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                let class_list = html_el.class_list();
                if class_list.contains("xyflow__viewport") ||
                   class_list.contains("leptos-flow") ||
                   class_list.contains("xyflow__background") {
                    // Clear all selections
                    selected_nodes.set(HashSet::new());
                    selected_edges.set(HashSet::new());
                }
            }
        }
    };

    // Clear log handler
    let clear_log = move |_| {
        selection_log.set(vec![]);
    };

    // Select all handler
    let select_all_nodes = move |_| {
        let all_node_ids: HashSet<String> = store.get_nodes().iter().map(|n| n.id.clone()).collect();
        selected_nodes.set(all_node_ids);
    };

    let select_all_edges = move |_| {
        let all_edge_ids: HashSet<String> = store.get_edges().iter().map(|e| e.id.clone()).collect();
        selected_edges.set(all_edge_ids);
    };

    let clear_selection = move |_| {
        selected_nodes.set(HashSet::new());
        selected_edges.set(HashSet::new());
    };

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
                    <SelectionChangeEdgeRenderer
                        store=store
                        selected_edges=selected_edges
                    />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <SelectableNode
                                    node=node.clone()
                                    store=store
                                    selected_nodes=selected_nodes
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
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Selection Change"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "Click nodes to select. Hold Shift and click to multi-select. Click edges to select them. The panel below shows selection state changes."
                        </p>

                        // Selection summary
                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Current Selection"</div>
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                                // Nodes count
                                <div style="background: #dbeafe; padding: 8px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 20px; font-weight: 700; color: #2563eb;">
                                        {move || selected_nodes.get().len()}
                                    </div>
                                    <div style="font-size: 10px; color: #3b82f6; font-weight: 500;">"Nodes"</div>
                                </div>
                                // Edges count
                                <div style="background: #dcfce7; padding: 8px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 20px; font-weight: 700; color: #16a34a;">
                                        {move || selected_edges.get().len()}
                                    </div>
                                    <div style="font-size: 10px; color: #22c55e; font-weight: 500;">"Edges"</div>
                                </div>
                            </div>

                            // Total change count
                            <div style="margin-top: 8px; padding-top: 8px; border-top: 1px solid #e2e8f0; text-align: center;">
                                <span style="font-size: 10px; color: #64748b;">"Total Selection Changes: "</span>
                                <span style="font-size: 12px; font-weight: 600; color: #6366f1;">{move || change_count.get()}</span>
                            </div>
                        </div>

                        // Selected items list
                        {move || {
                            let nodes = selected_nodes.get();
                            let edges = selected_edges.get();
                            let all_nodes = store.get_nodes();
                            let all_edges = store.get_edges();

                            if nodes.is_empty() && edges.is_empty() {
                                view! {
                                    <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; text-align: center; color: #999; font-size: 11px; margin-bottom: 12px;">
                                        "Click to select nodes and edges"
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="background: #f8fafc; padding: 10px; border-radius: 6px; margin-bottom: 12px; max-height: 100px; overflow-y: auto;">
                                        <div style="font-size: 10px; font-weight: 600; color: #333; margin-bottom: 6px;">"Selected Items"</div>
                                        // Nodes
                                        {nodes.iter().map(|node_id| {
                                            let label = all_nodes.iter()
                                                .find(|n| &n.id == node_id)
                                                .and_then(|n| n.data.get("label"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Node")
                                                .to_string();
                                            let color = all_nodes.iter()
                                                .find(|n| &n.id == node_id)
                                                .and_then(|n| n.data.get("color"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("#6366f1")
                                                .to_string();
                                            view! {
                                                <div style=format!(
                                                    "display: flex; align-items: center; gap: 6px; \
                                                     padding: 4px 6px; background: {}20; border-radius: 4px; margin-bottom: 4px;",
                                                    color
                                                )>
                                                    <div style=format!(
                                                        "width: 8px; height: 8px; border-radius: 2px; background: {};",
                                                        color
                                                    )></div>
                                                    <span style="font-size: 10px; color: #333; font-weight: 500;">{label}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                        // Edges
                                        {edges.iter().map(|edge_id| {
                                            let label = all_edges.iter()
                                                .find(|e| &e.id == edge_id)
                                                .and_then(|e| e.label.clone())
                                                .unwrap_or_else(|| edge_id.clone());
                                            view! {
                                                <div style="display: flex; align-items: center; gap: 6px; padding: 4px 6px; background: #e0f2fe; border-radius: 4px; margin-bottom: 4px;">
                                                    <div style="width: 16px; height: 2px; background: #0ea5e9;"></div>
                                                    <span style="font-size: 10px; color: #333; font-weight: 500;">{label}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        }}

                        // Quick actions
                        <div style="display: flex; gap: 4px; margin-bottom: 12px;">
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 9px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=select_all_nodes
                            >
                                "All Nodes"
                            </button>
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 9px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=select_all_edges
                            >
                                "All Edges"
                            </button>
                            <button
                                style="flex: 1; padding: 6px 8px; font-size: 9px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer;"
                                on:click=clear_selection
                            >
                                "Clear"
                            </button>
                        </div>

                        // Selection change log
                        <div style="border-top: 1px solid #eee; padding-top: 12px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                <div style="font-size: 11px; font-weight: 600; color: #333;">"Change Log"</div>
                                <button
                                    style="font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                                    on:click=clear_log
                                >
                                    "Clear"
                                </button>
                            </div>
                            <div style="background: #f8f9fa; border-radius: 6px; padding: 8px; max-height: 150px; overflow-y: auto;">
                                {move || {
                                    let log = selection_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                                "Selection changes will appear here"
                                            </div>
                                        }.into_any()
                                    } else {
                                        let log_len = log.len();
                                        log.into_iter().enumerate().map(|(idx, event)| {
                                            // Format timestamp as time only
                                            let date = js_sys::Date::new(&leptos::wasm_bindgen::JsValue::from_f64(event.timestamp));
                                            let time = format!(
                                                "{:02}:{:02}:{:02}",
                                                date.get_hours(),
                                                date.get_minutes(),
                                                date.get_seconds()
                                            );

                                            // Format changes
                                            let mut changes = Vec::new();
                                            if !event.added_nodes.is_empty() {
                                                changes.push(format!("+{} nodes", event.added_nodes.len()));
                                            }
                                            if !event.removed_nodes.is_empty() {
                                                changes.push(format!("-{} nodes", event.removed_nodes.len()));
                                            }
                                            if !event.added_edges.is_empty() {
                                                changes.push(format!("+{} edges", event.added_edges.len()));
                                            }
                                            if !event.removed_edges.is_empty() {
                                                changes.push(format!("-{} edges", event.removed_edges.len()));
                                            }
                                            let change_text = changes.join(", ");

                                            let bg_color = if idx == 0 { "#eef2ff" } else { "transparent" };
                                            let border = if idx < log_len - 1 { "1px solid #eee" } else { "none" };

                                            view! {
                                                <div style=format!(
                                                    "padding: 6px; background: {}; border-bottom: {}; font-size: 10px;",
                                                    bg_color, border
                                                )>
                                                    <div style="display: flex; justify-content: space-between; margin-bottom: 2px;">
                                                        <span style="color: #666; font-family: monospace; font-size: 9px;">{time}</span>
                                                        <span style="color: #999; font-size: 9px;">
                                                            "N:" {event.node_count} " E:" {event.edge_count}
                                                        </span>
                                                    </div>
                                                    <div style="color: #333;">
                                                        {if change_text.is_empty() { "cleared".to_string() } else { change_text }}
                                                    </div>
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

/// Selectable node component
#[component]
fn SelectableNode(
    node: Node,
    store: FlowStore,
    selected_nodes: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_style = node.id.clone();

    let drag_signal = get_selection_change_drag_signal();

    // Mouse down - start dragging and select
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let shift_key = ev.shift_key();
        let current_id = node_id_for_select.clone();

        // Update selection based on shift key
        selected_nodes.update(|selected| {
            if shift_key {
                // Toggle selection in multi-select mode
                if selected.contains(&current_id) {
                    selected.remove(&current_id);
                } else {
                    selected.insert(current_id.clone());
                }
            } else {
                // Single select - replace selection
                selected.clear();
                selected.insert(current_id.clone());
            }
        });

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
            class="xyflow__node selectable-node"
            style=move || {
                let nodes = store.get_nodes();
                let is_selected = selected_nodes.get().contains(&node_id_for_style);
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let border_color = if is_selected { color } else { "#ddd" };
                    let box_shadow = if is_selected {
                        format!("0 0 0 2px {}40, 0 4px 8px rgba(0,0,0,0.15)", color)
                    } else {
                        "0 2px 4px rgba(0,0,0,0.1)".to_string()
                    };
                    let background = if is_selected {
                        format!("{}10", color)
                    } else {
                        "white".to_string()
                    };

                    // Different styling for input/output nodes
                    let border_style = match node_type {
                        "input" => "2px solid",
                        "output" => "2px dashed",
                        _ => "2px solid",
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: {} {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: all 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(120.0), n.height.unwrap_or(50.0),
                        background, border_style, border_color, box_shadow
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

            // Handles
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node.id) {
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");
                    let has_source = node_type != "output";
                    let has_target = node_type != "input";

                    view! {
                        <>
                            {has_target.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Target
                                    position=HandlePosition::Top
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Bottom
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
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

/// Edge renderer with selection support
#[component]
fn SelectionChangeEdgeRenderer(
    store: FlowStore,
    selected_edges: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="selection-change-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#888" />
                </marker>
                <marker
                    id="selection-change-arrow-selected"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#0ea5e9" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let selected = selected_edges.get();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    let is_selected = selected.contains(&edge.id);
                    let edge_id = edge.id.clone();
                    let edge_id_for_click = edge.id.clone();

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

                    // Calculate midpoint for label
                    let mid_x = (sx + tx) / 2.0;
                    let mid_y = (sy + ty) / 2.0;

                    let stroke_color = if is_selected { "#0ea5e9" } else { "#888" };
                    let stroke_width = if is_selected { "3" } else { "2" };
                    let marker = if is_selected { "url(#selection-change-arrow-selected)" } else { "url(#selection-change-arrow)" };

                    let label = edge.label.clone().unwrap_or_else(|| edge_id.clone());

                    // Path clones for closures
                    let path_for_hitbox = path.clone();
                    let path_for_highlight = path.clone();
                    let path_for_main = path.clone();

                    Some(view! {
                        <g class="xyflow__edge" style="pointer-events: visibleStroke;">
                            // Highlight glow for selected edges
                            {is_selected.then(|| view! {
                                <path
                                    d=path_for_highlight
                                    stroke="#0ea5e920"
                                    stroke-width="10"
                                    fill="none"
                                />
                            })}

                            // Invisible hitbox for click detection
                            <path
                                d=path_for_hitbox
                                stroke="transparent"
                                stroke-width="20"
                                fill="none"
                                style="cursor: pointer; pointer-events: stroke;"
                                on:click=move |ev: leptos::ev::MouseEvent| {
                                    ev.stop_propagation();
                                    let shift_key = ev.shift_key();
                                    let current_id = edge_id_for_click.clone();

                                    selected_edges.update(|selected| {
                                        if shift_key {
                                            // Toggle in multi-select mode
                                            if selected.contains(&current_id) {
                                                selected.remove(&current_id);
                                            } else {
                                                selected.insert(current_id.clone());
                                            }
                                        } else {
                                            // Single select
                                            selected.clear();
                                            selected.insert(current_id.clone());
                                        }
                                    });
                                }
                            />

                            // Main edge path
                            <path
                                d=path_for_main
                                stroke=stroke_color
                                stroke-width=stroke_width
                                fill="none"
                                marker-end=marker
                            />

                            // Edge label
                            <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                <rect
                                    x="-30"
                                    y="-10"
                                    width="60"
                                    height="20"
                                    fill=if is_selected { "#e0f2fe" } else { "white" }
                                    stroke=if is_selected { "#0ea5e9" } else { "#ddd" }
                                    stroke-width="1"
                                    rx="4"
                                />
                                <text
                                    x="0"
                                    y="4"
                                    text-anchor="middle"
                                    font-size="10"
                                    fill=if is_selected { "#0284c7" } else { "#666" }
                                    font-weight=if is_selected { "600" } else { "normal" }
                                >
                                    {label}
                                </text>
                            </g>
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
