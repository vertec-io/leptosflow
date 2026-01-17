//! Subflow Example
//!
//! Demonstrates nested flow graphs (nodes containing flows):
//! - Parent nodes that contain child nodes
//! - Child nodes move with parent
//! - Support expand/collapse of subflows
//! - Edges can cross subflow boundaries

use leptos::prelude::*;
use leptos::serde_json::json;
use std::collections::HashSet;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Global drag state for Subflow example
static SUBFLOW_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

fn get_subflow_drag_signal() -> RwSignal<Option<DragState>> {
    *SUBFLOW_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Collapsed subflows state
static COLLAPSED_SUBFLOWS: std::sync::OnceLock<RwSignal<HashSet<String>>> = std::sync::OnceLock::new();

fn get_collapsed_signal() -> RwSignal<HashSet<String>> {
    *COLLAPSED_SUBFLOWS.get_or_init(|| RwSignal::new(HashSet::new()))
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
// Subflow Data Structures
// ============================================================================

/// Represents a subflow container with child nodes
#[derive(Clone, Debug)]
struct SubflowData {
    /// Child node IDs contained in this subflow
    child_ids: Vec<String>,
    /// Color theme for this subflow
    color: String,
    /// Whether this subflow is collapsed
    collapsed: bool,
    /// Padding around child nodes when expanded
    padding: f64,
}

// ============================================================================
// Subflow Example Component
// ============================================================================

/// Subflow example - Nested flow graphs
#[component]
pub fn SubflowExample() -> impl IntoView {
    // Define subflow structure
    // Subflow 1: Data Processing Pipeline (contains nodes s1-a, s1-b, s1-c)
    // Subflow 2: Compute Cluster (contains nodes s2-a, s2-b)
    // Regular nodes: input, output (connect to subflows)

    // Child nodes for Subflow 1 (relative positions within subflow)
    let s1_children = vec![
        Node::new("s1-a".to_string(), Position::new(30.0, 50.0))
            .with_data(json!({
                "label": "Parse",
                "type": "input",
                "parent": "subflow-1"
            }))
            .with_dimensions(80.0, 40.0),
        Node::new("s1-b".to_string(), Position::new(130.0, 50.0))
            .with_data(json!({
                "label": "Transform",
                "type": "default",
                "parent": "subflow-1"
            }))
            .with_dimensions(80.0, 40.0),
        Node::new("s1-c".to_string(), Position::new(230.0, 50.0))
            .with_data(json!({
                "label": "Validate",
                "type": "output",
                "parent": "subflow-1"
            }))
            .with_dimensions(80.0, 40.0),
    ];

    // Child nodes for Subflow 2 (relative positions within subflow)
    let s2_children = vec![
        Node::new("s2-a".to_string(), Position::new(30.0, 50.0))
            .with_data(json!({
                "label": "Worker 1",
                "type": "default",
                "parent": "subflow-2"
            }))
            .with_dimensions(80.0, 40.0),
        Node::new("s2-b".to_string(), Position::new(130.0, 50.0))
            .with_data(json!({
                "label": "Worker 2",
                "type": "default",
                "parent": "subflow-2"
            }))
            .with_dimensions(80.0, 40.0),
    ];

    // Create all nodes
    let mut initial_nodes = vec![
        // Input node
        Node::new("input".to_string(), Position::new(50.0, 150.0))
            .with_data(json!({
                "label": "Input Data",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(100.0, 50.0),
        // Subflow 1 container
        Node::new("subflow-1".to_string(), Position::new(200.0, 80.0))
            .with_data(json!({
                "label": "Data Processing",
                "type": "subflow",
                "color": "#6366f1",
                "child_ids": ["s1-a", "s1-b", "s1-c"],
                "collapsed": false
            }))
            .with_dimensions(340.0, 140.0),
        // Subflow 2 container
        Node::new("subflow-2".to_string(), Position::new(200.0, 280.0))
            .with_data(json!({
                "label": "Compute Cluster",
                "type": "subflow",
                "color": "#8b5cf6",
                "child_ids": ["s2-a", "s2-b"],
                "collapsed": false
            }))
            .with_dimensions(240.0, 140.0),
        // Output node
        Node::new("output".to_string(), Position::new(600.0, 180.0))
            .with_data(json!({
                "label": "Output",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(100.0, 50.0),
    ];

    // Add child nodes
    initial_nodes.extend(s1_children);
    initial_nodes.extend(s2_children);

    // Create edges
    // External edges (crossing subflow boundaries)
    let initial_edges = vec![
        // Input -> Subflow 1 (to first child)
        Edge::new("e-input-s1".to_string(), "input".to_string(), "s1-a".to_string())
            .with_label("Data In".to_string()),
        // Internal edges within Subflow 1
        Edge::new("e-s1-ab".to_string(), "s1-a".to_string(), "s1-b".to_string()),
        Edge::new("e-s1-bc".to_string(), "s1-b".to_string(), "s1-c".to_string()),
        // Subflow 1 -> Subflow 2 (cross-boundary)
        Edge::new("e-s1-s2".to_string(), "s1-c".to_string(), "s2-a".to_string())
            .with_label("Process".to_string()),
        // Internal edge within Subflow 2
        Edge::new("e-s2-ab".to_string(), "s2-a".to_string(), "s2-b".to_string()),
        // Subflow 2 -> Output
        Edge::new("e-s2-output".to_string(), "s2-b".to_string(), "output".to_string())
            .with_label("Results".to_string()),
        // Also connect Subflow 1 directly to Output
        Edge::new("e-s1-output".to_string(), "s1-c".to_string(), "output".to_string())
            .with_label("Direct".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

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
    let drag_signal = get_subflow_drag_signal();
    let collapsed_signal = get_collapsed_signal();

    // Global mouse move handler
    let on_global_mousemove = {
        let add_action = add_action.clone();
        move |ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let current_x = ev.client_x() as f64;
                let current_y = ev.client_y() as f64;
                let (start_x, start_y) = drag_state.start_mouse;
                let (node_start_x, node_start_y) = drag_state.start_pos;

                let viewport = store.get_viewport();
                let dx = (current_x - start_x) / viewport.zoom;
                let dy = (current_y - start_y) / viewport.zoom;

                let dragged_node_id = drag_state.node_id.clone();

                // Update the dragged node position
                store.update_node(&dragged_node_id, |n| {
                    n.position = Position::new(node_start_x + dx, node_start_y + dy);
                });

                // If the dragged node is a subflow, also move all child nodes
                let nodes = store.get_nodes();
                if let Some(node) = nodes.iter().find(|n| n.id == dragged_node_id) {
                    if let Some(child_ids) = node.data.get("child_ids").and_then(|v| v.as_array()) {
                        for child_id_val in child_ids {
                            if let Some(child_id) = child_id_val.as_str() {
                                // Find the child's original position relative to the parent's start position
                                if let Some(child_node) = nodes.iter().find(|n| n.id == child_id) {
                                    // Child positions are relative offsets stored at original positions
                                    // We need to calculate the new absolute position
                                    let child_rel_x = child_node.position.x - node_start_x;
                                    let child_rel_y = child_node.position.y - node_start_y;

                                    store.update_node(child_id, |n| {
                                        n.position = Position::new(
                                            node_start_x + dx + child_rel_x,
                                            node_start_y + dy + child_rel_y
                                        );
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    // Global mouse up handler
    let on_global_mouseup = {
        let add_action = add_action.clone();
        move |_ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
                add_action("Drag", &format!("Ended for {}", node_id));
            }
        }
    };

    // Toggle subflow collapse
    let toggle_collapse = {
        let add_action = add_action.clone();
        move |subflow_id: String| {
            let collapsed = collapsed_signal.get();
            let is_collapsed = collapsed.contains(&subflow_id);

            collapsed_signal.update(|set| {
                if is_collapsed {
                    set.remove(&subflow_id);
                } else {
                    set.insert(subflow_id.clone());
                }
            });

            // Update the node data
            store.update_node(&subflow_id, |n| {
                if let Some(collapsed_val) = n.data.get_mut("collapsed") {
                    *collapsed_val = json!(!is_collapsed);
                }
            });

            // Resize the subflow node based on collapse state
            let collapsed_height = 50.0;
            let expanded_height = 140.0;

            store.update_node(&subflow_id, |n| {
                if is_collapsed {
                    n.height = Some(expanded_height);
                } else {
                    n.height = Some(collapsed_height);
                }
            });

            add_action("Toggle", &format!("{} {}", subflow_id, if is_collapsed { "expanded" } else { "collapsed" }));
        }
    };

    // Expand all
    let expand_all = {
        let add_action = add_action.clone();
        move |_| {
            collapsed_signal.set(HashSet::new());

            // Update all subflow nodes
            let nodes = store.get_nodes();
            for node in nodes.iter() {
                if node.data.get("type").and_then(|v| v.as_str()) == Some("subflow") {
                    store.update_node(&node.id, |n| {
                        if let Some(collapsed_val) = n.data.get_mut("collapsed") {
                            *collapsed_val = json!(false);
                        }
                        n.height = Some(140.0);
                    });
                }
            }

            add_action("Expand All", "All subflows expanded");
        }
    };

    // Collapse all
    let collapse_all = {
        let add_action = add_action.clone();
        move |_| {
            let nodes = store.get_nodes();
            let subflow_ids: HashSet<String> = nodes.iter()
                .filter(|n| n.data.get("type").and_then(|v| v.as_str()) == Some("subflow"))
                .map(|n| n.id.clone())
                .collect();

            collapsed_signal.set(subflow_ids.clone());

            // Update all subflow nodes
            for subflow_id in subflow_ids {
                store.update_node(&subflow_id, |n| {
                    if let Some(collapsed_val) = n.data.get_mut("collapsed") {
                        *collapsed_val = json!(true);
                    }
                    n.height = Some(50.0);
                });
            }

            add_action("Collapse All", "All subflows collapsed");
        }
    };

    // Clear log handler
    let clear_log = move |_| {
        action_log.set(vec![]);
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow subflow-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <SubflowEdgeRenderer store=store collapsed_signal=collapsed_signal />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        let collapsed = collapsed_signal.get();
                        store.get_nodes().into_iter().map(|node| {
                            let node_type = node.data.get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("default")
                                .to_string();
                            let parent_id = node.data.get("parent")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            // Check if this is a child node whose parent is collapsed
                            let is_hidden = parent_id.as_ref()
                                .map(|pid| collapsed.contains(pid))
                                .unwrap_or(false);

                            if is_hidden {
                                return view! { <div style="display: none;"></div> }.into_any();
                            }

                            if node_type == "subflow" {
                                view! {
                                    <SubflowNode
                                        node=node.clone()
                                        store=store
                                        collapsed_signal=collapsed_signal
                                        on_toggle=toggle_collapse.clone()
                                    />
                                }.into_any()
                            } else if parent_id.is_some() {
                                // Child node - render with offset from parent
                                view! {
                                    <ChildNode
                                        node=node.clone()
                                        store=store
                                        parent_id=parent_id.unwrap()
                                    />
                                }.into_any()
                            } else {
                                view! {
                                    <RegularNode
                                        node=node.clone()
                                        store=store
                                    />
                                }.into_any()
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Instructions badge
                <div style="position: absolute; top: 10px; left: 10px; background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%); color: white; padding: 8px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; box-shadow: 0 2px 8px rgba(0,0,0,0.2);">
                    "Nested Subflows"
                </div>

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 280px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Subflow Example"</strong>

                        // Instructions
                        <div style="background: #f0f9ff; padding: 10px; border-radius: 6px; margin-bottom: 12px; font-size: 11px; color: #0369a1; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 6px;">"Features:"</div>
                            <ul style="margin: 0; padding-left: 16px;">
                                <li>"Click subflow header to expand/collapse"</li>
                                <li>"Drag subflow to move all child nodes"</li>
                                <li>"Edges cross subflow boundaries"</li>
                                <li>"Child nodes hidden when collapsed"</li>
                            </ul>
                        </div>

                        // Subflow status
                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Subflows"</div>
                            {move || {
                                let collapsed = collapsed_signal.get();
                                let nodes = store.get_nodes();

                                nodes.iter()
                                    .filter(|n| n.data.get("type").and_then(|v| v.as_str()) == Some("subflow"))
                                    .map(|n| {
                                        let label = n.data.get("label")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Subflow")
                                            .to_string();
                                        let color = n.data.get("color")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("#6366f1")
                                            .to_string();
                                        let is_collapsed = collapsed.contains(&n.id);
                                        let child_count = n.data.get("child_ids")
                                            .and_then(|v| v.as_array())
                                            .map(|a| a.len())
                                            .unwrap_or(0);

                                        view! {
                                            <div style=format!(
                                                "display: flex; justify-content: space-between; align-items: center; \
                                                 padding: 8px; background: {}15; border-radius: 6px; margin-bottom: 6px;",
                                                color
                                            )>
                                                <div style="display: flex; align-items: center; gap: 6px;">
                                                    <div style=format!(
                                                        "width: 10px; height: 10px; border-radius: 3px; background: {};",
                                                        color
                                                    )></div>
                                                    <span style="font-size: 11px; font-weight: 500;">{label}</span>
                                                </div>
                                                <div style="display: flex; align-items: center; gap: 6px;">
                                                    <span style="font-size: 10px; color: #666;">
                                                        {format!("{} nodes", child_count)}
                                                    </span>
                                                    <span style=format!(
                                                        "font-size: 9px; padding: 2px 6px; border-radius: 4px; \
                                                         background: {}; color: white; font-weight: 500;",
                                                        if is_collapsed { "#ef4444" } else { "#10b981" }
                                                    )>
                                                        {if is_collapsed { "Collapsed" } else { "Expanded" }}
                                                    </span>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()
                            }}
                        </div>

                        // Quick actions
                        <div style="display: flex; gap: 4px; margin-bottom: 12px;">
                            <button
                                style="flex: 1; padding: 8px; font-size: 10px; border: none; \
                                       border-radius: 4px; background: #10b981; color: white; \
                                       cursor: pointer; font-weight: 500;"
                                on:click=expand_all
                            >
                                "Expand All"
                            </button>
                            <button
                                style="flex: 1; padding: 8px; font-size: 10px; border: none; \
                                       border-radius: 4px; background: #ef4444; color: white; \
                                       cursor: pointer; font-weight: 500;"
                                on:click=collapse_all
                            >
                                "Collapse All"
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
                                                "Toggle" => "#8b5cf6",
                                                "Drag" => "#10b981",
                                                "Expand All" => "#10b981",
                                                "Collapse All" => "#ef4444",
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
// Subflow Node Component
// ============================================================================

/// Subflow container node with expand/collapse
#[component]
fn SubflowNode<F>(
    node: Node,
    store: FlowStore,
    collapsed_signal: RwSignal<HashSet<String>>,
    on_toggle: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_toggle = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();
    let node_id_for_collapsed = node.id.clone();

    let drag_signal = get_subflow_drag_signal();

    // Mouse down on header - start drag
    let on_header_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
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

    // Toggle collapse on header click
    let on_toggle_clone = on_toggle.clone();
    let on_toggle_click = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        on_toggle_clone(node_id_for_toggle.clone());
    };

    view! {
        <div
            class="xyflow__node subflow-node"
            style=move || {
                let nodes = store.get_nodes();
                let collapsed = collapsed_signal.get();
                let is_collapsed = collapsed.contains(&node_id_for_style);

                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");

                    let height = if is_collapsed { 50.0 } else { n.height.unwrap_or(140.0) };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); \
                         width: {}px; height: {}px; \
                         background: {}15; border: 2px solid {}; border-radius: 12px; \
                         box-shadow: 0 4px 16px rgba(0,0,0,0.1); cursor: move; \
                         overflow: hidden; transition: height 0.3s ease;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(340.0), height,
                        color, color
                    )
                } else {
                    String::new()
                }
            }
        >
            // Header bar
            <div
                class="subflow-header"
                style=move || {
                    let nodes = store.get_nodes();
                    if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_label) {
                        let color = n.data.get("color")
                            .and_then(|v| v.as_str())
                            .unwrap_or("#6366f1");

                        format!(
                            "background: {}; color: white; padding: 8px 12px; \
                             display: flex; justify-content: space-between; align-items: center; \
                             cursor: grab; user-select: none;",
                            color
                        )
                    } else {
                        String::new()
                    }
                }
                on:mousedown=on_header_mousedown
            >
                <div style="display: flex; align-items: center; gap: 8px;">
                    <span style="font-size: 10px; opacity: 0.8;">"⋮⋮"</span>
                    {move || {
                        let nodes = store.get_nodes();
                        nodes.iter()
                            .find(|n| n.id == node_id)
                            .and_then(|n| n.data.get("label"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("Subflow")
                            .to_string()
                    }}
                </div>
                <button
                    style="background: rgba(255,255,255,0.2); border: none; color: white; \
                           width: 24px; height: 24px; border-radius: 4px; cursor: pointer; \
                           display: flex; align-items: center; justify-content: center; \
                           font-size: 12px; font-weight: bold;"
                    on:click=on_toggle_click
                >
                    {move || {
                        let collapsed = collapsed_signal.get();
                        if collapsed.contains(&node_id_for_collapsed) { "+" } else { "−" }
                    }}
                </button>
            </div>

            // Handles on the subflow container
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Left
                connection_mode=ConnectionMode::Strict
                style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Right
                connection_mode=ConnectionMode::Strict
                style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
        </div>
    }
}

// ============================================================================
// Child Node Component
// ============================================================================

/// Child node inside a subflow
#[component]
fn ChildNode(
    node: Node,
    store: FlowStore,
    parent_id: String,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    let drag_signal = get_subflow_drag_signal();

    // Mouse down - start drag (individual child drag)
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
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
        }
    };

    view! {
        <div
            class="xyflow__node child-node"
            style=move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    // Get parent position to calculate absolute position
                    let parent_pos = nodes.iter()
                        .find(|p| p.id == parent_id)
                        .map(|p| (p.position.x, p.position.y))
                        .unwrap_or((0.0, 0.0));

                    // Child position is absolute (already includes parent offset from drag)
                    format!(
                        "position: absolute; transform: translate({}px, {}px); \
                         width: {}px; height: {}px; \
                         background: white; border: 2px solid #d1d5db; border-radius: 8px; \
                         box-shadow: 0 2px 8px rgba(0,0,0,0.1); cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 6px; box-sizing: border-box; z-index: 1;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(80.0), n.height.unwrap_or(40.0)
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

                    view! {
                        <div style="font-weight: 500; font-size: 10px; color: #333;">
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
                                    style="background: #6b7280; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #6b7280; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
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
// Regular Node Component
// ============================================================================

/// Regular node (input/output outside subflows)
#[component]
fn RegularNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    let drag_signal = get_subflow_drag_signal();

    // Mouse down - start drag
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
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
        }
    };

    view! {
        <div
            class="xyflow__node regular-node"
            style=move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let color = n.data.get("color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#6366f1");
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    let background = match node_type {
                        "input" => format!("linear-gradient(135deg, {}30 0%, {}60 100%)", color, color),
                        "output" => format!("linear-gradient(135deg, {}30 0%, {}60 100%)", color, color),
                        _ => "white".to_string(),
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); \
                         width: {}px; height: {}px; \
                         background: {}; border: 2px solid {}; border-radius: 10px; \
                         box-shadow: 0 4px 12px rgba(0,0,0,0.15); cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(100.0), n.height.unwrap_or(50.0),
                        background, color
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
                        <div style=format!("font-weight: 600; font-size: 12px; color: {}; text-align: center;", color)>
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
                                    style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
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
// Subflow Edge Renderer
// ============================================================================

/// Edge renderer for Subflow example
#[component]
fn SubflowEdgeRenderer(
    store: FlowStore,
    collapsed_signal: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="subflow-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#6366f1" />
                    <stop offset="100%" stop-color="#8b5cf6" />
                </linearGradient>
                <linearGradient id="subflow-edge-gradient-cross" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#10b981" />
                    <stop offset="100%" stop-color="#059669" />
                </linearGradient>
                <marker
                    id="subflow-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#8b5cf6" />
                </marker>
                <marker
                    id="subflow-arrow-cross"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#059669" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let collapsed = collapsed_signal.get();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Check if either node's parent is collapsed
                    let source_parent = source_node.data.get("parent")
                        .and_then(|v| v.as_str());
                    let target_parent = target_node.data.get("parent")
                        .and_then(|v| v.as_str());

                    let source_hidden = source_parent.map(|p| collapsed.contains(p)).unwrap_or(false);
                    let target_hidden = target_parent.map(|p| collapsed.contains(p)).unwrap_or(false);

                    // If both ends are hidden, don't render
                    // If one end is hidden, connect to the parent subflow instead
                    let (sx, sy, tx, ty) = if source_hidden && target_hidden {
                        return None;
                    } else if source_hidden {
                        // Source is collapsed - connect from parent subflow
                        let parent_node = nodes.iter().find(|n| n.id == source_parent.unwrap())?;
                        let sx = parent_node.position.x + parent_node.width.unwrap_or(340.0);
                        let sy = parent_node.position.y + 25.0; // Center of collapsed header
                        let tx = target_node.position.x;
                        let ty = target_node.position.y + target_node.height.unwrap_or(40.0) / 2.0;
                        (sx, sy, tx, ty)
                    } else if target_hidden {
                        // Target is collapsed - connect to parent subflow
                        let parent_node = nodes.iter().find(|n| n.id == target_parent.unwrap())?;
                        let sx = source_node.position.x + source_node.width.unwrap_or(80.0);
                        let sy = source_node.position.y + source_node.height.unwrap_or(40.0) / 2.0;
                        let tx = parent_node.position.x;
                        let ty = parent_node.position.y + 25.0; // Center of collapsed header
                        (sx, sy, tx, ty)
                    } else {
                        // Both visible - normal edge
                        let sx = source_node.position.x + source_node.width.unwrap_or(80.0);
                        let sy = source_node.position.y + source_node.height.unwrap_or(40.0) / 2.0;
                        let tx = target_node.position.x;
                        let ty = target_node.position.y + target_node.height.unwrap_or(40.0) / 2.0;
                        (sx, sy, tx, ty)
                    };

                    // Check if edge crosses subflow boundaries
                    let crosses_boundary = source_parent != target_parent;

                    let offset = (tx - sx).abs() * 0.3;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx + offset, sy,
                        tx - offset, ty,
                        tx, ty
                    );

                    let gradient = if crosses_boundary {
                        "url(#subflow-edge-gradient-cross)"
                    } else {
                        "url(#subflow-edge-gradient)"
                    };

                    let marker = if crosses_boundary {
                        "url(#subflow-arrow-cross)"
                    } else {
                        "url(#subflow-arrow)"
                    };

                    // Calculate midpoint for label
                    let mid_x = (sx + tx) / 2.0;
                    let mid_y = (sy + ty) / 2.0;

                    let label = edge.label.clone().unwrap_or_default();
                    let stroke_dash = if crosses_boundary { "5 3" } else { "none" };

                    Some(view! {
                        <g class="xyflow__edge">
                            // Shadow/glow
                            <path
                                d=path.clone()
                                stroke=if crosses_boundary { "#10b98130" } else { "#6366f130" }
                                stroke-width="6"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path.clone()
                                stroke=gradient
                                stroke-width="2"
                                stroke-dasharray=stroke_dash
                                fill="none"
                                marker-end=marker
                            />
                            // Label
                            {(!label.is_empty()).then(|| view! {
                                <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                    <rect
                                        x="-24"
                                        y="-10"
                                        width="48"
                                        height="20"
                                        fill="white"
                                        stroke=if crosses_boundary { "#10b981" } else { "#6366f1" }
                                        stroke-width="1"
                                        rx="4"
                                    />
                                    <text
                                        x="0"
                                        y="4"
                                        text-anchor="middle"
                                        font-size="9"
                                        fill=if crosses_boundary { "#059669" } else { "#6366f1" }
                                        font-weight="500"
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
