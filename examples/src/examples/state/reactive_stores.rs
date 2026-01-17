//! Reactive Stores Example
//!
//! Demonstrates integration with Leptos reactive_stores (equivalent to Redux).
//! Shows how to use #[derive(Store)] for flow state with fine-grained reactivity
//! and nested state updates.

use leptos::prelude::*;
use leptos::serde_json::json;
use reactive_stores::{Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for reactive stores example
static REACTIVE_STORES_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_reactive_stores_drag_signal() -> RwSignal<Option<DragState>> {
    *REACTIVE_STORES_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Reactive Store Data Structures
// ============================================================================

/// Node data stored in the reactive store
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct NodeData {
    pub label: String,
    pub node_type: String,
    pub color: String,
    pub priority: u8,
    pub active: bool,
}

/// Edge data stored in the reactive store
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct EdgeData {
    pub label: String,
    pub edge_type: String,
    pub animated: bool,
}

/// A single node in the flow
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct FlowNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub data: NodeData,
    pub selected: bool,
    pub dragging: bool,
}

/// A single edge in the flow
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct FlowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub data: EdgeData,
    pub selected: bool,
}

/// Viewport state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct ViewportState {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

/// UI state for tracking updates
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct UIState {
    pub node_updates: u32,
    pub edge_updates: u32,
    pub viewport_updates: u32,
    pub selected_node_id: Option<String>,
}

/// The main flow state with fine-grained reactivity
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, reactive_stores::Store)]
pub struct ReactiveFlowState {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub viewport: ViewportState,
    pub ui: UIState,
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Reactive Stores example component
#[component]
pub fn ReactiveStoresExample() -> impl IntoView {
    // Create the reactive store with initial state
    let initial_state = ReactiveFlowState {
        nodes: vec![
            FlowNode {
                id: "1".to_string(),
                x: 50.0,
                y: 50.0,
                width: 140.0,
                height: 60.0,
                data: NodeData {
                    label: "Input Node".to_string(),
                    node_type: "input".to_string(),
                    color: "#4ade80".to_string(),
                    priority: 1,
                    active: true,
                },
                selected: false,
                dragging: false,
            },
            FlowNode {
                id: "2".to_string(),
                x: 200.0,
                y: 150.0,
                width: 140.0,
                height: 60.0,
                data: NodeData {
                    label: "Process A".to_string(),
                    node_type: "default".to_string(),
                    color: "#818cf8".to_string(),
                    priority: 2,
                    active: true,
                },
                selected: false,
                dragging: false,
            },
            FlowNode {
                id: "3".to_string(),
                x: 50.0,
                y: 250.0,
                width: 140.0,
                height: 60.0,
                data: NodeData {
                    label: "Process B".to_string(),
                    node_type: "default".to_string(),
                    color: "#818cf8".to_string(),
                    priority: 2,
                    active: false,
                },
                selected: false,
                dragging: false,
            },
            FlowNode {
                id: "4".to_string(),
                x: 200.0,
                y: 350.0,
                width: 140.0,
                height: 60.0,
                data: NodeData {
                    label: "Output Node".to_string(),
                    node_type: "output".to_string(),
                    color: "#f87171".to_string(),
                    priority: 3,
                    active: true,
                },
                selected: false,
                dragging: false,
            },
        ],
        edges: vec![
            FlowEdge {
                id: "e1-2".to_string(),
                source: "1".to_string(),
                target: "2".to_string(),
                data: EdgeData {
                    label: "Flow A".to_string(),
                    edge_type: "default".to_string(),
                    animated: false,
                },
                selected: false,
            },
            FlowEdge {
                id: "e1-3".to_string(),
                source: "1".to_string(),
                target: "3".to_string(),
                data: EdgeData {
                    label: "Flow B".to_string(),
                    edge_type: "default".to_string(),
                    animated: true,
                },
                selected: false,
            },
            FlowEdge {
                id: "e2-4".to_string(),
                source: "2".to_string(),
                target: "4".to_string(),
                data: EdgeData {
                    label: "To Output".to_string(),
                    edge_type: "default".to_string(),
                    animated: false,
                },
                selected: false,
            },
            FlowEdge {
                id: "e3-4".to_string(),
                source: "3".to_string(),
                target: "4".to_string(),
                data: EdgeData {
                    label: "Alternate".to_string(),
                    edge_type: "default".to_string(),
                    animated: true,
                },
                selected: false,
            },
        ],
        viewport: ViewportState::default(),
        ui: UIState::default(),
    };

    // Create the reactive store
    let store = Store::new(initial_state);

    // Also create a FlowStore for the actual rendering
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Input Node", "nodeType": "input", "color": "#4ade80"}))
            .with_dimensions(140.0, 60.0),
        Node::new("2".to_string(), Position::new(200.0, 150.0))
            .with_data(json!({"label": "Process A", "nodeType": "default", "color": "#818cf8"}))
            .with_dimensions(140.0, 60.0),
        Node::new("3".to_string(), Position::new(50.0, 250.0))
            .with_data(json!({"label": "Process B", "nodeType": "default", "color": "#818cf8"}))
            .with_dimensions(140.0, 60.0),
        Node::new("4".to_string(), Position::new(200.0, 350.0))
            .with_data(json!({"label": "Output Node", "nodeType": "output", "color": "#f87171"}))
            .with_dimensions(140.0, 60.0),
    ];

    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_label("Flow A".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Flow B".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string())
            .with_label("To Output".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string())
            .with_label("Alternate".to_string()),
    ];

    let flow_store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(flow_store);

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

    // Demonstrate fine-grained updates by incrementing node update counter
    let increment_node_updates = {
        let store = store.clone();
        let add_log = add_log.clone();
        move || {
            store.ui().node_updates().update(|c| *c += 1);
            add_log("Incremented node updates (fine-grained)");
        }
    };

    // Update a specific node's label (nested update)
    let update_node_label = {
        let store = store.clone();
        let flow_store = flow_store;
        let add_log = add_log.clone();
        move |node_idx: usize, new_label: String| {
            // Update in reactive store (nested field update)
            store.nodes().iter_unkeyed().nth(node_idx).map(|node| {
                node.data().label().set(new_label.clone());
            });

            // Also update in flow store for rendering
            let node_id = match node_idx {
                0 => "1",
                1 => "2",
                2 => "3",
                3 => "4",
                _ => return,
            };
            flow_store.update_node(node_id, |n| {
                if let Some(data) = n.data.as_object_mut() {
                    data.insert("label".to_string(), json!(new_label.clone()));
                }
            });

            add_log(&format!("Updated node {} label to '{}'", node_idx + 1, new_label));
        }
    };

    // Toggle node active state
    let toggle_node_active = {
        let store = store.clone();
        let add_log = add_log.clone();
        move |node_idx: usize| {
            store.nodes().iter_unkeyed().nth(node_idx).map(|node| {
                let current = node.data().active().get();
                node.data().active().set(!current);
            });
            add_log(&format!("Toggled node {} active state", node_idx + 1));
        }
    };

    // Update node priority
    let update_node_priority = {
        let store = store.clone();
        let add_log = add_log.clone();
        move |node_idx: usize, new_priority: u8| {
            store.nodes().iter_unkeyed().nth(node_idx).map(|node| {
                node.data().priority().set(new_priority);
            });
            add_log(&format!("Updated node {} priority to {}", node_idx + 1, new_priority));
        }
    };

    // Toggle edge animated state
    let toggle_edge_animated = {
        let store = store.clone();
        let add_log = add_log.clone();
        move |edge_idx: usize| {
            store.edges().iter_unkeyed().nth(edge_idx).map(|edge| {
                let current = edge.data().animated().get();
                edge.data().animated().set(!current);
            });
            add_log(&format!("Toggled edge {} animation", edge_idx + 1));
        }
    };

    // Update viewport zoom
    let update_zoom = {
        let store = store.clone();
        let flow_store = flow_store;
        let add_log = add_log.clone();
        move |new_zoom: f64| {
            store.viewport().zoom().set(new_zoom);
            store.ui().viewport_updates().update(|c| *c += 1);

            // Update flow store viewport
            let mut viewport = flow_store.get_viewport();
            viewport.zoom = new_zoom;
            flow_store.set_viewport(viewport);

            add_log(&format!("Updated zoom to {:.1}x", new_zoom));
        }
    };

    // Global drag handlers
    let drag_signal = get_reactive_stores_drag_signal();

    let on_global_mousemove = {
        let store = store.clone();
        let flow_store = flow_store;
        move |ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let current_x = ev.client_x() as f64;
                let current_y = ev.client_y() as f64;
                let (start_x, start_y) = drag_state.start_mouse;
                let (node_start_x, node_start_y) = drag_state.start_pos;

                // Get viewport for zoom adjustment
                let zoom = store.viewport().zoom().get();
                let dx = (current_x - start_x) / zoom;
                let dy = (current_y - start_y) / zoom;

                let new_x = node_start_x + dx;
                let new_y = node_start_y + dy;

                // Update node position in flow store
                flow_store.update_node(&drag_state.node_id, |n| {
                    n.position = Position::new(new_x, new_y);
                });

                // Update in reactive store
                let node_idx = match drag_state.node_id.as_str() {
                    "1" => 0,
                    "2" => 1,
                    "3" => 2,
                    "4" => 3,
                    _ => return,
                };
                store.nodes().iter_unkeyed().nth(node_idx).map(|node| {
                    node.x().set(new_x);
                    node.y().set(new_y);
                });
            }
        }
    };

    let on_global_mouseup = {
        let store = store.clone();
        let flow_store = flow_store;
        move |_ev: leptos::ev::MouseEvent| {
            if let Some(drag_state) = drag_signal.get() {
                let node_id = drag_state.node_id.clone();
                flow_store.update_node(&node_id, |n| {
                    n.dragging = false;
                });

                // Update dragging state in reactive store
                let node_idx = match node_id.as_str() {
                    "1" => 0,
                    "2" => 1,
                    "3" => 2,
                    "4" => 3,
                    _ => {
                        drag_signal.set(None);
                        return;
                    }
                };
                store.nodes().iter_unkeyed().nth(node_idx).map(|node| {
                    node.dragging().set(false);
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
                    <ReactiveStoresEdgeRenderer flow_store=flow_store reactive_store=store.clone() />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        flow_store.get_nodes().into_iter()
                            .map(|node| {
                                view! {
                                    <ReactiveStoresNode
                                        node=node.clone()
                                        flow_store=flow_store
                                        reactive_store=store.clone()
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
                <div style="position: absolute; top: 16px; right: 16px; width: 340px; \
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); \
                            border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.2); \
                            padding: 16px; color: white; font-family: system-ui, -apple-system, sans-serif; \
                            max-height: calc(100vh - 100px); overflow-y: auto;">
                    <div style="font-size: 18px; font-weight: 600; margin-bottom: 12px; \
                                display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 20px;">"🗃️"</span>
                        "Reactive Stores"
                    </div>

                    // Store Overview
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"STORE STRUCTURE"</div>
                        <div style="font-family: monospace; font-size: 10px; line-height: 1.6; opacity: 0.9;">
                            <div>"#[derive(Store)]"</div>
                            <div>"ReactiveFlowState {"</div>
                            <div style="padding-left: 12px;">"nodes: Vec<FlowNode>,"</div>
                            <div style="padding-left: 12px;">"edges: Vec<FlowEdge>,"</div>
                            <div style="padding-left: 12px;">"viewport: ViewportState,"</div>
                            <div style="padding-left: 12px;">"ui: UIState,"</div>
                            <div>"}"</div>
                        </div>
                    </div>

                    // Fine-grained Reactivity Demo
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"FINE-GRAINED UPDATES"</div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 8px;">
                            <div style="background: rgba(255,255,255,0.1); padding: 6px 8px; border-radius: 4px; text-align: center;">
                                <div style="font-size: 16px; font-weight: 700;">{move || store.ui().node_updates().get()}</div>
                                <div style="font-size: 9px; opacity: 0.8;">"Node Updates"</div>
                            </div>
                            <div style="background: rgba(255,255,255,0.1); padding: 6px 8px; border-radius: 4px; text-align: center;">
                                <div style="font-size: 16px; font-weight: 700;">{move || store.ui().viewport_updates().get()}</div>
                                <div style="font-size: 9px; opacity: 0.8;">"Viewport Updates"</div>
                            </div>
                        </div>
                        <button
                            on:click={
                                let increment_node_updates = increment_node_updates.clone();
                                move |_| increment_node_updates()
                            }
                            style="width: 100%; padding: 8px; background: rgba(255,255,255,0.2); \
                                   border: 1px solid rgba(255,255,255,0.3); border-radius: 6px; \
                                   color: white; font-size: 11px; cursor: pointer;"
                        >
                            "Increment Node Updates (Fine-Grained)"
                        </button>
                    </div>

                    // Nested State Updates
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"NESTED STATE UPDATES"</div>

                        // Node list with update buttons
                        <div style="font-size: 10px; margin-bottom: 8px;">
                            {move || {
                                store.nodes().iter_unkeyed()
                                    .enumerate()
                                    .map(|(idx, node)| {
                                        let label = node.data().label().get();
                                        let active = node.data().active().get();
                                        let priority = node.data().priority().get();
                                        let x = node.x().get();
                                        let y = node.y().get();

                                        let toggle_active = {
                                            let toggle_node_active = toggle_node_active.clone();
                                            move |_| toggle_node_active(idx)
                                        };

                                        let update_priority_up = {
                                            let update_node_priority = update_node_priority.clone();
                                            move |_| update_node_priority(idx, (priority + 1).min(5))
                                        };

                                        view! {
                                            <div style="background: rgba(255,255,255,0.1); padding: 6px 8px; \
                                                        border-radius: 4px; margin-bottom: 4px;">
                                                <div style="display: flex; justify-content: space-between; \
                                                            align-items: center; margin-bottom: 4px;">
                                                    <span style="font-weight: 600;">{label}</span>
                                                    <span style=format!("padding: 2px 6px; border-radius: 4px; \
                                                                         background: {}; font-size: 9px;",
                                                                         if active { "rgba(74,222,128,0.5)" } else { "rgba(255,100,100,0.5)" })>
                                                        {if active { "Active" } else { "Inactive" }}
                                                    </span>
                                                </div>
                                                <div style="display: flex; gap: 4px; align-items: center; \
                                                            font-size: 9px; opacity: 0.8;">
                                                    <span>"Pos: ("{format!("{:.0}", x)}","{ format!("{:.0}", y)}")"</span>
                                                    <span style="margin-left: auto;">"Pri: "{priority}</span>
                                                    <button
                                                        on:click=update_priority_up
                                                        style="padding: 2px 6px; background: rgba(255,255,255,0.2); \
                                                               border: none; border-radius: 3px; color: white; \
                                                               cursor: pointer; font-size: 9px;"
                                                    >
                                                        "+1"
                                                    </button>
                                                    <button
                                                        on:click=toggle_active
                                                        style="padding: 2px 6px; background: rgba(255,255,255,0.2); \
                                                               border: none; border-radius: 3px; color: white; \
                                                               cursor: pointer; font-size: 9px;"
                                                    >
                                                        "Toggle"
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()
                            }}
                        </div>

                        // Quick label update
                        <div style="display: flex; gap: 4px;">
                            <button
                                on:click={
                                    let update_node_label = update_node_label.clone();
                                    move |_| update_node_label(0, "Modified Input".to_string())
                                }
                                style="flex: 1; padding: 6px; background: rgba(74,222,128,0.3); \
                                       border: 1px solid rgba(74,222,128,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Rename Node 1"
                            </button>
                            <button
                                on:click={
                                    let update_node_label = update_node_label.clone();
                                    move |_| update_node_label(3, "Modified Output".to_string())
                                }
                                style="flex: 1; padding: 6px; background: rgba(248,113,113,0.3); \
                                       border: 1px solid rgba(248,113,113,0.5); border-radius: 4px; \
                                       color: white; font-size: 10px; cursor: pointer;"
                            >
                                "Rename Node 4"
                            </button>
                        </div>
                    </div>

                    // Edge Updates
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"EDGE STATE"</div>
                        <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                            {move || {
                                store.edges().iter_unkeyed()
                                    .enumerate()
                                    .map(|(idx, edge)| {
                                        let label = edge.data().label().get();
                                        let animated = edge.data().animated().get();

                                        let toggle_animated = {
                                            let toggle_edge_animated = toggle_edge_animated.clone();
                                            move |_| toggle_edge_animated(idx)
                                        };

                                        view! {
                                            <button
                                                on:click=toggle_animated
                                                style=format!("padding: 4px 8px; border-radius: 4px; \
                                                              border: 1px solid rgba(255,255,255,0.3); \
                                                              color: white; font-size: 9px; cursor: pointer; \
                                                              background: {};",
                                                              if animated { "rgba(129,140,248,0.4)" } else { "rgba(255,255,255,0.1)" })
                                            >
                                                {label}
                                                <span style="margin-left: 4px; opacity: 0.7;">
                                                    {if animated { "⚡" } else { "—" }}
                                                </span>
                                            </button>
                                        }
                                    }).collect_view()
                            }}
                        </div>
                    </div>

                    // Viewport Control
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px; margin-bottom: 12px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; opacity: 0.9;">"VIEWPORT STATE"</div>
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                            <span style="font-size: 10px; opacity: 0.8;">"Zoom: "{move || format!("{:.1}x", store.viewport().zoom().get())}</span>
                            <div style="display: flex; gap: 4px;">
                                {[0.5, 1.0, 1.5, 2.0].into_iter().map(|z| {
                                    let update_zoom = update_zoom.clone();
                                    view! {
                                        <button
                                            on:click=move |_| update_zoom(z)
                                            style="padding: 4px 8px; background: rgba(255,255,255,0.15); \
                                                   border: 1px solid rgba(255,255,255,0.2); border-radius: 4px; \
                                                   color: white; font-size: 9px; cursor: pointer;"
                                        >
                                            {format!("{}x", z)}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    </div>

                    // Action log
                    <div style="background: rgba(0,0,0,0.2); padding: 10px 12px; border-radius: 8px;">
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px; opacity: 0.9;">"ACTION LOG"</div>
                        <div style="max-height: 100px; overflow-y: auto; font-size: 9px; font-family: monospace;">
                            {move || {
                                let log = action_log.get();
                                if log.is_empty() {
                                    view! {
                                        <div style="color: rgba(255,255,255,0.5); font-style: italic;">
                                            "Interact with the store..."
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

                // Info badge
                <div style="position: absolute; bottom: 60px; left: 16px; \
                            background: rgba(102, 126, 234, 0.9); color: white; \
                            padding: 8px 12px; border-radius: 8px; font-size: 11px; \
                            max-width: 220px; line-height: 1.4;">
                    <div style="font-weight: 600; margin-bottom: 4px;">"#[derive(Store)] Features"</div>
                    <div>"• Fine-grained field reactivity"</div>
                    <div>"• Nested state updates"</div>
                    <div>"• Siblings don't trigger rerenders"</div>
                    <div>"• Type-safe field access"</div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Node Component
// ============================================================================

/// Node component for reactive stores example
#[component]
fn ReactiveStoresNode(
    node: Node,
    flow_store: FlowStore,
    reactive_store: Store<ReactiveFlowState>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let drag_signal = get_reactive_stores_drag_signal();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("nodeType")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#818cf8")
        .to_string();

    // Get reactive label from store
    let node_idx = match node_id.as_str() {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        "4" => 3,
        _ => 0,
    };

    // Mouse handlers
    let on_mousedown = {
        let node_id = node_id.clone();
        let flow_store = flow_store;
        let reactive_store = reactive_store.clone();
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

                // Mark node as dragging in flow store
                flow_store.update_node(&node_id, |n| {
                    n.dragging = true;
                });

                // Mark node as dragging in reactive store
                reactive_store.nodes().iter_unkeyed().nth(node_idx).map(|n| {
                    n.dragging().set(true);
                });
            }
        }
    };

    // Get reactive node position from flow store
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

    // Get reactive label from reactive store
    let reactive_label = {
        let reactive_store = reactive_store.clone();
        move || {
            reactive_store.nodes().iter_unkeyed().nth(node_idx)
                .map(|n| n.data().label().get())
                .unwrap_or(label.clone())
        }
    };

    // Get active state from reactive store
    let reactive_active = {
        let reactive_store = reactive_store.clone();
        move || {
            reactive_store.nodes().iter_unkeyed().nth(node_idx)
                .map(|n| n.data().active().get())
                .unwrap_or(true)
        }
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab; \
                 background: {}; border: 2px solid {}; border-radius: 8px; \
                 padding: 12px 16px; min-width: 120px; text-align: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.15); transition: opacity 0.2s ease; \
                 opacity: {};",
                pos().x, pos().y, color,
                if reactive_active() { &color } else { "#666" },
                if reactive_active() { "1" } else { "0.6" }
            )
            on:mousedown=on_mousedown
        >
            // Node label
            <div style="color: white; font-weight: 600; font-size: 13px; text-shadow: 0 1px 2px rgba(0,0,0,0.2);">
                {reactive_label}
            </div>

            // Active indicator
            <div style="position: absolute; top: 4px; right: 4px;">
                {move || if reactive_active() {
                    view! { <span style="font-size: 10px;">"✓"</span> }.into_any()
                } else {
                    view! { <span style="font-size: 10px; opacity: 0.5;">"○"</span> }.into_any()
                }}
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

/// Edge renderer for reactive stores example
#[component]
fn ReactiveStoresEdgeRenderer(
    flow_store: FlowStore,
    reactive_store: Store<ReactiveFlowState>,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                // Gradient for edges
                <linearGradient id="reactive-stores-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #667eea; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #764ba2; stop-opacity: 1" />
                </linearGradient>

                // Animated gradient
                <linearGradient id="reactive-stores-edge-animated" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #a855f7; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #ec4899; stop-opacity: 1" />
                </linearGradient>

                // Arrow marker
                <marker
                    id="reactive-stores-arrow"
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
                let nodes = flow_store.get_nodes();
                let edges = flow_store.get_edges();

                edges.into_iter().enumerate().filter_map(|(edge_idx, edge)| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source)?;
                    let target_node = nodes.iter().find(|n| n.id == edge.target)?;

                    // Get animated state from reactive store
                    let animated = reactive_store.edges().iter_unkeyed().nth(edge_idx)
                        .map(|e| e.data().animated().get())
                        .unwrap_or(false);

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
                                stroke=if animated { "rgba(168, 85, 247, 0.3)" } else { "rgba(102, 126, 234, 0.3)" }
                                stroke-width="6"
                                stroke-linecap="round"
                            />

                            // Main edge
                            <path
                                d=path.clone()
                                fill="none"
                                stroke=if animated { "url(#reactive-stores-edge-animated)" } else { "url(#reactive-stores-edge-gradient)" }
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-dasharray=if animated { "8 4" } else { "none" }
                                marker-end="url(#reactive-stores-arrow)"
                            >
                                {animated.then(|| view! {
                                    <animate
                                        attributeName="stroke-dashoffset"
                                        values="0;24"
                                        dur="0.5s"
                                        repeatCount="indefinite"
                                    />
                                })}
                            </path>

                            // Edge label
                            {(!label.is_empty()).then(|| view! {
                                <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                    <rect
                                        x="-30"
                                        y="-10"
                                        width="60"
                                        height="20"
                                        rx="4"
                                        fill="white"
                                        stroke=if animated { "#a855f7" } else { "#667eea" }
                                        stroke-width="1"
                                    />
                                    <text
                                        x="0"
                                        y="4"
                                        text-anchor="middle"
                                        font-size="10"
                                        font-weight="500"
                                        fill=if animated { "#a855f7" } else { "#667eea" }
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
