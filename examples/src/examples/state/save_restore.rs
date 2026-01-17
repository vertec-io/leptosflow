//! Save/Restore Example
//!
//! Demonstrates how to serialize and deserialize flow state.
//! Shows save button that serializes nodes, edges, viewport to JSON,
//! restore button that loads from JSON, and persists to localStorage.

use leptos::prelude::*;
use leptos::serde_json::json;
use serde::{Deserialize, Serialize};
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for save/restore example
static SAVE_RESTORE_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_save_restore_drag_signal() -> RwSignal<Option<DragState>> {
    *SAVE_RESTORE_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// The localStorage key for persisting flow state
const STORAGE_KEY: &str = "xyflow-save-restore-state";

/// Serializable flow state
#[derive(Clone, Debug, Serialize, Deserialize)]
struct FlowState {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    viewport: Viewport,
}

/// Save/Restore example component
#[component]
pub fn SaveRestoreExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({
                "label": "Node A",
                "nodeType": "input"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("2".to_string(), Position::new(100.0, 150.0))
            .with_data(json!({
                "label": "Node B",
                "nodeType": "default"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("3".to_string(), Position::new(250.0, 150.0))
            .with_data(json!({
                "label": "Node C",
                "nodeType": "default"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("4".to_string(), Position::new(175.0, 270.0))
            .with_data(json!({
                "label": "Node D",
                "nodeType": "output"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("A → B".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("A → C".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("B → D".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("C → D".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes.clone(), initial_edges.clone());

    // Provide the store to child components via context
    provide_context(store);

    // State for JSON textarea display
    let json_text = RwSignal::new(String::new());
    let save_count = RwSignal::new(0);
    let restore_count = RwSignal::new(0);
    let action_log = RwSignal::new(Vec::<String>::new());

    // Helper to add log entry
    let add_log = move |message: &str| {
        let timestamp = js_sys::Date::now();
        let time_str = format!("{:.1}s", (timestamp % 100000.0) / 1000.0);
        action_log.update(|log| {
            log.insert(0, format!("[{}] {}", time_str, message));
            if log.len() > 10 {
                log.pop();
            }
        });
    };

    // Try to load saved state from localStorage on mount
    Effect::new(move |_| {
        if let Some(storage) = leptos::prelude::window()
            .local_storage()
            .ok()
            .flatten()
        {
            if let Ok(Some(saved)) = storage.get_item(STORAGE_KEY) {
                if let Ok(state) = serde_json::from_str::<FlowState>(&saved) {
                    // Restore the state
                    store.set_nodes(state.nodes);
                    store.set_edges(state.edges);
                    store.set_viewport(state.viewport);
                    json_text.set(saved);
                    add_log("Loaded state from localStorage");
                }
            }
        }
    });

    // Save state to JSON and localStorage
    let save_state = move |_| {
        let state = FlowState {
            nodes: store.get_nodes(),
            edges: store.get_edges(),
            viewport: store.get_viewport(),
        };

        if let Ok(json) = serde_json::to_string_pretty(&state) {
            json_text.set(json.clone());
            save_count.update(|c| *c += 1);
            add_log("Saved state to JSON");

            // Save to localStorage
            if let Some(storage) = leptos::prelude::window()
                .local_storage()
                .ok()
                .flatten()
            {
                let _ = storage.set_item(STORAGE_KEY, &json);
                add_log("Persisted to localStorage");
            }
        }
    };

    // Restore state from JSON textarea
    let restore_state = move |_| {
        let json = json_text.get();
        if json.is_empty() {
            add_log("No JSON to restore");
            return;
        }

        match serde_json::from_str::<FlowState>(&json) {
            Ok(state) => {
                store.set_nodes(state.nodes);
                store.set_edges(state.edges);
                store.set_viewport(state.viewport);
                restore_count.update(|c| *c += 1);
                add_log("Restored state from JSON");
            }
            Err(e) => {
                add_log(&format!("Parse error: {}", e));
            }
        }
    };

    // Reset to default state
    let reset_state = move |_| {
        store.set_nodes(initial_nodes.clone());
        store.set_edges(initial_edges.clone());
        store.set_viewport(Viewport::default());
        json_text.set(String::new());
        add_log("Reset to default state");

        // Clear localStorage
        if let Some(storage) = leptos::prelude::window()
            .local_storage()
            .ok()
            .flatten()
        {
            let _ = storage.remove_item(STORAGE_KEY);
        }
    };

    // Clear localStorage only
    let clear_storage = move |_| {
        if let Some(storage) = leptos::prelude::window()
            .local_storage()
            .ok()
            .flatten()
        {
            let _ = storage.remove_item(STORAGE_KEY);
            add_log("Cleared localStorage");
        }
    };

    // Global drag handlers
    let drag_signal = get_save_restore_drag_signal();

    let on_global_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let current_x = ev.client_x() as f64;
            let current_y = ev.client_y() as f64;
            let (start_x, start_y) = drag_state.start_mouse;
            let (node_start_x, node_start_y) = drag_state.start_pos;

            // Calculate delta accounting for zoom
            let viewport = store.get_viewport();
            let dx = (current_x - start_x) / viewport.zoom;
            let dy = (current_y - start_y) / viewport.zoom;

            // Update node position
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

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Edge renderer
                    <SaveRestoreEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter()
                            .map(|node| {
                                view! {
                                    <SaveRestoreNode
                                        node=node.clone()
                                        store=store
                                    />
                                }
                            }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <div style="position: absolute; top: 16px; right: 16px; width: 360px; \
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); \
                            border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.2); \
                            padding: 16px; color: white; font-family: system-ui, -apple-system, sans-serif;">
                    <div style="font-size: 18px; font-weight: 600; margin-bottom: 12px; \
                                display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 20px;">"💾"</span>
                        "Save / Restore"
                    </div>

                    // Action buttons
                    <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                        <button
                            on:click=save_state
                            style="flex: 1; padding: 10px; background: rgba(255,255,255,0.2); \
                                   border: 1px solid rgba(255,255,255,0.3); border-radius: 8px; \
                                   color: white; font-weight: 600; cursor: pointer; \
                                   transition: all 0.2s ease;"
                        >
                            "💾 Save"
                        </button>
                        <button
                            on:click=restore_state
                            style="flex: 1; padding: 10px; background: rgba(255,255,255,0.2); \
                                   border: 1px solid rgba(255,255,255,0.3); border-radius: 8px; \
                                   color: white; font-weight: 600; cursor: pointer; \
                                   transition: all 0.2s ease;"
                        >
                            "📤 Restore"
                        </button>
                    </div>

                    // Stats
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px;">
                        <div style="background: rgba(0,0,0,0.2); padding: 8px 12px; border-radius: 6px; \
                                    display: flex; justify-content: space-between; align-items: center;">
                            <span style="font-size: 12px; opacity: 0.9;">"Saves"</span>
                            <span style="font-weight: 600;">{move || save_count.get()}</span>
                        </div>
                        <div style="background: rgba(0,0,0,0.2); padding: 8px 12px; border-radius: 6px; \
                                    display: flex; justify-content: space-between; align-items: center;">
                            <span style="font-size: 12px; opacity: 0.9;">"Restores"</span>
                            <span style="font-weight: 600;">{move || restore_count.get()}</span>
                        </div>
                    </div>

                    // Current state stats
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 12px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"Current State"</div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 8px; font-size: 11px;">
                            <div style="text-align: center;">
                                <div style="font-size: 18px; font-weight: 700;">{move || store.get_nodes().len()}</div>
                                <div style="opacity: 0.8;">"Nodes"</div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 18px; font-weight: 700;">{move || store.get_edges().len()}</div>
                                <div style="opacity: 0.8;">"Edges"</div>
                            </div>
                            <div style="text-align: center;">
                                <div style="font-size: 18px; font-weight: 700;">{move || format!("{:.1}x", store.get_viewport().zoom)}</div>
                                <div style="opacity: 0.8;">"Zoom"</div>
                            </div>
                        </div>
                    </div>

                    // JSON textarea
                    <div style="margin-bottom: 12px;">
                        <div style="font-size: 12px; font-weight: 600; margin-bottom: 6px; opacity: 0.9; \
                                    display: flex; align-items: center; gap: 6px;">
                            <span>"📋"</span>
                            "JSON State"
                        </div>
                        <textarea
                            style="width: 100%; height: 120px; padding: 8px; border-radius: 6px; \
                                   border: 1px solid rgba(255,255,255,0.2); background: rgba(0,0,0,0.3); \
                                   color: white; font-family: monospace; font-size: 10px; \
                                   resize: vertical; box-sizing: border-box;"
                            prop:value=move || json_text.get()
                            on:input=move |ev| {
                                json_text.set(event_target_value(&ev));
                            }
                            placeholder="Save to see JSON state, or paste JSON to restore..."
                        ></textarea>
                    </div>

                    // Additional actions
                    <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                        <button
                            on:click=reset_state
                            style="flex: 1; padding: 8px; background: rgba(255,100,100,0.3); \
                                   border: 1px solid rgba(255,100,100,0.5); border-radius: 6px; \
                                   color: white; font-size: 12px; cursor: pointer;"
                        >
                            "🔄 Reset"
                        </button>
                        <button
                            on:click=clear_storage
                            style="flex: 1; padding: 8px; background: rgba(255,200,100,0.3); \
                                   border: 1px solid rgba(255,200,100,0.5); border-radius: 6px; \
                                   color: white; font-size: 12px; cursor: pointer;"
                        >
                            "🗑️ Clear Storage"
                        </button>
                    </div>

                    // Action log
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px;">
                        <div style="font-size: 12px; font-weight: 600; margin-bottom: 6px; opacity: 0.9;">"Activity Log"</div>
                        <div style="max-height: 100px; overflow-y: auto; font-size: 10px; font-family: monospace;">
                            {move || {
                                let log = action_log.get();
                                if log.is_empty() {
                                    view! {
                                        <div style="color: rgba(255,255,255,0.5); font-style: italic;">
                                            "No activity yet..."
                                        </div>
                                    }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        view! {
                                            <div style="padding: 2px 0; border-bottom: 1px solid rgba(255,255,255,0.1);">
                                                {entry.clone()}
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>

                // Instructions badge
                <div style="position: absolute; bottom: 60px; left: 16px; \
                            background: rgba(102, 126, 234, 0.9); color: white; \
                            padding: 8px 12px; border-radius: 8px; font-size: 11px; \
                            max-width: 200px; line-height: 1.4;">
                    <div style="font-weight: 600; margin-bottom: 4px;">"💡 How it works"</div>
                    <div>"• Move nodes around"</div>
                    <div>"• Click Save to serialize"</div>
                    <div>"• Edit JSON manually"</div>
                    <div>"• Click Restore to apply"</div>
                    <div>"• State persists in localStorage"</div>
                </div>
            </div>
        </div>
    }
}

/// Custom node component for SaveRestore example
#[component]
fn SaveRestoreNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let drag_signal = get_save_restore_drag_signal();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    // Get colors based on node type
    let (bg_color, border_color) = match node_type.as_str() {
        "input" => ("#4ade80", "#22c55e"),    // Green
        "output" => ("#f87171", "#ef4444"),   // Red
        _ => ("#818cf8", "#6366f1"),          // Purple (default)
    };

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            // Mark node as dragging
            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
        }
    };

    // Get reactive node position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab; \
                 background: {}; border: 2px solid {}; border-radius: 8px; \
                 padding: 12px 16px; min-width: 100px; text-align: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.15); transition: box-shadow 0.2s ease;",
                pos().x, pos().y, bg_color, border_color
            )
            on:mousedown=on_mousedown
        >
            // Node label
            <div style="color: white; font-weight: 600; font-size: 13px; text-shadow: 0 1px 2px rgba(0,0,0,0.2);">
                {label}
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

/// Edge renderer component for SaveRestore example
#[component]
fn SaveRestoreEdgeRenderer(
    store: FlowStore,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                // Gradient for edges
                <linearGradient id="save-restore-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #667eea; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #764ba2; stop-opacity: 1" />
                </linearGradient>

                // Arrow marker
                <marker
                    id="save-restore-arrow"
                    viewBox="0 0 10 10"
                    refX="10"
                    refY="5"
                    markerUnits="strokeWidth"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#764ba2" />
                </marker>
            </defs>

            {move || {
                let nodes = store.get_nodes();
                let edges = store.get_edges();

                edges.into_iter().filter_map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate edge positions
                    let source_x = source_node.position.x + source_node.width.unwrap_or(120.0) / 2.0;
                    let source_y = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let target_x = target_node.position.x + target_node.width.unwrap_or(120.0) / 2.0;
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
                                stroke="rgba(102, 126, 234, 0.3)"
                                stroke-width="6"
                                stroke-linecap="round"
                            />

                            // Main edge
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="url(#save-restore-edge-gradient)"
                                stroke-width="2"
                                stroke-linecap="round"
                                marker-end="url(#save-restore-arrow)"
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
                                        stroke="#667eea"
                                        stroke-width="1"
                                    />
                                    <text
                                        x="0"
                                        y="4"
                                        text-anchor="middle"
                                        font-size="10"
                                        font-weight="500"
                                        fill="#667eea"
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
