//! Multi Flows Example
//!
//! Demonstrates multiple independent flow instances on one page:
//! - Render two or more SvelteFlow instances
//! - Each flow has independent state
//! - Inter-flow communication (copy/paste between flows)

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State (separate for each flow)
// ============================================================================

/// Drag state for Flow 1
static FLOW1_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

fn get_flow1_drag_signal() -> RwSignal<Option<DragState>> {
    *FLOW1_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Drag state for Flow 2
static FLOW2_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

fn get_flow2_drag_signal() -> RwSignal<Option<DragState>> {
    *FLOW2_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Action Log
// ============================================================================

#[derive(Clone, Debug)]
struct ActionEvent {
    timestamp: f64,
    flow_id: u32,
    action: String,
    details: String,
}

// ============================================================================
// Flow Stats
// ============================================================================

#[derive(Clone, Debug, Default)]
struct FlowStats {
    nodes: usize,
    edges: usize,
    selected: usize,
}

// ============================================================================
// Multi Flows Example Component
// ============================================================================

/// Multi Flows example - Multiple independent flow instances
#[component]
pub fn MultiFlowsExample() -> impl IntoView {
    // Create Flow 1 - Data Pipeline
    let flow1_nodes = vec![
        Node::new("f1-1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Source",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f1-2".to_string(), Position::new(200.0, 50.0))
            .with_data(json!({
                "label": "Filter",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f1-3".to_string(), Position::new(350.0, 50.0))
            .with_data(json!({
                "label": "Transform",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f1-4".to_string(), Position::new(500.0, 50.0))
            .with_data(json!({
                "label": "Output",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(100.0, 50.0),
    ];

    let flow1_edges = vec![
        Edge::new("f1-e1".to_string(), "f1-1".to_string(), "f1-2".to_string()),
        Edge::new("f1-e2".to_string(), "f1-2".to_string(), "f1-3".to_string()),
        Edge::new("f1-e3".to_string(), "f1-3".to_string(), "f1-4".to_string()),
    ];

    // Create Flow 2 - ML Pipeline
    let flow2_nodes = vec![
        Node::new("f2-1".to_string(), Position::new(50.0, 30.0))
            .with_data(json!({
                "label": "Dataset",
                "type": "input",
                "color": "#f59e0b"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f2-2".to_string(), Position::new(50.0, 120.0))
            .with_data(json!({
                "label": "Config",
                "type": "input",
                "color": "#f59e0b"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f2-3".to_string(), Position::new(200.0, 75.0))
            .with_data(json!({
                "label": "Train",
                "type": "default",
                "color": "#3b82f6"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f2-4".to_string(), Position::new(350.0, 30.0))
            .with_data(json!({
                "label": "Evaluate",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(100.0, 50.0),
        Node::new("f2-5".to_string(), Position::new(350.0, 120.0))
            .with_data(json!({
                "label": "Deploy",
                "type": "output",
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),
    ];

    let flow2_edges = vec![
        Edge::new("f2-e1".to_string(), "f2-1".to_string(), "f2-3".to_string()),
        Edge::new("f2-e2".to_string(), "f2-2".to_string(), "f2-3".to_string()),
        Edge::new("f2-e3".to_string(), "f2-3".to_string(), "f2-4".to_string()),
        Edge::new("f2-e4".to_string(), "f2-3".to_string(), "f2-5".to_string()),
    ];

    // Create stores for each flow
    let store1 = FlowStore::new(flow1_nodes, flow1_edges);
    let store2 = FlowStore::new(flow2_nodes, flow2_edges);

    // Selected nodes for each flow
    let selected1 = RwSignal::new(None::<String>);
    let selected2 = RwSignal::new(None::<String>);

    // Action log (shared)
    let action_log = RwSignal::new(Vec::<ActionEvent>::new());

    // Add action to log
    let add_action = move |flow_id: u32, action: &str, details: &str| {
        action_log.update(|log| {
            log.insert(0, ActionEvent {
                timestamp: js_sys::Date::now(),
                flow_id,
                action: action.to_string(),
                details: details.to_string(),
            });
            if log.len() > 15 {
                log.pop();
            }
        });
    };

    // Flow stats
    let flow1_stats = {
        let store1 = store1.clone();
        let selected1 = selected1.clone();
        move || FlowStats {
            nodes: store1.get_nodes().len(),
            edges: store1.get_edges().len(),
            selected: if selected1.get().is_some() { 1 } else { 0 },
        }
    };

    let flow2_stats = {
        let store2 = store2.clone();
        let selected2 = selected2.clone();
        move || FlowStats {
            nodes: store2.get_nodes().len(),
            edges: store2.get_edges().len(),
            selected: if selected2.get().is_some() { 1 } else { 0 },
        }
    };

    // Add node to flow 1
    let add_node_flow1 = {
        let store1 = store1.clone();
        let add_action = add_action.clone();
        move |_| {
            let nodes = store1.get_nodes();
            let new_id = format!("f1-{}", nodes.len() + 1);
            let max_x = nodes.iter().map(|n| n.position.x).fold(0.0_f64, f64::max);

            let new_node = Node::new(new_id.clone(), Position::new(max_x + 150.0, 50.0))
                .with_data(json!({
                    "label": format!("Node {}", nodes.len() + 1),
                    "type": "default",
                    "color": "#6366f1"
                }))
                .with_dimensions(100.0, 50.0);

            store1.add_node(new_node);
            add_action(1, "Add Node", &format!("Added {}", new_id));
        }
    };

    // Add node to flow 2
    let add_node_flow2 = {
        let store2 = store2.clone();
        let add_action = add_action.clone();
        move |_| {
            let nodes = store2.get_nodes();
            let new_id = format!("f2-{}", nodes.len() + 1);
            let max_x = nodes.iter().map(|n| n.position.x).fold(0.0_f64, f64::max);

            let new_node = Node::new(new_id.clone(), Position::new(max_x + 150.0, 75.0))
                .with_data(json!({
                    "label": format!("Node {}", nodes.len() + 1),
                    "type": "default",
                    "color": "#3b82f6"
                }))
                .with_dimensions(100.0, 50.0);

            store2.add_node(new_node);
            add_action(2, "Add Node", &format!("Added {}", new_id));
        }
    };

    // Copy selected node between flows
    let copy_to_flow2 = {
        let store1 = store1.clone();
        let store2 = store2.clone();
        let selected1 = selected1.clone();
        let add_action = add_action.clone();
        move |_| {
            if let Some(node_id) = selected1.get() {
                let nodes1 = store1.get_nodes();
                if let Some(source_node) = nodes1.iter().find(|n| n.id == node_id) {
                    let nodes2 = store2.get_nodes();
                    let new_id = format!("f2-{}", nodes2.len() + 1);

                    let mut new_data = source_node.data.clone();
                    if let Some(label) = new_data.get_mut("label") {
                        *label = json!(format!("{} (copy)", label.as_str().unwrap_or("Node")));
                    }

                    let new_node = Node::new(new_id.clone(), Position::new(50.0, 200.0))
                        .with_data(new_data)
                        .with_dimensions(source_node.width.unwrap_or(100.0), source_node.height.unwrap_or(50.0));

                    store2.add_node(new_node);
                    add_action(2, "Copy", &format!("Copied {} from Flow 1", node_id));
                }
            }
        }
    };

    let copy_to_flow1 = {
        let store1 = store1.clone();
        let store2 = store2.clone();
        let selected2 = selected2.clone();
        let add_action = add_action.clone();
        move |_| {
            if let Some(node_id) = selected2.get() {
                let nodes2 = store2.get_nodes();
                if let Some(source_node) = nodes2.iter().find(|n| n.id == node_id) {
                    let nodes1 = store1.get_nodes();
                    let new_id = format!("f1-{}", nodes1.len() + 1);

                    let mut new_data = source_node.data.clone();
                    if let Some(label) = new_data.get_mut("label") {
                        *label = json!(format!("{} (copy)", label.as_str().unwrap_or("Node")));
                    }

                    let new_node = Node::new(new_id.clone(), Position::new(50.0, 150.0))
                        .with_data(new_data)
                        .with_dimensions(source_node.width.unwrap_or(100.0), source_node.height.unwrap_or(50.0));

                    store1.add_node(new_node);
                    add_action(1, "Copy", &format!("Copied {} from Flow 2", node_id));
                }
            }
        }
    };

    // Clear log
    let clear_log = move |_| {
        action_log.set(vec![]);
    };

    view! {
        <div class="example-container" style="display: flex; flex-direction: column; height: 100%;">
            // Top section: Two flows side by side
            <div style="display: flex; flex: 1; gap: 12px; padding: 12px; min-height: 0;">
                // Flow 1
                <div style="flex: 1; display: flex; flex-direction: column; min-width: 0;">
                    <div style="background: linear-gradient(135deg, #10b981 0%, #059669 100%); color: white; \
                                padding: 8px 12px; border-radius: 8px 8px 0 0; font-size: 12px; font-weight: 600; \
                                display: flex; justify-content: space-between; align-items: center;">
                        <div style="display: flex; align-items: center; gap: 8px;">
                            <span style="background: rgba(255,255,255,0.2); padding: 2px 6px; border-radius: 4px; font-size: 10px;">
                                "Flow 1"
                            </span>
                            "Data Pipeline"
                        </div>
                        <div style="display: flex; gap: 8px; font-size: 10px; opacity: 0.9;">
                            <span>{move || format!("{} nodes", flow1_stats().nodes)}</span>
                            <span>{move || format!("{} edges", flow1_stats().edges)}</span>
                        </div>
                    </div>
                    <FlowInstance
                        store=store1.clone()
                        drag_signal=get_flow1_drag_signal()
                        selected=selected1
                        flow_id=1
                        color="#10b981".to_string()
                        add_action=add_action.clone()
                    />
                </div>

                // Flow 2
                <div style="flex: 1; display: flex; flex-direction: column; min-width: 0;">
                    <div style="background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); color: white; \
                                padding: 8px 12px; border-radius: 8px 8px 0 0; font-size: 12px; font-weight: 600; \
                                display: flex; justify-content: space-between; align-items: center;">
                        <div style="display: flex; align-items: center; gap: 8px;">
                            <span style="background: rgba(255,255,255,0.2); padding: 2px 6px; border-radius: 4px; font-size: 10px;">
                                "Flow 2"
                            </span>
                            "ML Pipeline"
                        </div>
                        <div style="display: flex; gap: 8px; font-size: 10px; opacity: 0.9;">
                            <span>{move || format!("{} nodes", flow2_stats().nodes)}</span>
                            <span>{move || format!("{} edges", flow2_stats().edges)}</span>
                        </div>
                    </div>
                    <FlowInstance
                        store=store2.clone()
                        drag_signal=get_flow2_drag_signal()
                        selected=selected2
                        flow_id=2
                        color="#3b82f6".to_string()
                        add_action=add_action.clone()
                    />
                </div>
            </div>

            // Bottom section: Control panel
            <div style="background: white; border-top: 1px solid #e5e7eb; padding: 12px; display: flex; gap: 16px;">
                // Instructions
                <div style="flex: 1; background: #f0f9ff; padding: 12px; border-radius: 8px;">
                    <div style="font-size: 11px; font-weight: 600; color: #0369a1; margin-bottom: 8px;">
                        "Multiple Independent Flows"
                    </div>
                    <ul style="margin: 0; padding-left: 16px; font-size: 10px; color: #0284c7; line-height: 1.6;">
                        <li>"Each flow has its own independent state"</li>
                        <li>"Click nodes to select them"</li>
                        <li>"Drag nodes to reposition"</li>
                        <li>"Copy selected nodes between flows"</li>
                    </ul>
                </div>

                // Flow 1 controls
                <div style="flex: 1; background: #f8fafc; padding: 12px; border-radius: 8px;">
                    <div style="font-size: 11px; font-weight: 600; color: #10b981; margin-bottom: 8px;">
                        "Flow 1 Actions"
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 6px;">
                        <button
                            style="padding: 6px 12px; font-size: 10px; border: none; border-radius: 4px; \
                                   background: #10b981; color: white; cursor: pointer; font-weight: 500;"
                            on:click=add_node_flow1
                        >
                            "+ Add Node"
                        </button>
                        <button
                            style="padding: 6px 12px; font-size: 10px; border: 1px solid #10b981; border-radius: 4px; \
                                   background: white; color: #10b981; cursor: pointer; font-weight: 500;"
                            on:click=copy_to_flow2.clone()
                            disabled=move || selected1.get().is_none()
                        >
                            "Copy to Flow 2 →"
                        </button>
                    </div>
                </div>

                // Flow 2 controls
                <div style="flex: 1; background: #f8fafc; padding: 12px; border-radius: 8px;">
                    <div style="font-size: 11px; font-weight: 600; color: #3b82f6; margin-bottom: 8px;">
                        "Flow 2 Actions"
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 6px;">
                        <button
                            style="padding: 6px 12px; font-size: 10px; border: none; border-radius: 4px; \
                                   background: #3b82f6; color: white; cursor: pointer; font-weight: 500;"
                            on:click=add_node_flow2
                        >
                            "+ Add Node"
                        </button>
                        <button
                            style="padding: 6px 12px; font-size: 10px; border: 1px solid #3b82f6; border-radius: 4px; \
                                   background: white; color: #3b82f6; cursor: pointer; font-weight: 500;"
                            on:click=copy_to_flow1.clone()
                            disabled=move || selected2.get().is_none()
                        >
                            "← Copy to Flow 1"
                        </button>
                    </div>
                </div>

                // Action log
                <div style="flex: 2; background: #f8fafc; padding: 12px; border-radius: 8px; max-height: 120px; overflow-y: auto;">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        <div style="font-size: 11px; font-weight: 600; color: #333;">"Activity Log"</div>
                        <button
                            style="font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                                   border-radius: 3px; background: white; cursor: pointer; color: #666;"
                            on:click=clear_log
                        >
                            "Clear"
                        </button>
                    </div>
                    {move || {
                        let log = action_log.get();
                        if log.is_empty() {
                            view! {
                                <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                    "Actions will appear here"
                                </div>
                            }.into_any()
                        } else {
                            log.into_iter().take(8).map(|event| {
                                let date = js_sys::Date::new(&leptos::wasm_bindgen::JsValue::from_f64(event.timestamp));
                                let time = format!(
                                    "{:02}:{:02}:{:02}",
                                    date.get_hours(),
                                    date.get_minutes(),
                                    date.get_seconds()
                                );

                                let flow_color = if event.flow_id == 1 { "#10b981" } else { "#3b82f6" };

                                view! {
                                    <div style="display: flex; align-items: center; gap: 8px; padding: 4px 0; \
                                                font-size: 10px; border-bottom: 1px solid #eee;">
                                        <span style=format!(
                                            "background: {}; color: white; padding: 1px 5px; \
                                             border-radius: 3px; font-size: 9px; font-weight: 500;",
                                            flow_color
                                        )>
                                            {format!("F{}", event.flow_id)}
                                        </span>
                                        <span style="font-weight: 500; color: #333;">{event.action}</span>
                                        <span style="color: #666; flex: 1;">{event.details}</span>
                                        <span style="color: #999; font-family: monospace; font-size: 9px;">{time}</span>
                                    </div>
                                }
                            }).collect_view().into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Flow Instance Component
// ============================================================================

/// A single flow instance with its own state
#[component]
fn FlowInstance<F>(
    store: FlowStore,
    drag_signal: RwSignal<Option<DragState>>,
    selected: RwSignal<Option<String>>,
    flow_id: u32,
    color: String,
    add_action: F,
) -> impl IntoView
where
    F: Fn(u32, &str, &str) + Clone + Send + Sync + 'static,
{
    // Provide context for this flow instance
    provide_context(store.clone());

    let store_for_mouse = store.clone();
    let store_for_view = store.clone();

    // Global mouse move handler
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
        let add_action = add_action.clone();
        move |_ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
                add_action(flow_id, "Drag", &format!("Moved {}", node_id));
            }
        }
    };

    // Click on background to deselect
    let on_background_click = {
        let add_action = add_action.clone();
        move |_ev: leptos::ev::MouseEvent| {
            if selected.get().is_some() {
                selected.set(None);
                add_action(flow_id, "Select", "Cleared selection");
            }
        }
    };

    view! {
        <div
            class="xyflow leptos-flow"
            style="flex: 1; position: relative; background: #fafafa; border: 1px solid #e5e7eb; \
                   border-top: none; border-radius: 0 0 8px 8px; overflow: hidden;"
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:click=on_background_click
        >
            // Background
            <Background variant=BackgroundVariant::Dots />

            // Flow viewport
            <FlowViewport store=store_for_view.clone()>
                // Edge renderer
                <FlowEdgeRenderer store=store_for_view.clone() color=color.clone() />

                // Connection line
                <ConnectionLine />

                // Render nodes
                {move || {
                    store_for_view.get_nodes().into_iter().map(|node| {
                        view! {
                            <FlowNode
                                node=node.clone()
                                store=store_for_mouse.clone()
                                drag_signal=drag_signal
                                selected=selected
                                flow_id=flow_id
                                add_action=add_action.clone()
                            />
                        }
                    }).collect_view()
                }}
            </FlowViewport>

            // Mini controls
            <div style="position: absolute; bottom: 8px; left: 8px; display: flex; gap: 4px;">
                <button
                    style="width: 24px; height: 24px; border-radius: 4px; border: 1px solid #ddd; \
                           background: white; cursor: pointer; font-size: 12px; display: flex; \
                           align-items: center; justify-content: center;"
                    on:click={
                        let store = store.clone();
                        move |_| {
                            let vp = store.get_viewport();
                            store.set_viewport(Viewport::new(vp.x, vp.y, (vp.zoom + 0.2).min(2.0)));
                        }
                    }
                >
                    "+"
                </button>
                <button
                    style="width: 24px; height: 24px; border-radius: 4px; border: 1px solid #ddd; \
                           background: white; cursor: pointer; font-size: 12px; display: flex; \
                           align-items: center; justify-content: center;"
                    on:click={
                        let store = store.clone();
                        move |_| {
                            let vp = store.get_viewport();
                            store.set_viewport(Viewport::new(vp.x, vp.y, (vp.zoom - 0.2).max(0.3)));
                        }
                    }
                >
                    "−"
                </button>
            </div>
        </div>
    }
}

// ============================================================================
// Flow Node Component
// ============================================================================

/// Node component for a flow instance
#[component]
fn FlowNode<F>(
    node: Node,
    store: FlowStore,
    drag_signal: RwSignal<Option<DragState>>,
    selected: RwSignal<Option<String>>,
    flow_id: u32,
    add_action: F,
) -> impl IntoView
where
    F: Fn(u32, &str, &str) + Clone + Send + Sync + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_select = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    // Mouse down - start drag and select
    let on_mousedown = {
        let store = store.clone();
        let add_action = add_action.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();

            // Select this node
            let prev_selected = selected.get();
            if prev_selected.as_ref() != Some(&node_id_for_select) {
                selected.set(Some(node_id_for_select.clone()));
                add_action(flow_id, "Select", &node_id_for_select);
            }

            // Start drag
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
            class="xyflow__node"
            style=move || {
                let nodes = store.get_nodes();
                let is_selected = selected.get() == Some(node_id_for_style.clone());

                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let border = if is_selected {
                        "2px solid #1a1a1a".to_string()
                    } else {
                        format!("2px solid {}60", color)
                    };

                    let box_shadow = if is_selected {
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
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: box-shadow 0.15s, border 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(100.0), n.height.unwrap_or(50.0),
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
                                    position=HandlePosition::Left
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #666; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #666; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
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
// Flow Edge Renderer Component
// ============================================================================

/// Edge renderer for a flow instance
#[component]
fn FlowEdgeRenderer(store: FlowStore, color: String) -> impl IntoView {
    let gradient_id = format!("edge-gradient-{}", color.replace("#", ""));
    let arrow_id = format!("edge-arrow-{}", color.replace("#", ""));
    let gradient_id_clone = gradient_id.clone();
    let arrow_id_clone = arrow_id.clone();

    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id=gradient_id.clone() x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color=color.clone() />
                    <stop offset="100%" stop-color=color.clone() stop-opacity="0.6" />
                </linearGradient>
                <marker
                    id=arrow_id.clone()
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill=color.clone() />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let gradient_ref = format!("url(#{})", gradient_id_clone);
                let arrow_ref = format!("url(#{})", arrow_id_clone);

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    let sx = source_node.position.x + source_node.width.unwrap_or(100.0);
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

                    let gradient_url = gradient_ref.clone();
                    let marker_url = arrow_ref.clone();

                    Some(view! {
                        <g class="xyflow__edge">
                            <path
                                d=path.clone()
                                stroke=gradient_url
                                stroke-width="2"
                                fill="none"
                                marker-end=marker_url
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
