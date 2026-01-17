//! Basic Example
//!
//! Demonstrates core XYFlow features: draggable nodes, pan/zoom, background,
//! minimap, and controls.

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::web_sys;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use xyflow_leptos::*;

use crate::shared::{DraggableNode, get_drag_signal, SourceCodeViewer};

/// Basic flow example with nodes, edges, background, minimap, and controls
#[component]
pub fn BasicExample() -> impl IntoView {
    // Create initial nodes matching React Basic example layout
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(250.0, 5.0))
            .with_data(json!({"label": "Node 1", "type": "input", "class": "light"})),
        Node::new("2".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Node 2", "type": "default", "class": "light"})),
        Node::new("3".to_string(), Position::new(400.0, 100.0))
            .with_data(json!({"label": "Node 3", "type": "default", "class": "light"})),
        Node::new("4".to_string(), Position::new(400.0, 200.0))
            .with_data(json!({"label": "Node 4", "type": "default", "class": "light"})),
    ];

    // Create initial edges matching React Basic example
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string())
            .with_animated(true),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Signal for hiding the flow (matching React example's Hide Flow button)
    let is_hidden = RwSignal::new(false);

    // Signal for tracking light/dark class toggle
    let use_dark_class = RwSignal::new(false);

    // Measure handle bounds after nodes are mounted
    Effect::new(move |_| {
        let store_clone = store;
        // Use request_animation_frame to wait for DOM to be ready
        if let Some(window) = web_sys::window() {
            let closure = Closure::once(move || {
                // Measure handles for each node
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        for node_id in ["1", "2", "3", "4"] {
                            // Find the node element by looking for handles with this node-id
                            let selector = format!("[data-nodeid='{}']", node_id);
                            if let Ok(Some(handle_elem)) = document.query_selector(&selector) {
                                // Get the parent node element
                                if let Some(node_elem) = handle_elem.parent_element() {
                                    if let Some(node_elem) = node_elem.parent_element() {
                                        if let Ok(node_html) = node_elem.dyn_into::<web_sys::HtmlElement>() {
                                            // Measure handles
                                            if let Some(handle_bounds) = xyflow_leptos::utils::measure_node_handles(
                                                &node_html,
                                                1.0, // zoom
                                                node_id,
                                            ) {
                                                // Update the node with handle bounds
                                                store_clone.state.nodes.update(|nodes| {
                                                    if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
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
            });
            let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    // Button action handlers
    let reset_transform = move |_| {
        store.set_viewport(Viewport { x: 0.0, y: 0.0, zoom: 1.0 });
    };

    let change_pos = move |_| {
        let nodes = store.get_nodes();
        let updated_nodes: Vec<Node> = nodes.into_iter().map(|mut node| {
            node.position = Position::new(
                js_sys::Math::random() * 400.0,
                js_sys::Math::random() * 400.0,
            );
            node
        }).collect();
        store.set_nodes(updated_nodes);
    };

    let toggle_classnames = move |_| {
        use_dark_class.update(|v| *v = !*v);
        let dark = use_dark_class.get();
        let nodes = store.get_nodes();
        let updated_nodes: Vec<Node> = nodes.into_iter().map(|mut node| {
            let new_class = if dark { "dark" } else { "light" };
            if let Some(data) = node.data.as_object_mut() {
                data.insert("class".to_string(), json!(new_class));
            }
            node
        }).collect();
        store.set_nodes(updated_nodes);
    };

    let log_to_object = move |_| {
        let nodes = store.get_nodes();
        let edges = store.get_edges();
        let viewport = store.get_viewport();
        web_sys::console::log_1(&format!(
            "toObject: {{ nodes: {:?}, edges: {:?}, viewport: {:?} }}",
            nodes.len(), edges.len(), viewport
        ).into());
    };

    let delete_selected = move |_| {
        let selected_nodes = store.get_selected_nodes();
        let selected_edges = store.get_selected_edges();
        for id in selected_nodes {
            store.remove_node(&id);
        }
        for id in selected_edges {
            store.remove_edge(&id);
        }
    };

    let delete_some = move |_| {
        store.remove_node("2");
        store.remove_edge("e1-3");
    };

    let set_nodes = move |_| {
        store.set_nodes(vec![
            Node::new("a".to_string(), Position::new(0.0, 0.0))
                .with_data(json!({"label": "Node a", "type": "default", "class": "light"})),
            Node::new("b".to_string(), Position::new(0.0, 150.0))
                .with_data(json!({"label": "Node b", "type": "default", "class": "light"})),
        ]);
        store.set_edges(vec![
            Edge::new("a-b".to_string(), "a".to_string(), "b".to_string()),
        ]);
    };

    let update_node = move |_| {
        store.update_node("1", |node| {
            if let Some(data) = node.data.as_object_mut() {
                data.insert("label".to_string(), json!("update"));
            }
        });
        store.update_node("2", |node| {
            if let Some(data) = node.data.as_object_mut() {
                data.insert("label".to_string(), json!("update"));
            }
        });
    };

    let add_node = move |_| {
        let id = format!("{}", js_sys::Math::random());
        store.add_node(
            Node::new(id, Position::new(
                js_sys::Math::random() * 300.0,
                js_sys::Math::random() * 300.0,
            )).with_data(json!({"label": "Node", "type": "default", "class": "light"}))
        );
    };

    let toggle_visibility = move |_| {
        is_hidden.update(|v| *v = !*v);
    };

    // Global drag handlers
    let drag_signal = get_drag_signal();

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
            store.update_node(&drag_state.node_id, |n| {
                n.dragging = false;
            });
            drag_signal.set(None);
        }
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow react-flow-basic-example"
                 style=move || format!(
                     "width: 100%; height: 100%; position: relative; {}",
                     if is_hidden.get() { "display: none;" } else { "" }
                 )
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background with dots
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <DraggableNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Panel with action buttons (top-right)
                <Panel position=PanelPosition::TopRight>
                    <button on:click=reset_transform>"reset transform"</button>
                    <button on:click=change_pos>"change pos"</button>
                    <button on:click=toggle_classnames>"toggle classnames"</button>
                    <button on:click=log_to_object>"toObject"</button>
                    <button on:click=delete_selected>"deleteSelectedElements"</button>
                    <button on:click=delete_some>"deleteSomeElements"</button>
                    <button on:click=set_nodes>"setNodes"</button>
                    <button on:click=update_node>"updateNode"</button>
                    <button on:click=add_node>"addNode"</button>
                </Panel>
            </div>

            // Hide Flow button outside the flow container
            <button
                on:click=toggle_visibility
                style="position: absolute; z-index: 10; right: 150px; top: 15px;"
            >
                {move || if is_hidden.get() { "Show Flow" } else { "Hide Flow" }}
            </button>

            <SourceCodeViewer
                source=include_str!("basic.rs")
                title="basic.rs"
            />
        </div>
    }
}
