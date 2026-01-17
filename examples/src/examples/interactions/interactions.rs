//! Interactions Example
//!
//! Demonstrates selection, multi-select, and deletion interactions with logging.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::get_drag_signal;

/// Interactions example with logging
#[component]
pub fn InteractionsExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({"label": "Click me!", "type": "input", "class": "light"})),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({"label": "Drag me!", "type": "default", "class": "light"})),
        Node::new("3".to_string(), Position::new(300.0, 125.0))
            .with_data(json!({"label": "Select me!", "type": "output", "class": "light"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e2-3".to_string(), "2".to_string(), "3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Interaction log
    let interaction_log = RwSignal::new(Vec::<String>::new());

    // Helper to add log entry (keeps last 10 entries)
    let add_log = move |msg: String| {
        interaction_log.update(|logs| {
            logs.push(msg);
            // Keep only the last 10 entries
            if logs.len() > 10 {
                logs.remove(0);
            }
        });
    };

    // Global drag handlers
    let drag_signal = get_drag_signal();
    let add_log_for_drag = add_log.clone();

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
            add_log_for_drag(format!("dragstop: node {}", node_id));
            drag_signal.set(None);
        }
    };

    // Node click handler
    let add_log_for_click = add_log.clone();
    let on_node_click = move |node_id: String| {
        add_log_for_click(format!("click: node {}", node_id));
    };

    // Node drag start handler
    let add_log_for_dragstart = add_log.clone();
    let on_node_drag_start = move |node_id: String| {
        add_log_for_dragstart(format!("dragstart: node {}", node_id));
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow interactions-example"
                 style="width: 100%; height: 100%; position: relative;"
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
                        let on_click = on_node_click.clone();
                        let on_drag_start = on_node_drag_start.clone();
                        store.get_nodes().into_iter().map(move |node| {
                            let node_id = node.id.clone();
                            let node_id_for_drag = node.id.clone();
                            let on_click = on_click.clone();
                            let on_drag_start = on_drag_start.clone();
                            view! {
                                <InteractiveNode
                                    node=node.clone()
                                    store=store
                                    on_click=move || on_click(node_id.clone())
                                    on_drag_start=move || on_drag_start(node_id_for_drag.clone())
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Interaction Log Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 10px; border-radius: 4px; max-width: 250px; box-shadow: 0 2px 6px rgba(0,0,0,0.1);">
                        <strong>"Interaction Log"</strong>
                        <div style="margin-top: 8px; font-size: 11px; font-family: monospace; max-height: 200px; overflow-y: auto;">
                            {move || {
                                let logs = interaction_log.get();
                                if logs.is_empty() {
                                    view! { <p style="color: #666;">"Interact with nodes..."</p> }.into_any()
                                } else {
                                    logs.into_iter().rev().map(|log| {
                                        view! { <p style="margin: 2px 0; padding: 2px 4px; background: #f5f5f5; border-radius: 2px;">{log}</p> }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                        <button
                            style="margin-top: 8px; font-size: 11px; padding: 4px 8px; cursor: pointer;"
                            on:click=move |_| interaction_log.set(vec![])
                        >
                            "Clear Log"
                        </button>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

/// Interactive node wrapper with click and drag event logging
#[component]
fn InteractiveNode<F, G>(
    node: Node,
    store: FlowStore,
    on_click: F,
    on_drag_start: G,
) -> impl IntoView
where
    F: Fn() + Clone + 'static,
    G: Fn() + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let class = node.data.get("class")
        .and_then(|v| v.as_str())
        .unwrap_or("light")
        .to_string();

    let drag_signal = get_drag_signal();
    let mouse_down_pos = RwSignal::new(None::<(i32, i32)>);

    // Mouse down - prepare for drag or click
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Record mouse position to distinguish click from drag
        mouse_down_pos.set(Some((ev.client_x(), ev.client_y())));

        // Get current node position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(crate::shared::DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            // Mark node as dragging and log drag start
            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
            on_drag_start();
        }
    };

    // Handle click (distinguish from drag by checking if mouse moved)
    let on_click_handler = on_click.clone();
    let on_mouseup = move |ev: leptos::ev::MouseEvent| {
        if let Some((start_x, start_y)) = mouse_down_pos.get() {
            let dx = (ev.client_x() - start_x).abs();
            let dy = (ev.client_y() - start_y).abs();
            // If mouse didn't move much, it's a click
            if dx < 5 && dy < 5 {
                on_click_handler();
            }
        }
        mouse_down_pos.set(None);
    };

    // Get reactive node position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    // Determine node class based on type
    let has_source = node_type != "output";
    let has_target = node_type != "input";

    let node_class = match node_type.as_str() {
        "input" => "xyflow__node-input",
        "output" => "xyflow__node-output",
        _ => "xyflow__node-default",
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
            on:mouseup=on_mouseup
        >
            <div class=format!("{} {}", node_class, class)>
                {has_target.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div class="xyflow__node-label">
                    {label}
                </div>

                {has_source.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Bottom
                        connection_mode=ConnectionMode::Strict
                    />
                })}
            </div>
        </div>
    }
}
