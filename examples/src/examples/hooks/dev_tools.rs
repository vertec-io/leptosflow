//! Dev Tools Example
//!
//! Demonstrates a debug panel showing flow internals.
//! Shows current nodes, edges, viewport state, connection state,
//! and event log with a toggleable panel.

use leptos::prelude::*;
use serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Drag state for DevTools example
static DEV_TOOLS_DRAG_STATE: OnceLock<RwSignal<Option<DragState>>> = OnceLock::new();

fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *DEV_TOOLS_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// DevTools panel visibility
static DEV_TOOLS_PANEL_VISIBLE: OnceLock<RwSignal<bool>> = OnceLock::new();

fn get_panel_visible() -> RwSignal<bool> {
    *DEV_TOOLS_PANEL_VISIBLE.get_or_init(|| RwSignal::new(true))
}

/// Event log entries
#[derive(Clone, Debug)]
pub struct EventLogEntry {
    pub timestamp: String,
    pub event_type: String,
    pub details: String,
}

static DEV_TOOLS_EVENT_LOG: OnceLock<RwSignal<Vec<EventLogEntry>>> = OnceLock::new();

fn get_event_log() -> RwSignal<Vec<EventLogEntry>> {
    *DEV_TOOLS_EVENT_LOG.get_or_init(|| RwSignal::new(Vec::new()))
}

fn log_event(event_type: &str, details: &str) {
    let timestamp = get_timestamp();
    get_event_log().update(|entries| {
        entries.push(EventLogEntry {
            timestamp,
            event_type: event_type.to_string(),
            details: details.to_string(),
        });
        // Keep last 50 entries
        if entries.len() > 50 {
            entries.remove(0);
        }
    });
}

fn get_timestamp() -> String {
    // Simple counter-based timestamp for demo
    static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{:04}", count)
}

// ============================================================================
// DevTools Example Component
// ============================================================================

/// DevTools example - Demonstrates a debug panel showing flow internals
#[component]
pub fn DevToolsExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("input-1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Data Source",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(130.0, 50.0),
        Node::new("process-1".to_string(), Position::new(250.0, 30.0))
            .with_data(json!({
                "label": "Transform",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("process-2".to_string(), Position::new(250.0, 120.0))
            .with_data(json!({
                "label": "Validate",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("output-1".to_string(), Position::new(450.0, 75.0))
            .with_data(json!({
                "label": "Output",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    let initial_edges = vec![
        Edge::new("e1".to_string(), "input-1".to_string(), "process-1".to_string()),
        Edge::new("e2".to_string(), "input-1".to_string(), "process-2".to_string()),
        Edge::new("e3".to_string(), "process-1".to_string(), "output-1".to_string()),
        Edge::new("e4".to_string(), "process-2".to_string(), "output-1".to_string()),
    ];

    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store.clone());

    let drag_signal = get_drag_signal();
    let panel_visible = get_panel_visible();
    let event_log = get_event_log();

    // Reset state on mount
    panel_visible.set(true);
    event_log.set(vec![EventLogEntry {
        timestamp: "0000".to_string(),
        event_type: "init".to_string(),
        details: "DevTools example loaded".to_string(),
    }]);

    // Track viewport changes
    let prev_viewport = RwSignal::new(store.get_viewport());
    Effect::new({
        let store = store.clone();
        move |_| {
            let current = store.get_viewport();
            let prev = prev_viewport.get();
            if (current.x - prev.x).abs() > 0.1
                || (current.y - prev.y).abs() > 0.1
                || (current.zoom - prev.zoom).abs() > 0.01
            {
                prev_viewport.set(current.clone());
                log_event("viewport", &format!(
                    "x: {:.1}, y: {:.1}, zoom: {:.2}",
                    current.x, current.y, current.zoom
                ));
            }
        }
    });

    // Track selection changes
    let prev_selection = RwSignal::new((0usize, 0usize));
    Effect::new({
        let store = store.clone();
        move |_| {
            let nodes = store.get_selected_nodes().len();
            let edges = store.get_selected_edges().len();
            let (prev_n, prev_e) = prev_selection.get();
            if nodes != prev_n || edges != prev_e {
                prev_selection.set((nodes, edges));
                if nodes > 0 || edges > 0 {
                    log_event("selection", &format!("{} nodes, {} edges", nodes, edges));
                }
            }
        }
    });

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
        move |_ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                store.update_node(&node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
                log_event("drag_end", &format!("Node: {}", node_id));
            }
        }
    };

    // Toggle panel visibility
    let toggle_panel = move |_| {
        let visible = panel_visible.get();
        panel_visible.set(!visible);
        log_event("panel", if visible { "DevTools hidden" } else { "DevTools shown" });
    };

    view! {
        <div class="example-container" style="display: flex; flex-direction: column; height: 100%;">
            // Header
            <div style="padding: 12px; background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%); \
                        border-bottom: 1px solid #4c1d95; display: flex; align-items: center; gap: 12px;">
                <div style="background: #7c3aed; color: white; padding: 6px 12px; border-radius: 6px; \
                            font-size: 11px; font-weight: 600; display: flex; align-items: center; gap: 6px;">
                    <span style="font-size: 14px;">"🔧"</span>
                    "DevTools"
                </div>
                <div style="font-size: 12px; color: #c4b5fd;">
                    "Debug panel showing flow internals"
                </div>
                <button
                    style="margin-left: auto; padding: 6px 12px; background: #4c1d95; color: white; \
                           border: 1px solid #7c3aed; border-radius: 6px; font-size: 11px; cursor: pointer; \
                           display: flex; align-items: center; gap: 6px;"
                    on:click=toggle_panel
                >
                    {move || if panel_visible.get() { "Hide Panel" } else { "Show Panel" }}
                    <span style="font-size: 10px; opacity: 0.7;">
                        {move || if panel_visible.get() { "▼" } else { "▲" }}
                    </span>
                </button>
            </div>

            // Main content
            <div style="display: flex; flex: 1; min-height: 0;">
                // Flow canvas
                <div
                    class="xyflow leptos-flow"
                    style="flex: 1; position: relative; background: #0f0f1a;"
                    on:mousemove=on_mousemove
                    on:mouseup=on_mouseup
                >
                    <Background variant=BackgroundVariant::Dots />

                    <FlowViewport store=store.clone()>
                        <DevToolsEdgeRenderer store=store.clone() />
                        <ConnectionLine />

                        {move || {
                            store.get_nodes().into_iter().map(|node| {
                                view! {
                                    <DevToolsNode
                                        node=node.clone()
                                        store=store.clone()
                                        drag_signal=drag_signal
                                    />
                                }
                            }).collect_view()
                        }}
                    </FlowViewport>

                    <Controls position=PanelPosition::BottomLeft />

                    // Quick stats overlay
                    <div style="position: absolute; top: 12px; left: 12px; background: rgba(30, 27, 75, 0.9); \
                                padding: 8px 12px; border-radius: 8px; border: 1px solid #4c1d95; \
                                font-family: monospace; font-size: 10px; color: #a5b4fc;">
                        <div style="display: flex; gap: 16px;">
                            {move || {
                                let nodes = store.get_nodes().len();
                                let edges = store.get_edges().len();
                                let selected = store.get_selected_nodes().len();
                                view! {
                                    <>
                                        <span>"Nodes: "<b style="color: #10b981;">{nodes}</b></span>
                                        <span>"Edges: "<b style="color: #8b5cf6;">{edges}</b></span>
                                        <span>"Selected: "<b style="color: #f59e0b;">{selected}</b></span>
                                    </>
                                }
                            }}
                        </div>
                    </div>
                </div>

                // DevTools Panel
                {move || {
                    if panel_visible.get() {
                        view! {
                            <div style="width: 340px; background: #0f0f1a; border-left: 1px solid #312e81; \
                                        display: flex; flex-direction: column; overflow-y: auto;">
                                <NodesInspector store=store.clone() />
                                <EdgesInspector store=store.clone() />
                                <ViewportInspector store=store.clone() />
                                <ConnectionStateInspector store=store.clone() />
                                <EventLogPanel />
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Nodes Inspector Component
// ============================================================================

#[component]
fn NodesInspector(store: FlowStore) -> impl IntoView {
    view! {
        <div style="padding: 12px; border-bottom: 1px solid #312e81;">
            <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #10b981; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"NODES"</span>
                "Node State"
                <span style="margin-left: auto; font-size: 10px; color: #6366f1;">
                    {move || format!("({})", store.get_nodes().len())}
                </span>
            </div>

            <div style="max-height: 200px; overflow-y: auto;">
                {move || {
                    let nodes = store.get_nodes();
                    nodes.into_iter().map(|node| {
                        let color = node.data.get("color")
                            .and_then(|v| v.as_str())
                            .unwrap_or("#6366f1");
                        let label = node.data.get("label")
                            .and_then(|v| v.as_str())
                            .unwrap_or(&node.id);
                        let is_selected = node.selected;
                        let is_dragging = node.dragging;

                        view! {
                            <div style="background: #1e1b4b; padding: 8px; border-radius: 6px; \
                                        margin-bottom: 6px; font-family: monospace; font-size: 10px;">
                                <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px;">
                                    <span style=format!(
                                        "width: 8px; height: 8px; border-radius: 50%; background: {};",
                                        color
                                    )></span>
                                    <span style="color: #e9d5ff; font-weight: 600;">{label.to_string()}</span>
                                    <span style="color: #6366f1; font-size: 9px;">{"id: "}{node.id.clone()}</span>
                                    {is_selected.then(|| view! {
                                        <span style="background: #f59e0b; color: #1a1a1a; padding: 1px 4px; \
                                                     border-radius: 3px; font-size: 8px; font-weight: 600;">
                                            "SEL"
                                        </span>
                                    })}
                                    {is_dragging.then(|| view! {
                                        <span style="background: #22c55e; color: #1a1a1a; padding: 1px 4px; \
                                                     border-radius: 3px; font-size: 8px; font-weight: 600;">
                                            "DRAG"
                                        </span>
                                    })}
                                </div>
                                <div style="color: #a5b4fc; display: flex; gap: 12px;">
                                    <span>"x: "{format!("{:.1}", node.position.x)}</span>
                                    <span>"y: "{format!("{:.1}", node.position.y)}</span>
                                    <span>"w: "{format!("{:.0}", node.width.unwrap_or(120.0))}</span>
                                    <span>"h: "{format!("{:.0}", node.height.unwrap_or(50.0))}</span>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Edges Inspector Component
// ============================================================================

#[component]
fn EdgesInspector(store: FlowStore) -> impl IntoView {
    view! {
        <div style="padding: 12px; border-bottom: 1px solid #312e81;">
            <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #8b5cf6; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"EDGES"</span>
                "Edge State"
                <span style="margin-left: auto; font-size: 10px; color: #6366f1;">
                    {move || format!("({})", store.get_edges().len())}
                </span>
            </div>

            <div style="max-height: 150px; overflow-y: auto;">
                {move || {
                    let edges = store.get_edges();
                    edges.into_iter().map(|edge| {
                        let is_selected = edge.selected;

                        view! {
                            <div style="background: #1e1b4b; padding: 6px 8px; border-radius: 4px; \
                                        margin-bottom: 4px; font-family: monospace; font-size: 10px; \
                                        display: flex; align-items: center; gap: 8px;">
                                <span style="color: #6366f1; width: 40px;">{edge.id.clone()}</span>
                                <span style="color: #10b981;">{edge.source.clone()}</span>
                                <span style="color: #6b7280;">"→"</span>
                                <span style="color: #ef4444;">{edge.target.clone()}</span>
                                {is_selected.then(|| view! {
                                    <span style="background: #f59e0b; color: #1a1a1a; padding: 1px 4px; \
                                                 border-radius: 3px; font-size: 8px; font-weight: 600; \
                                                 margin-left: auto;">
                                        "SEL"
                                    </span>
                                })}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Viewport Inspector Component
// ============================================================================

#[component]
fn ViewportInspector(store: FlowStore) -> impl IntoView {
    view! {
        <div style="padding: 12px; border-bottom: 1px solid #312e81;">
            <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #f59e0b; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"VIEW"</span>
                "Viewport State"
            </div>

            {move || {
                let vp = store.get_viewport();
                view! {
                    <div style="background: #1e1b4b; padding: 10px; border-radius: 6px; \
                                font-family: monospace;">
                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 8px;">
                            <div style="text-align: center;">
                                <div style="font-size: 16px; font-weight: 700; color: #10b981;">
                                    {format!("{:.1}", vp.x)}
                                </div>
                                <div style="font-size: 9px; color: #6b7280; text-transform: uppercase;">
                                    "X"
                                </div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 16px; font-weight: 700; color: #8b5cf6;">
                                    {format!("{:.1}", vp.y)}
                                </div>
                                <div style="font-size: 9px; color: #6b7280; text-transform: uppercase;">
                                    "Y"
                                </div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 16px; font-weight: 700; color: #f59e0b;">
                                    {format!("{:.2}x", vp.zoom)}
                                </div>
                                <div style="font-size: 9px; color: #6b7280; text-transform: uppercase;">
                                    "Zoom"
                                </div>
                            </div>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}

// ============================================================================
// Connection State Inspector Component
// ============================================================================

#[component]
fn ConnectionStateInspector(store: FlowStore) -> impl IntoView {
    view! {
        <div style="padding: 12px; border-bottom: 1px solid #312e81;">
            <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #ec4899; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"CONN"</span>
                "Connection State"
            </div>

            {move || {
                let conn = store.state.connection_in_progress.get();
                if let Some(conn) = conn {
                    view! {
                        <div style="background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%); \
                                    padding: 10px; border-radius: 6px; border: 1px solid #ec4899;">
                            <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 8px;">
                                <span style="width: 8px; height: 8px; border-radius: 50%; background: #22c55e; \
                                             animation: pulse 1s infinite;"></span>
                                <span style="font-size: 10px; font-weight: 600; color: #22c55e;">
                                    "Connection Active"
                                </span>
                            </div>
                            <div style="font-family: monospace; font-size: 10px; color: #c4b5fd; \
                                        display: grid; grid-template-columns: auto 1fr; gap: 4px 8px;">
                                <span style="color: #6b7280;">"From Node:"</span>
                                <span style="color: #10b981;">{conn.from_node.clone()}</span>
                                <span style="color: #6b7280;">"Handle:"</span>
                                <span style="color: #8b5cf6;">
                                    {conn.from_handle.clone().unwrap_or("default".to_string())}
                                </span>
                                <span style="color: #6b7280;">"Type:"</span>
                                <span style="color: #f59e0b;">
                                    {format!("{:?}", conn.from_handle_type)}
                                </span>
                                <span style="color: #6b7280;">"Position:"</span>
                                <span style="color: #a5b4fc;">
                                    {format!("({:.1}, {:.1})", conn.to_position.x, conn.to_position.y)}
                                </span>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="background: #1e1b4b; padding: 10px; border-radius: 6px; \
                                    text-align: center;">
                            <span style="color: #6b7280; font-size: 10px; font-style: italic;">
                                "No active connection"
                            </span>
                            <div style="color: #4b5563; font-size: 9px; margin-top: 4px;">
                                "Drag from a handle to start connecting"
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

// ============================================================================
// Event Log Panel Component
// ============================================================================

#[component]
fn EventLogPanel() -> impl IntoView {
    let event_log = get_event_log();

    view! {
        <div style="padding: 12px; flex: 1; display: flex; flex-direction: column; min-height: 150px;">
            <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #6366f1; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"LOG"</span>
                "Event Log"
                <span style="margin-left: auto; font-size: 10px; color: #6366f1;">
                    {move || format!("({})", event_log.get().len())}
                </span>
                <button
                    style="font-size: 9px; padding: 2px 6px; border: 1px solid #4c1d95; \
                           border-radius: 3px; background: #1e1b4b; cursor: pointer; color: #a5b4fc;"
                    on:click=move |_| event_log.set(Vec::new())
                >
                    "Clear"
                </button>
            </div>

            <div style="flex: 1; background: #0a0a12; border-radius: 6px; padding: 8px; \
                        overflow-y: auto; font-family: monospace; font-size: 9px;">
                {move || {
                    let entries = event_log.get();
                    if entries.is_empty() {
                        view! {
                            <div style="color: #4b5563; text-align: center; padding: 12px;">
                                "No events logged yet..."
                            </div>
                        }.into_any()
                    } else {
                        entries.into_iter().rev().enumerate().map(|(i, entry)| {
                            let color = match entry.event_type.as_str() {
                                "init" => "#6b7280",
                                "viewport" => "#f59e0b",
                                "selection" => "#3b82f6",
                                "drag_start" => "#22c55e",
                                "drag_end" => "#10b981",
                                "node_click" => "#8b5cf6",
                                "panel" => "#ec4899",
                                _ => "#a5b4fc",
                            };
                            let type_badge_color = match entry.event_type.as_str() {
                                "viewport" => "#78350f",
                                "selection" => "#1e3a8a",
                                "drag_start" | "drag_end" => "#14532d",
                                "node_click" => "#4c1d95",
                                "panel" => "#831843",
                                _ => "#1e1b4b",
                            };
                            let opacity = if i < 10 { 1.0 } else { 0.7 - (i as f64 - 10.0) * 0.02 };

                            view! {
                                <div style=format!(
                                    "padding: 4px 6px; border-bottom: 1px solid #1e1b4b; \
                                     opacity: {}; display: flex; align-items: center; gap: 6px;",
                                    opacity
                                )>
                                    <span style="color: #4b5563; font-size: 8px; width: 24px;">
                                        {entry.timestamp.clone()}
                                    </span>
                                    <span style=format!(
                                        "background: {}; color: {}; padding: 1px 4px; \
                                         border-radius: 3px; font-size: 8px; font-weight: 600; \
                                         min-width: 50px; text-align: center;",
                                        type_badge_color, color
                                    )>
                                        {entry.event_type.clone()}
                                    </span>
                                    <span style=format!("color: {}; flex: 1;", color)>
                                        {entry.details.clone()}
                                    </span>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Node Component
// ============================================================================

#[component]
fn DevToolsNode(
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
                log_event("drag_start", &format!("Node: {}", node_id_for_drag));
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
                        "2px solid #f59e0b".to_string()
                    } else {
                        format!("2px solid {}80", color)
                    };

                    let box_shadow = if n.selected {
                        format!("0 0 0 2px #f59e0b40, 0 4px 12px rgba(0,0,0,0.4)")
                    } else if n.dragging {
                        format!("0 0 0 2px {}40, 0 8px 20px rgba(0,0,0,0.5)", color)
                    } else {
                        "0 2px 8px rgba(0,0,0,0.3)".to_string()
                    };

                    let background = match node_type {
                        "input" => format!("linear-gradient(135deg, {}40 0%, {}60 100%)", color, color),
                        "output" => format!("linear-gradient(135deg, {}40 0%, {}60 100%)", color, color),
                        _ => format!("linear-gradient(135deg, #1e1b4b 0%, #312e81 100%)")
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
                        .unwrap_or("#c4b5fd")
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
                                    style="background: #4c1d95; width: 10px; height: 10px; border: 2px solid #a78bfa; \
                                           box-shadow: 0 0 6px rgba(167, 139, 250, 0.5);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #4c1d95; width: 10px; height: 10px; border: 2px solid #a78bfa; \
                                           box-shadow: 0 0 6px rgba(167, 139, 250, 0.5);".to_string()
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
fn DevToolsEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="dev-tools-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#8b5cf6" />
                    <stop offset="100%" stop-color="#a78bfa" />
                </linearGradient>
                <marker
                    id="dev-tools-edge-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#a78bfa" />
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

                    let is_selected = edge.selected;
                    let stroke = if is_selected {
                        "#f59e0b".to_string()
                    } else {
                        "url(#dev-tools-edge-gradient)".to_string()
                    };
                    let stroke_width = if is_selected { "3" } else { "2" };

                    Some(view! {
                        <g class="xyflow__edge">
                            <path
                                d=path.clone()
                                stroke=stroke
                                stroke-width=stroke_width
                                fill="none"
                                marker-end="url(#dev-tools-edge-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
