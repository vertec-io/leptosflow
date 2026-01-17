//! Drag & Drop Example
//!
//! Demonstrates dragging nodes from a sidebar onto the canvas.
//! Uses HTML5 drag and drop API with dragstart, dragover, and drop events.
//! Supports multiple node types in the sidebar.

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Drag state for nodes in the flow
static DRAG_N_DROP_DRAG_STATE: OnceLock<RwSignal<Option<DragState>>> = OnceLock::new();

fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *DRAG_N_DROP_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// State for sidebar drag operation
#[derive(Clone, Debug)]
pub struct SidebarDragState {
    pub node_type: String,
    pub label: String,
    pub color: String,
}

static SIDEBAR_DRAG_STATE: OnceLock<RwSignal<Option<SidebarDragState>>> = OnceLock::new();

fn get_sidebar_drag_signal() -> RwSignal<Option<SidebarDragState>> {
    *SIDEBAR_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Is dragging over the drop zone
static IS_DRAG_OVER: OnceLock<RwSignal<bool>> = OnceLock::new();

fn get_is_drag_over() -> RwSignal<bool> {
    *IS_DRAG_OVER.get_or_init(|| RwSignal::new(false))
}

/// Action log for drag operations
static DRAG_N_DROP_ACTION_LOG: OnceLock<RwSignal<Vec<String>>> = OnceLock::new();

fn get_action_log() -> RwSignal<Vec<String>> {
    *DRAG_N_DROP_ACTION_LOG.get_or_init(|| RwSignal::new(vec!["Drag a node from the sidebar onto the canvas...".to_string()]))
}

fn log_action(action: &str) {
    get_action_log().update(|entries| {
        entries.push(action.to_string());
        if entries.len() > 20 {
            entries.remove(0);
        }
    });
}

/// Node counter for unique IDs
static NODE_COUNTER: OnceLock<RwSignal<u32>> = OnceLock::new();

fn get_node_counter() -> RwSignal<u32> {
    *NODE_COUNTER.get_or_init(|| RwSignal::new(0))
}

// ============================================================================
// Node Template Data
// ============================================================================

#[derive(Clone, Debug)]
pub struct NodeTemplate {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub node_type: &'static str,
    pub color: &'static str,
    pub icon: &'static str,
}

fn get_node_templates() -> Vec<NodeTemplate> {
    vec![
        NodeTemplate {
            id: "input",
            label: "Input",
            description: "Data source node",
            node_type: "input",
            color: "#10b981",
            icon: "I",
        },
        NodeTemplate {
            id: "default",
            label: "Default",
            description: "Standard processing node",
            node_type: "default",
            color: "#6366f1",
            icon: "D",
        },
        NodeTemplate {
            id: "output",
            label: "Output",
            description: "Data destination node",
            node_type: "output",
            color: "#ef4444",
            icon: "O",
        },
        NodeTemplate {
            id: "process",
            label: "Process",
            description: "Data transformation node",
            node_type: "default",
            color: "#8b5cf6",
            icon: "P",
        },
        NodeTemplate {
            id: "decision",
            label: "Decision",
            description: "Branching logic node",
            node_type: "default",
            color: "#f59e0b",
            icon: "?",
        },
        NodeTemplate {
            id: "storage",
            label: "Storage",
            description: "Data storage node",
            node_type: "default",
            color: "#06b6d4",
            icon: "S",
        },
    ]
}

// ============================================================================
// Drag & Drop Example Component
// ============================================================================

/// DragNDrop example - Demonstrates dragging nodes from sidebar onto canvas
#[component]
pub fn DragNDropExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("start".to_string(), Position::new(200.0, 100.0))
            .with_data(json!({
                "label": "Start",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("end".to_string(), Position::new(200.0, 300.0))
            .with_data(json!({
                "label": "End",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    let initial_edges = vec![
        Edge::new("e-start-end".to_string(), "start".to_string(), "end".to_string()),
    ];

    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store.clone());

    let drag_signal = get_drag_signal();
    let sidebar_drag = get_sidebar_drag_signal();
    let is_drag_over = get_is_drag_over();
    let action_log = get_action_log();

    // Reset state on mount
    sidebar_drag.set(None);
    is_drag_over.set(false);
    action_log.set(vec!["Drag a node from the sidebar onto the canvas...".to_string()]);
    get_node_counter().set(0);

    // Global mouse move handler for node dragging
    let on_mousemove = {
        let store = store.clone();
        move |ev: leptos::ev::MouseEvent| {
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
        }
    };

    // Global mouse up handler
    let on_mouseup = {
        let store = store.clone();
        move |_ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                store.update_node(&drag_state.node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
            }
        }
    };

    // Drop handler for creating new nodes
    let on_drop = {
        let store = store.clone();
        move |ev: leptos::ev::DragEvent| {
            ev.prevent_default();
            is_drag_over.set(false);

            if let Some(sidebar_state) = sidebar_drag.get() {
                // Get drop position relative to canvas
                let target = ev.target();
                if let Some(target) = target {
                    if let Some(element) = target.dyn_ref::<leptos::web_sys::HtmlElement>() {
                        let rect = element.get_bounding_client_rect();
                        let viewport = store.get_viewport();

                        // Calculate position in flow coordinates
                        let x = ((ev.client_x() as f64 - rect.left()) - viewport.x) / viewport.zoom - 60.0;
                        let y = ((ev.client_y() as f64 - rect.top()) - viewport.y) / viewport.zoom - 25.0;

                        // Generate unique ID
                        let counter = get_node_counter().get();
                        get_node_counter().set(counter + 1);
                        let node_id = format!("node-{}", counter);

                        // Create new node
                        let new_node = Node::new(node_id.clone(), Position::new(x, y))
                            .with_data(json!({
                                "label": sidebar_state.label,
                                "type": sidebar_state.node_type,
                                "color": sidebar_state.color
                            }))
                            .with_dimensions(120.0, 50.0);

                        store.add_node(new_node);
                        log_action(&format!("Dropped: {} node at ({:.0}, {:.0})", sidebar_state.label, x, y));
                    }
                }
            }

            sidebar_drag.set(None);
        }
    };

    // Drag over handler
    let on_dragover = move |ev: leptos::ev::DragEvent| {
        ev.prevent_default();
        is_drag_over.set(true);
    };

    // Drag leave handler
    let on_dragleave = move |_ev: leptos::ev::DragEvent| {
        is_drag_over.set(false);
    };

    view! {
        <div
            class="example-container"
            style="display: flex; flex-direction: column; height: 100%;"
        >
            // Header
            <div style="padding: 12px; background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%); \
                        border-bottom: 1px solid #3b82f6;">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <div style="background: #3b82f6; color: white; padding: 6px 12px; border-radius: 6px; \
                                font-size: 11px; font-weight: 600;">
                        "Drag & Drop"
                    </div>
                    <div style="font-size: 12px; color: #1e40af;">
                        "Drag nodes from the sidebar onto the canvas"
                    </div>
                </div>
            </div>

            // Main content
            <div style="display: flex; flex: 1; min-height: 0;">
                // Sidebar with draggable node templates
                <NodeTemplateSidebar />

                // Flow canvas (drop zone)
                <div
                    class="xyflow leptos-flow"
                    style=move || format!(
                        "flex: 1; position: relative; background: {}; transition: background 0.2s;",
                        if is_drag_over.get() { "#e0f2fe" } else { "#fafafa" }
                    )
                    on:mousemove=on_mousemove
                    on:mouseup=on_mouseup
                    on:drop=on_drop
                    on:dragover=on_dragover
                    on:dragleave=on_dragleave
                >
                    // Drop zone indicator
                    {move || {
                        if is_drag_over.get() {
                            view! {
                                <div style="position: absolute; inset: 20px; border: 3px dashed #3b82f6; \
                                            border-radius: 12px; background: rgba(59, 130, 246, 0.05); \
                                            display: flex; align-items: center; justify-content: center; \
                                            pointer-events: none; z-index: 1000;">
                                    <div style="background: rgba(59, 130, 246, 0.9); color: white; \
                                                padding: 12px 24px; border-radius: 8px; font-size: 14px; \
                                                font-weight: 600; box-shadow: 0 4px 12px rgba(0,0,0,0.2);">
                                        "Drop here to create node"
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}

                    <Background variant=BackgroundVariant::Dots />

                    <FlowViewport store=store.clone()>
                        <DragDropEdgeRenderer store=store.clone() />
                        <ConnectionLine />

                        {move || {
                            store.get_nodes().into_iter().map(|node| {
                                view! {
                                    <DragDropNode
                                        node=node.clone()
                                        store=store.clone()
                                        drag_signal=drag_signal
                                    />
                                }
                            }).collect_view()
                        }}
                    </FlowViewport>

                    <Controls position=PanelPosition::BottomLeft />
                </div>

                // Info Panel
                <div style="width: 260px; background: #f8fafc; border-left: 1px solid #e2e8f0; \
                            display: flex; flex-direction: column; overflow-y: auto;">
                    <StatsPanel />
                    <ActionLogPanel />
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Node Template Sidebar
// ============================================================================

#[component]
fn NodeTemplateSidebar() -> impl IntoView {
    let templates = get_node_templates();

    view! {
        <div style="width: 200px; background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%); \
                    border-right: 1px solid #334155; display: flex; flex-direction: column;">
            // Header
            <div style="padding: 16px; border-bottom: 1px solid #334155;">
                <div style="font-size: 13px; font-weight: 600; color: #f1f5f9; margin-bottom: 4px;">
                    "Node Library"
                </div>
                <div style="font-size: 10px; color: #94a3b8;">
                    "Drag nodes to the canvas"
                </div>
            </div>

            // Node templates
            <div style="flex: 1; padding: 12px; display: flex; flex-direction: column; gap: 8px; \
                        overflow-y: auto;">
                {templates.into_iter().map(|template| {
                    view! { <DraggableNodeTemplate template=template /> }
                }).collect_view()}
            </div>

            // Footer hint
            <div style="padding: 12px; border-top: 1px solid #334155; background: rgba(0,0,0,0.2);">
                <div style="font-size: 9px; color: #64748b; text-align: center;">
                    "Tip: Drag templates with your mouse"
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Draggable Node Template
// ============================================================================

#[component]
fn DraggableNodeTemplate(template: NodeTemplate) -> impl IntoView {
    let sidebar_drag = get_sidebar_drag_signal();
    let _template_clone = template.clone();
    let color = template.color.to_string();
    let color_for_style = template.color.to_string();
    let label = template.label.to_string();
    let description = template.description.to_string();
    let icon = template.icon.to_string();
    let node_type = template.node_type.to_string();

    let on_dragstart = move |_ev: leptos::ev::DragEvent| {
        // Set our custom state for tracking the drag
        // Note: We use custom state instead of data_transfer for better cross-browser compatibility
        sidebar_drag.set(Some(SidebarDragState {
            node_type: node_type.clone(),
            label: label.clone(),
            color: color.clone(),
        }));

        log_action(&format!("Dragging: {} node", label));
    };

    let on_dragend = move |_ev: leptos::ev::DragEvent| {
        get_is_drag_over().set(false);
    };

    view! {
        <div
            draggable="true"
            on:dragstart=on_dragstart
            on:dragend=on_dragend
            style=format!(
                "display: flex; align-items: center; gap: 10px; padding: 10px 12px; \
                 background: linear-gradient(135deg, #1e293b 0%, #334155 100%); \
                 border: 1px solid {}40; border-radius: 8px; cursor: grab; \
                 transition: all 0.2s; user-select: none;",
                color_for_style
            )
            class="draggable-template"
        >
            // Icon
            <div style=format!(
                "width: 32px; height: 32px; background: {}30; border: 2px solid {}; \
                 border-radius: 6px; display: flex; align-items: center; justify-content: center; \
                 font-weight: 700; font-size: 14px; color: {};",
                color_for_style, color_for_style, color_for_style
            )>
                {icon}
            </div>

            // Info
            <div style="flex: 1; min-width: 0;">
                <div style=format!("font-size: 12px; font-weight: 600; color: {}; margin-bottom: 2px;",
                    color_for_style
                )>
                    {template.label}
                </div>
                <div style="font-size: 9px; color: #64748b; white-space: nowrap; overflow: hidden; \
                            text-overflow: ellipsis;">
                    {description}
                </div>
            </div>

            // Drag indicator
            <div style="color: #475569; font-size: 10px;">
                ":::"
            </div>
        </div>
    }
}

// ============================================================================
// Stats Panel
// ============================================================================

#[component]
fn StatsPanel() -> impl IntoView {
    let store = use_context::<FlowStore>().expect("FlowStore in context");

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #3b82f6; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"STATS"</span>
                "Flow Statistics"
            </div>

            <div style="display: flex; flex-direction: column; gap: 8px;">
                // Node count
                <div style="display: flex; align-items: center; justify-content: space-between; \
                            padding: 8px; background: #f1f5f9; border-radius: 6px;">
                    <span style="font-size: 11px; color: #64748b;">Nodes</span>
                    <span style="font-size: 14px; font-weight: 700; color: #3b82f6;">
                        {move || store.get_nodes().len()}
                    </span>
                </div>

                // Edge count
                <div style="display: flex; align-items: center; justify-content: space-between; \
                            padding: 8px; background: #f1f5f9; border-radius: 6px;">
                    <span style="font-size: 11px; color: #64748b;">Edges</span>
                    <span style="font-size: 14px; font-weight: 700; color: #8b5cf6;">
                        {move || store.get_edges().len()}
                    </span>
                </div>

                // Dropped count
                <div style="display: flex; align-items: center; justify-content: space-between; \
                            padding: 8px; background: #f1f5f9; border-radius: 6px;">
                    <span style="font-size: 11px; color: #64748b;">Dropped</span>
                    <span style="font-size: 14px; font-weight: 700; color: #10b981;">
                        {move || get_node_counter().get()}
                    </span>
                </div>
            </div>

            // Clear button
            <button
                style="margin-top: 12px; width: 100%; padding: 8px; background: #fee2e2; \
                       color: #dc2626; border: 1px solid #fecaca; border-radius: 6px; \
                       font-size: 11px; font-weight: 600; cursor: pointer;"
                on:click=move |_| {
                    // Remove all dropped nodes (keep start and end)
                    let nodes = store.get_nodes();
                    for node in nodes {
                        if node.id.starts_with("node-") {
                            store.remove_node(&node.id);
                        }
                    }
                    get_node_counter().set(0);
                    log_action("Cleared all dropped nodes");
                }
            >
                "Clear Dropped Nodes"
            </button>
        </div>
    }
}

// ============================================================================
// Action Log Panel
// ============================================================================

#[component]
fn ActionLogPanel() -> impl IntoView {
    let action_log = get_action_log();

    view! {
        <div style="padding: 12px; flex: 1; display: flex; flex-direction: column;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #6366f1; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"LOG"</span>
                "Activity Log"
                <button
                    style="margin-left: auto; font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                    on:click=move |_| action_log.set(vec!["Log cleared".to_string()])
                >
                    "Clear"
                </button>
            </div>

            <div style="flex: 1; background: #1a1a2e; border-radius: 6px; padding: 8px; \
                        overflow-y: auto; font-family: monospace; font-size: 9px;">
                {move || {
                    let entries = action_log.get();
                    entries.into_iter().rev().enumerate().map(|(i, entry)| {
                        let color = if entry.contains("Dropped") {
                            "#10b981"
                        } else if entry.contains("Dragging") {
                            "#3b82f6"
                        } else if entry.contains("Cleared") {
                            "#ef4444"
                        } else {
                            "#9ca3af"
                        };
                        view! {
                            <div style=format!(
                                "color: {}; padding: 2px 0; border-bottom: 1px solid #2a2a4e; \
                                 opacity: {};",
                                color,
                                if i < 5 { 1.0 } else { 0.7 - (i as f64 - 5.0) * 0.05 }
                            )>
                                {entry}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Node Component
// ============================================================================

#[component]
fn DragDropNode(
    node: Node,
    store: FlowStore,
    drag_signal: RwSignal<Option<DragState>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    // Mouse down - start drag and select
    let on_mousedown = {
        let store = store.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();

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

                store.select_node(&node_id_for_drag, ev.shift_key());
            }
        }
    };

    view! {
        <div
            class="xyflow__node"
            style=move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let border = if n.selected {
                        "2px solid #1a1a1a".to_string()
                    } else {
                        format!("2px solid {}60", color)
                    };

                    let box_shadow = if n.selected {
                        format!("0 0 0 2px {}40, 0 4px 12px rgba(0,0,0,0.2)", color)
                    } else {
                        "0 2px 6px rgba(0,0,0,0.1)".to_string()
                    };

                    let background = match node_type {
                        "input" => format!("linear-gradient(135deg, {}30 0%, {}50 100%)", color, color),
                        "output" => format!("linear-gradient(135deg, {}30 0%, {}50 100%)", color, color),
                        _ => "white".to_string(),
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); \
                         width: {}px; height: {}px; background: {}; border: {}; \
                         border-radius: 8px; box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; \
                         align-items: center; padding: 8px; box-sizing: border-box; \
                         transition: box-shadow 0.15s, border 0.15s;",
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
                        .unwrap_or("#333")
                        .to_string();

                    view! {
                        <div style=format!("font-weight: 600; font-size: 11px; color: {};", color)>
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
                                    position=HandlePosition::Left
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #666; width: 8px; height: 8px; border: 2px solid white; \
                                           box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #666; width: 8px; height: 8px; border: 2px solid white; \
                                           box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
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
// Edge Renderer Component
// ============================================================================

#[component]
fn DragDropEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="drag-drop-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#6366f1" />
                    <stop offset="100%" stop-color="#8b5cf6" />
                </linearGradient>
                <marker
                    id="drag-drop-edge-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#6366f1" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    let sx = source_node.position.x + source_node.width.unwrap_or(120.0);
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0) / 2.0;
                    let tx = target_node.position.x;
                    let ty = target_node.position.y + target_node.height.unwrap_or(50.0) / 2.0;

                    let offset = (tx - sx).abs() * 0.4;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx + offset, sy,
                        tx - offset, ty,
                        tx, ty
                    );

                    Some(view! {
                        <g class="xyflow__edge">
                            <path
                                d=path.clone()
                                stroke="url(#drag-drop-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#drag-drop-edge-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
