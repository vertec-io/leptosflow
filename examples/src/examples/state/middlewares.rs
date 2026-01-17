//! Middlewares Example
//!
//! Demonstrates custom middleware/hooks for state operations.
//! Shows logging middleware, validation middleware, and middleware composition.

use leptos::prelude::*;
use leptos::serde_json::json;
use serde::{Deserialize, Serialize};
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for middlewares example
static MIDDLEWARES_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_middlewares_drag_signal() -> RwSignal<Option<DragState>> {
    *MIDDLEWARES_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Middleware Types and Traits
// ============================================================================

/// Action types that can be processed by middleware
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FlowAction {
    AddNode { id: String, label: String, x: f64, y: f64 },
    RemoveNode { id: String },
    MoveNode { id: String, x: f64, y: f64 },
    UpdateNodeLabel { id: String, label: String },
    AddEdge { id: String, source: String, target: String },
    RemoveEdge { id: String },
    SelectNode { id: String },
    DeselectAll,
}

impl FlowAction {
    fn description(&self) -> String {
        match self {
            FlowAction::AddNode { id, label, .. } => format!("Add node '{}' ({})", label, id),
            FlowAction::RemoveNode { id } => format!("Remove node '{}'", id),
            FlowAction::MoveNode { id, x, y } => format!("Move node '{}' to ({:.0}, {:.0})", id, x, y),
            FlowAction::UpdateNodeLabel { id, label } => format!("Update node '{}' label to '{}'", id, label),
            FlowAction::AddEdge { id, source, target } => format!("Add edge '{}': {} -> {}", id, source, target),
            FlowAction::RemoveEdge { id } => format!("Remove edge '{}'", id),
            FlowAction::SelectNode { id } => format!("Select node '{}'", id),
            FlowAction::DeselectAll => "Deselect all".to_string(),
        }
    }
}

/// Result of middleware processing
#[derive(Clone, Debug)]
pub struct MiddlewareResult {
    pub allowed: bool,
    pub reason: Option<String>,
    pub modified_action: Option<FlowAction>,
}

impl MiddlewareResult {
    pub fn allow() -> Self {
        Self { allowed: true, reason: None, modified_action: None }
    }

    pub fn deny(reason: &str) -> Self {
        Self { allowed: false, reason: Some(reason.to_string()), modified_action: None }
    }

    pub fn modify(action: FlowAction) -> Self {
        Self { allowed: true, reason: None, modified_action: Some(action) }
    }
}

/// Log entry for the middleware action log
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub action: String,
    pub middleware: String,
    pub result: String,
    pub success: bool,
}

// ============================================================================
// Middleware Implementations
// ============================================================================

/// Logging middleware - logs all actions
fn logging_middleware(
    action: &FlowAction,
    log_signal: RwSignal<Vec<LogEntry>>,
) -> MiddlewareResult {
    let timestamp = js_sys::Date::now();
    let time_str = format!("{:.1}s", (timestamp % 100000.0) / 1000.0);

    log_signal.update(|log| {
        log.insert(0, LogEntry {
            timestamp: time_str,
            action: action.description(),
            middleware: "Logging".to_string(),
            result: "Logged".to_string(),
            success: true,
        });
        if log.len() > 20 {
            log.pop();
        }
    });

    MiddlewareResult::allow()
}

/// Validation middleware - validates actions before execution
fn validation_middleware(
    action: &FlowAction,
    nodes: &[Node],
    edges: &[Edge],
    log_signal: RwSignal<Vec<LogEntry>>,
) -> MiddlewareResult {
    let timestamp = js_sys::Date::now();
    let time_str = format!("{:.1}s", (timestamp % 100000.0) / 1000.0);

    let result = match action {
        FlowAction::AddNode { id, x, y, .. } => {
            // Check if node ID already exists
            if nodes.iter().any(|n| n.id == *id) {
                MiddlewareResult::deny("Node ID already exists")
            }
            // Check bounds (must be within visible area)
            else if *x < -500.0 || *x > 1000.0 || *y < -500.0 || *y > 1000.0 {
                MiddlewareResult::deny("Node position out of bounds")
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::RemoveNode { id } => {
            // Check if node exists
            if !nodes.iter().any(|n| n.id == *id) {
                MiddlewareResult::deny("Node does not exist")
            }
            // Check if node is connected
            else if edges.iter().any(|e| e.source == *id || e.target == *id) {
                MiddlewareResult::deny("Cannot remove connected node")
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::MoveNode { id, x, y } => {
            // Check if node exists
            if !nodes.iter().any(|n| n.id == *id) {
                MiddlewareResult::deny("Node does not exist")
            }
            // Constrain to bounds
            else if *x < -500.0 || *x > 1000.0 || *y < -500.0 || *y > 1000.0 {
                // Modify action to constrain position
                let constrained_x = x.max(-500.0).min(1000.0);
                let constrained_y = y.max(-500.0).min(1000.0);
                MiddlewareResult::modify(FlowAction::MoveNode {
                    id: id.clone(),
                    x: constrained_x,
                    y: constrained_y,
                })
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::UpdateNodeLabel { id, label } => {
            // Check if node exists
            if !nodes.iter().any(|n| n.id == *id) {
                MiddlewareResult::deny("Node does not exist")
            }
            // Validate label (not empty, reasonable length)
            else if label.trim().is_empty() {
                MiddlewareResult::deny("Label cannot be empty")
            }
            else if label.len() > 50 {
                // Truncate label
                MiddlewareResult::modify(FlowAction::UpdateNodeLabel {
                    id: id.clone(),
                    label: label[..50].to_string(),
                })
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::AddEdge { id, source, target } => {
            // Check if edge ID already exists
            if edges.iter().any(|e| e.id == *id) {
                MiddlewareResult::deny("Edge ID already exists")
            }
            // Check if source and target exist
            else if !nodes.iter().any(|n| n.id == *source) {
                MiddlewareResult::deny("Source node does not exist")
            }
            else if !nodes.iter().any(|n| n.id == *target) {
                MiddlewareResult::deny("Target node does not exist")
            }
            // Check for self-loop
            else if source == target {
                MiddlewareResult::deny("Self-loops not allowed")
            }
            // Check for duplicate connection
            else if edges.iter().any(|e| e.source == *source && e.target == *target) {
                MiddlewareResult::deny("Connection already exists")
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::RemoveEdge { id } => {
            // Check if edge exists
            if !edges.iter().any(|e| e.id == *id) {
                MiddlewareResult::deny("Edge does not exist")
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::SelectNode { id } => {
            if !nodes.iter().any(|n| n.id == *id) {
                MiddlewareResult::deny("Node does not exist")
            }
            else {
                MiddlewareResult::allow()
            }
        },
        FlowAction::DeselectAll => MiddlewareResult::allow(),
    };

    // Log validation result
    log_signal.update(|log| {
        log.insert(0, LogEntry {
            timestamp: time_str,
            action: action.description(),
            middleware: "Validation".to_string(),
            result: if result.allowed {
                if result.modified_action.is_some() { "Modified".to_string() }
                else { "Allowed".to_string() }
            } else {
                result.reason.clone().unwrap_or("Denied".to_string())
            },
            success: result.allowed,
        });
        if log.len() > 20 {
            log.pop();
        }
    });

    result
}

/// Throttle middleware - limits frequency of certain actions
/// This demonstrates the pattern for throttling actions, used in the full pipeline.
#[allow(dead_code)]
fn throttle_middleware(
    action: &FlowAction,
    last_action_time: RwSignal<Option<f64>>,
    throttle_ms: f64,
    log_signal: RwSignal<Vec<LogEntry>>,
) -> MiddlewareResult {
    let now = js_sys::Date::now();

    // Only throttle move actions
    if let FlowAction::MoveNode { .. } = action {
        if let Some(last_time) = last_action_time.get() {
            if now - last_time < throttle_ms {
                return MiddlewareResult::allow(); // Still allow but don't log repeatedly
            }
        }
    }

    last_action_time.set(Some(now));

    let timestamp = format!("{:.1}s", (now % 100000.0) / 1000.0);
    log_signal.update(|log| {
        log.insert(0, LogEntry {
            timestamp,
            action: action.description(),
            middleware: "Throttle".to_string(),
            result: "Passed".to_string(),
            success: true,
        });
        if log.len() > 20 {
            log.pop();
        }
    });

    MiddlewareResult::allow()
}

// ============================================================================
// Middleware Pipeline
// ============================================================================

/// Processes an action through the middleware pipeline
/// This is an example of a full middleware chain - simplified dispatch is used in the component.
#[allow(dead_code)]
fn process_middleware_pipeline(
    action: FlowAction,
    store: FlowStore,
    log_signal: RwSignal<Vec<LogEntry>>,
    last_action_time: RwSignal<Option<f64>>,
    enabled_middlewares: RwSignal<EnabledMiddlewares>,
) -> bool {
    let nodes = store.get_nodes();
    let edges = store.get_edges();
    let enabled = enabled_middlewares.get();

    let mut current_action = action;

    // 1. Logging middleware (always first)
    if enabled.logging {
        logging_middleware(&current_action, log_signal);
    }

    // 2. Throttle middleware
    if enabled.throttle {
        let result = throttle_middleware(&current_action, last_action_time, 100.0, log_signal);
        if !result.allowed {
            return false;
        }
    }

    // 3. Validation middleware (last, can modify or deny)
    if enabled.validation {
        let result = validation_middleware(&current_action, &nodes, &edges, log_signal);
        if !result.allowed {
            return false;
        }
        if let Some(modified) = result.modified_action {
            current_action = modified;
        }
    }

    // Execute the action
    execute_action(current_action, store);
    true
}

/// Execute the action on the store
fn execute_action(action: FlowAction, store: FlowStore) {
    match action {
        FlowAction::AddNode { id, label, x, y } => {
            let mut nodes = store.get_nodes();
            let new_node = Node::new(id, Position::new(x, y))
                .with_data(json!({"label": label, "nodeType": "default"}))
                .with_dimensions(140.0, 60.0);
            nodes.push(new_node);
            store.set_nodes(nodes);
        },
        FlowAction::RemoveNode { id } => {
            let nodes: Vec<_> = store.get_nodes().into_iter().filter(|n| n.id != id).collect();
            store.set_nodes(nodes);
        },
        FlowAction::MoveNode { id, x, y } => {
            store.update_node(&id, |n| {
                n.position = Position::new(x, y);
            });
        },
        FlowAction::UpdateNodeLabel { id, label } => {
            store.update_node(&id, |n| {
                if let Some(data) = n.data.as_object_mut() {
                    data.insert("label".to_string(), json!(label));
                }
            });
        },
        FlowAction::AddEdge { id, source, target } => {
            let mut edges = store.get_edges();
            edges.push(Edge::new(id, source, target));
            store.set_edges(edges);
        },
        FlowAction::RemoveEdge { id } => {
            let edges: Vec<_> = store.get_edges().into_iter().filter(|e| e.id != id).collect();
            store.set_edges(edges);
        },
        FlowAction::SelectNode { id } => {
            store.update_node(&id, |n| {
                n.selected = true;
            });
        },
        FlowAction::DeselectAll => {
            let nodes = store.get_nodes();
            for node in &nodes {
                store.update_node(&node.id, |n| {
                    n.selected = false;
                });
            }
        },
    }
}

// ============================================================================
// Enabled Middlewares State
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub struct EnabledMiddlewares {
    pub logging: bool,
    pub validation: bool,
    pub throttle: bool,
}

impl Default for EnabledMiddlewares {
    fn default() -> Self {
        Self {
            logging: true,
            validation: true,
            throttle: false,
        }
    }
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Middlewares example component
#[component]
pub fn MiddlewaresExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Start", "nodeType": "input"}))
            .with_dimensions(140.0, 60.0),
        Node::new("2".to_string(), Position::new(200.0, 150.0))
            .with_data(json!({"label": "Process", "nodeType": "default"}))
            .with_dimensions(140.0, 60.0),
        Node::new("3".to_string(), Position::new(50.0, 250.0))
            .with_data(json!({"label": "Validate", "nodeType": "default"}))
            .with_dimensions(140.0, 60.0),
        Node::new("4".to_string(), Position::new(200.0, 350.0))
            .with_data(json!({"label": "Output", "nodeType": "output"}))
            .with_dimensions(140.0, 60.0),
    ];

    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("data".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("check".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("result".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("valid".to_string()),
    ];

    let flow_store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(flow_store);

    // Middleware state
    let log_signal = RwSignal::new(Vec::<LogEntry>::new());
    let last_action_time = RwSignal::new(None::<f64>);
    let enabled_middlewares = RwSignal::new(EnabledMiddlewares::default());

    // Stats
    let total_actions = RwSignal::new(0u32);
    let blocked_actions = RwSignal::new(0u32);
    let modified_actions = RwSignal::new(0u32);

    // Node counter for generating unique IDs
    let node_counter = RwSignal::new(5u32);

    // Dispatch action through middleware
    let dispatch = {
        let flow_store = flow_store;
        let log_signal = log_signal;
        let _last_action_time = last_action_time; // For throttle middleware (not used in simplified dispatch)
        let enabled_middlewares = enabled_middlewares;
        move |action: FlowAction| {
            total_actions.update(|c| *c += 1);

            let nodes = flow_store.get_nodes();
            let edges = flow_store.get_edges();
            let enabled = enabled_middlewares.get();

            // Track if action was modified
            let mut was_modified = false;
            let mut current_action = action.clone();

            // Process through middleware
            if enabled.validation {
                let result = validation_middleware(&current_action, &nodes, &edges, log_signal);
                if !result.allowed {
                    blocked_actions.update(|c| *c += 1);
                    return;
                }
                if let Some(modified) = result.modified_action {
                    current_action = modified;
                    was_modified = true;
                }
            }

            if enabled.logging && !was_modified {
                logging_middleware(&current_action, log_signal);
            }

            if was_modified {
                modified_actions.update(|c| *c += 1);
            }

            // Execute
            execute_action(current_action, flow_store);
        }
    };

    // Test actions
    let add_node = {
        let dispatch = dispatch.clone();
        let node_counter = node_counter;
        move |_| {
            let id = node_counter.get();
            node_counter.update(|c| *c += 1);
            dispatch(FlowAction::AddNode {
                id: id.to_string(),
                label: format!("Node {}", id),
                x: 100.0 + (id as f64 % 3.0) * 100.0,
                y: 400.0 + (id as f64 / 3.0).floor() * 80.0,
            });
        }
    };

    let add_invalid_node = {
        let dispatch = dispatch.clone();
        move |_| {
            dispatch(FlowAction::AddNode {
                id: "1".to_string(), // Duplicate ID
                label: "Duplicate".to_string(),
                x: 100.0,
                y: 100.0,
            });
        }
    };

    let add_out_of_bounds = {
        let dispatch = dispatch.clone();
        let node_counter = node_counter;
        move |_| {
            let id = node_counter.get();
            node_counter.update(|c| *c += 1);
            dispatch(FlowAction::AddNode {
                id: id.to_string(),
                label: format!("OOB {}", id),
                x: 2000.0, // Out of bounds
                y: 2000.0,
            });
        }
    };

    let remove_connected_node = {
        let dispatch = dispatch.clone();
        move |_| {
            dispatch(FlowAction::RemoveNode {
                id: "2".to_string(), // Connected node
            });
        }
    };

    let add_self_loop = {
        let dispatch = dispatch.clone();
        move |_| {
            dispatch(FlowAction::AddEdge {
                id: "e-self".to_string(),
                source: "1".to_string(),
                target: "1".to_string(), // Self-loop
            });
        }
    };

    let update_label_empty = {
        let dispatch = dispatch.clone();
        move |_| {
            dispatch(FlowAction::UpdateNodeLabel {
                id: "1".to_string(),
                label: "   ".to_string(), // Empty label
            });
        }
    };

    let update_label_long = {
        let dispatch = dispatch.clone();
        move |_| {
            dispatch(FlowAction::UpdateNodeLabel {
                id: "1".to_string(),
                label: "This is a very long label that exceeds the maximum allowed length and should be truncated by the middleware".to_string(),
            });
        }
    };

    // Toggle middlewares
    let toggle_logging = move |_| {
        enabled_middlewares.update(|m| m.logging = !m.logging);
    };

    let toggle_validation = move |_| {
        enabled_middlewares.update(|m| m.validation = !m.validation);
    };

    let toggle_throttle = move |_| {
        enabled_middlewares.update(|m| m.throttle = !m.throttle);
    };

    let clear_log = move |_| {
        log_signal.set(Vec::new());
    };

    // Global drag handlers
    let drag_signal = get_middlewares_drag_signal();

    let on_global_mousemove = {
        let dispatch = dispatch.clone();
        let flow_store = flow_store;
        move |ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let current_x = ev.client_x() as f64;
                let current_y = ev.client_y() as f64;
                let (start_x, start_y) = drag_state.start_mouse;
                let (node_start_x, node_start_y) = drag_state.start_pos;

                let viewport = flow_store.get_viewport();
                let dx = (current_x - start_x) / viewport.zoom;
                let dy = (current_y - start_y) / viewport.zoom;

                let new_x = node_start_x + dx;
                let new_y = node_start_y + dy;

                // Dispatch through middleware (will be constrained if out of bounds)
                dispatch(FlowAction::MoveNode {
                    id: drag_state.node_id.clone(),
                    x: new_x,
                    y: new_y,
                });
            }
        }
    };

    let on_global_mouseup = {
        let flow_store = flow_store;
        move |_ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                flow_store.update_node(&drag_state.node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
            }
        }
    };

    view! {
        <div class="example-container">
            <div
                class="xyflow leptos-flow svelte-flow"
                style="width: 100%; height: 100%; position: relative;"
                on:mousemove=on_global_mousemove
                on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container
                <FlowViewport store=flow_store>
                    // Edge renderer
                    <MiddlewaresEdgeRenderer flow_store=flow_store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        flow_store.get_nodes().into_iter()
                            .map(|node| {
                                view! {
                                    <MiddlewaresNode node=node.clone() flow_store=flow_store />
                                }
                            }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <div style="position: absolute; top: 16px; right: 16px; width: 360px; \
                            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); \
                            border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.2); \
                            padding: 16px; color: white; font-family: system-ui, -apple-system, sans-serif; \
                            max-height: calc(100vh - 100px); overflow-y: auto;">
                    <div style="font-size: 18px; font-weight: 600; margin-bottom: 12px; \
                                display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 20px;">"🔗"</span>
                        "Middlewares"
                    </div>

                    // Middleware Pipeline Visualization
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"MIDDLEWARE PIPELINE"</div>
                        <div style="display: flex; align-items: center; gap: 6px; font-size: 10px;">
                            <span style="background: rgba(255,255,255,0.2); padding: 4px 8px; border-radius: 4px;">"Action"</span>
                            <span>"→"</span>
                            <button
                                on:click=toggle_logging
                                style=move || format!("padding: 4px 8px; border-radius: 4px; border: 1px solid rgba(255,255,255,0.3); \
                                       cursor: pointer; color: white; font-size: 10px; background: {};",
                                       if enabled_middlewares.get().logging { "rgba(74,222,128,0.4)" } else { "rgba(255,255,255,0.1)" })
                            >
                                "Logging"
                            </button>
                            <span>"→"</span>
                            <button
                                on:click=toggle_throttle
                                style=move || format!("padding: 4px 8px; border-radius: 4px; border: 1px solid rgba(255,255,255,0.3); \
                                       cursor: pointer; color: white; font-size: 10px; background: {};",
                                       if enabled_middlewares.get().throttle { "rgba(74,222,128,0.4)" } else { "rgba(255,255,255,0.1)" })
                            >
                                "Throttle"
                            </button>
                            <span>"→"</span>
                            <button
                                on:click=toggle_validation
                                style=move || format!("padding: 4px 8px; border-radius: 4px; border: 1px solid rgba(255,255,255,0.3); \
                                       cursor: pointer; color: white; font-size: 10px; background: {};",
                                       if enabled_middlewares.get().validation { "rgba(74,222,128,0.4)" } else { "rgba(255,255,255,0.1)" })
                            >
                                "Validation"
                            </button>
                            <span>"→"</span>
                            <span style="background: rgba(255,255,255,0.2); padding: 4px 8px; border-radius: 4px;">"Store"</span>
                        </div>
                    </div>

                    // Stats
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"STATS"</div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 8px;">
                            <div style="background: rgba(255,255,255,0.1); padding: 6px 8px; border-radius: 4px; text-align: center;">
                                <div style="font-size: 16px; font-weight: 700;">{move || total_actions.get()}</div>
                                <div style="font-size: 9px; opacity: 0.8;">"Total"</div>
                            </div>
                            <div style="background: rgba(248,113,113,0.3); padding: 6px 8px; border-radius: 4px; text-align: center;">
                                <div style="font-size: 16px; font-weight: 700;">{move || blocked_actions.get()}</div>
                                <div style="font-size: 9px; opacity: 0.8;">"Blocked"</div>
                            </div>
                            <div style="background: rgba(250,204,21,0.3); padding: 6px 8px; border-radius: 4px; text-align: center;">
                                <div style="font-size: 16px; font-weight: 700;">{move || modified_actions.get()}</div>
                                <div style="font-size: 9px; opacity: 0.8;">"Modified"</div>
                            </div>
                        </div>
                    </div>

                    // Test Actions
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"TEST ACTIONS"</div>

                        // Valid actions
                        <div style="font-size: 10px; margin-bottom: 6px; opacity: 0.8;">"Valid actions:"</div>
                        <div style="display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 10px;">
                            <button
                                on:click=add_node
                                style="padding: 4px 8px; background: rgba(74,222,128,0.3); \
                                       border: 1px solid rgba(74,222,128,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "+ Add Node"
                            </button>
                        </div>

                        // Invalid actions (should be blocked)
                        <div style="font-size: 10px; margin-bottom: 6px; opacity: 0.8;">"Invalid actions (should be blocked):"</div>
                        <div style="display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 10px;">
                            <button
                                on:click=add_invalid_node
                                style="padding: 4px 8px; background: rgba(248,113,113,0.3); \
                                       border: 1px solid rgba(248,113,113,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Duplicate ID"
                            </button>
                            <button
                                on:click=add_out_of_bounds
                                style="padding: 4px 8px; background: rgba(248,113,113,0.3); \
                                       border: 1px solid rgba(248,113,113,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Out of Bounds"
                            </button>
                            <button
                                on:click=remove_connected_node
                                style="padding: 4px 8px; background: rgba(248,113,113,0.3); \
                                       border: 1px solid rgba(248,113,113,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Remove Connected"
                            </button>
                            <button
                                on:click=add_self_loop
                                style="padding: 4px 8px; background: rgba(248,113,113,0.3); \
                                       border: 1px solid rgba(248,113,113,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Self-Loop Edge"
                            </button>
                            <button
                                on:click=update_label_empty
                                style="padding: 4px 8px; background: rgba(248,113,113,0.3); \
                                       border: 1px solid rgba(248,113,113,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Empty Label"
                            </button>
                        </div>

                        // Actions that get modified
                        <div style="font-size: 10px; margin-bottom: 6px; opacity: 0.8;">"Actions that get modified:"</div>
                        <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                            <button
                                on:click=update_label_long
                                style="padding: 4px 8px; background: rgba(250,204,21,0.3); \
                                       border: 1px solid rgba(250,204,21,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Long Label (Truncate)"
                            </button>
                        </div>
                    </div>

                    // Action Log
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px;">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                            <div style="font-size: 11px; font-weight: 600; opacity: 0.9;">"ACTION LOG"</div>
                            <button
                                on:click=clear_log
                                style="padding: 2px 8px; background: rgba(255,255,255,0.1); \
                                       border: 1px solid rgba(255,255,255,0.2); border-radius: 4px; \
                                       color: white; font-size: 9px; cursor: pointer;"
                            >
                                "Clear"
                            </button>
                        </div>
                        <div style="max-height: 150px; overflow-y: auto; font-size: 9px; font-family: monospace;">
                            {move || {
                                let log = log_signal.get();
                                if log.is_empty() {
                                    view! {
                                        <div style="color: rgba(255,255,255,0.5); font-style: italic;">
                                            "Try the test actions above..."
                                        </div>
                                    }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        let bg_color = if !entry.success {
                                            "rgba(248,113,113,0.2)"
                                        } else if entry.result == "Modified" {
                                            "rgba(250,204,21,0.2)"
                                        } else {
                                            "rgba(255,255,255,0.05)"
                                        };

                                        view! {
                                            <div style=format!("padding: 4px 6px; border-radius: 4px; margin-bottom: 4px; background: {};", bg_color)>
                                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2px;">
                                                    <span style="opacity: 0.6;">{entry.timestamp.clone()}</span>
                                                    <span style=format!("padding: 1px 4px; border-radius: 2px; font-size: 8px; background: {};",
                                                        if !entry.success { "rgba(248,113,113,0.5)" }
                                                        else if entry.result == "Modified" { "rgba(250,204,21,0.5)" }
                                                        else { "rgba(74,222,128,0.5)" })>
                                                        {entry.middleware.clone()}
                                                    </span>
                                                </div>
                                                <div style="opacity: 0.9;">{entry.action.clone()}</div>
                                                <div style=format!("font-size: 8px; opacity: 0.7; color: {};",
                                                    if !entry.success { "#f87171" }
                                                    else if entry.result == "Modified" { "#fbbf24" }
                                                    else { "#4ade80" })>
                                                    {entry.result.clone()}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>

                // Info badge
                <div style="position: absolute; bottom: 60px; left: 16px; \
                            background: rgba(240, 147, 251, 0.9); color: white; \
                            padding: 8px 12px; border-radius: 8px; font-size: 11px; \
                            max-width: 220px; line-height: 1.4;">
                    <div style="font-weight: 600; margin-bottom: 4px;">"Middleware Features"</div>
                    <div>"• Logging: Track all actions"</div>
                    <div>"• Validation: Block invalid ops"</div>
                    <div>"• Modification: Transform data"</div>
                    <div>"• Composition: Chain middlewares"</div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Node Component
// ============================================================================

/// Node component for middlewares example
#[component]
fn MiddlewaresNode(node: Node, flow_store: FlowStore) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let drag_signal = get_middlewares_drag_signal();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    // Get color based on node type
    let color = match node_type.as_str() {
        "input" => "#f093fb",
        "output" => "#f5576c",
        _ => "#ec4899",
    };

    // Mouse handlers
    let on_mousedown = {
        let node_id = node_id.clone();
        let flow_store = flow_store;
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();

            // Get current node position
            let nodes = flow_store.get_nodes();
            if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                drag_signal.set(Some(DragState {
                    node_id: node_id.clone(),
                    start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                    start_pos: (node.position.x, node.position.y),
                }));

                // Mark node as dragging
                flow_store.update_node(&node_id, |n| {
                    n.dragging = true;
                });
            }
        }
    };

    // Get reactive node position
    let pos = {
        let node_id_for_render = node_id_for_render.clone();
        move || {
            flow_store.get_nodes()
                .iter()
                .find(|n| n.id == node_id_for_render)
                .map(|n| n.position)
                .unwrap_or(Position::new(0.0, 0.0))
        }
    };

    // Get reactive label
    let reactive_label = {
        let node_id_for_label = node_id_for_render.clone();
        move || {
            flow_store.get_nodes()
                .iter()
                .find(|n| n.id == node_id_for_label)
                .and_then(|n| n.data.get("label"))
                .and_then(|v| v.as_str())
                .unwrap_or(&label)
                .to_string()
        }
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab; \
                 background: {}; border: 2px solid {}; border-radius: 8px; \
                 padding: 12px 16px; min-width: 120px; text-align: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.15);",
                pos().x, pos().y, color, color
            )
            on:mousedown=on_mousedown
        >
            // Node label
            <div style="color: white; font-weight: 600; font-size: 13px; text-shadow: 0 1px 2px rgba(0,0,0,0.2);">
                {reactive_label}
            </div>

            // Handles based on node type
            {match node_type.as_str() {
                "input" => view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Bottom
                        connection_mode=ConnectionMode::Strict
                    />
                }.into_any(),
                "output" => view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                }.into_any(),
                _ => view! {
                    <>
                        <Handle
                            node_id=node.id.clone()
                            r#type=HandleType::Target
                            position=HandlePosition::Top
                            connection_mode=ConnectionMode::Strict
                        />
                        <Handle
                            node_id=node.id.clone()
                            r#type=HandleType::Source
                            position=HandlePosition::Bottom
                            connection_mode=ConnectionMode::Strict
                        />
                    </>
                }.into_any(),
            }}
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

/// Edge renderer for middlewares example
#[component]
fn MiddlewaresEdgeRenderer(flow_store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                // Gradient for edges
                <linearGradient id="middlewares-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #f093fb; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #f5576c; stop-opacity: 1" />
                </linearGradient>

                // Arrow marker
                <marker
                    id="middlewares-arrow"
                    viewBox="0 0 10 10"
                    refX="10"
                    refY="5"
                    markerUnits="strokeWidth"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f5576c" />
                </marker>
            </defs>

            {move || {
                let nodes = flow_store.get_nodes();
                let edges = flow_store.get_edges();

                edges.into_iter().filter_map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate edge positions
                    let source_x = source_node.position.x + source_node.width.unwrap_or(140.0) / 2.0;
                    let source_y = source_node.position.y + source_node.height.unwrap_or(60.0);
                    let target_x = target_node.position.x + target_node.width.unwrap_or(140.0) / 2.0;
                    let target_y = target_node.position.y;

                    // Generate bezier path
                    let control_offset = (target_y - source_y).abs() * 0.5;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        source_x, source_y,
                        source_x, source_y + control_offset,
                        target_x, target_y - control_offset,
                        target_x, target_y
                    );

                    // Calculate midpoint for label
                    let mid_x = (source_x + target_x) / 2.0;
                    let mid_y = (source_y + target_y) / 2.0;

                    let label = edge.label.clone().unwrap_or_default();

                    Some(view! {
                        <g>
                            // Shadow/glow
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="rgba(240, 147, 251, 0.3)"
                                stroke-width="6"
                                stroke-linecap="round"
                            />

                            // Main edge
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="url(#middlewares-edge-gradient)"
                                stroke-width="2"
                                stroke-linecap="round"
                                marker-end="url(#middlewares-arrow)"
                            />

                            // Edge label
                            {(!label.is_empty()).then(|| view! {
                                <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                    <rect
                                        x="-25"
                                        y="-10"
                                        width="50"
                                        height="20"
                                        rx="4"
                                        fill="white"
                                        stroke="#f093fb"
                                        stroke-width="1"
                                    />
                                    <text
                                        x="0"
                                        y="4"
                                        text-anchor="middle"
                                        font-size="10"
                                        font-weight="500"
                                        fill="#ec4899"
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
