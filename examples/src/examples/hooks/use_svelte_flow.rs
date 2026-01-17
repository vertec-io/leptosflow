//! Use Svelte Flow Example
//!
//! Demonstrates the main FlowStore API for programmatic flow control.
//! Shows all available methods: fitView, zoomIn, zoomOut, setCenter,
//! getNodes, getEdges, setNodes, setEdges, and more.

use leptos::prelude::*;
use serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Drag state for UseSvelteFlow example
static USE_SVELTE_FLOW_DRAG_STATE: OnceLock<RwSignal<Option<DragState>>> = OnceLock::new();

fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *USE_SVELTE_FLOW_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Action log for tracking API calls
static USE_SVELTE_FLOW_ACTION_LOG: OnceLock<RwSignal<Vec<String>>> = OnceLock::new();

fn get_action_log() -> RwSignal<Vec<String>> {
    *USE_SVELTE_FLOW_ACTION_LOG.get_or_init(|| RwSignal::new(vec!["Example loaded".to_string()]))
}

fn log_action(action: &str) {
    get_action_log().update(|entries| {
        entries.push(action.to_string());
        if entries.len() > 15 {
            entries.remove(0);
        }
    });
}

// ============================================================================
// Use Svelte Flow Example Component
// ============================================================================

/// UseSvelteFlow example - Demonstrates the FlowStore API
#[component]
pub fn UseSvelteFlowExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("a".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({
                "label": "Node A",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("b".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({
                "label": "Node B",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("c".to_string(), Position::new(150.0, 150.0))
            .with_data(json!({
                "label": "Node C",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("d".to_string(), Position::new(350.0, 150.0))
            .with_data(json!({
                "label": "Node D",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    let initial_edges = vec![
        Edge::new("e-a-b".to_string(), "a".to_string(), "b".to_string()),
        Edge::new("e-a-c".to_string(), "a".to_string(), "c".to_string()),
        Edge::new("e-b-d".to_string(), "b".to_string(), "d".to_string()),
        Edge::new("e-c-d".to_string(), "c".to_string(), "d".to_string()),
    ];

    // Create the store - this is equivalent to useSvelteFlow() in React
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store via context so components like Background, Controls, etc. can access it
    provide_context(store.clone());

    let drag_signal = get_drag_signal();
    let action_log = get_action_log();

    // Reset log on mount
    action_log.set(vec!["Example loaded".to_string()]);

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
                store.update_node(&drag_state.node_id, |n| {
                    n.dragging = false;
                });
                drag_signal.set(None);
            }
        }
    };

    view! {
        <div class="example-container" style="display: flex; flex-direction: column; height: 100%;">
            // Header
            <div style="padding: 12px; background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%); \
                        border-bottom: 1px solid #86efac;">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <div style="background: #10b981; color: white; padding: 6px 12px; border-radius: 6px; \
                                font-size: 11px; font-weight: 600;">
                        "useSvelteFlow / FlowStore"
                    </div>
                    <div style="font-size: 12px; color: #166534;">
                        "The main API hook for programmatic flow control"
                    </div>
                </div>
            </div>

            // Main content
            <div style="display: flex; flex: 1; min-height: 0;">
                // Flow canvas
                <div
                    class="xyflow leptos-flow"
                    style="flex: 1; position: relative; background: #fafafa;"
                    on:mousemove=on_mousemove
                    on:mouseup=on_mouseup
                >
                    <Background variant=BackgroundVariant::Dots />

                    <FlowViewport store=store.clone()>
                        <UseSvelteFlowEdgeRenderer store=store.clone() />
                        <ConnectionLine />

                        {move || {
                            store.get_nodes().into_iter().map(|node| {
                                view! {
                                    <UseSvelteFlowNode
                                        node=node.clone()
                                        store=store.clone()
                                        drag_signal=drag_signal
                                    />
                                }
                            }).collect_view()
                        }}
                    </FlowViewport>

                    <Controls position=PanelPosition::BottomLeft />
                </div>

                // Control Panel
                <div style="width: 320px; background: #f8fafc; border-left: 1px solid #e2e8f0; \
                            display: flex; flex-direction: column; overflow-y: auto;">
                    <ApiMethodsPanel store=store.clone() />
                    <StateInspector store=store.clone() />
                    <ActionLogPanel />
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// API Methods Panel
// ============================================================================

/// Panel demonstrating FlowStore API methods
#[component]
fn ApiMethodsPanel(store: FlowStore) -> impl IntoView {
    // ===== Viewport Methods =====

    // Fit view - centers and zooms to fit all nodes
    let fit_view = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            if nodes.is_empty() {
                return;
            }

            // Calculate bounds
            let mut min_x = f64::MAX;
            let mut min_y = f64::MAX;
            let mut max_x = f64::MIN;
            let mut max_y = f64::MIN;

            for node in &nodes {
                let w = node.width.unwrap_or(120.0);
                let h = node.height.unwrap_or(50.0);
                min_x = min_x.min(node.position.x);
                min_y = min_y.min(node.position.y);
                max_x = max_x.max(node.position.x + w);
                max_y = max_y.max(node.position.y + h);
            }

            // Add padding
            let padding = 50.0;
            min_x -= padding;
            min_y -= padding;
            max_x += padding;
            max_y += padding;

            // Calculate center
            let center_x = (min_x + max_x) / 2.0;
            let center_y = (min_y + max_y) / 2.0;

            // Set viewport to center on the nodes
            store.set_viewport(Viewport::new(-center_x + 200.0, -center_y + 150.0, 1.0));
            log_action("fitView() - Centered viewport on all nodes");
        }
    };

    // Zoom In
    let zoom_in = {
        let store = store.clone();
        move |_| {
            store.zoom_by(1.2);
            let zoom = store.get_viewport().zoom;
            log_action(&format!("zoomIn() - Zoom: {:.2}", zoom));
        }
    };

    // Zoom Out
    let zoom_out = {
        let store = store.clone();
        move |_| {
            store.zoom_by(0.8);
            let zoom = store.get_viewport().zoom;
            log_action(&format!("zoomOut() - Zoom: {:.2}", zoom));
        }
    };

    // Set Center
    let set_center = {
        let store = store.clone();
        move |_| {
            store.set_viewport(Viewport::new(0.0, 0.0, 1.0));
            log_action("setCenter(0, 0, 1.0) - Reset viewport");
        }
    };

    // Set Zoom
    let set_zoom = {
        let store = store.clone();
        move |_| {
            let mut vp = store.get_viewport();
            vp.zoom = 1.5;
            store.set_viewport(vp);
            log_action("setViewport(zoom: 1.5) - Set zoom to 1.5x");
        }
    };

    // ===== Node Methods =====

    // Get Nodes
    let get_nodes = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            log_action(&format!("getNodes() - {} nodes", nodes.len()));
        }
    };

    // Set Nodes
    let set_nodes = {
        let store = store.clone();
        move |_| {
            let mut nodes = store.get_nodes();
            // Shift all nodes by a small amount
            for node in &mut nodes {
                node.position.x += 20.0;
            }
            store.set_nodes(nodes);
            log_action("setNodes() - Shifted all nodes right by 20px");
        }
    };

    // Add Node
    let add_node = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            let new_id = format!("new-{}", nodes.len());
            let max_y = nodes.iter().map(|n| n.position.y).fold(0.0_f64, f64::max);

            let new_node = Node::new(new_id.clone(), Position::new(150.0, max_y + 80.0))
                .with_data(json!({
                    "label": format!("New {}", nodes.len()),
                    "type": "default",
                    "color": "#f59e0b"
                }))
                .with_dimensions(120.0, 50.0);

            store.add_node(new_node);
            log_action(&format!("addNode('{}') - Added new node", new_id));
        }
    };

    // Update Node
    let update_node = {
        let store = store.clone();
        move |_| {
            let updated = store.update_node("a", |node| {
                node.position.y += 30.0;
            });
            if updated {
                log_action("updateNode('a') - Moved Node A down 30px");
            } else {
                log_action("updateNode('a') - Node not found");
            }
        }
    };

    // Delete Node
    let delete_node = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            if let Some(last) = nodes.last() {
                let id = last.id.clone();
                if store.remove_node(&id) {
                    log_action(&format!("deleteNode('{}') - Removed node", id));
                }
            } else {
                log_action("deleteNode() - No nodes to delete");
            }
        }
    };

    // ===== Edge Methods =====

    // Get Edges
    let get_edges = {
        let store = store.clone();
        move |_| {
            let edges = store.get_edges();
            log_action(&format!("getEdges() - {} edges", edges.len()));
        }
    };

    // Add Edge
    let add_edge = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            let edges = store.get_edges();

            if nodes.len() >= 2 {
                let source = &nodes[0].id;
                let target = &nodes[nodes.len() - 1].id;
                let edge_id = format!("e-new-{}", edges.len());

                let new_edge = Edge::new(edge_id.clone(), source.clone(), target.clone());
                store.add_edge(new_edge);
                log_action(&format!("addEdge('{}') - {} -> {}", edge_id, source, target));
            } else {
                log_action("addEdge() - Not enough nodes");
            }
        }
    };

    // Delete Edge
    let delete_edge = {
        let store = store.clone();
        move |_| {
            let edges = store.get_edges();
            if let Some(last) = edges.last() {
                let id = last.id.clone();
                if store.remove_edge(&id) {
                    log_action(&format!("deleteEdge('{}') - Removed edge", id));
                }
            } else {
                log_action("deleteEdge() - No edges to delete");
            }
        }
    };

    // ===== Selection Methods =====

    // Select All
    let select_all = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            for (i, node) in nodes.iter().enumerate() {
                store.select_node(&node.id, i > 0);
            }
            log_action(&format!("selectAll() - Selected {} nodes", nodes.len()));
        }
    };

    // Clear Selection
    let clear_selection = {
        let store = store.clone();
        move |_| {
            store.clear_node_selection();
            store.clear_edge_selection();
            log_action("clearSelection() - Cleared all selections");
        }
    };

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 12px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #10b981; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"API"</span>
                "FlowStore Methods"
            </div>

            // Viewport Methods
            <div style="margin-bottom: 12px;">
                <div style="font-size: 10px; font-weight: 600; color: #6366f1; margin-bottom: 6px; \
                            text-transform: uppercase; letter-spacing: 0.5px;">
                    "Viewport"
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4px;">
                    <ApiButton label="fitView()" on_click=fit_view />
                    <ApiButton label="zoomIn()" on_click=zoom_in />
                    <ApiButton label="zoomOut()" on_click=zoom_out />
                    <ApiButton label="setCenter()" on_click=set_center />
                    <ApiButton label="setZoom(1.5)" on_click=set_zoom />
                </div>
            </div>

            // Node Methods
            <div style="margin-bottom: 12px;">
                <div style="font-size: 10px; font-weight: 600; color: #10b981; margin-bottom: 6px; \
                            text-transform: uppercase; letter-spacing: 0.5px;">
                    "Nodes"
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4px;">
                    <ApiButton label="getNodes()" on_click=get_nodes />
                    <ApiButton label="setNodes()" on_click=set_nodes />
                    <ApiButton label="addNode()" on_click=add_node color="#10b981" />
                    <ApiButton label="updateNode()" on_click=update_node />
                    <ApiButton label="deleteNode()" on_click=delete_node color="#ef4444" />
                </div>
            </div>

            // Edge Methods
            <div style="margin-bottom: 12px;">
                <div style="font-size: 10px; font-weight: 600; color: #8b5cf6; margin-bottom: 6px; \
                            text-transform: uppercase; letter-spacing: 0.5px;">
                    "Edges"
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4px;">
                    <ApiButton label="getEdges()" on_click=get_edges />
                    <ApiButton label="addEdge()" on_click=add_edge color="#8b5cf6" />
                    <ApiButton label="deleteEdge()" on_click=delete_edge color="#ef4444" />
                </div>
            </div>

            // Selection Methods
            <div>
                <div style="font-size: 10px; font-weight: 600; color: #f59e0b; margin-bottom: 6px; \
                            text-transform: uppercase; letter-spacing: 0.5px;">
                    "Selection"
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 4px;">
                    <ApiButton label="selectAll()" on_click=select_all />
                    <ApiButton label="clearSelection()" on_click=clear_selection />
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// API Button Component
// ============================================================================

#[component]
fn ApiButton<F>(
    label: &'static str,
    on_click: F,
    #[prop(default = "#6366f1")]
    color: &'static str,
) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <button
            style=format!(
                "padding: 5px 8px; font-size: 9px; font-family: monospace; \
                 border: 1px solid {}40; border-radius: 4px; background: {}10; \
                 color: {}; cursor: pointer; font-weight: 500; \
                 transition: all 0.15s;",
                color, color, color
            )
            on:click=on_click
        >
            {label}
        </button>
    }
}

// ============================================================================
// State Inspector Component
// ============================================================================

#[component]
fn StateInspector(store: FlowStore) -> impl IntoView {
    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #f59e0b; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"STATE"</span>
                "Current State"
            </div>

            // Viewport
            <div style="background: #fffbeb; padding: 8px; border-radius: 6px; margin-bottom: 8px;">
                <div style="font-size: 10px; font-weight: 600; color: #92400e; margin-bottom: 4px;">
                    "Viewport"
                </div>
                {move || {
                    let vp = store.get_viewport();
                    view! {
                        <div style="font-family: monospace; font-size: 10px; color: #78350f;">
                            <div>"x: "{format!("{:.1}", vp.x)}</div>
                            <div>"y: "{format!("{:.1}", vp.y)}</div>
                            <div>"zoom: "{format!("{:.2}", vp.zoom)}</div>
                        </div>
                    }
                }}
            </div>

            // Counts
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                <div style="background: #f0fdf4; padding: 8px; border-radius: 6px; text-align: center;">
                    <div style="font-size: 18px; font-weight: 700; color: #166534;">
                        {move || store.get_nodes().len()}
                    </div>
                    <div style="font-size: 9px; color: #15803d; text-transform: uppercase;">"Nodes"</div>
                </div>
                <div style="background: #faf5ff; padding: 8px; border-radius: 6px; text-align: center;">
                    <div style="font-size: 18px; font-weight: 700; color: #6b21a8;">
                        {move || store.get_edges().len()}
                    </div>
                    <div style="font-size: 9px; color: #7e22ce; text-transform: uppercase;">"Edges"</div>
                </div>
            </div>

            // Selection
            <div style="background: #eff6ff; padding: 8px; border-radius: 6px; margin-top: 8px;">
                <div style="font-size: 10px; font-weight: 600; color: #1e40af; margin-bottom: 4px;">
                    "Selection"
                </div>
                {move || {
                    let selected_nodes = store.get_selected_nodes();
                    let selected_edges = store.get_selected_edges();
                    view! {
                        <div style="font-family: monospace; font-size: 10px; color: #1e3a8a;">
                            <div>
                                "Nodes: "
                                {if selected_nodes.is_empty() {
                                    "none".to_string()
                                } else {
                                    selected_nodes.into_iter().collect::<Vec<_>>().join(", ")
                                }}
                            </div>
                            <div>
                                "Edges: "
                                {if selected_edges.is_empty() {
                                    "none".to_string()
                                } else {
                                    selected_edges.into_iter().collect::<Vec<_>>().join(", ")
                                }}
                            </div>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Action Log Component
// ============================================================================

#[component]
fn ActionLogPanel() -> impl IntoView {
    let action_log = get_action_log();

    view! {
        <div style="padding: 12px; flex: 1; display: flex; flex-direction: column;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #6366f1; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"LOG"</span>
                "API Calls"
                <button
                    style="margin-left: auto; font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                    on:click=move |_| action_log.set(Vec::new())
                >
                    "Clear"
                </button>
            </div>

            <div style="flex: 1; background: #1a1a2e; border-radius: 6px; padding: 8px; \
                        overflow-y: auto; font-family: monospace; font-size: 9px;">
                {move || {
                    let entries = action_log.get();
                    if entries.is_empty() {
                        view! {
                            <div style="color: #666;">"No API calls yet..."</div>
                        }.into_any()
                    } else {
                        entries.into_iter().enumerate().map(|(i, entry)| {
                            let color = if entry.contains("Error") || entry.contains("not found") {
                                "#ef4444"
                            } else if entry.contains("Added") || entry.contains("Selected") {
                                "#10b981"
                            } else if entry.contains("Removed") || entry.contains("Cleared") {
                                "#f59e0b"
                            } else {
                                "#a5b4fc"
                            };
                            view! {
                                <div style=format!(
                                    "color: {}; padding: 2px 0; border-bottom: 1px solid #2a2a4e;",
                                    color
                                )>
                                    <span style="color: #4b5563;">{format!("[{}] ", i)}</span>
                                    {entry}
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
fn UseSvelteFlowNode(
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
                        "2px solid #1a1a1a".to_string()
                    } else {
                        format!("2px solid {}60", color)
                    };

                    let box_shadow = if n.selected {
                        format!("0 0 0 2px {}40, 0 4px 12px rgba(0,0,0,0.2)", color)
                    } else {
                        "0 2px 6px rgba(0,0,0,0.1)".to_string()
                    };

                    let background = match node_type {
                        "input" => format!("linear-gradient(135deg, {}30 0%, {}50 100%)", color, color),
                        "output" => format!("linear-gradient(135deg, {}30 0%, {}50 100%)", color, color),
                        _ => "white".to_string(),
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
                        .unwrap_or("#333")
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
                                    style="background: #666; width: 8px; height: 8px; border: 2px solid white; \
                                           box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=node.id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #666; width: 8px; height: 8px; border: 2px solid white; \
                                           box-shadow: 0 1px 3px rgba(0,0,0,0.2);".to_string()
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
fn UseSvelteFlowEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="use-svelte-flow-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#8b5cf6" />
                    <stop offset="100%" stop-color="#6366f1" />
                </linearGradient>
                <marker
                    id="use-svelte-flow-edge-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#8b5cf6" />
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

                    Some(view! {
                        <g class="xyflow__edge">
                            <path
                                d=path.clone()
                                stroke="url(#use-svelte-flow-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#use-svelte-flow-edge-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
