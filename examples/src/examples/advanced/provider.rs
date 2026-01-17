//! Provider Example
//!
//! Demonstrates using the FlowStore provider pattern:
//! - Using FlowStore context outside of SvelteFlow
//! - Accessing flow state from sibling components
//! - Demonstrating the benefits of the provider pattern

use leptos::prelude::*;
use serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Drag state for Provider example
static PROVIDER_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *PROVIDER_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Provider Example Component
// ============================================================================

/// Provider example - Demonstrates FlowStore context pattern
#[component]
pub fn ProviderExample() -> impl IntoView {
    // Create nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({
                "label": "Node A",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("2".to_string(), Position::new(300.0, 50.0))
            .with_data(json!({
                "label": "Node B",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("3".to_string(), Position::new(200.0, 150.0))
            .with_data(json!({
                "label": "Node C",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("4".to_string(), Position::new(400.0, 150.0))
            .with_data(json!({
                "label": "Node D",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
    ];

    // Create the store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // IMPORTANT: Provide the store to all child components
    // This is the key pattern - providing context at a higher level
    provide_context(store);

    view! {
        <div class="example-container" style="display: flex; flex-direction: column; height: 100%;">
            // Top section: Explanation panel
            <div style="padding: 12px; background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%); \
                        border-bottom: 1px solid #bae6fd;">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <div style="background: #0284c7; color: white; padding: 6px 12px; border-radius: 6px; \
                                font-size: 11px; font-weight: 600;">
                        "Provider Pattern"
                    </div>
                    <div style="font-size: 12px; color: #0369a1;">
                        "FlowStore is provided at the top level - sibling components can access it via context"
                    </div>
                </div>
            </div>

            // Main content area: Flow + Sidebar
            <div style="display: flex; flex: 1; min-height: 0;">
                // Flow canvas area
                <div style="flex: 2; position: relative; display: flex; flex-direction: column;">
                    <FlowCanvas />
                </div>

                // Sidebar - These components access FlowStore via context!
                <div style="width: 280px; background: #f8fafc; border-left: 1px solid #e2e8f0; \
                            display: flex; flex-direction: column; overflow-y: auto;">
                    // These are SIBLING components that access FlowStore via context
                    <NodeList />
                    <EdgeList />
                    <StoreActions />
                    <StateInspector />
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Flow Canvas Component (uses FlowStore from context)
// ============================================================================

/// The flow canvas - accesses FlowStore via context
#[component]
fn FlowCanvas() -> impl IntoView {
    // Access the FlowStore from context - this is the key pattern!
    let store = use_context::<FlowStore>()
        .expect("FlowStore must be provided by a parent component");

    let drag_signal = get_drag_signal();

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
        <div
            class="xyflow leptos-flow"
            style="flex: 1; position: relative; background: #fafafa;"
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
        >
            // Header
            <div style="position: absolute; top: 8px; left: 8px; z-index: 10; \
                        background: rgba(255,255,255,0.95); padding: 8px 12px; border-radius: 6px; \
                        box-shadow: 0 1px 4px rgba(0,0,0,0.1); font-size: 11px;">
                <div style="font-weight: 600; color: #333; margin-bottom: 4px;">
                    "Flow Canvas"
                </div>
                <div style="color: #666;">
                    "Uses "<code style="background: #f0f0f0; padding: 1px 4px; border-radius: 2px;">"use_context::<FlowStore>()"</code>
                </div>
            </div>

            // Background
            <Background variant=BackgroundVariant::Dots />

            // Flow viewport
            <FlowViewport store=store.clone()>
                // Edge renderer
                <ProviderEdgeRenderer store=store.clone() />

                // Connection line
                <ConnectionLine />

                // Render nodes
                {move || {
                    store.get_nodes().into_iter().map(|node| {
                        view! {
                            <ProviderNode
                                node=node.clone()
                                store=store.clone()
                                drag_signal=drag_signal
                            />
                        }
                    }).collect_view()
                }}
            </FlowViewport>

            // Controls
            <Controls position=PanelPosition::BottomLeft />
        </div>
    }
}

// ============================================================================
// Node List Component (sibling component using context)
// ============================================================================

/// Node list sidebar panel - accesses FlowStore via context
#[component]
fn NodeList() -> impl IntoView {
    // This component is OUTSIDE the flow canvas, but can still access the store!
    let store = use_context::<FlowStore>()
        .expect("FlowStore must be provided");

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
                <div style="background: #6366f1; color: white; padding: 2px 6px; border-radius: 4px; \
                            font-size: 9px; font-weight: 600;">
                    "CONTEXT"
                </div>
                <div style="font-size: 12px; font-weight: 600; color: #333;">"Nodes"</div>
                <div style="margin-left: auto; font-size: 10px; background: #6366f130; color: #6366f1; \
                            padding: 2px 6px; border-radius: 10px; font-weight: 500;">
                    {move || store.get_nodes().len()}
                </div>
            </div>

            <div style="display: flex; flex-direction: column; gap: 6px;">
                {move || {
                    store.get_nodes().into_iter().map(|node| {
                        let color = node.data.get("color")
                            .and_then(|v| v.as_str())
                            .unwrap_or("#666")
                            .to_string();
                        let label = node.data.get("label")
                            .and_then(|v| v.as_str())
                            .unwrap_or(&node.id)
                            .to_string();
                        let node_type = node.data.get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("default")
                            .to_string();
                        let is_selected = node.selected;

                        view! {
                            <div style=format!(
                                "display: flex; align-items: center; gap: 8px; padding: 6px 8px; \
                                 background: {}; border-radius: 6px; font-size: 10px; \
                                 border: 1px solid {};",
                                if is_selected { "#eff6ff" } else { "white" },
                                if is_selected { "#3b82f6" } else { "#e2e8f0" }
                            )>
                                <div style=format!(
                                    "width: 8px; height: 8px; border-radius: 50%; background: {};",
                                    color
                                )></div>
                                <div style="font-weight: 500;">{label}</div>
                                <div style="font-size: 9px; color: #888; margin-left: auto;">{node_type}</div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Edge List Component (sibling component using context)
// ============================================================================

/// Edge list sidebar panel - accesses FlowStore via context
#[component]
fn EdgeList() -> impl IntoView {
    let store = use_context::<FlowStore>()
        .expect("FlowStore must be provided");

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
                <div style="background: #8b5cf6; color: white; padding: 2px 6px; border-radius: 4px; \
                            font-size: 9px; font-weight: 600;">
                    "CONTEXT"
                </div>
                <div style="font-size: 12px; font-weight: 600; color: #333;">"Edges"</div>
                <div style="margin-left: auto; font-size: 10px; background: #8b5cf630; color: #8b5cf6; \
                            padding: 2px 6px; border-radius: 10px; font-weight: 500;">
                    {move || store.get_edges().len()}
                </div>
            </div>

            <div style="display: flex; flex-direction: column; gap: 4px;">
                {move || {
                    store.get_edges().into_iter().map(|edge| {
                        let source = edge.source.clone();
                        let target = edge.target.clone();
                        view! {
                            <div style="display: flex; align-items: center; gap: 6px; padding: 4px 8px; \
                                        background: #f8fafc; border-radius: 4px; font-size: 10px;">
                                <span style="color: #10b981; font-weight: 500;">{source}</span>
                                <span style="color: #999;">"->"</span>
                                <span style="color: #ef4444; font-weight: 500;">{target}</span>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Store Actions Component (sibling component using context)
// ============================================================================

/// Actions panel - demonstrates modifying store from outside flow
#[component]
fn StoreActions() -> impl IntoView {
    let store = use_context::<FlowStore>()
        .expect("FlowStore must be provided");

    // Add a new node
    let add_node = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            let new_id = format!("{}", nodes.len() + 1);
            let max_y = nodes.iter().map(|n| n.position.y).fold(0.0_f64, f64::max);

            let new_node = Node::new(new_id.clone(), Position::new(200.0, max_y + 80.0))
                .with_data(json!({
                    "label": format!("Node {}", new_id),
                    "type": "default",
                    "color": "#6366f1"
                }))
                .with_dimensions(120.0, 50.0);

            store.add_node(new_node);
        }
    };

    // Remove last node
    let remove_node = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            if let Some(last) = nodes.last() {
                store.remove_node(&last.id);
            }
        }
    };

    // Select all nodes
    let select_all = {
        let store = store.clone();
        move |_| {
            let nodes = store.get_nodes();
            for (i, node) in nodes.iter().enumerate() {
                store.select_node(&node.id, i > 0);
            }
        }
    };

    // Clear selection
    let clear_selection = {
        let store = store.clone();
        move |_| {
            store.clear_node_selection();
            store.clear_edge_selection();
        }
    };

    // Reset viewport
    let reset_viewport = {
        let store = store.clone();
        move |_| {
            store.set_viewport(Viewport::new(0.0, 0.0, 1.0));
        }
    };

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
                <div style="background: #10b981; color: white; padding: 2px 6px; border-radius: 4px; \
                            font-size: 9px; font-weight: 600;">
                    "ACTIONS"
                </div>
                <div style="font-size: 12px; font-weight: 600; color: #333;">"Store Actions"</div>
            </div>

            <div style="font-size: 10px; color: #666; margin-bottom: 10px;">
                "Modify the store from outside the flow component"
            </div>

            <div style="display: flex; flex-direction: column; gap: 6px;">
                <button
                    style="padding: 6px 10px; font-size: 10px; border: none; border-radius: 4px; \
                           background: #10b981; color: white; cursor: pointer; font-weight: 500; \
                           text-align: left;"
                    on:click=add_node
                >
                    "+ Add Node"
                </button>
                <button
                    style="padding: 6px 10px; font-size: 10px; border: none; border-radius: 4px; \
                           background: #ef4444; color: white; cursor: pointer; font-weight: 500; \
                           text-align: left;"
                    on:click=remove_node
                >
                    "- Remove Last Node"
                </button>
                <button
                    style="padding: 6px 10px; font-size: 10px; border: 1px solid #6366f1; border-radius: 4px; \
                           background: white; color: #6366f1; cursor: pointer; font-weight: 500; \
                           text-align: left;"
                    on:click=select_all
                >
                    "Select All"
                </button>
                <button
                    style="padding: 6px 10px; font-size: 10px; border: 1px solid #888; border-radius: 4px; \
                           background: white; color: #666; cursor: pointer; font-weight: 500; \
                           text-align: left;"
                    on:click=clear_selection
                >
                    "Clear Selection"
                </button>
                <button
                    style="padding: 6px 10px; font-size: 10px; border: 1px solid #888; border-radius: 4px; \
                           background: white; color: #666; cursor: pointer; font-weight: 500; \
                           text-align: left;"
                    on:click=reset_viewport
                >
                    "Reset Viewport"
                </button>
            </div>
        </div>
    }
}

// ============================================================================
// State Inspector Component (sibling component using context)
// ============================================================================

/// State inspector - shows current store state
#[component]
fn StateInspector() -> impl IntoView {
    let store = use_context::<FlowStore>()
        .expect("FlowStore must be provided");

    view! {
        <div style="padding: 12px; flex: 1;">
            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
                <div style="background: #f59e0b; color: white; padding: 2px 6px; border-radius: 4px; \
                            font-size: 9px; font-weight: 600;">
                    "INSPECT"
                </div>
                <div style="font-size: 12px; font-weight: 600; color: #333;">"State Inspector"</div>
            </div>

            // Viewport state
            <div style="background: #fffbeb; padding: 8px; border-radius: 6px; margin-bottom: 8px;">
                <div style="font-size: 10px; font-weight: 600; color: #92400e; margin-bottom: 6px;">
                    "Viewport"
                </div>
                {move || {
                    let vp = store.get_viewport();
                    view! {
                        <div style="display: grid; grid-template-columns: auto 1fr; gap: 4px 8px; \
                                    font-size: 10px; font-family: monospace;">
                            <span style="color: #666;">"x:"</span>
                            <span style="color: #333;">{format!("{:.1}", vp.x)}</span>
                            <span style="color: #666;">"y:"</span>
                            <span style="color: #333;">{format!("{:.1}", vp.y)}</span>
                            <span style="color: #666;">"zoom:"</span>
                            <span style="color: #333;">{format!("{:.2}", vp.zoom)}</span>
                        </div>
                    }
                }}
            </div>

            // Selection state
            <div style="background: #eff6ff; padding: 8px; border-radius: 6px; margin-bottom: 8px;">
                <div style="font-size: 10px; font-weight: 600; color: #1e40af; margin-bottom: 6px;">
                    "Selection"
                </div>
                {move || {
                    let selected_nodes = store.get_selected_nodes();
                    let selected_edges = store.get_selected_edges();
                    view! {
                        <div style="font-size: 10px; font-family: monospace;">
                            <div style="color: #333;">
                                <span style="color: #666;">"Nodes: "</span>
                                {if selected_nodes.is_empty() {
                                    "none".to_string()
                                } else {
                                    selected_nodes.into_iter().collect::<Vec<_>>().join(", ")
                                }}
                            </div>
                            <div style="color: #333; margin-top: 2px;">
                                <span style="color: #666;">"Edges: "</span>
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

            // Pattern explanation
            <div style="background: #f0fdf4; padding: 8px; border-radius: 6px;">
                <div style="font-size: 10px; font-weight: 600; color: #166534; margin-bottom: 6px;">
                    "Provider Pattern Benefits"
                </div>
                <ul style="margin: 0; padding-left: 16px; font-size: 9px; color: #15803d; line-height: 1.5;">
                    <li>"Shared state across components"</li>
                    <li>"No prop drilling needed"</li>
                    <li>"Sibling components stay in sync"</li>
                    <li>"Actions from anywhere update flow"</li>
                </ul>
            </div>
        </div>
    }
}

// ============================================================================
// Provider Node Component
// ============================================================================

/// Node component for the provider example
#[component]
fn ProviderNode(
    node: Node,
    store: FlowStore,
    drag_signal: RwSignal<Option<DragState>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    // Mouse down - start drag
    let on_mousedown = {
        let store = store.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();

            // Start drag
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

                // Select this node
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
                if let Some(n) = nodes.iter().find(|n| n.id == node.id) {
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
// Provider Edge Renderer Component
// ============================================================================

/// Edge renderer for the provider example
#[component]
fn ProviderEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="provider-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#8b5cf6" />
                    <stop offset="100%" stop-color="#6366f1" />
                </linearGradient>
                <marker
                    id="provider-edge-arrow"
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
                                stroke="url(#provider-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#provider-edge-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
