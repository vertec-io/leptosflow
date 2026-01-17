//! Switch Example
//!
//! Demonstrates switching between two different sets of nodes and edges at runtime.
//! Buttons allow swapping the entire flow state between "Flow A" and "Flow B".

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::web_sys;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use xyflow_leptos::*;

use crate::shared::{DraggableNode, get_drag_signal, SourceCodeViewer};

/// Flow A nodes - triangular layout
fn get_nodes_a() -> Vec<Node> {
    vec![
        Node::new("1a".to_string(), Position::new(250.0, 5.0))
            .with_data(json!({"label": "Node 1", "type": "input", "class": "light"})),
        Node::new("2a".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Node 2", "type": "default", "class": "light"})),
        Node::new("3a".to_string(), Position::new(400.0, 100.0))
            .with_data(json!({"label": "Node 3", "type": "default", "class": "light"})),
        Node::new("4a".to_string(), Position::new(400.0, 200.0))
            .with_data(json!({"label": "Node 4", "type": "default", "class": "light"})),
    ]
}

/// Flow A edges - connects node 1 to nodes 2 and 3
fn get_edges_a() -> Vec<Edge> {
    vec![
        Edge::new("e1-2".to_string(), "1a".to_string(), "2a".to_string()),
        Edge::new("e1-3".to_string(), "1a".to_string(), "3a".to_string()),
    ]
}

/// Flow B nodes - horizontal layout with more nodes
fn get_nodes_b() -> Vec<Node> {
    vec![
        Node::new("inputb".to_string(), Position::new(300.0, 5.0))
            .with_data(json!({"label": "Input", "type": "input", "class": "light"})),
        Node::new("1b".to_string(), Position::new(0.0, 100.0))
            .with_data(json!({"label": "Node 1", "type": "default", "class": "light"})),
        Node::new("2b".to_string(), Position::new(200.0, 100.0))
            .with_data(json!({"label": "Node 2", "type": "default", "class": "light"})),
        Node::new("3b".to_string(), Position::new(400.0, 100.0))
            .with_data(json!({"label": "Node 3", "type": "default", "class": "light"})),
        Node::new("4b".to_string(), Position::new(600.0, 100.0))
            .with_data(json!({"label": "Node 4", "type": "default", "class": "light"})),
    ]
}

/// Flow B edges - connects input to all nodes
fn get_edges_b() -> Vec<Edge> {
    vec![
        Edge::new("e1b".to_string(), "inputb".to_string(), "1b".to_string()),
        Edge::new("e2b".to_string(), "inputb".to_string(), "2b".to_string()),
        Edge::new("e3b".to_string(), "inputb".to_string(), "3b".to_string()),
        Edge::new("e4b".to_string(), "inputb".to_string(), "4b".to_string()),
    ]
}

/// Helper to measure handle bounds for all nodes
fn measure_all_handles(store: FlowStore, node_ids: &[&str]) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            for node_id in node_ids {
                let selector = format!("[data-nodeid='{}']", node_id);
                if let Ok(Some(handle_elem)) = document.query_selector(&selector) {
                    if let Some(node_elem) = handle_elem.parent_element() {
                        if let Some(node_elem) = node_elem.parent_element() {
                            if let Ok(node_html) = node_elem.dyn_into::<web_sys::HtmlElement>() {
                                if let Some(handle_bounds) = xyflow_leptos::utils::measure_node_handles(
                                    &node_html,
                                    1.0,
                                    node_id,
                                ) {
                                    let node_id_owned = node_id.to_string();
                                    store.state.nodes.update(|nodes| {
                                        if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id_owned) {
                                            node.internals.set_handle_bounds(handle_bounds);
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Switch example - swap between two different flow configurations
#[component]
pub fn SwitchExample() -> impl IntoView {
    // Start with Flow A
    let store = FlowStore::new(get_nodes_a(), get_edges_a());
    provide_context(store);

    // Track which flow is active
    let active_flow = RwSignal::new("a");

    // Switch to Flow A
    let switch_to_a = move |_| {
        store.set_nodes(get_nodes_a());
        store.set_edges(get_edges_a());
        active_flow.set("a");

        // Re-measure handles after DOM update
        let store_clone = store;
        if let Some(window) = web_sys::window() {
            let closure = Closure::once(move || {
                measure_all_handles(store_clone, &["1a", "2a", "3a", "4a"]);
            });
            let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

    // Switch to Flow B
    let switch_to_b = move |_| {
        store.set_nodes(get_nodes_b());
        store.set_edges(get_edges_b());
        active_flow.set("b");

        // Re-measure handles after DOM update
        let store_clone = store;
        if let Some(window) = web_sys::window() {
            let closure = Closure::once(move || {
                measure_all_handles(store_clone, &["inputb", "1b", "2b", "3b", "4b"]);
            });
            let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

    // Node event handlers (log to console like React example)
    let on_node_click = move |node_id: String| {
        web_sys::console::log_1(&format!("click: {}", node_id).into());
    };

    let on_node_drag_start = move |node_id: String| {
        web_sys::console::log_1(&format!("drag start: {}", node_id).into());
    };

    let on_node_drag = move |node_id: String, pos: Position| {
        web_sys::console::log_1(&format!("drag {}: ({:.1}, {:.1})", node_id, pos.x, pos.y).into());
    };

    let on_node_drag_stop = move |node_id: String| {
        web_sys::console::log_1(&format!("drag stop: {}", node_id).into());
    };

    // Measure handles on mount
    Effect::new(move |_| {
        let store_clone = store;
        if let Some(window) = web_sys::window() {
            let closure = Closure::once(move || {
                measure_all_handles(store_clone, &["1a", "2a", "3a", "4a"]);
            });
            let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

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

            let new_pos = Position::new(node_start_x + dx, node_start_y + dy);
            let node_id = drag_state.node_id.clone();
            on_node_drag(node_id.clone(), new_pos.clone());

            store.update_node(&node_id, |n| {
                n.position = new_pos;
            });
        }
    };

    let on_global_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let node_id = drag_state.node_id.clone();
            on_node_drag_stop(node_id.clone());
            store.update_node(&node_id, |n| {
                n.dragging = false;
            });
            drag_signal.set(None);
        }
    };

    // Custom node click and drag start handlers
    let handle_node_mousedown = move |node_id: String, ev: leptos::ev::MouseEvent| {
        on_node_click(node_id.clone());
        on_node_drag_start(node_id);
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow switch-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow viewport
                <FlowViewport store=store>
                    // Edges
                    <EdgeRenderer />

                    // Connection line
                    <ConnectionLine />

                    // Nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <DraggableNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Switch buttons panel
                <Panel position=PanelPosition::TopRight>
                    <div style="display: flex; gap: 8px;">
                        <button
                            on:click=switch_to_a
                            style=move || format!(
                                "padding: 8px 16px; border-radius: 4px; border: 1px solid #ccc; cursor: pointer; {}",
                                if active_flow.get() == "a" { "background: #4f46e5; color: white; border-color: #4f46e5;" } else { "background: white;" }
                            )
                        >
                            "Flow A"
                        </button>
                        <button
                            on:click=switch_to_b
                            style=move || format!(
                                "padding: 8px 16px; border-radius: 4px; border: 1px solid #ccc; cursor: pointer; {}",
                                if active_flow.get() == "b" { "background: #4f46e5; color: white; border-color: #4f46e5;" } else { "background: white;" }
                            )
                        >
                            "Flow B"
                        </button>
                    </div>
                </Panel>

                // Info panel
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: rgba(255,255,255,0.9); padding: 12px; border-radius: 8px; font-size: 14px; max-width: 250px;">
                        <strong>"Switch Example"</strong>
                        <p style="margin: 8px 0 0 0; color: #666;">
                            "Click the buttons above to switch between two different flow configurations. "
                            "Node interactions are logged to the browser console."
                        </p>
                        <p style="margin: 8px 0 0 0; color: #888; font-size: 12px;">
                            {move || format!("Active: Flow {}", active_flow.get().to_uppercase())}
                        </p>
                    </div>
                </Panel>
            </div>

            <SourceCodeViewer
                source=include_str!("switch.rs")
                title="switch.rs"
            />
        </div>
    }
}
