//! Use Nodes Initialized Example
//!
//! Demonstrates the lifecycle hook for when nodes are initialized:
//! - Tracks when nodes are created and measured
//! - Shows initialization timing compared to render
//! - Displays initialization status in real-time
//! - Demonstrates batch vs individual initialization patterns

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for this example
static USE_NODES_INIT_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_use_nodes_init_drag_signal() -> RwSignal<Option<DragState>> {
    *USE_NODES_INIT_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Node initialization state
#[derive(Clone, Debug)]
struct NodeInitState {
    node_id: String,
    created_at: f64,
    initialized_at: Option<f64>,
    label: String,
}

impl NodeInitState {
    fn is_initialized(&self) -> bool {
        self.initialized_at.is_some()
    }

    fn init_duration_ms(&self) -> Option<f64> {
        self.initialized_at.map(|init| init - self.created_at)
    }
}

/// Use Nodes Initialized example
#[component]
pub fn UseNodesInitExample() -> impl IntoView {
    // Get current timestamp for timing (using js_sys::Date for simplicity)
    let get_timestamp = || {
        js_sys::Date::now()
    };

    // Store the component mount time
    let mount_time = get_timestamp();

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({
                "label": "Node 1",
                "order": 1
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("2".to_string(), Position::new(300.0, 80.0))
            .with_data(json!({
                "label": "Node 2",
                "order": 2
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("3".to_string(), Position::new(200.0, 220.0))
            .with_data(json!({
                "label": "Node 3",
                "order": 3
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("4".to_string(), Position::new(400.0, 180.0))
            .with_data(json!({
                "label": "Node 4",
                "order": 4
            }))
            .with_dimensions(140.0, 50.0),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Track node initialization states
    let init_states = RwSignal::new(Vec::<NodeInitState>::new());

    // Track overall initialization status
    let all_nodes_initialized = RwSignal::new(false);
    let initialization_complete_time = RwSignal::new(Option::<f64>::None);

    // Event log
    let event_log = RwSignal::new(Vec::<String>::new());

    // Add event to log
    let add_event = move |event: String| {
        let timestamp = get_timestamp() - mount_time;
        event_log.update(|log| {
            log.insert(0, format!("[{:.1}ms] {}", timestamp, event));
            if log.len() > 10 {
                log.pop();
            }
        });
    };

    // Initialize nodes - simulate the initialization hook
    Effect::new(move || {
        let nodes = store.get_nodes();
        let current_time = get_timestamp();

        // Log initial render
        if init_states.get().is_empty() {
            add_event("Component mounted".to_string());
        }

        // Track new nodes
        init_states.update(|states| {
            for node in nodes.iter() {
                if !states.iter().any(|s| s.node_id == node.id) {
                    let label = node.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();

                    states.push(NodeInitState {
                        node_id: node.id.clone(),
                        created_at: current_time,
                        initialized_at: None,
                        label,
                    });

                    add_event(format!("Node '{}' created", node.id));
                }
            }

            // Mark nodes as initialized if they have dimensions
            for state in states.iter_mut() {
                if !state.is_initialized() {
                    if let Some(node) = nodes.iter().find(|n| n.id == state.node_id) {
                        if node.width.is_some() && node.height.is_some() {
                            state.initialized_at = Some(current_time);
                            add_event(format!("Node '{}' initialized", state.node_id));
                        }
                    }
                }
            }
        });

        // Check if all nodes are initialized
        let states = init_states.get();
        let all_init = !states.is_empty() && states.iter().all(|s| s.is_initialized());

        if all_init && !all_nodes_initialized.get() {
            all_nodes_initialized.set(true);
            let complete_time = current_time - mount_time;
            initialization_complete_time.set(Some(complete_time));
            add_event(format!("All nodes initialized in {:.1}ms", complete_time));
        }
    });

    // Global drag handlers
    let drag_signal = get_use_nodes_init_drag_signal();

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

    // Add node action
    let node_counter = RwSignal::new(5);
    let add_node = move |_| {
        let id = node_counter.get();
        node_counter.update(|c| *c += 1);

        // Get random position
        let x = 50.0 + (js_sys::Math::random() * 350.0);
        let y = 50.0 + (js_sys::Math::random() * 250.0);

        let new_node = Node::new(format!("{}", id), Position::new(x, y))
            .with_data(json!({
                "label": format!("Node {}", id),
                "order": id
            }))
            .with_dimensions(140.0, 50.0);

        let mut nodes = store.get_nodes();
        nodes.push(new_node);
        store.set_nodes(nodes);

        add_event(format!("Added new Node '{}'", id));
    };

    // Reset action
    let reset_flow = move |_| {
        // Reset to initial state
        let nodes = vec![
            Node::new("1".to_string(), Position::new(100.0, 100.0))
                .with_data(json!({
                    "label": "Node 1",
                    "order": 1
                }))
                .with_dimensions(140.0, 50.0),
            Node::new("2".to_string(), Position::new(300.0, 80.0))
                .with_data(json!({
                    "label": "Node 2",
                    "order": 2
                }))
                .with_dimensions(140.0, 50.0),
            Node::new("3".to_string(), Position::new(200.0, 220.0))
                .with_data(json!({
                    "label": "Node 3",
                    "order": 3
                }))
                .with_dimensions(140.0, 50.0),
            Node::new("4".to_string(), Position::new(400.0, 180.0))
                .with_data(json!({
                    "label": "Node 4",
                    "order": 4
                }))
                .with_dimensions(140.0, 50.0),
        ];
        store.set_nodes(nodes);

        let edges = vec![
            Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
            Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
            Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
            Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
        ];
        store.set_edges(edges);

        // Reset tracking state
        init_states.set(vec![]);
        all_nodes_initialized.set(false);
        initialization_complete_time.set(None);
        event_log.set(vec![]);
        node_counter.set(5);

        add_event("Flow reset".to_string());
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer
                    <UseNodesInitEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <InitNode
                                    node=node.clone()
                                    store=store
                                    init_states=init_states
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
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Use Nodes Initialized"</strong>

                        <p style="margin: 0 0 12px 0; font-size: 11px; color: #666; line-height: 1.4;">
                            "This example demonstrates tracking node initialization lifecycle. Nodes show their init status with visual indicators."
                        </p>

                        // Overall status
                        <div style=move || format!(
                            "background: {}; padding: 12px; border-radius: 6px; margin-bottom: 12px; \
                             border: 1px solid {};",
                            if all_nodes_initialized.get() { "#ecfdf5" } else { "#fef3c7" },
                            if all_nodes_initialized.get() { "#10b981" } else { "#f59e0b" }
                        )>
                            <div style="display: flex; align-items: center; gap: 8px;">
                                <span style=move || format!(
                                    "width: 10px; height: 10px; border-radius: 50%; background: {};",
                                    if all_nodes_initialized.get() { "#10b981" } else { "#f59e0b" }
                                )></span>
                                <span style=move || format!(
                                    "font-weight: 600; font-size: 12px; color: {};",
                                    if all_nodes_initialized.get() { "#065f46" } else { "#92400e" }
                                )>
                                    {move || if all_nodes_initialized.get() { "All Nodes Initialized" } else { "Initializing..." }}
                                </span>
                            </div>
                            {move || {
                                initialization_complete_time.get().map(|time| {
                                    view! {
                                        <div style="margin-top: 6px; font-size: 10px; color: #065f46;">
                                            "Total init time: " <strong>{format!("{:.1}ms", time)}</strong>
                                        </div>
                                    }
                                })
                            }}
                        </div>

                        // Node init status
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Node Status"</div>
                            <div style="background: #f5f5f5; border-radius: 6px; padding: 8px; max-height: 120px; overflow-y: auto;">
                                {move || {
                                    let states = init_states.get();
                                    if states.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic;">
                                                "No nodes tracked"
                                            </div>
                                        }.into_any()
                                    } else {
                                        states.iter().map(|state| {
                                            let is_init = state.is_initialized();
                                            let duration = state.init_duration_ms();

                                            view! {
                                                <div style="display: flex; align-items: center; justify-content: space-between; \
                                                            padding: 4px 0; border-bottom: 1px solid #eee; font-size: 10px;">
                                                    <div style="display: flex; align-items: center; gap: 6px;">
                                                        <span style=format!(
                                                            "width: 8px; height: 8px; border-radius: 50%; background: {};",
                                                            if is_init { "#10b981" } else { "#f59e0b" }
                                                        )></span>
                                                        <span style="color: #333;">{state.label.clone()}</span>
                                                    </div>
                                                    <span style=format!(
                                                        "font-size: 9px; padding: 2px 6px; border-radius: 4px; background: {}; color: {};",
                                                        if is_init { "#ecfdf5" } else { "#fef3c7" },
                                                        if is_init { "#065f46" } else { "#92400e" }
                                                    )>
                                                        {if is_init {
                                                            format!("✓ {:.1}ms", duration.unwrap_or(0.0))
                                                        } else {
                                                            "pending".to_string()
                                                        }}
                                                    </span>
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // Actions
                        <div style="display: flex; gap: 8px; margin-bottom: 12px;">
                            <button
                                style="flex: 1; padding: 8px 12px; font-size: 11px; border: 1px solid #2196f3; \
                                       border-radius: 6px; background: #e3f2fd; color: #1976d2; cursor: pointer; \
                                       font-weight: 500;"
                                on:click=add_node
                            >
                                "+ Add Node"
                            </button>
                            <button
                                style="flex: 1; padding: 8px 12px; font-size: 11px; border: 1px solid #9e9e9e; \
                                       border-radius: 6px; background: #f5f5f5; color: #616161; cursor: pointer; \
                                       font-weight: 500;"
                                on:click=reset_flow
                            >
                                "⟳ Reset"
                            </button>
                        </div>

                        // Event log
                        <div style="padding-top: 12px; border-top: 1px solid #eee;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 6px;">"Event Log"</div>
                            <div style="background: #1e293b; border-radius: 6px; padding: 8px; max-height: 100px; \
                                        overflow-y: auto; font-family: monospace;">
                                {move || {
                                    let log = event_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #64748b; font-style: italic;">
                                                "No events yet"
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.into_iter().map(|entry| {
                                            view! {
                                                <div style="font-size: 9px; color: #94a3b8; padding: 2px 0;">
                                                    {entry}
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

/// Node component with initialization indicator
#[component]
fn InitNode(
    node: Node,
    store: FlowStore,
    init_states: RwSignal<Vec<NodeInitState>>,
) -> impl IntoView {
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_status = node.id.clone();

    let drag_signal = get_use_nodes_init_drag_signal();

    // Mouse down - start dragging
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
            class="xyflow__node init-node"
            style=move || {
                let nodes = store.get_nodes();
                let states = init_states.get();
                let is_initialized = states.iter()
                    .find(|s| s.node_id == node_id_for_style)
                    .map(|s| s.is_initialized())
                    .unwrap_or(false);

                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    let border_color = if is_initialized { "#10b981" } else { "#f59e0b" };
                    let bg_color = if is_initialized { "#f0fdf4" } else { "#fffbeb" };
                    let shadow = if is_initialized {
                        "0 0 0 2px rgba(16, 185, 129, 0.2), 0 2px 8px rgba(0,0,0,0.1)"
                    } else {
                        "0 0 0 2px rgba(245, 158, 11, 0.2), 0 2px 8px rgba(0,0,0,0.1)"
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: 2px solid {}; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: all 0.3s ease;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(140.0), n.height.unwrap_or(50.0),
                        bg_color, border_color, shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_mousedown
        >
            // Status indicator
            {move || {
                let states = init_states.get();
                let state = states.iter().find(|s| s.node_id == node_id_for_status);
                let is_initialized = state.map(|s| s.is_initialized()).unwrap_or(false);

                view! {
                    <div style="position: absolute; top: -6px; right: -6px; \
                                width: 14px; height: 14px; border-radius: 50%; \
                                display: flex; align-items: center; justify-content: center; \
                                font-size: 8px; color: white;"
                         style=("background", if is_initialized { "#10b981" } else { "#f59e0b" })>
                        {if is_initialized { "✓" } else { "…" }}
                    </div>
                }
            }}

            // Node label
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_label) {
                    let label = n.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Node")
                        .to_string();
                    let order = n.data.get("order")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    view! {
                        <>
                            <div style="font-weight: 600; font-size: 12px; color: #333;">{label}</div>
                            <div style="font-size: 9px; color: #888; margin-top: 2px;">
                                "Order: " {order}
                            </div>
                        </>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Handles
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Top
                connection_mode=ConnectionMode::Strict
                style="background: #10b981; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
            <Handle
                node_id=node.id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Bottom
                connection_mode=ConnectionMode::Strict
                style="background: #10b981; width: 8px; height: 8px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
            />
        </div>
    }
}

/// Edge renderer for use nodes init example
#[component]
fn UseNodesInitEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <marker
                    id="use-nodes-init-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#10b981" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.into_iter().filter_map(move |edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Calculate edge path
                    let sx = source_node.position.x + source_node.width.unwrap_or(140.0) / 2.0;
                    let sy = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let tx = target_node.position.x + target_node.width.unwrap_or(140.0) / 2.0;
                    let ty = target_node.position.y;

                    let offset = (ty - sy).abs() * 0.5;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx, sy + offset,
                        tx, ty - offset,
                        tx, ty
                    );

                    Some(view! {
                        <g class="xyflow__edge">
                            // Shadow
                            <path
                                d=path.clone()
                                stroke="rgba(16, 185, 129, 0.2)"
                                stroke-width="6"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path
                                stroke="#10b981"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#use-nodes-init-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
