//! Edge Renderer Example
//!
//! Demonstrates how to customize the overall edge rendering layer:
//! - Custom edge layer rendering
//! - Edge z-index control (bring to front/send to back)
//! - Edge grouping by category
//! - Multiple edge layers with different rendering styles

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DraggableNode};

/// Edge category for grouping
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EdgeCategory {
    /// Primary flow edges (rendered above)
    Primary,
    /// Secondary/optional edges (rendered below)
    Secondary,
    /// Reference/info edges (rendered at bottom with dashed style)
    Reference,
}

impl EdgeCategory {
    fn from_str(s: &str) -> Self {
        match s {
            "primary" => Self::Primary,
            "secondary" => Self::Secondary,
            "reference" => Self::Reference,
            _ => Self::Primary,
        }
    }

    fn z_index(&self) -> i32 {
        match self {
            Self::Primary => 3,
            Self::Secondary => 2,
            Self::Reference => 1,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Secondary => "Secondary",
            Self::Reference => "Reference",
        }
    }

    fn color(&self) -> &'static str {
        match self {
            Self::Primary => "#667eea",
            Self::Secondary => "#43e97b",
            Self::Reference => "#ff9a9e",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Primary => "Main flow (top layer)",
            Self::Secondary => "Alternative paths (middle)",
            Self::Reference => "Info links (bottom, dashed)",
        }
    }
}

/// Edge Renderer example
#[component]
pub fn EdgeRendererExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("start".to_string(), Position::new(50.0, 150.0))
            .with_data(json!({"label": "Start", "type": "input", "class": "light"})),
        Node::new("process-a".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({"label": "Process A", "type": "default", "class": "light"})),
        Node::new("process-b".to_string(), Position::new(250.0, 150.0))
            .with_data(json!({"label": "Process B", "type": "default", "class": "light"})),
        Node::new("process-c".to_string(), Position::new(250.0, 250.0))
            .with_data(json!({"label": "Process C", "type": "default", "class": "light"})),
        Node::new("end".to_string(), Position::new(450.0, 150.0))
            .with_data(json!({"label": "End", "type": "output", "class": "light"})),
    ];

    // Create initial edges with categories and z-index
    let initial_edges = vec![
        // Primary edges (main flow)
        Edge::new("e-start-b".to_string(), "start".to_string(), "process-b".to_string())
            .with_label("Main Flow".to_string())
            .with_data(json!({"category": "primary", "zIndex": 10})),
        Edge::new("e-b-end".to_string(), "process-b".to_string(), "end".to_string())
            .with_label("Main Flow".to_string())
            .with_data(json!({"category": "primary", "zIndex": 10})),

        // Secondary edges (alternative paths)
        Edge::new("e-start-a".to_string(), "start".to_string(), "process-a".to_string())
            .with_label("Alt Path".to_string())
            .with_data(json!({"category": "secondary", "zIndex": 5})),
        Edge::new("e-a-end".to_string(), "process-a".to_string(), "end".to_string())
            .with_label("Alt Path".to_string())
            .with_data(json!({"category": "secondary", "zIndex": 5})),

        // Reference edges (info links)
        Edge::new("e-start-c".to_string(), "start".to_string(), "process-c".to_string())
            .with_label("Ref".to_string())
            .with_data(json!({"category": "reference", "zIndex": 1})),
        Edge::new("e-c-end".to_string(), "process-c".to_string(), "end".to_string())
            .with_label("Ref".to_string())
            .with_data(json!({"category": "reference", "zIndex": 1})),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components
    provide_context(store);

    // Selected edge for z-index control
    let selected_edge_id = RwSignal::new(Option::<String>::None);

    // Show/hide categories
    let show_primary = RwSignal::new(true);
    let show_secondary = RwSignal::new(true);
    let show_reference = RwSignal::new(true);

    // Action log
    let action_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        action_log.update(|log| {
            log.insert(0, msg);
            if log.len() > 8 {
                log.pop();
            }
        });
    };

    // Global drag handlers
    let drag_signal = get_drag_signal();

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

    // Handle background click to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        selected_edge_id.set(None);
        store.clear_node_selection();
        store.clear_edge_selection();
    };

    // Z-index controls
    let add_log_front = add_log.clone();
    let bring_to_front = move |_| {
        if let Some(edge_id) = selected_edge_id.get() {
            // Find the maximum z-index and set this edge above it
            let edges = store.get_edges();
            let max_z = edges.iter()
                .filter_map(|e| e.data.get("zIndex").and_then(|v| v.as_i64()))
                .max()
                .unwrap_or(0) as i32;

            store.state.edges.update(|edges| {
                if let Some(edge) = edges.iter_mut().find(|e| e.id == edge_id) {
                    if let Some(obj) = edge.data.as_object_mut() {
                        obj.insert("zIndex".to_string(), json!(max_z + 1));
                    }
                }
            });
            add_log_front(format!("Brought {} to front (z={})", edge_id, max_z + 1));
        }
    };

    let add_log_back = add_log.clone();
    let send_to_back = move |_| {
        if let Some(edge_id) = selected_edge_id.get() {
            // Find the minimum z-index and set this edge below it
            let edges = store.get_edges();
            let min_z = edges.iter()
                .filter_map(|e| e.data.get("zIndex").and_then(|v| v.as_i64()))
                .min()
                .unwrap_or(0) as i32;

            store.state.edges.update(|edges| {
                if let Some(edge) = edges.iter_mut().find(|e| e.id == edge_id) {
                    if let Some(obj) = edge.data.as_object_mut() {
                        obj.insert("zIndex".to_string(), json!(min_z - 1));
                    }
                }
            });
            add_log_back(format!("Sent {} to back (z={})", edge_id, min_z - 1));
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
                 on:click=on_background_click
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Multi-layer edge renderer with z-index ordering
                    <LayeredEdgeRenderer
                        store=store
                        selected_edge_id=selected_edge_id
                        show_primary=show_primary
                        show_secondary=show_secondary
                        show_reference=show_reference
                        add_log=add_log.clone()
                    />

                    // Connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            view! {
                                <DraggableNode
                                    node=node.clone()
                                    store=store
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
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 220px;">
                        <strong style="display: block; margin-bottom: 8px;">"Edge Renderer"</strong>
                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666;">
                            "Custom edge layers with z-index control"
                        </p>

                        // Category visibility toggles
                        <div style="margin-bottom: 12px; padding: 8px; background: #f8f9fa; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px;">"Edge Layers:"</div>
                            {[EdgeCategory::Primary, EdgeCategory::Secondary, EdgeCategory::Reference].into_iter().map(|cat| {
                                let (signal, _) = match cat {
                                    EdgeCategory::Primary => (show_primary, show_primary),
                                    EdgeCategory::Secondary => (show_secondary, show_secondary),
                                    EdgeCategory::Reference => (show_reference, show_reference),
                                };
                                let cat_copy = cat;
                                view! {
                                    <label style="display: flex; align-items: center; gap: 6px; margin-bottom: 4px; font-size: 11px; cursor: pointer;">
                                        <input
                                            type="checkbox"
                                            checked=move || signal.get()
                                            on:change=move |ev| {
                                                use leptos::wasm_bindgen::JsCast;
                                                let target = ev.target().unwrap();
                                                let input = target.dyn_into::<leptos::web_sys::HtmlInputElement>().unwrap();
                                                signal.set(input.checked());
                                            }
                                        />
                                        <span style=format!("width: 12px; height: 2px; background: {};", cat.color())></span>
                                        <span>{cat.label()}</span>
                                        <span style="color: #999; font-size: 10px;">"(z="{{cat_copy.z_index()}}")"</span>
                                    </label>
                                }
                            }).collect_view()}
                        </div>

                        // Z-index controls
                        <div style="margin-bottom: 12px; padding: 8px; background: #e3f2fd; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Z-Index Control:"</div>
                            {move || {
                                if let Some(edge_id) = selected_edge_id.get() {
                                    let edges = store.get_edges();
                                    let z = edges.iter()
                                        .find(|e| e.id == edge_id)
                                        .and_then(|e| e.data.get("zIndex").and_then(|v| v.as_i64()))
                                        .unwrap_or(0);
                                    view! {
                                        <div>
                                            <div style="font-size: 10px; margin-bottom: 4px;">
                                                "Selected: "{edge_id}" (z="{{z}}")"
                                            </div>
                                            <div style="display: flex; gap: 4px;">
                                                <button
                                                    style="flex: 1; padding: 4px 8px; font-size: 10px; background: #667eea; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                                    on:click=bring_to_front.clone()
                                                >
                                                    "↑ Front"
                                                </button>
                                                <button
                                                    style="flex: 1; padding: 4px 8px; font-size: 10px; background: #ff6b6b; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                                    on:click=send_to_back.clone()
                                                >
                                                    "↓ Back"
                                                </button>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div style="font-size: 10px; color: #999;">"Click an edge to select"</div>
                                    }.into_any()
                                }
                            }}
                        </div>

                        // Layer info
                        <div style="margin-bottom: 12px; padding: 8px; background: #f5f5f5; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Layer System:"</div>
                            <div style="font-size: 10px; color: #666; line-height: 1.4;">
                                "Edges are grouped into layers. Higher z-index edges render on top. Each layer can be toggled independently."
                            </div>
                        </div>

                        // Action log
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"Action Log:"</div>
                        <div style="max-height: 80px; overflow-y: auto; font-size: 10px; color: #666;">
                            {move || {
                                let log = action_log.get();
                                if log.is_empty() {
                                    view! { <div style="color: #999;">"No actions yet"</div> }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        view! { <div style="padding: 2px 0; border-bottom: 1px solid #eee;">{entry.clone()}</div> }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Get handle position for edge endpoints
fn get_handle_position(node: &Node, handle_id: &Option<String>, is_source: bool) -> Position {
    let node_pos = &node.position;
    let node_width = node.width.unwrap_or(150.0);
    let node_height = node.height.unwrap_or(40.0);

    if let Some(ref bounds) = node.internals.handle_bounds {
        let handles = if is_source { &bounds.source } else { &bounds.target };

        let handle = if let Some(id) = handle_id {
            handles.iter().find(|h| h.id.as_ref() == Some(id))
        } else {
            handles.first()
        };

        if let Some(handle) = handle {
            return handle.center_absolute(node_pos);
        }
    }

    if is_source {
        Position::new(node_pos.x + node_width / 2.0, node_pos.y + node_height)
    } else {
        Position::new(node_pos.x + node_width / 2.0, node_pos.y)
    }
}

/// Generate bezier path
fn generate_bezier_path(from: Position, to: Position) -> String {
    let mid_x = (from.x + to.x) / 2.0;
    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y, mid_x, from.y, mid_x, to.y, to.x, to.y
    )
}

/// Calculate label position
fn calculate_label_position(from: Position, to: Position) -> Position {
    Position::new(
        (from.x + to.x) / 2.0,
        (from.y + to.y) / 2.0,
    )
}

/// Layered edge renderer with z-index ordering
#[component]
fn LayeredEdgeRenderer<F>(
    store: FlowStore,
    selected_edge_id: RwSignal<Option<String>>,
    show_primary: RwSignal<bool>,
    show_secondary: RwSignal<bool>,
    show_reference: RwSignal<bool>,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    view! {
        <svg class="xyflow__edges leptos-flow__edges" style="position: absolute; width: 100%; height: 100%; pointer-events: none;">
            // SVG definitions
            <defs>
                // Gradients for each category
                <linearGradient id="edge-gradient-primary" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <linearGradient id="edge-gradient-secondary" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#43e97b;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#38f9d7;stop-opacity:1" />
                </linearGradient>
                <linearGradient id="edge-gradient-reference" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#ff9a9e;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#fecfef;stop-opacity:1" />
                </linearGradient>

                // Arrow markers
                <marker id="edge-arrow-primary" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#764ba2" />
                </marker>
                <marker id="edge-arrow-secondary" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#38f9d7" />
                </marker>
                <marker id="edge-arrow-reference" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#fecfef" />
                </marker>
            </defs>

            // Render edges in three layers by z-index
            // Layer 1: Reference edges (lowest)
            {
                let add_log_ref = add_log.clone();
                move || {
                    if !show_reference.get() {
                        return view! { <g /> }.into_any();
                    }

                    let edges = store.get_edges();
                    let mut ref_edges: Vec<_> = edges.into_iter()
                        .filter(|e| {
                            e.data.get("category")
                                .and_then(|v| v.as_str())
                                .unwrap_or("primary") == "reference"
                        })
                        .collect();
                    ref_edges.sort_by_key(|e| {
                        e.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0)
                    });

                    view! {
                        <g class="edge-layer-reference">
                            {ref_edges.into_iter().map(|edge| {
                                let add_log = add_log_ref.clone();
                                view! {
                                    <LayeredEdgeComponent
                                        edge=edge.clone()
                                        store=store
                                        selected_edge_id=selected_edge_id
                                        add_log=add_log
                                    />
                                }
                            }).collect_view()}
                        </g>
                    }.into_any()
                }
            }

            // Layer 2: Secondary edges (middle)
            {
                let add_log_sec = add_log.clone();
                move || {
                    if !show_secondary.get() {
                        return view! { <g /> }.into_any();
                    }

                    let edges = store.get_edges();
                    let mut sec_edges: Vec<_> = edges.into_iter()
                        .filter(|e| {
                            e.data.get("category")
                                .and_then(|v| v.as_str())
                                .unwrap_or("primary") == "secondary"
                        })
                        .collect();
                    sec_edges.sort_by_key(|e| {
                        e.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0)
                    });

                    view! {
                        <g class="edge-layer-secondary">
                            {sec_edges.into_iter().map(|edge| {
                                let add_log = add_log_sec.clone();
                                view! {
                                    <LayeredEdgeComponent
                                        edge=edge.clone()
                                        store=store
                                        selected_edge_id=selected_edge_id
                                        add_log=add_log
                                    />
                                }
                            }).collect_view()}
                        </g>
                    }.into_any()
                }
            }

            // Layer 3: Primary edges (top)
            {
                let add_log_prim = add_log.clone();
                move || {
                    if !show_primary.get() {
                        return view! { <g /> }.into_any();
                    }

                    let edges = store.get_edges();
                    let mut prim_edges: Vec<_> = edges.into_iter()
                        .filter(|e| {
                            e.data.get("category")
                                .and_then(|v| v.as_str())
                                .unwrap_or("primary") == "primary"
                        })
                        .collect();
                    prim_edges.sort_by_key(|e| {
                        e.data.get("zIndex").and_then(|v| v.as_i64()).unwrap_or(0)
                    });

                    view! {
                        <g class="edge-layer-primary">
                            {prim_edges.into_iter().map(|edge| {
                                let add_log = add_log_prim.clone();
                                view! {
                                    <LayeredEdgeComponent
                                        edge=edge.clone()
                                        store=store
                                        selected_edge_id=selected_edge_id
                                        add_log=add_log
                                    />
                                }
                            }).collect_view()}
                        </g>
                    }.into_any()
                }
            }
        </svg>
    }
}

/// Single edge component with layer-aware styling
#[component]
fn LayeredEdgeComponent<F>(
    edge: Edge,
    store: FlowStore,
    selected_edge_id: RwSignal<Option<String>>,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    let edge_id = edge.id.clone();
    let source_id = edge.source.clone();
    let target_id = edge.target.clone();
    let source_handle = edge.source_handle.clone();
    let target_handle = edge.target_handle.clone();
    let label = edge.label.clone();

    let category = EdgeCategory::from_str(
        edge.data.get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("primary")
    );

    // Calculate path reactively
    let path_data = Memo::new({
        let store = store.clone();
        let source_id = source_id.clone();
        let target_id = target_id.clone();
        let source_handle = source_handle.clone();
        let target_handle = target_handle.clone();
        move |_| {
            let nodes = store.get_nodes();
            let source = nodes.iter().find(|n| n.id == source_id);
            let target = nodes.iter().find(|n| n.id == target_id);

            if let (Some(source), Some(target)) = (source, target) {
                let source_pos = get_handle_position(source, &source_handle, true);
                let target_pos = get_handle_position(target, &target_handle, false);
                let path = generate_bezier_path(source_pos, target_pos);
                let label_pos = calculate_label_position(source_pos, target_pos);
                (path, label_pos.x, label_pos.y)
            } else {
                (String::new(), 0.0, 0.0)
            }
        }
    });

    // Get gradient and marker based on category
    let (gradient_id, marker_id, stroke_style) = match category {
        EdgeCategory::Primary => ("url(#edge-gradient-primary)", "url(#edge-arrow-primary)", ""),
        EdgeCategory::Secondary => ("url(#edge-gradient-secondary)", "url(#edge-arrow-secondary)", ""),
        EdgeCategory::Reference => ("url(#edge-gradient-reference)", "url(#edge-arrow-reference)", "8,4"),
    };

    // Click handler
    let edge_id_click = edge_id.clone();
    let add_log_click = add_log.clone();
    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        selected_edge_id.set(Some(edge_id_click.clone()));
        add_log_click(format!("Selected: {}", edge_id_click));
    };

    // Selection state
    let edge_id_for_selected = edge_id.clone();

    view! {
        <g class="edge-component" data-id=edge_id.clone()>
            // Hitbox for clicking
            <path
                class="edge-hitbox"
                d=move || path_data.get().0
                fill="none"
                stroke="transparent"
                stroke-width="20"
                style="pointer-events: stroke; cursor: pointer;"
                on:click=on_click.clone()
            />

            // Selection highlight
            {move || {
                if selected_edge_id.get() == Some(edge_id_for_selected.clone()) {
                    let color = match category {
                        EdgeCategory::Primary => "#667eea",
                        EdgeCategory::Secondary => "#43e97b",
                        EdgeCategory::Reference => "#ff9a9e",
                    };
                    Some(view! {
                        <path
                            class="edge-selection"
                            d=move || path_data.get().0
                            fill="none"
                            stroke=color
                            stroke-width="6"
                            stroke-opacity="0.3"
                            stroke-linecap="round"
                        />
                    })
                } else {
                    None
                }
            }}

            // Main edge path
            <path
                class="edge-path"
                d=move || path_data.get().0
                fill="none"
                stroke=gradient_id.to_string()
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-dasharray=stroke_style
                attr:marker-end=marker_id.to_string()
                style="pointer-events: none;"
            />

            // Edge label
            {move || {
                let (_, label_x, label_y) = path_data.get();
                label.clone().map(|text| {
                    let color = match category {
                        EdgeCategory::Primary => "#667eea",
                        EdgeCategory::Secondary => "#43e97b",
                        EdgeCategory::Reference => "#ff9a9e",
                    };
                    view! {
                        <g transform=format!("translate({}, {})", label_x, label_y) style="pointer-events: all;">
                            <rect
                                x="-35"
                                y="-10"
                                width="70"
                                height="20"
                                rx="10"
                                fill="white"
                                stroke=color
                                stroke-width="1"
                            />
                            <text
                                text-anchor="middle"
                                dominant-baseline="middle"
                                font-size="10"
                                font-weight="500"
                                fill=color
                                style="user-select: none;"
                            >
                                {text}
                            </text>
                        </g>
                    }
                })
            }}
        </g>
    }
}
