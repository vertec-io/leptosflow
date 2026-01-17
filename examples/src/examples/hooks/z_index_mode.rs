//! Z-Index Mode Example
//!
//! Demonstrates how z-index stacking is managed for nodes.
//! Shows node stacking order controls, bring-to-front on selection or click,
//! and manual z-index control.

use leptos::prelude::*;
use serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Drag state for z-index example
static ZINDEX_DRAG_STATE: OnceLock<RwSignal<Option<DragState>>> = OnceLock::new();

fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *ZINDEX_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Action log entries
#[derive(Clone, Debug)]
pub struct ZIndexLogEntry {
    pub timestamp: String,
    pub action: String,
    pub details: String,
}

static ZINDEX_ACTION_LOG: OnceLock<RwSignal<Vec<ZIndexLogEntry>>> = OnceLock::new();

fn get_action_log() -> RwSignal<Vec<ZIndexLogEntry>> {
    *ZINDEX_ACTION_LOG.get_or_init(|| RwSignal::new(Vec::new()))
}

fn log_action(action: &str, details: &str) {
    let timestamp = get_timestamp();
    get_action_log().update(|entries| {
        entries.push(ZIndexLogEntry {
            timestamp,
            action: action.to_string(),
            details: details.to_string(),
        });
        // Keep last 20 entries
        if entries.len() > 20 {
            entries.remove(0);
        }
    });
}

fn get_timestamp() -> String {
    static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{:04}", count)
}

/// Z-index bring-to-front mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BringToFrontMode {
    /// Bring to front on click/selection
    OnSelection,
    /// Manual control only
    Manual,
    /// Bring to front when dragging
    OnDrag,
}

static ZINDEX_MODE: OnceLock<RwSignal<BringToFrontMode>> = OnceLock::new();

fn get_zindex_mode() -> RwSignal<BringToFrontMode> {
    *ZINDEX_MODE.get_or_init(|| RwSignal::new(BringToFrontMode::OnSelection))
}

// ============================================================================
// ZIndex Mode Example Component
// ============================================================================

/// ZIndexMode example - Demonstrates node stacking order controls
#[component]
pub fn ZIndexModeExample() -> impl IntoView {
    // Create overlapping nodes to demonstrate z-index
    let initial_nodes = vec![
        Node::new("node-1".to_string(), Position::new(80.0, 60.0))
            .with_data(json!({
                "label": "Node A",
                "color": "#ef4444",
                "zIndex": 1
            }))
            .with_dimensions(140.0, 80.0),
        Node::new("node-2".to_string(), Position::new(150.0, 100.0))
            .with_data(json!({
                "label": "Node B",
                "color": "#f59e0b",
                "zIndex": 2
            }))
            .with_dimensions(140.0, 80.0),
        Node::new("node-3".to_string(), Position::new(220.0, 140.0))
            .with_data(json!({
                "label": "Node C",
                "color": "#10b981",
                "zIndex": 3
            }))
            .with_dimensions(140.0, 80.0),
        Node::new("node-4".to_string(), Position::new(290.0, 80.0))
            .with_data(json!({
                "label": "Node D",
                "color": "#3b82f6",
                "zIndex": 4
            }))
            .with_dimensions(140.0, 80.0),
        Node::new("node-5".to_string(), Position::new(360.0, 120.0))
            .with_data(json!({
                "label": "Node E",
                "color": "#8b5cf6",
                "zIndex": 5
            }))
            .with_dimensions(140.0, 80.0),
    ];

    let initial_edges = vec![
        Edge::new("e1".to_string(), "node-1".to_string(), "node-2".to_string()),
        Edge::new("e2".to_string(), "node-2".to_string(), "node-3".to_string()),
        Edge::new("e3".to_string(), "node-3".to_string(), "node-4".to_string()),
        Edge::new("e4".to_string(), "node-4".to_string(), "node-5".to_string()),
    ];

    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store.clone());

    let drag_signal = get_drag_signal();
    let action_log = get_action_log();
    let zindex_mode = get_zindex_mode();

    // Reset state on mount
    action_log.set(vec![ZIndexLogEntry {
        timestamp: "0000".to_string(),
        action: "init".to_string(),
        details: "Z-Index Mode example loaded".to_string(),
    }]);
    zindex_mode.set(BringToFrontMode::OnSelection);

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
            }
        }
    };

    // Helper to get max z-index
    let get_max_zindex = {
        let store = store.clone();
        move || {
            store.get_nodes().iter()
                .filter_map(|n| n.data.get("zIndex").and_then(|v| v.as_i64()))
                .max()
                .unwrap_or(0) as i32
        }
    };

    // Bring node to front
    let bring_to_front = {
        let store = store.clone();
        let get_max_zindex = get_max_zindex.clone();
        move |node_id: &str| {
            let max_z = get_max_zindex();
            let new_z = max_z + 1;
            store.update_node(node_id, |n| {
                if let Some(z) = n.data.get_mut("zIndex") {
                    *z = json!(new_z);
                }
            });
            log_action("bring_to_front", &format!("{} -> z:{}", node_id, new_z));
        }
    };

    // Send node to back
    let send_to_back = {
        let store = store.clone();
        move |node_id: &str| {
            let min_z = store.get_nodes().iter()
                .filter_map(|n| n.data.get("zIndex").and_then(|v| v.as_i64()))
                .min()
                .unwrap_or(1) as i32;
            let new_z = (min_z - 1).max(0);
            store.update_node(node_id, |n| {
                if let Some(z) = n.data.get_mut("zIndex") {
                    *z = json!(new_z);
                }
            });
            log_action("send_to_back", &format!("{} -> z:{}", node_id, new_z));
        }
    };

    // Bring selected node to front (for button)
    let bring_selected_to_front = {
        let store = store.clone();
        let bring_to_front = bring_to_front.clone();
        move |_| {
            let selected_ids = store.get_selected_nodes();
            if let Some(node_id) = selected_ids.iter().next() {
                bring_to_front(node_id);
            }
        }
    };

    // Send selected node to back (for button)
    let send_selected_to_back = {
        let store = store.clone();
        let send_to_back = send_to_back.clone();
        move |_| {
            let selected_ids = store.get_selected_nodes();
            if let Some(node_id) = selected_ids.iter().next() {
                send_to_back(node_id);
            }
        }
    };

    // Increment selected z-index
    let increment_selected_z = {
        let store = store.clone();
        move |_| {
            let selected_ids = store.get_selected_nodes();
            if let Some(node_id) = selected_ids.iter().next() {
                let nodes = store.get_nodes();
                if let Some(node) = nodes.iter().find(|n| &n.id == node_id) {
                    let current_z = node.data.get("zIndex")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(1) as i32;
                    let new_z = current_z + 1;
                    store.update_node(node_id, |n| {
                        if let Some(z) = n.data.get_mut("zIndex") {
                            *z = json!(new_z);
                        }
                    });
                    log_action("z_increment", &format!("{} -> z:{}", node_id, new_z));
                }
            }
        }
    };

    // Decrement selected z-index
    let decrement_selected_z = {
        let store = store.clone();
        move |_| {
            let selected_ids = store.get_selected_nodes();
            if let Some(node_id) = selected_ids.iter().next() {
                let nodes = store.get_nodes();
                if let Some(node) = nodes.iter().find(|n| &n.id == node_id) {
                    let current_z = node.data.get("zIndex")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(1) as i32;
                    let new_z = (current_z - 1).max(0);
                    store.update_node(node_id, |n| {
                        if let Some(z) = n.data.get_mut("zIndex") {
                            *z = json!(new_z);
                        }
                    });
                    log_action("z_decrement", &format!("{} -> z:{}", node_id, new_z));
                }
            }
        }
    };

    // Reset z-indices to defaults
    let reset_zindices = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            for (i, node) in nodes.iter().enumerate() {
                let node_id = node.id.clone();
                let new_z = (i + 1) as i32;
                store.update_node(&node_id, |n| {
                    if let Some(z) = n.data.get_mut("zIndex") {
                        *z = json!(new_z);
                    }
                });
            }
            log_action("reset", "All z-indices reset to defaults");
        }
    };

    // Mode change handlers
    let set_mode_on_selection = move |_| {
        zindex_mode.set(BringToFrontMode::OnSelection);
        log_action("mode_change", "Bring to front on selection");
    };

    let set_mode_manual = move |_| {
        zindex_mode.set(BringToFrontMode::Manual);
        log_action("mode_change", "Manual z-index control only");
    };

    let set_mode_on_drag = move |_| {
        zindex_mode.set(BringToFrontMode::OnDrag);
        log_action("mode_change", "Bring to front on drag");
    };

    view! {
        <div class="example-container" style="display: flex; flex-direction: column; height: 100%;">
            // Header
            <div style="padding: 12px; background: linear-gradient(135deg, #1e1b4b 0%, #312e81 100%); \
                        border-bottom: 1px solid #4c1d95; display: flex; align-items: center; gap: 12px;">
                <div style="background: #8b5cf6; color: white; padding: 6px 12px; border-radius: 6px; \
                            font-size: 11px; font-weight: 600; display: flex; align-items: center; gap: 6px;">
                    <span style="font-size: 14px;">"📚"</span>
                    "Z-Index Mode"
                </div>
                <div style="font-size: 12px; color: #c4b5fd;">
                    "Node stacking order controls - click nodes to see z-index behavior"
                </div>
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
                        <ZIndexEdgeRenderer store=store.clone() />
                        <ConnectionLine />

                        {move || {
                            // Sort nodes by z-index for correct rendering order
                            let mut nodes = store.get_nodes();
                            nodes.sort_by(|a, b| {
                                let z_a = a.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0);
                                let z_b = b.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0);
                                z_a.cmp(&z_b)
                            });

                            nodes.into_iter().map(|node| {
                                view! {
                                    <ZIndexNode
                                        node=node.clone()
                                        store=store.clone()
                                        drag_signal=drag_signal
                                        bring_to_front=bring_to_front.clone()
                                    />
                                }
                            }).collect_view()
                        }}
                    </FlowViewport>

                    <Controls position=PanelPosition::BottomLeft />

                    // Z-Index Stack visualization
                    <div style="position: absolute; bottom: 12px; right: 12px; background: rgba(30, 27, 75, 0.95); \
                                padding: 12px; border-radius: 8px; border: 1px solid #4c1d95; min-width: 180px;">
                        <div style="font-size: 10px; font-weight: 600; color: #c4b5fd; margin-bottom: 8px; \
                                    display: flex; align-items: center; gap: 6px;">
                            <span style="font-size: 12px;">"📊"</span>
                            "Z-Index Stack"
                        </div>
                        {move || {
                            let mut nodes = store.get_nodes();
                            nodes.sort_by(|a, b| {
                                let z_a = a.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0);
                                let z_b = b.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0);
                                z_b.cmp(&z_a) // Reverse order (highest first)
                            });

                            nodes.into_iter().map(|node| {
                                let color = node.data.get("color")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("#6366f1")
                                    .to_string();
                                let label = node.data.get("label")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Node")
                                    .to_string();
                                let z = node.data.get("zIndex")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0);
                                let is_selected = node.selected;

                                view! {
                                    <div style=format!(
                                        "display: flex; align-items: center; gap: 8px; padding: 4px 8px; \
                                         margin-bottom: 2px; border-radius: 4px; font-size: 10px; \
                                         background: {}; border: 1px solid {};",
                                        if is_selected { "rgba(245, 158, 11, 0.2)" } else { "rgba(30, 27, 75, 0.5)" },
                                        if is_selected { "#f59e0b" } else { "transparent" }
                                    )>
                                        <span style=format!(
                                            "width: 12px; height: 12px; border-radius: 3px; background: {};",
                                            color
                                        )></span>
                                        <span style="color: #e9d5ff; flex: 1;">{label}</span>
                                        <span style="color: #a5b4fc; font-family: monospace; font-weight: 600;">
                                            "z:"{z}
                                        </span>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </div>

                // Control Panel
                <div style="width: 280px; background: linear-gradient(135deg, #1e1b4b 0%, #0f0f1a 100%); \
                            border-left: 1px solid #4c1d95; display: flex; flex-direction: column; \
                            overflow-y: auto;">
                    // Mode selection
                    <div style="padding: 12px; border-bottom: 1px solid #312e81;">
                        <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                                    display: flex; align-items: center; gap: 6px;">
                            <span style="background: #8b5cf6; color: white; padding: 2px 6px; border-radius: 4px; \
                                         font-size: 9px;">"MODE"</span>
                            "Bring to Front Mode"
                        </div>

                        <div style="display: flex; flex-direction: column; gap: 6px;">
                            <button
                                style=move || format!(
                                    "padding: 8px 12px; border-radius: 6px; font-size: 11px; cursor: pointer; \
                                     text-align: left; border: 1px solid {}; background: {}; color: {};",
                                    if zindex_mode.get() == BringToFrontMode::OnSelection { "#8b5cf6" } else { "#4c1d95" },
                                    if zindex_mode.get() == BringToFrontMode::OnSelection { "#8b5cf620" } else { "#1e1b4b" },
                                    if zindex_mode.get() == BringToFrontMode::OnSelection { "#a78bfa" } else { "#a5b4fc" }
                                )
                                on:click=set_mode_on_selection
                            >
                                <div style="font-weight: 600;">"On Selection"</div>
                                <div style="font-size: 9px; color: #6b7280; margin-top: 2px;">
                                    "Bring to front when node is clicked"
                                </div>
                            </button>

                            <button
                                style=move || format!(
                                    "padding: 8px 12px; border-radius: 6px; font-size: 11px; cursor: pointer; \
                                     text-align: left; border: 1px solid {}; background: {}; color: {};",
                                    if zindex_mode.get() == BringToFrontMode::OnDrag { "#8b5cf6" } else { "#4c1d95" },
                                    if zindex_mode.get() == BringToFrontMode::OnDrag { "#8b5cf620" } else { "#1e1b4b" },
                                    if zindex_mode.get() == BringToFrontMode::OnDrag { "#a78bfa" } else { "#a5b4fc" }
                                )
                                on:click=set_mode_on_drag
                            >
                                <div style="font-weight: 600;">"On Drag"</div>
                                <div style="font-size: 9px; color: #6b7280; margin-top: 2px;">
                                    "Bring to front when dragging starts"
                                </div>
                            </button>

                            <button
                                style=move || format!(
                                    "padding: 8px 12px; border-radius: 6px; font-size: 11px; cursor: pointer; \
                                     text-align: left; border: 1px solid {}; background: {}; color: {};",
                                    if zindex_mode.get() == BringToFrontMode::Manual { "#8b5cf6" } else { "#4c1d95" },
                                    if zindex_mode.get() == BringToFrontMode::Manual { "#8b5cf620" } else { "#1e1b4b" },
                                    if zindex_mode.get() == BringToFrontMode::Manual { "#a78bfa" } else { "#a5b4fc" }
                                )
                                on:click=set_mode_manual
                            >
                                <div style="font-weight: 600;">"Manual"</div>
                                <div style="font-size: 9px; color: #6b7280; margin-top: 2px;">
                                    "Use buttons to control z-index"
                                </div>
                            </button>
                        </div>
                    </div>

                    // Z-Index Controls
                    <div style="padding: 12px; border-bottom: 1px solid #312e81;">
                        <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                                    display: flex; align-items: center; gap: 6px;">
                            <span style="background: #10b981; color: white; padding: 2px 6px; border-radius: 4px; \
                                         font-size: 9px;">"CTRL"</span>
                            "Z-Index Controls"
                        </div>

                        // Selected node display
                        {move || {
                            let selected_ids = store.get_selected_nodes();
                            let nodes = store.get_nodes();
                            let selected_node = selected_ids.iter()
                                .next()
                                .and_then(|id| nodes.iter().find(|n| &n.id == id));

                            if let Some(node) = selected_node {
                                let color = node.data.get("color")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("#6366f1")
                                    .to_string();
                                let label = node.data.get("label")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Node")
                                    .to_string();
                                let z = node.data.get("zIndex")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0);

                                view! {
                                    <div style="background: #0f0f1a; padding: 10px; border-radius: 6px; \
                                                margin-bottom: 10px; display: flex; align-items: center; gap: 10px;">
                                        <span style=format!(
                                            "width: 20px; height: 20px; border-radius: 4px; background: {};",
                                            color
                                        )></span>
                                        <div style="flex: 1;">
                                            <div style="font-weight: 600; color: #e9d5ff; font-size: 12px;">
                                                {label}
                                            </div>
                                            <div style="font-size: 10px; color: #6b7280;">
                                                "z-index: "<span style="color: #a78bfa; font-weight: 600;">{z}</span>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="background: #0f0f1a; padding: 10px; border-radius: 6px; \
                                                margin-bottom: 10px; text-align: center; color: #6b7280; \
                                                font-size: 11px;">
                                        "Select a node to modify"
                                    </div>
                                }.into_any()
                            }
                        }}

                        // Z-index modification buttons
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 6px;">
                            <button
                                style="padding: 8px 12px; background: #10b981; color: white; border: none; \
                                       border-radius: 6px; font-size: 11px; cursor: pointer; font-weight: 600; \
                                       display: flex; align-items: center; justify-content: center; gap: 4px;"
                                on:click=bring_selected_to_front
                            >
                                <span>"⬆"</span>" Bring Front"
                            </button>
                            <button
                                style="padding: 8px 12px; background: #ef4444; color: white; border: none; \
                                       border-radius: 6px; font-size: 11px; cursor: pointer; font-weight: 600; \
                                       display: flex; align-items: center; justify-content: center; gap: 4px;"
                                on:click=send_selected_to_back
                            >
                                <span>"⬇"</span>" Send Back"
                            </button>
                            <button
                                style="padding: 8px 12px; background: #3b82f6; color: white; border: none; \
                                       border-radius: 6px; font-size: 11px; cursor: pointer; font-weight: 600; \
                                       display: flex; align-items: center; justify-content: center; gap: 4px;"
                                on:click=increment_selected_z
                            >
                                <span>"+"</span>" Z + 1"
                            </button>
                            <button
                                style="padding: 8px 12px; background: #f59e0b; color: white; border: none; \
                                       border-radius: 6px; font-size: 11px; cursor: pointer; font-weight: 600; \
                                       display: flex; align-items: center; justify-content: center; gap: 4px;"
                                on:click=decrement_selected_z
                            >
                                <span>"-"</span>" Z - 1"
                            </button>
                        </div>

                        <button
                            style="margin-top: 8px; width: 100%; padding: 8px 12px; background: #4c1d95; \
                                   color: #c4b5fd; border: 1px solid #6d28d9; border-radius: 6px; \
                                   font-size: 11px; cursor: pointer;"
                            on:click=reset_zindices
                        >
                            "Reset All Z-Indices"
                        </button>
                    </div>

                    // Action Log
                    <div style="padding: 12px; flex: 1; display: flex; flex-direction: column; min-height: 150px;">
                        <div style="font-size: 11px; font-weight: 600; color: #c4b5fd; margin-bottom: 10px; \
                                    display: flex; align-items: center; gap: 6px;">
                            <span style="background: #6366f1; color: white; padding: 2px 6px; border-radius: 4px; \
                                         font-size: 9px;">"LOG"</span>
                            "Action Log"
                            <span style="margin-left: auto; font-size: 10px; color: #6366f1;">
                                {move || format!("({})", action_log.get().len())}
                            </span>
                        </div>

                        <div style="flex: 1; background: #0a0a12; border-radius: 6px; padding: 8px; \
                                    overflow-y: auto; font-family: monospace; font-size: 9px;">
                            {move || {
                                let entries = action_log.get();
                                entries.into_iter().rev().map(|entry| {
                                    let color = match entry.action.as_str() {
                                        "init" => "#6b7280",
                                        "bring_to_front" => "#10b981",
                                        "send_to_back" => "#ef4444",
                                        "z_increment" => "#3b82f6",
                                        "z_decrement" => "#f59e0b",
                                        "select" => "#8b5cf6",
                                        "mode_change" => "#ec4899",
                                        "reset" => "#6d28d9",
                                        _ => "#a5b4fc",
                                    };

                                    view! {
                                        <div style="padding: 3px 0; border-bottom: 1px solid #1e1b4b;">
                                            <span style="color: #4b5563; margin-right: 6px;">{entry.timestamp.clone()}</span>
                                            <span style=format!("color: {};", color)>{entry.action.clone()}</span>
                                            <span style="color: #6b7280; margin-left: 6px;">{entry.details.clone()}</span>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// ZIndex Node Component
// ============================================================================

#[component]
fn ZIndexNode<F>(
    node: Node,
    store: FlowStore,
    drag_signal: RwSignal<Option<DragState>>,
    bring_to_front: F,
) -> impl IntoView
where
    F: Fn(&str) + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();
    let zindex_mode = get_zindex_mode();

    // Mouse down - start drag and optionally bring to front
    let on_mousedown = {
        let store = store.clone();
        let bring_to_front = bring_to_front.clone();
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
                log_action("select", &format!("Selected {}", node_id_for_drag));

                // Handle bring-to-front based on mode
                let mode = zindex_mode.get();
                match mode {
                    BringToFrontMode::OnSelection => {
                        bring_to_front(&node_id_for_drag);
                    }
                    BringToFrontMode::OnDrag => {
                        bring_to_front(&node_id_for_drag);
                    }
                    BringToFrontMode::Manual => {
                        // Do nothing - manual control only
                    }
                }
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
                    let z_index = n.data.get("zIndex")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    let border = if n.selected {
                        "3px solid #f59e0b".to_string()
                    } else {
                        format!("3px solid {}cc", color)
                    };

                    let box_shadow = if n.selected {
                        format!("0 0 0 3px #f59e0b40, 0 6px 16px rgba(0,0,0,0.5)")
                    } else if n.dragging {
                        format!("0 0 0 3px {}40, 0 10px 24px rgba(0,0,0,0.6)", color)
                    } else {
                        format!("0 4px 12px rgba(0,0,0,0.4)")
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); \
                         z-index: {}; \
                         width: {}px; height: {}px; \
                         background: linear-gradient(145deg, {}40 0%, {}80 100%); \
                         border: {}; border-radius: 10px; box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; \
                         align-items: center; padding: 10px; box-sizing: border-box; \
                         transition: box-shadow 0.15s, border 0.15s, transform 0.05s;",
                        n.position.x, n.position.y,
                        z_index,
                        n.width.unwrap_or(140.0), n.height.unwrap_or(80.0),
                        color, color,
                        border, box_shadow
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
                    let z = n.data.get("zIndex")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    view! {
                        <>
                            <div style="font-weight: 700; font-size: 14px; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.3);">
                                {label}
                            </div>
                            <div style="font-size: 10px; color: rgba(255,255,255,0.7); margin-top: 4px; \
                                        background: rgba(0,0,0,0.3); padding: 2px 8px; border-radius: 10px;">
                                "z-index: "<span style="font-weight: 600; color: white;">{z}</span>
                            </div>
                        </>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Handles
            {move || {
                let nodes = store.get_nodes();
                if let Some(_n) = nodes.iter().find(|n| n.id == node_id) {
                    view! {
                        <>
                            <Handle
                                node_id=node.id.clone()
                                r#type=HandleType::Target
                                position=HandlePosition::Left
                                connection_mode=ConnectionMode::Strict
                                style="background: #4c1d95; width: 10px; height: 10px; border: 2px solid #a78bfa; \
                                       box-shadow: 0 0 6px rgba(167, 139, 250, 0.5);".to_string()
                            />
                            <Handle
                                node_id=node.id.clone()
                                r#type=HandleType::Source
                                position=HandlePosition::Right
                                connection_mode=ConnectionMode::Strict
                                style="background: #4c1d95; width: 10px; height: 10px; border: 2px solid #a78bfa; \
                                       box-shadow: 0 0 6px rgba(167, 139, 250, 0.5);".to_string()
                            />
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
fn ZIndexEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="zindex-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#8b5cf6" />
                    <stop offset="100%" stop-color="#a78bfa" />
                </linearGradient>
                <marker
                    id="zindex-edge-arrow"
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

                    let sx = source_node.position.x + source_node.width.unwrap_or(140.0);
                    let sy = source_node.position.y + source_node.height.unwrap_or(80.0) / 2.0;
                    let tx = target_node.position.x;
                    let ty = target_node.position.y + target_node.height.unwrap_or(80.0) / 2.0;

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
                        "url(#zindex-edge-gradient)".to_string()
                    };
                    let stroke_width = if is_selected { "3" } else { "2" };

                    Some(view! {
                        <g class="xyflow__edge">
                            <path
                                d=path.clone()
                                stroke=stroke
                                stroke-width=stroke_width
                                fill="none"
                                marker-end="url(#zindex-edge-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
