//! Figma Example
//!
//! Demonstrates Figma-like interaction patterns:
//! - Selection box on empty canvas drag
//! - Multi-select nodes within box
//! - Drag selection to move all selected
//! - Shift-click to add to selection

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use std::collections::HashSet;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Global drag state for Figma example
static FIGMA_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_figma_drag_signal() -> RwSignal<Option<DragState>> {
    *FIGMA_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Selection box state for marquee selection
#[derive(Clone, Debug, Default)]
struct SelectionBoxState {
    /// Is the user currently drawing a selection box
    is_selecting: bool,
    /// Start position in flow coordinates
    start_x: f64,
    start_y: f64,
    /// Current position in flow coordinates
    current_x: f64,
    current_y: f64,
}

/// Global selection box state
static SELECTION_BOX_STATE: std::sync::OnceLock<RwSignal<SelectionBoxState>> = std::sync::OnceLock::new();

fn get_selection_box_signal() -> RwSignal<SelectionBoxState> {
    *SELECTION_BOX_STATE.get_or_init(|| RwSignal::new(SelectionBoxState::default()))
}

/// Multi-drag state for moving multiple selected nodes
#[derive(Clone, Debug)]
struct MultiDragState {
    /// IDs of nodes being dragged
    node_ids: HashSet<String>,
    /// Start mouse position
    start_mouse: (f64, f64),
    /// Start positions for each node
    start_positions: Vec<(String, f64, f64)>,
}

static MULTI_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<MultiDragState>>> = std::sync::OnceLock::new();

fn get_multi_drag_signal() -> RwSignal<Option<MultiDragState>> {
    *MULTI_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Action Log
// ============================================================================

#[derive(Clone, Debug)]
struct ActionEvent {
    timestamp: f64,
    action: String,
    details: String,
}

// ============================================================================
// Figma Example Component
// ============================================================================

/// Figma example - Figma-like selection and interaction patterns
#[component]
pub fn FigmaExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 80.0))
            .with_data(json!({
                "label": "Node A",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("2".to_string(), Position::new(280.0, 60.0))
            .with_data(json!({
                "label": "Node B",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("3".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({
                "label": "Node C",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("4".to_string(), Position::new(280.0, 180.0))
            .with_data(json!({
                "label": "Node D",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("5".to_string(), Position::new(460.0, 120.0))
            .with_data(json!({
                "label": "Node E",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("6".to_string(), Position::new(180.0, 320.0))
            .with_data(json!({
                "label": "Node F",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("A to B".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("A to C".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("B to D".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("C to D".to_string()),
        Edge::new("e4-5".to_string(), "4".to_string(), "5".to_string())
            .with_label("D to E".to_string()),
        Edge::new("e3-6".to_string(), "3".to_string(), "6".to_string())
            .with_label("C to F".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Track selected nodes
    let selected_nodes = RwSignal::new(HashSet::<String>::new());

    // Action log
    let action_log = RwSignal::new(Vec::<ActionEvent>::new());

    // Add action to log
    let add_action = move |action: &str, details: &str| {
        action_log.update(|log| {
            log.insert(0, ActionEvent {
                timestamp: js_sys::Date::now(),
                action: action.to_string(),
                details: details.to_string(),
            });
            if log.len() > 10 {
                log.pop();
            }
        });
    };

    // Get signals
    let drag_signal = get_figma_drag_signal();
    let selection_box = get_selection_box_signal();
    let multi_drag = get_multi_drag_signal();

    // Mouse down on background - start selection box
    let on_background_mousedown = move |ev: leptos::ev::MouseEvent| {
        let target = ev.target();
        let is_on_background = if let Some(el) = target {
            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                let class_list = html_el.class_list();
                class_list.contains("xyflow__viewport") ||
                class_list.contains("leptos-flow") ||
                class_list.contains("xyflow__background") ||
                class_list.contains("figma-background")
            } else {
                false
            }
        } else {
            false
        };

        if is_on_background && drag_signal.get().is_none() && multi_drag.get().is_none() {
            ev.prevent_default();

            // Convert screen coordinates to flow coordinates
            let viewport = store.get_viewport();
            let flow_x = (ev.offset_x() as f64 - viewport.x) / viewport.zoom;
            let flow_y = (ev.offset_y() as f64 - viewport.y) / viewport.zoom;

            // If not shift-clicking, clear selection first
            if !ev.shift_key() {
                selected_nodes.set(HashSet::new());
            }

            // Start selection box
            selection_box.set(SelectionBoxState {
                is_selecting: true,
                start_x: flow_x,
                start_y: flow_y,
                current_x: flow_x,
                current_y: flow_y,
            });

            add_action("Selection Box", "Started");
        }
    };

    // Global mouse move handler
    let on_global_mousemove = {
        let add_action = add_action.clone();
        move |ev: leptos::ev::MouseEvent| {
            // Handle single node drag
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

            // Handle multi-node drag
            if let Some(multi_state) = multi_drag.get() {
                let current_x = ev.client_x() as f64;
                let current_y = ev.client_y() as f64;
                let (start_x, start_y) = multi_state.start_mouse;

                let viewport = store.get_viewport();
                let dx = (current_x - start_x) / viewport.zoom;
                let dy = (current_y - start_y) / viewport.zoom;

                for (node_id, start_pos_x, start_pos_y) in multi_state.start_positions.iter() {
                    store.update_node(node_id, |n| {
                        n.position = Position::new(start_pos_x + dx, start_pos_y + dy);
                    });
                }
            }

            // Handle selection box drag
            let box_state = selection_box.get();
            if box_state.is_selecting {
                let viewport = store.get_viewport();
                let flow_x = (ev.offset_x() as f64 - viewport.x) / viewport.zoom;
                let flow_y = (ev.offset_y() as f64 - viewport.y) / viewport.zoom;

                selection_box.update(|state| {
                    state.current_x = flow_x;
                    state.current_y = flow_y;
                });

                // Find nodes within selection box
                let nodes = store.get_nodes();
                let box_state = selection_box.get();

                let box_left = box_state.start_x.min(box_state.current_x);
                let box_right = box_state.start_x.max(box_state.current_x);
                let box_top = box_state.start_y.min(box_state.current_y);
                let box_bottom = box_state.start_y.max(box_state.current_y);

                let nodes_in_box: HashSet<String> = nodes.iter()
                    .filter(|n| {
                        let node_left = n.position.x;
                        let node_right = n.position.x + n.width.unwrap_or(120.0);
                        let node_top = n.position.y;
                        let node_bottom = n.position.y + n.height.unwrap_or(50.0);

                        // Check if node intersects with selection box
                        node_left < box_right &&
                        node_right > box_left &&
                        node_top < box_bottom &&
                        node_bottom > box_top
                    })
                    .map(|n| n.id.clone())
                    .collect();

                // If shift is held, add to existing selection
                if ev.shift_key() {
                    selected_nodes.update(|sel| {
                        for id in nodes_in_box {
                            sel.insert(id);
                        }
                    });
                } else {
                    selected_nodes.set(nodes_in_box);
                }
            }
        }
    };

    // Global mouse up handler
    let on_global_mouseup = {
        let add_action = add_action.clone();
        move |_ev: leptos::ev::MouseEvent| {
            // End single node drag
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
                add_action("Drag", &format!("Ended for {}", node_id));
            }

            // End multi-node drag
            if let Some(multi_state) = multi_drag.get() {
                let count = multi_state.node_ids.len();
                multi_drag.set(None);
                add_action("Multi-Drag", &format!("Ended for {} nodes", count));
            }

            // End selection box
            let box_state = selection_box.get();
            if box_state.is_selecting {
                let count = selected_nodes.get().len();
                selection_box.set(SelectionBoxState::default());
                add_action("Selection Box", &format!("Selected {} nodes", count));
            }
        }
    };

    // Clear selection handler
    let clear_selection = {
        let add_action = add_action.clone();
        move |_| {
            selected_nodes.set(HashSet::new());
            add_action("Selection", "Cleared");
        }
    };

    // Select all handler
    let select_all = {
        let add_action = add_action.clone();
        move |_| {
            let all_ids: HashSet<String> = store.get_nodes().iter().map(|n| n.id.clone()).collect();
            let count = all_ids.len();
            selected_nodes.set(all_ids);
            add_action("Selection", &format!("Selected all {} nodes", count));
        }
    };

    // Clear log handler
    let clear_log = move |_| {
        action_log.set(vec![]);
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow figma-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousedown=on_background_mousedown
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Figma-style selection box overlay
                {move || {
                    let box_state = selection_box.get();
                    if box_state.is_selecting {
                        let viewport = store.get_viewport();
                        let left = box_state.start_x.min(box_state.current_x);
                        let top = box_state.start_y.min(box_state.current_y);
                        let width = (box_state.current_x - box_state.start_x).abs();
                        let height = (box_state.current_y - box_state.start_y).abs();

                        // Convert to screen coordinates
                        let screen_left = left * viewport.zoom + viewport.x;
                        let screen_top = top * viewport.zoom + viewport.y;
                        let screen_width = width * viewport.zoom;
                        let screen_height = height * viewport.zoom;

                        Some(view! {
                            <div
                                class="figma-selection-box"
                                style=format!(
                                    "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; \
                                     border: 2px dashed #0ea5e9; background: rgba(14, 165, 233, 0.1); \
                                     pointer-events: none; z-index: 1000;",
                                    screen_left, screen_top, screen_width, screen_height
                                )
                            />
                        })
                    } else {
                        None
                    }
                }}

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <FigmaEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <FigmaNode
                                    node=node.clone()
                                    store=store
                                    selected_nodes=selected_nodes
                                    multi_drag=multi_drag
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Instructions badge
                <div style="position: absolute; top: 10px; left: 10px; background: linear-gradient(135deg, #0ea5e9 0%, #6366f1 100%); color: white; padding: 8px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; box-shadow: 0 2px 8px rgba(0,0,0,0.2);">
                    "Figma-Style Interactions"
                </div>

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Figma Interactions"</strong>

                        // Instructions
                        <div style="background: #f0f9ff; padding: 10px; border-radius: 6px; margin-bottom: 12px; font-size: 11px; color: #0369a1; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 6px;">"How to use:"</div>
                            <ul style="margin: 0; padding-left: 16px;">
                                <li>"Drag on canvas to draw selection box"</li>
                                <li>"Shift+drag to add to selection"</li>
                                <li>"Click node to select it"</li>
                                <li>"Shift+click to toggle in selection"</li>
                                <li>"Drag selected nodes to move them together"</li>
                            </ul>
                        </div>

                        // Selection summary
                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Selection Status"</div>
                            <div style="display: flex; align-items: center; gap: 8px;">
                                <div style="flex: 1; background: #dbeafe; padding: 10px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 24px; font-weight: 700; color: #2563eb;">
                                        {move || selected_nodes.get().len()}
                                    </div>
                                    <div style="font-size: 10px; color: #3b82f6; font-weight: 500;">"Selected"</div>
                                </div>
                                <div style="flex: 1; background: #f3e8ff; padding: 10px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 24px; font-weight: 700; color: #7c3aed;">
                                        {move || store.get_nodes().len()}
                                    </div>
                                    <div style="font-size: 10px; color: #8b5cf6; font-weight: 500;">"Total"</div>
                                </div>
                            </div>
                        </div>

                        // Selected nodes list
                        {move || {
                            let nodes = selected_nodes.get();
                            let all_nodes = store.get_nodes();

                            if nodes.is_empty() {
                                view! {
                                    <div style="background: #f5f5f5; padding: 12px; border-radius: 6px; text-align: center; color: #999; font-size: 11px; margin-bottom: 12px;">
                                        "No nodes selected"
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="background: #f8fafc; padding: 10px; border-radius: 6px; margin-bottom: 12px; max-height: 100px; overflow-y: auto;">
                                        <div style="font-size: 10px; font-weight: 600; color: #333; margin-bottom: 6px;">"Selected Nodes"</div>
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
                                                     padding: 4px 6px; background: {}15; border-radius: 4px; margin-bottom: 4px;",
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
                                    </div>
                                }.into_any()
                            }
                        }}

                        // Quick actions
                        <div style="display: flex; gap: 4px; margin-bottom: 12px;">
                            <button
                                style="flex: 1; padding: 8px; font-size: 10px; border: none; \
                                       border-radius: 4px; background: #2563eb; color: white; \
                                       cursor: pointer; font-weight: 500;"
                                on:click=select_all
                            >
                                "Select All"
                            </button>
                            <button
                                style="flex: 1; padding: 8px; font-size: 10px; border: 1px solid #ddd; \
                                       border-radius: 4px; background: white; cursor: pointer; font-weight: 500;"
                                on:click=clear_selection
                            >
                                "Clear"
                            </button>
                        </div>

                        // Action log
                        <div style="border-top: 1px solid #eee; padding-top: 12px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                <div style="font-size: 11px; font-weight: 600; color: #333;">"Action Log"</div>
                                <button
                                    style="font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                                    on:click=clear_log
                                >
                                    "Clear"
                                </button>
                            </div>
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
                                        let log_len = log.len();
                                        log.into_iter().enumerate().map(|(idx, event)| {
                                            let date = js_sys::Date::new(&leptos::wasm_bindgen::JsValue::from_f64(event.timestamp));
                                            let time = format!(
                                                "{:02}:{:02}:{:02}",
                                                date.get_hours(),
                                                date.get_minutes(),
                                                date.get_seconds()
                                            );

                                            let bg_color = if idx == 0 { "#eef2ff" } else { "transparent" };
                                            let border = if idx < log_len - 1 { "1px solid #eee" } else { "none" };

                                            // Color based on action type
                                            let action_color = match event.action.as_str() {
                                                "Selection Box" => "#0ea5e9",
                                                "Selection" => "#6366f1",
                                                "Drag" => "#10b981",
                                                "Multi-Drag" => "#8b5cf6",
                                                _ => "#666",
                                            };

                                            view! {
                                                <div style=format!(
                                                    "padding: 6px; background: {}; border-bottom: {}; font-size: 10px;",
                                                    bg_color, border
                                                )>
                                                    <div style="display: flex; justify-content: space-between; align-items: center;">
                                                        <span style=format!(
                                                            "font-weight: 600; color: {}; font-size: 10px;",
                                                            action_color
                                                        )>
                                                            {event.action.clone()}
                                                        </span>
                                                        <span style="color: #999; font-family: monospace; font-size: 9px;">{time}</span>
                                                    </div>
                                                    <div style="color: #666; font-size: 9px; margin-top: 2px;">
                                                        {event.details.clone()}
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

// ============================================================================
// Figma Node Component
// ============================================================================

/// Figma-style selectable and draggable node
#[component]
fn FigmaNode(
    node: Node,
    store: FlowStore,
    selected_nodes: RwSignal<HashSet<String>>,
    multi_drag: RwSignal<Option<MultiDragState>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_multi = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    let drag_signal = get_figma_drag_signal();

    // Mouse down handler - select and possibly start drag
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let shift_key = ev.shift_key();
        let current_id = node_id_for_select.clone();
        let is_already_selected = selected_nodes.get().contains(&current_id);

        // Update selection
        selected_nodes.update(|selected| {
            if shift_key {
                // Toggle selection in multi-select mode
                if selected.contains(&current_id) {
                    selected.remove(&current_id);
                } else {
                    selected.insert(current_id.clone());
                }
            } else if !is_already_selected {
                // Single select - replace selection
                selected.clear();
                selected.insert(current_id.clone());
            }
            // If already selected and no shift, keep selection (for multi-drag)
        });

        // Determine if we should do multi-drag or single drag
        let selected = selected_nodes.get();
        if selected.len() > 1 && selected.contains(&node_id_for_multi) {
            // Start multi-drag
            let nodes = store.get_nodes();
            let start_positions: Vec<(String, f64, f64)> = nodes.iter()
                .filter(|n| selected.contains(&n.id))
                .map(|n| (n.id.clone(), n.position.x, n.position.y))
                .collect();

            multi_drag.set(Some(MultiDragState {
                node_ids: selected.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_positions,
            }));
        } else {
            // Start single drag
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
        }
    };

    view! {
        <div
            class="xyflow__node figma-node"
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

                    // Figma-style selection: blue border for selected items
                    let border = if is_selected {
                        "2px solid #0ea5e9".to_string()
                    } else {
                        format!("2px solid {}40", color)
                    };

                    let box_shadow = if is_selected {
                        "0 0 0 2px rgba(14, 165, 233, 0.3), 0 4px 12px rgba(0,0,0,0.15)".to_string()
                    } else {
                        "0 2px 6px rgba(0,0,0,0.1)".to_string()
                    };

                    // Background based on node type
                    let background = match node_type {
                        "input" => format!("linear-gradient(135deg, {}20 0%, {}40 100%)", color, color),
                        "output" => format!("linear-gradient(135deg, {}20 0%, {}40 100%)", color, color),
                        _ => "white".to_string(),
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: box-shadow 0.15s, border 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(120.0), n.height.unwrap_or(50.0),
                        background, border, box_shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_mousedown
        >
            // Selection indicator (Figma-style corner resize handles shown when selected)
            {move || {
                let is_selected = selected_nodes.get().contains(&node_id);
                is_selected.then(|| view! {
                    // Corner indicators
                    <div style="position: absolute; top: -4px; left: -4px; width: 8px; height: 8px; background: #0ea5e9; border-radius: 2px; pointer-events: none;"></div>
                    <div style="position: absolute; top: -4px; right: -4px; width: 8px; height: 8px; background: #0ea5e9; border-radius: 2px; pointer-events: none;"></div>
                    <div style="position: absolute; bottom: -4px; left: -4px; width: 8px; height: 8px; background: #0ea5e9; border-radius: 2px; pointer-events: none;"></div>
                    <div style="position: absolute; bottom: -4px; right: -4px; width: 8px; height: 8px; background: #0ea5e9; border-radius: 2px; pointer-events: none;"></div>
                })
            }}

            // Node label
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_label) {
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

// ============================================================================
// Figma Edge Renderer
// ============================================================================

/// Edge renderer for Figma example
#[component]
fn FigmaEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="figma-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#6366f1" />
                    <stop offset="100%" stop-color="#8b5cf6" />
                </linearGradient>
                <marker
                    id="figma-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#8b5cf6" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

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

                    let label = edge.label.clone().unwrap_or_default();

                    Some(view! {
                        <g class="xyflow__edge">
                            // Shadow/glow
                            <path
                                d=path.clone()
                                stroke="#6366f140"
                                stroke-width="6"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path.clone()
                                stroke="url(#figma-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#figma-arrow)"
                            />
                            // Label
                            {(!label.is_empty()).then(|| view! {
                                <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                    <rect
                                        x="-28"
                                        y="-10"
                                        width="56"
                                        height="20"
                                        fill="white"
                                        stroke="#e5e7eb"
                                        stroke-width="1"
                                        rx="4"
                                    />
                                    <text
                                        x="0"
                                        y="4"
                                        text-anchor="middle"
                                        font-size="9"
                                        fill="#666"
                                    >
                                        {label}
                                    </text>
                                </g>
                            })}
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
