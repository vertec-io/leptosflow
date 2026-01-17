//! UseNodesData Example
//!
//! Demonstrates how to reactively access node data.
//! Shows reactive node data access, displaying data in sidebar that updates when nodes change,
//! and demonstrates data transformation/mapping.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for use_nodes_data example
static USE_NODES_DATA_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> =
    std::sync::OnceLock::new();

/// Get or initialize the drag state signal
fn get_use_nodes_data_drag_signal() -> RwSignal<Option<DragState>> {
    *USE_NODES_DATA_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Node category for grouping
#[derive(Clone, Debug, PartialEq)]
enum NodeCategory {
    Source,
    Process,
    Output,
}

impl NodeCategory {
    fn label(&self) -> &'static str {
        match self {
            NodeCategory::Source => "Source",
            NodeCategory::Process => "Process",
            NodeCategory::Output => "Output",
        }
    }

    fn color(&self) -> &'static str {
        match self {
            NodeCategory::Source => "#22c55e",
            NodeCategory::Process => "#6366f1",
            NodeCategory::Output => "#f59e0b",
        }
    }
}

/// Transformed node data for display
#[derive(Clone, Debug)]
struct NodeDataView {
    id: String,
    label: String,
    category: NodeCategory,
    priority: i32,
    status: String,
    x: f64,
    y: f64,
}

/// UseNodesData example component
#[component]
pub fn UseNodesDataExample() -> impl IntoView {
    // Create initial nodes with rich data
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Data Source A",
                "category": "source",
                "priority": 1,
                "status": "active"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("2".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({
                "label": "Data Source B",
                "category": "source",
                "priority": 2,
                "status": "active"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("3".to_string(), Position::new(150.0, 150.0))
            .with_data(json!({
                "label": "Transform",
                "category": "process",
                "priority": 3,
                "status": "running"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("4".to_string(), Position::new(50.0, 250.0))
            .with_data(json!({
                "label": "Aggregate",
                "category": "process",
                "priority": 4,
                "status": "idle"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("5".to_string(), Position::new(250.0, 250.0))
            .with_data(json!({
                "label": "Filter",
                "category": "process",
                "priority": 5,
                "status": "running"
            }))
            .with_dimensions(140.0, 50.0),
        Node::new("6".to_string(), Position::new(150.0, 350.0))
            .with_data(json!({
                "label": "Output Sink",
                "category": "output",
                "priority": 6,
                "status": "waiting"
            }))
            .with_dimensions(140.0, 50.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
        Edge::new("e3-5".to_string(), "3".to_string(), "5".to_string()),
        Edge::new("e4-6".to_string(), "4".to_string(), "6".to_string()),
        Edge::new("e5-6".to_string(), "5".to_string(), "6".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes.clone(), initial_edges.clone());

    // Provide the store to child components via context
    provide_context(store);

    // Selected node for detailed view
    let selected_node_id = RwSignal::new(Option::<String>::None);

    // Filter options
    let filter_category = RwSignal::new(Option::<String>::None);
    let sort_by = RwSignal::new("priority".to_string());

    // Transformed and filtered node data - this is the key reactive data access
    let transformed_nodes = move || {
        let nodes = store.get_nodes();
        let filter = filter_category.get();
        let sort = sort_by.get();

        // Transform raw nodes into view model
        let mut node_views: Vec<NodeDataView> = nodes
            .iter()
            .filter_map(|node| {
                let label = node.data.get("label")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let cat_str = node.data.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("process");

                let category = match cat_str {
                    "source" => NodeCategory::Source,
                    "output" => NodeCategory::Output,
                    _ => NodeCategory::Process,
                };

                // Apply filter
                if let Some(ref filter_val) = filter {
                    if cat_str != filter_val.as_str() {
                        return None;
                    }
                }

                let priority = node.data.get("priority")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;

                let status = node.data.get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                Some(NodeDataView {
                    id: node.id.clone(),
                    label,
                    category,
                    priority,
                    status,
                    x: node.position.x,
                    y: node.position.y,
                })
            })
            .collect();

        // Sort based on selected criteria
        match sort.as_str() {
            "priority" => node_views.sort_by(|a, b| a.priority.cmp(&b.priority)),
            "name" => node_views.sort_by(|a, b| a.label.cmp(&b.label)),
            "position" => node_views.sort_by(|a, b| {
                let a_dist = (a.x * a.x + a.y * a.y).sqrt();
                let b_dist = (b.x * b.x + b.y * b.y).sqrt();
                a_dist.partial_cmp(&b_dist).unwrap_or(std::cmp::Ordering::Equal)
            }),
            _ => {}
        }

        node_views
    };

    // Aggregate statistics - derived data
    let stats = move || {
        let nodes = store.get_nodes();

        let source_count = nodes.iter().filter(|n| {
            n.data.get("category").and_then(|v| v.as_str()) == Some("source")
        }).count();

        let process_count = nodes.iter().filter(|n| {
            n.data.get("category").and_then(|v| v.as_str()) == Some("process")
        }).count();

        let output_count = nodes.iter().filter(|n| {
            n.data.get("category").and_then(|v| v.as_str()) == Some("output")
        }).count();

        let active_count = nodes.iter().filter(|n| {
            n.data.get("status").and_then(|v| v.as_str()) == Some("active") ||
            n.data.get("status").and_then(|v| v.as_str()) == Some("running")
        }).count();

        let avg_x = if nodes.is_empty() { 0.0 } else {
            nodes.iter().map(|n| n.position.x).sum::<f64>() / nodes.len() as f64
        };

        let avg_y = if nodes.is_empty() { 0.0 } else {
            nodes.iter().map(|n| n.position.y).sum::<f64>() / nodes.len() as f64
        };

        (source_count, process_count, output_count, active_count, avg_x, avg_y)
    };

    // Update node status
    let update_node_status = move |node_id: String, new_status: &'static str| {
        store.update_node(&node_id, |n| {
            if let Some(obj) = n.data.as_object_mut() {
                obj.insert("status".to_string(), json!(new_status));
            }
        });
    };

    // Update node priority
    let update_node_priority = move |node_id: String, delta: i32| {
        store.update_node(&node_id, |n| {
            let current = n.data.get("priority")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let new_priority = (current + delta).max(1).min(10);
            if let Some(obj) = n.data.as_object_mut() {
                obj.insert("priority".to_string(), json!(new_priority));
            }
        });
    };

    // Global drag handlers
    let drag_signal = get_use_nodes_data_drag_signal();

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

    // Click on background to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        selected_node_id.set(None);
    };

    view! {
        <div class="example-container" style="display: flex; width: 100%; height: 100%;">
            // Main flow area
            <div style="flex: 1; position: relative;">
                <div
                    class="xyflow leptos-flow svelte-flow"
                    style="width: 100%; height: 100%; position: relative;"
                    on:mousemove=on_global_mousemove
                    on:mouseup=on_global_mouseup
                    on:click=on_background_click
                >
                    // Background
                    <Background variant=BackgroundVariant::Dots />

                    // Main flow container with pan/zoom
                    <FlowViewport store=store>
                        // Edge renderer
                        <UseNodesDataEdgeRenderer store=store />

                        // Render connection line while dragging
                        <ConnectionLine />

                        // Render nodes
                        {move || {
                            let sel_id = selected_node_id.get();
                            store.get_nodes().into_iter()
                                .map(|node| {
                                    let is_selected = sel_id.as_ref() == Some(&node.id);
                                    view! {
                                        <UseNodesDataNode
                                            node=node.clone()
                                            store=store
                                            is_selected=is_selected
                                            on_select=move |id: String| selected_node_id.set(Some(id))
                                        />
                                    }
                                }).collect_view()
                        }}
                    </FlowViewport>

                    // Controls (zoom buttons)
                    <Controls position=PanelPosition::BottomLeft />

                    // Header badge
                    <div style="position: absolute; top: 16px; left: 16px; \
                                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); \
                                color: white; padding: 10px 16px; border-radius: 10px; \
                                font-size: 14px; font-weight: 600; \
                                box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);">
                        <span style="margin-right: 8px;">"📊"</span>
                        "useNodesData Hook"
                    </div>

                    // Instructions
                    <div style="position: absolute; bottom: 60px; left: 16px; \
                                background: rgba(102, 126, 234, 0.9); color: white; \
                                padding: 8px 12px; border-radius: 8px; font-size: 11px; \
                                max-width: 200px; line-height: 1.4;">
                        <div style="font-weight: 600; margin-bottom: 4px;">"💡 How it works"</div>
                        <div>"• Click nodes to select"</div>
                        <div>"• Drag nodes to update position"</div>
                        <div>"• Watch sidebar update reactively"</div>
                        <div>"• Use filters and sorting"</div>
                    </div>
                </div>
            </div>

            // Sidebar - reactive data display
            <div style="width: 340px; background: #1a1b26; color: #a9b1d6; \
                        padding: 16px; overflow-y: auto; font-family: system-ui, -apple-system, sans-serif; \
                        border-left: 1px solid #2f3349;">
                // Header
                <div style="font-size: 16px; font-weight: 600; color: #c0caf5; margin-bottom: 16px; \
                            display: flex; align-items: center; gap: 8px;">
                    <span>"📊"</span>
                    "Node Data View"
                </div>

                // Stats summary
                <div style="background: #24283b; border-radius: 8px; padding: 12px; margin-bottom: 16px;">
                    <div style="font-size: 11px; font-weight: 600; color: #7aa2f7; margin-bottom: 8px; text-transform: uppercase;">
                        "Aggregate Statistics"
                    </div>
                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 8px;">
                        <div style="text-align: center;">
                            <div style="font-size: 20px; font-weight: 700; color: #22c55e;">
                                {move || stats().0}
                            </div>
                            <div style="font-size: 10px; color: #565f89;">"Sources"</div>
                        </div>
                        <div style="text-align: center;">
                            <div style="font-size: 20px; font-weight: 700; color: #6366f1;">
                                {move || stats().1}
                            </div>
                            <div style="font-size: 10px; color: #565f89;">"Process"</div>
                        </div>
                        <div style="text-align: center;">
                            <div style="font-size: 20px; font-weight: 700; color: #f59e0b;">
                                {move || stats().2}
                            </div>
                            <div style="font-size: 10px; color: #565f89;">"Outputs"</div>
                        </div>
                    </div>
                    <div style="margin-top: 10px; padding-top: 10px; border-top: 1px solid #2f3349;">
                        <div style="display: flex; justify-content: space-between; font-size: 11px;">
                            <span style="color: #565f89;">"Active Nodes"</span>
                            <span style="color: #9ece6a; font-weight: 600;">
                                {move || format!("{}/{}", stats().3, store.get_nodes().len())}
                            </span>
                        </div>
                        <div style="display: flex; justify-content: space-between; font-size: 11px; margin-top: 4px;">
                            <span style="color: #565f89;">"Avg Position"</span>
                            <span style="color: #7aa2f7; font-weight: 500; font-family: monospace;">
                                {move || format!("({:.0}, {:.0})", stats().4, stats().5)}
                            </span>
                        </div>
                    </div>
                </div>

                // Filters
                <div style="background: #24283b; border-radius: 8px; padding: 12px; margin-bottom: 16px;">
                    <div style="font-size: 11px; font-weight: 600; color: #7aa2f7; margin-bottom: 8px; text-transform: uppercase;">
                        "Filter & Sort"
                    </div>
                    <div style="margin-bottom: 8px;">
                        <div style="font-size: 10px; color: #565f89; margin-bottom: 4px;">"Category Filter"</div>
                        <div style="display: flex; gap: 4px;">
                            <button
                                on:click=move |_| filter_category.set(None)
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if filter_category.get().is_none() { "#7aa2f7" } else { "#2f3349" },
                                    if filter_category.get().is_none() { "#7aa2f7" } else { "transparent" },
                                    if filter_category.get().is_none() { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "All"
                            </button>
                            <button
                                on:click=move |_| filter_category.set(Some("source".to_string()))
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if filter_category.get().as_deref() == Some("source") { "#22c55e" } else { "#2f3349" },
                                    if filter_category.get().as_deref() == Some("source") { "#22c55e" } else { "transparent" },
                                    if filter_category.get().as_deref() == Some("source") { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "Source"
                            </button>
                            <button
                                on:click=move |_| filter_category.set(Some("process".to_string()))
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if filter_category.get().as_deref() == Some("process") { "#6366f1" } else { "#2f3349" },
                                    if filter_category.get().as_deref() == Some("process") { "#6366f1" } else { "transparent" },
                                    if filter_category.get().as_deref() == Some("process") { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "Process"
                            </button>
                            <button
                                on:click=move |_| filter_category.set(Some("output".to_string()))
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if filter_category.get().as_deref() == Some("output") { "#f59e0b" } else { "#2f3349" },
                                    if filter_category.get().as_deref() == Some("output") { "#f59e0b" } else { "transparent" },
                                    if filter_category.get().as_deref() == Some("output") { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "Output"
                            </button>
                        </div>
                    </div>
                    <div>
                        <div style="font-size: 10px; color: #565f89; margin-bottom: 4px;">"Sort By"</div>
                        <div style="display: flex; gap: 4px;">
                            <button
                                on:click=move |_| sort_by.set("priority".to_string())
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if sort_by.get() == "priority" { "#bb9af7" } else { "#2f3349" },
                                    if sort_by.get() == "priority" { "#bb9af7" } else { "transparent" },
                                    if sort_by.get() == "priority" { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "Priority"
                            </button>
                            <button
                                on:click=move |_| sort_by.set("name".to_string())
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if sort_by.get() == "name" { "#bb9af7" } else { "#2f3349" },
                                    if sort_by.get() == "name" { "#bb9af7" } else { "transparent" },
                                    if sort_by.get() == "name" { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "Name"
                            </button>
                            <button
                                on:click=move |_| sort_by.set("position".to_string())
                                style=move || format!(
                                    "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                     border: 1px solid {}; background: {}; color: {};",
                                    if sort_by.get() == "position" { "#bb9af7" } else { "#2f3349" },
                                    if sort_by.get() == "position" { "#bb9af7" } else { "transparent" },
                                    if sort_by.get() == "position" { "#1a1b26" } else { "#a9b1d6" }
                                )
                            >
                                "Position"
                            </button>
                        </div>
                    </div>
                </div>

                // Node list
                <div style="background: #24283b; border-radius: 8px; padding: 12px; margin-bottom: 16px;">
                    <div style="font-size: 11px; font-weight: 600; color: #7aa2f7; margin-bottom: 8px; text-transform: uppercase; \
                                display: flex; justify-content: space-between; align-items: center;">
                        <span>"Transformed Node Data"</span>
                        <span style="color: #565f89; font-weight: 400;">
                            {move || format!("{} nodes", transformed_nodes().len())}
                        </span>
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 6px; max-height: 250px; overflow-y: auto;">
                        {move || {
                            let nodes = transformed_nodes();
                            let selected = selected_node_id.get();

                            if nodes.is_empty() {
                                view! {
                                    <div style="color: #565f89; font-style: italic; font-size: 11px; text-align: center; padding: 16px;">
                                        "No nodes match filter"
                                    </div>
                                }.into_any()
                            } else {
                                nodes.into_iter().map(|node| {
                                    let is_selected = selected.as_ref() == Some(&node.id);
                                    let node_id = node.id.clone();
                                    let node_id_for_click = node.id.clone();
                                    let color = node.category.color();

                                    view! {
                                        <div
                                            style=move || format!(
                                                "background: {}; border-radius: 6px; padding: 8px 10px; cursor: pointer; \
                                                 border-left: 3px solid {}; transition: all 0.2s ease;",
                                                if is_selected { "#2f3349" } else { "#1a1b26" },
                                                color
                                            )
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                selected_node_id.set(Some(node_id_for_click.clone()));
                                            }
                                        >
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
                                                <span style="font-weight: 600; color: #c0caf5; font-size: 12px;">
                                                    {node.label.clone()}
                                                </span>
                                                <span style=format!("font-size: 9px; padding: 2px 6px; border-radius: 4px; \
                                                                     background: {}22; color: {};", color, color)>
                                                    {node.category.label()}
                                                </span>
                                            </div>
                                            <div style="display: flex; gap: 12px; font-size: 10px; color: #565f89;">
                                                <span>"P: "{node.priority}</span>
                                                <span style=format!("color: {};",
                                                    match node.status.as_str() {
                                                        "active" | "running" => "#9ece6a",
                                                        "waiting" => "#e0af68",
                                                        _ => "#565f89"
                                                    }
                                                )>
                                                    {node.status.clone()}
                                                </span>
                                                <span style="font-family: monospace;">
                                                    {format!("({:.0}, {:.0})", node.x, node.y)}
                                                </span>
                                            </div>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </div>

                // Selected node details
                <div style="background: #24283b; border-radius: 8px; padding: 12px;">
                    <div style="font-size: 11px; font-weight: 600; color: #7aa2f7; margin-bottom: 8px; text-transform: uppercase;">
                        "Selected Node Details"
                    </div>
                    {move || {
                        let selected = selected_node_id.get();

                        if let Some(node_id) = selected {
                            let nodes = store.get_nodes();
                            if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                                let label = node.data.get("label")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("Unknown")
                                    .to_string();

                                let category = node.data.get("category")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("process")
                                    .to_string();

                                let priority = node.data.get("priority")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0) as i32;

                                let status = node.data.get("status")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown")
                                    .to_string();

                                let node_id_for_status = node_id.clone();
                                let node_id_for_priority_up = node_id.clone();
                                let node_id_for_priority_down = node_id.clone();

                                view! {
                                    <div>
                                        <div style="font-size: 14px; font-weight: 600; color: #c0caf5; margin-bottom: 8px;">
                                            {label}
                                        </div>

                                        // Properties grid
                                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px;">
                                            <div style="background: #1a1b26; padding: 8px; border-radius: 4px;">
                                                <div style="font-size: 9px; color: #565f89; text-transform: uppercase;">"ID"</div>
                                                <div style="font-size: 11px; font-family: monospace; color: #7aa2f7;">{node_id.clone()}</div>
                                            </div>
                                            <div style="background: #1a1b26; padding: 8px; border-radius: 4px;">
                                                <div style="font-size: 9px; color: #565f89; text-transform: uppercase;">"Category"</div>
                                                <div style="font-size: 11px; color: #bb9af7;">{category}</div>
                                            </div>
                                            <div style="background: #1a1b26; padding: 8px; border-radius: 4px;">
                                                <div style="font-size: 9px; color: #565f89; text-transform: uppercase;">"Position"</div>
                                                <div style="font-size: 11px; font-family: monospace; color: #9ece6a;">
                                                    {format!("({:.0}, {:.0})", node.position.x, node.position.y)}
                                                </div>
                                            </div>
                                            <div style="background: #1a1b26; padding: 8px; border-radius: 4px;">
                                                <div style="font-size: 9px; color: #565f89; text-transform: uppercase;">"Status"</div>
                                                <div style="font-size: 11px; color: #e0af68;">{status.clone()}</div>
                                            </div>
                                        </div>

                                        // Priority controls
                                        <div style="margin-bottom: 12px;">
                                            <div style="font-size: 10px; color: #565f89; margin-bottom: 4px;">"Priority"</div>
                                            <div style="display: flex; align-items: center; gap: 8px;">
                                                <button
                                                    on:click=move |_| update_node_priority(node_id_for_priority_down.clone(), -1)
                                                    style="width: 28px; height: 28px; border-radius: 4px; \
                                                           background: #2f3349; border: none; color: #a9b1d6; \
                                                           font-size: 16px; cursor: pointer;"
                                                >
                                                    "-"
                                                </button>
                                                <div style="font-size: 18px; font-weight: 700; color: #bb9af7; min-width: 24px; text-align: center;">
                                                    {priority}
                                                </div>
                                                <button
                                                    on:click=move |_| update_node_priority(node_id_for_priority_up.clone(), 1)
                                                    style="width: 28px; height: 28px; border-radius: 4px; \
                                                           background: #2f3349; border: none; color: #a9b1d6; \
                                                           font-size: 16px; cursor: pointer;"
                                                >
                                                    "+"
                                                </button>
                                            </div>
                                        </div>

                                        // Status controls
                                        <div>
                                            <div style="font-size: 10px; color: #565f89; margin-bottom: 4px;">"Change Status"</div>
                                            <div style="display: flex; gap: 4px; flex-wrap: wrap;">
                                                {["active", "running", "idle", "waiting"].into_iter().map(|s| {
                                                    let is_current = status == s;
                                                    let node_id_clone = node_id_for_status.clone();
                                                    view! {
                                                        <button
                                                            on:click=move |_| update_node_status(node_id_clone.clone(), s)
                                                            style=move || format!(
                                                                "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; \
                                                                 border: 1px solid {}; background: {}; color: {};",
                                                                if is_current { "#9ece6a" } else { "#2f3349" },
                                                                if is_current { "#9ece6a" } else { "transparent" },
                                                                if is_current { "#1a1b26" } else { "#a9b1d6" }
                                                            )
                                                        >
                                                            {s}
                                                        </button>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="color: #565f89; font-style: italic; font-size: 11px; text-align: center; padding: 16px;">
                                        "Node not found"
                                    </div>
                                }.into_any()
                            }
                        } else {
                            view! {
                                <div style="color: #565f89; font-style: italic; font-size: 11px; text-align: center; padding: 16px;">
                                    "Click a node to see details"
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Custom node component for UseNodesData example
#[component]
fn UseNodesDataNode<F>(
    node: Node,
    store: FlowStore,
    is_selected: bool,
    on_select: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_click = node.id.clone();
    let drag_signal = get_use_nodes_data_drag_signal();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();

    let category = node.data.get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("process")
        .to_string();

    let status = node.data.get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let priority = node.data.get("priority")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Get colors based on category
    let (bg_color, border_color) = match category.as_str() {
        "source" => ("#22c55e", "#16a34a"),
        "output" => ("#f59e0b", "#d97706"),
        _ => ("#6366f1", "#4f46e5"),
    };

    // Status indicator color
    let status_color = match status.as_str() {
        "active" | "running" => "#9ece6a",
        "waiting" => "#e0af68",
        _ => "#565f89",
    };

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Select this node
        on_select(node_id_for_click.clone());

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
                 padding: 10px 14px; min-width: 120px; text-align: center; \
                 box-shadow: {}; transition: box-shadow 0.2s ease;",
                pos().x, pos().y, bg_color, border_color,
                if is_selected {
                    format!("0 0 0 3px {}66, 0 4px 12px rgba(0,0,0,0.2)", border_color)
                } else {
                    "0 2px 8px rgba(0,0,0,0.15)".to_string()
                }
            )
            on:mousedown=on_mousedown
        >
            // Status indicator dot
            <div style=format!(
                "position: absolute; top: -4px; right: -4px; width: 10px; height: 10px; \
                 border-radius: 50%; background: {}; border: 2px solid #1a1b26;",
                status_color
            )></div>

            // Node label
            <div style="color: white; font-weight: 600; font-size: 12px; text-shadow: 0 1px 2px rgba(0,0,0,0.2);">
                {label}
            </div>

            // Priority badge
            <div style="position: absolute; bottom: -6px; left: 50%; transform: translateX(-50%); \
                        background: #1a1b26; color: #bb9af7; font-size: 9px; font-weight: 600; \
                        padding: 2px 6px; border-radius: 4px;">
                "P"{priority}
            </div>

            // Handles based on category
            {match category.as_str() {
                "source" => view! {
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

/// Edge renderer component for UseNodesData example
#[component]
fn UseNodesDataEdgeRenderer(
    store: FlowStore,
) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                // Gradient for edges
                <linearGradient id="use-nodes-data-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color: #6366f1; stop-opacity: 1" />
                    <stop offset="100%" style="stop-color: #8b5cf6; stop-opacity: 1" />
                </linearGradient>

                // Arrow marker
                <marker
                    id="use-nodes-data-arrow"
                    viewBox="0 0 10 10"
                    refX="10"
                    refY="5"
                    markerUnits="strokeWidth"
                    markerWidth="6"
                    markerHeight="6"
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

                    // Calculate edge positions
                    let source_x = source_node.position.x + source_node.width.unwrap_or(140.0) / 2.0;
                    let source_y = source_node.position.y + source_node.height.unwrap_or(50.0);
                    let target_x = target_node.position.x + target_node.width.unwrap_or(140.0) / 2.0;
                    let target_y = target_node.position.y;

                    // Generate bezier path
                    let control_offset = (target_y - source_y).abs() * 0.4;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        source_x, source_y,
                        source_x, source_y + control_offset,
                        target_x, target_y - control_offset,
                        target_x, target_y
                    );

                    Some(view! {
                        <g>
                            // Shadow/glow
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="rgba(99, 102, 241, 0.2)"
                                stroke-width="6"
                                stroke-linecap="round"
                            />

                            // Main edge
                            <path
                                d=path
                                fill="none"
                                stroke="url(#use-nodes-data-edge-gradient)"
                                stroke-width="2"
                                stroke-linecap="round"
                                marker-end="url(#use-nodes-data-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
