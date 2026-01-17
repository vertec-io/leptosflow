//! Set Nodes Batching Example
//!
//! Demonstrates how to batch multiple node updates efficiently.
//! Shows batched updates vs individual updates and performance comparison visualization.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for set nodes batching example
static SET_NODES_BATCHING_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_set_nodes_batching_drag_signal() -> RwSignal<Option<DragState>> {
    *SET_NODES_BATCHING_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Timing result from an update operation
#[derive(Clone, Debug)]
struct TimingResult {
    operation: String,
    duration_ms: f64,
    node_count: usize,
    timestamp: f64,
}

/// Set Nodes Batching example component
#[component]
pub fn SetNodesBatchingExample() -> impl IntoView {
    // Create initial nodes in a grid pattern
    let initial_nodes = create_grid_nodes(4, 3); // 4 cols x 3 rows = 12 nodes

    // Create initial edges connecting adjacent nodes
    let initial_edges = create_grid_edges(4, 3);

    // Create the flow store
    let store = FlowStore::new(initial_nodes.clone(), initial_edges.clone());

    // Provide the store to child components via context
    provide_context(store);

    // Performance tracking state
    let timing_results = RwSignal::new(Vec::<TimingResult>::new());
    let individual_update_count = RwSignal::new(0);
    let batched_update_count = RwSignal::new(0);
    let is_updating = RwSignal::new(false);

    // Action log
    let action_log = RwSignal::new(Vec::<String>::new());

    // Helper to add log entry
    let add_log = move |message: &str| {
        let timestamp = js_sys::Date::now();
        let time_str = format!("{:.1}s", (timestamp % 100000.0) / 1000.0);
        action_log.update(|log| {
            log.insert(0, format!("[{}] {}", time_str, message));
            if log.len() > 15 {
                log.pop();
            }
        });
    };

    // Individual updates - updates each node one at a time
    let run_individual_updates = move |_| {
        if is_updating.get() {
            return;
        }
        is_updating.set(true);

        let nodes = store.get_nodes();
        let node_count = nodes.len();
        let start_time = js_sys::Date::now();

        // Update each node individually with a random offset
        for node in nodes.iter() {
            let node_id = node.id.clone();
            let offset_x = (js_sys::Math::random() - 0.5) * 20.0;
            let offset_y = (js_sys::Math::random() - 0.5) * 20.0;

            store.update_node(&node_id, |n| {
                n.position.x += offset_x;
                n.position.y += offset_y;
            });
        }

        let end_time = js_sys::Date::now();
        let duration = end_time - start_time;

        individual_update_count.update(|c| *c += 1);
        timing_results.update(|results| {
            results.insert(
                0,
                TimingResult {
                    operation: "Individual".to_string(),
                    duration_ms: duration,
                    node_count,
                    timestamp: end_time,
                },
            );
            if results.len() > 20 {
                results.pop();
            }
        });

        add_log(&format!(
            "Individual updates: {} nodes in {:.2}ms",
            node_count, duration
        ));
        is_updating.set(false);
    };

    // Batched updates - updates all nodes at once using set_nodes
    let run_batched_updates = move |_| {
        if is_updating.get() {
            return;
        }
        is_updating.set(true);

        let mut nodes = store.get_nodes();
        let node_count = nodes.len();
        let start_time = js_sys::Date::now();

        // Modify all nodes in memory first
        for node in nodes.iter_mut() {
            let offset_x = (js_sys::Math::random() - 0.5) * 20.0;
            let offset_y = (js_sys::Math::random() - 0.5) * 20.0;
            node.position.x += offset_x;
            node.position.y += offset_y;
        }

        // Then apply all changes at once using set_nodes (batched)
        store.set_nodes(nodes);

        let end_time = js_sys::Date::now();
        let duration = end_time - start_time;

        batched_update_count.update(|c| *c += 1);
        timing_results.update(|results| {
            results.insert(
                0,
                TimingResult {
                    operation: "Batched".to_string(),
                    duration_ms: duration,
                    node_count,
                    timestamp: end_time,
                },
            );
            if results.len() > 20 {
                results.pop();
            }
        });

        add_log(&format!(
            "Batched update: {} nodes in {:.2}ms",
            node_count, duration
        ));
        is_updating.set(false);
    };

    // Run multiple updates for comparison
    let run_comparison_test = {
        let add_log = add_log.clone();
        move |_| {
            if is_updating.get() {
                return;
            }

            add_log("Starting comparison test...");

            // Run 5 individual updates
            for i in 0..5 {
                let nodes = store.get_nodes();
                let node_count = nodes.len();
                let start_time = js_sys::Date::now();

                for node in nodes.iter() {
                    let node_id = node.id.clone();
                    let offset_x = (js_sys::Math::random() - 0.5) * 10.0;
                    let offset_y = (js_sys::Math::random() - 0.5) * 10.0;

                    store.update_node(&node_id, |n| {
                        n.position.x += offset_x;
                        n.position.y += offset_y;
                    });
                }

                let end_time = js_sys::Date::now();
                let duration = end_time - start_time;

                timing_results.update(|results| {
                    results.insert(
                        0,
                        TimingResult {
                            operation: format!("Individual #{}", i + 1),
                            duration_ms: duration,
                            node_count,
                            timestamp: end_time,
                        },
                    );
                });
                individual_update_count.update(|c| *c += 1);
            }

            // Run 5 batched updates
            for i in 0..5 {
                let mut nodes = store.get_nodes();
                let node_count = nodes.len();
                let start_time = js_sys::Date::now();

                for node in nodes.iter_mut() {
                    let offset_x = (js_sys::Math::random() - 0.5) * 10.0;
                    let offset_y = (js_sys::Math::random() - 0.5) * 10.0;
                    node.position.x += offset_x;
                    node.position.y += offset_y;
                }

                store.set_nodes(nodes);

                let end_time = js_sys::Date::now();
                let duration = end_time - start_time;

                timing_results.update(|results| {
                    results.insert(
                        0,
                        TimingResult {
                            operation: format!("Batched #{}", i + 1),
                            duration_ms: duration,
                            node_count,
                            timestamp: end_time,
                        },
                    );
                });
                batched_update_count.update(|c| *c += 1);
            }

            add_log("Comparison test complete!");
        }
    };

    // Reset positions to grid
    let reset_positions = {
        let add_log = add_log.clone();
        move |_| {
            let new_nodes = create_grid_nodes(4, 3);
            store.set_nodes(new_nodes);
            add_log("Reset node positions");
        }
    };

    // Add more nodes
    let add_nodes = {
        let add_log = add_log.clone();
        move |_| {
            let mut nodes = store.get_nodes();
            let count = nodes.len();

            // Add 6 more nodes
            for i in 0..6 {
                let row = (count + i) / 4;
                let col = (count + i) % 4;
                let new_node = Node::new(
                    format!("node-{}", count + i + 1),
                    Position::new(100.0 + col as f64 * 120.0, 80.0 + row as f64 * 100.0),
                )
                .with_data(json!({
                    "label": format!("Node {}", count + i + 1),
                    "nodeType": "default"
                }))
                .with_dimensions(100.0, 50.0);

                nodes.push(new_node);
            }

            store.set_nodes(nodes);
            add_log(&format!("Added 6 nodes (total: {})", count + 6));
        }
    };

    // Clear timing results
    let clear_results = move |_| {
        timing_results.set(Vec::new());
        individual_update_count.set(0);
        batched_update_count.set(0);
        action_log.set(Vec::new());
    };

    // Global drag handlers
    let drag_signal = get_set_nodes_batching_drag_signal();

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
                    <SetNodesBatchingEdgeRenderer store=store />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter()
                            .map(|node| {
                                view! {
                                    <BatchingNode
                                        node=node.clone()
                                        store=store
                                    />
                                }
                            }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // Info Panel
                <div style="position: absolute; top: 16px; right: 16px; width: 380px; \
                            background: linear-gradient(135deg, #0ea5e9 0%, #8b5cf6 100%); \
                            border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.2); \
                            padding: 16px; color: white; font-family: system-ui, -apple-system, sans-serif; \
                            max-height: calc(100vh - 120px); overflow-y: auto;">
                    <div style="font-size: 18px; font-weight: 600; margin-bottom: 12px; \
                                display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 20px;">"⚡"</span>
                        "Set Nodes Batching"
                    </div>

                    // Stats summary
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px;">
                        <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; text-align: center;">
                            <div style="font-size: 10px; opacity: 0.8; text-transform: uppercase; letter-spacing: 0.5px;">"Individual"</div>
                            <div style="font-size: 24px; font-weight: 700;">{move || individual_update_count.get()}</div>
                            <div style="font-size: 11px; opacity: 0.7;">"updates"</div>
                        </div>
                        <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; text-align: center;">
                            <div style="font-size: 10px; opacity: 0.8; text-transform: uppercase; letter-spacing: 0.5px;">"Batched"</div>
                            <div style="font-size: 24px; font-weight: 700;">{move || batched_update_count.get()}</div>
                            <div style="font-size: 11px; opacity: 0.7;">"updates"</div>
                        </div>
                    </div>

                    // Current node count
                    <div style="background: rgba(0,0,0,0.2); padding: 8px 12px; border-radius: 6px; \
                                margin-bottom: 12px; display: flex; justify-content: space-between; align-items: center;">
                        <span style="font-size: 12px; opacity: 0.9;">"Current Nodes"</span>
                        <span style="font-weight: 600; font-size: 16px;">{move || store.get_nodes().len()}</span>
                    </div>

                    // Action buttons
                    <div style="margin-bottom: 12px;">
                        <div style="font-size: 12px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"Update Methods"</div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                            <button
                                on:click=run_individual_updates
                                disabled=move || is_updating.get()
                                style="padding: 10px; background: rgba(239, 68, 68, 0.3); \
                                       border: 1px solid rgba(239, 68, 68, 0.5); border-radius: 8px; \
                                       color: white; font-weight: 600; cursor: pointer; \
                                       transition: all 0.2s ease; font-size: 12px;"
                            >
                                "🔄 Individual"
                            </button>
                            <button
                                on:click=run_batched_updates
                                disabled=move || is_updating.get()
                                style="padding: 10px; background: rgba(34, 197, 94, 0.3); \
                                       border: 1px solid rgba(34, 197, 94, 0.5); border-radius: 8px; \
                                       color: white; font-weight: 600; cursor: pointer; \
                                       transition: all 0.2s ease; font-size: 12px;"
                            >
                                "⚡ Batched"
                            </button>
                        </div>
                    </div>

                    // Comparison test button
                    <button
                        on:click=run_comparison_test
                        disabled=move || is_updating.get()
                        style="width: 100%; padding: 10px; background: rgba(168, 85, 247, 0.3); \
                               border: 1px solid rgba(168, 85, 247, 0.5); border-radius: 8px; \
                               color: white; font-weight: 600; cursor: pointer; margin-bottom: 12px; \
                               transition: all 0.2s ease;"
                    >
                        "🧪 Run Comparison Test (5x each)"
                    </button>

                    // Performance visualization
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 12px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"Performance Results"</div>
                        <div style="max-height: 150px; overflow-y: auto;">
                            {move || {
                                let results = timing_results.get();
                                if results.is_empty() {
                                    view! {
                                        <div style="font-size: 11px; opacity: 0.6; font-style: italic; text-align: center; padding: 16px 0;">
                                            "Run an update to see results..."
                                        </div>
                                    }.into_any()
                                } else {
                                    // Calculate averages
                                    let individual_times: Vec<f64> = results.iter()
                                        .filter(|r| r.operation.starts_with("Individual"))
                                        .map(|r| r.duration_ms)
                                        .collect();
                                    let batched_times: Vec<f64> = results.iter()
                                        .filter(|r| r.operation.starts_with("Batched"))
                                        .map(|r| r.duration_ms)
                                        .collect();

                                    let avg_individual = if individual_times.is_empty() {
                                        0.0
                                    } else {
                                        individual_times.iter().sum::<f64>() / individual_times.len() as f64
                                    };
                                    let avg_batched = if batched_times.is_empty() {
                                        0.0
                                    } else {
                                        batched_times.iter().sum::<f64>() / batched_times.len() as f64
                                    };

                                    view! {
                                        <div>
                                            // Averages
                                            {(avg_individual > 0.0 || avg_batched > 0.0).then(|| view! {
                                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 8px;">
                                                    <div style="background: rgba(239, 68, 68, 0.3); padding: 6px; border-radius: 4px; text-align: center;">
                                                        <div style="font-size: 10px; opacity: 0.8;">"Avg Individual"</div>
                                                        <div style="font-weight: 600; font-size: 14px;">{format!("{:.2}ms", avg_individual)}</div>
                                                    </div>
                                                    <div style="background: rgba(34, 197, 94, 0.3); padding: 6px; border-radius: 4px; text-align: center;">
                                                        <div style="font-size: 10px; opacity: 0.8;">"Avg Batched"</div>
                                                        <div style="font-weight: 600; font-size: 14px;">{format!("{:.2}ms", avg_batched)}</div>
                                                    </div>
                                                </div>
                                            })}

                                            // Results list
                                            {results.iter().take(10).map(|result| {
                                                let is_batched = result.operation.starts_with("Batched");
                                                let bg_color = if is_batched {
                                                    "rgba(34, 197, 94, 0.2)"
                                                } else {
                                                    "rgba(239, 68, 68, 0.2)"
                                                };
                                                let border_color = if is_batched {
                                                    "rgba(34, 197, 94, 0.4)"
                                                } else {
                                                    "rgba(239, 68, 68, 0.4)"
                                                };
                                                let operation = result.operation.clone();
                                                let duration = result.duration_ms;
                                                let node_count = result.node_count;

                                                view! {
                                                    <div style=format!(
                                                        "display: flex; justify-content: space-between; align-items: center; \
                                                         padding: 4px 8px; margin-bottom: 4px; border-radius: 4px; \
                                                         background: {}; border: 1px solid {}; font-size: 11px;",
                                                        bg_color, border_color
                                                    )>
                                                        <span style="font-weight: 500;">{operation}</span>
                                                        <span>
                                                            {format!("{} nodes", node_count)}
                                                            " • "
                                                            <strong>{format!("{:.2}ms", duration)}</strong>
                                                        </span>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>

                    // Quick actions
                    <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                        <button
                            on:click=add_nodes
                            style="flex: 1; padding: 8px; background: rgba(255,255,255,0.2); \
                                   border: 1px solid rgba(255,255,255,0.3); border-radius: 6px; \
                                   color: white; font-size: 11px; cursor: pointer;"
                        >
                            "+ Add Nodes"
                        </button>
                        <button
                            on:click=reset_positions
                            style="flex: 1; padding: 8px; background: rgba(255,255,255,0.2); \
                                   border: 1px solid rgba(255,255,255,0.3); border-radius: 6px; \
                                   color: white; font-size: 11px; cursor: pointer;"
                        >
                            "↺ Reset Positions"
                        </button>
                        <button
                            on:click=clear_results
                            style="flex: 1; padding: 8px; background: rgba(255,255,255,0.2); \
                                   border: 1px solid rgba(255,255,255,0.3); border-radius: 6px; \
                                   color: white; font-size: 11px; cursor: pointer;"
                        >
                            "🗑️ Clear"
                        </button>
                    </div>

                    // Action log
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px;">
                        <div style="font-size: 12px; font-weight: 600; margin-bottom: 6px; opacity: 0.9;">"Activity Log"</div>
                        <div style="max-height: 80px; overflow-y: auto; font-size: 10px; font-family: monospace;">
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
                            background: rgba(14, 165, 233, 0.9); color: white; \
                            padding: 10px 14px; border-radius: 8px; font-size: 11px; \
                            max-width: 220px; line-height: 1.5;">
                    <div style="font-weight: 600; margin-bottom: 6px;">"💡 How Batching Works"</div>
                    <div style="margin-bottom: 4px;"><strong>"Individual:"</strong>" Updates each node with separate update_node() calls"</div>
                    <div><strong>"Batched:"</strong>" Modifies all nodes in memory, then applies with single set_nodes()"</div>
                </div>
            </div>
        </div>
    }
}

/// Create a grid of nodes
fn create_grid_nodes(cols: usize, rows: usize) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut id = 1;

    for row in 0..rows {
        for col in 0..cols {
            let node_type = if row == 0 && col == 0 {
                "input"
            } else if row == rows - 1 && col == cols - 1 {
                "output"
            } else {
                "default"
            };

            nodes.push(
                Node::new(
                    format!("node-{}", id),
                    Position::new(100.0 + col as f64 * 120.0, 80.0 + row as f64 * 100.0),
                )
                .with_data(json!({
                    "label": format!("Node {}", id),
                    "nodeType": node_type
                }))
                .with_dimensions(100.0, 50.0),
            );

            id += 1;
        }
    }

    nodes
}

/// Create edges connecting adjacent nodes in a grid
fn create_grid_edges(cols: usize, rows: usize) -> Vec<Edge> {
    let mut edges = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            let id = row * cols + col + 1;

            // Connect to right neighbor
            if col < cols - 1 {
                let right_id = id + 1;
                edges.push(
                    Edge::new(
                        format!("e{}-{}", id, right_id),
                        format!("node-{}", id),
                        format!("node-{}", right_id),
                    )
                    .with_label(format!("{} → {}", id, right_id)),
                );
            }

            // Connect to bottom neighbor
            if row < rows - 1 {
                let bottom_id = id + cols;
                edges.push(
                    Edge::new(
                        format!("e{}-{}", id, bottom_id),
                        format!("node-{}", id),
                        format!("node-{}", bottom_id),
                    )
                    .with_label(format!("{} → {}", id, bottom_id)),
                );
            }
        }
    }

    edges
}

/// Custom node component for batching example
#[component]
fn BatchingNode(node: Node, store: FlowStore) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let drag_signal = get_set_nodes_batching_drag_signal();

    // Extract node data
    let label = node
        .data
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node
        .data
        .get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    // Get colors based on node type
    let (bg_color, border_color, text_color) = match node_type.as_str() {
        "input" => ("#22c55e", "#16a34a", "white"),  // Green
        "output" => ("#ef4444", "#dc2626", "white"), // Red
        _ => ("#8b5cf6", "#7c3aed", "white"),        // Purple (default)
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
        store
            .get_nodes()
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
                 padding: 10px 14px; min-width: 80px; text-align: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.15); transition: box-shadow 0.2s ease;",
                pos().x, pos().y, bg_color, border_color
            )
            on:mousedown=on_mousedown
        >
            // Node label
            <div style=format!(
                "color: {}; font-weight: 600; font-size: 12px; text-shadow: 0 1px 2px rgba(0,0,0,0.2);",
                text_color
            )>
                {label}
            </div>

            // Handles based on node type
            {match node_type.as_str() {
                "input" => view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Right
                        connection_mode=ConnectionMode::Strict
                    />
                }.into_any(),
                "output" => view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Left
                        connection_mode=ConnectionMode::Strict
                    />
                }.into_any(),
                _ => view! {
                    <>
                        <Handle
                            node_id=node.id.clone()
                            r#type=HandleType::Target
                            position=HandlePosition::Left
                            connection_mode=ConnectionMode::Strict
                        />
                        <Handle
                            node_id=node.id.clone()
                            r#type=HandleType::Source
                            position=HandlePosition::Right
                            connection_mode=ConnectionMode::Strict
                        />
                    </>
                }.into_any(),
            }}
        </div>
    }
}

/// Edge renderer component for batching example
#[component]
fn SetNodesBatchingEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                // Gradient for edges
                <linearGradient id="batching-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #0ea5e9; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #8b5cf6; stop-opacity: 1" />
                </linearGradient>

                // Arrow marker
                <marker
                    id="batching-arrow"
                    viewBox="0 0 10 10"
                    refX="10"
                    refY="5"
                    markerUnits="strokeWidth"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#8b5cf6" />
                </marker>
            </defs>

            {move || {
                let nodes = store.get_nodes();
                let edges = store.get_edges();

                edges.into_iter().filter_map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Get source/target types
                    let source_type = source_node.data.get("nodeType")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");

                    // Calculate edge positions (right side of source, left side of target)
                    let source_x = source_node.position.x + source_node.width.unwrap_or(100.0);
                    let source_y = source_node.position.y + source_node.height.unwrap_or(50.0) / 2.0;
                    let target_x = target_node.position.x;
                    let target_y = target_node.position.y + target_node.height.unwrap_or(50.0) / 2.0;

                    // Generate smooth bezier path
                    let dx = (target_x - source_x).abs();
                    let control_offset = dx.max(40.0) * 0.4;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        source_x, source_y,
                        source_x + control_offset, source_y,
                        target_x - control_offset, target_y,
                        target_x, target_y
                    );

                    // Choose color based on source type
                    let stroke_color = match source_type {
                        "input" => "#22c55e",
                        "output" => "#ef4444",
                        _ => "#8b5cf6",
                    };

                    Some(view! {
                        <g>
                            // Shadow/glow
                            <path
                                d=path.clone()
                                fill="none"
                                stroke=format!("{}40", stroke_color)
                                stroke-width="4"
                                stroke-linecap="round"
                            />

                            // Main edge
                            <path
                                d=path.clone()
                                fill="none"
                                stroke=stroke_color
                                stroke-width="1.5"
                                stroke-linecap="round"
                                marker-end="url(#batching-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
