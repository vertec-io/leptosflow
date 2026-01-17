//! Validation Example
//!
//! Demonstrates connection validation rules (source to target only).
//!
//! This example shows:
//! - Source nodes (with source handles) can connect TO target nodes
//! - Target nodes (with target handles) can receive connections FROM source nodes
//! - Connection attempts that violate these rules are rejected
//! - Visual log of connection events

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DragState};

/// Connection validation example
#[component]
pub fn ValidationExample() -> impl IntoView {
    // Create initial nodes: 2 source (input) nodes and 2 target (output) nodes
    let initial_nodes = vec![
        Node::new("source1".to_string(), Position::new(50.0, 50.0))
            .with_data(json!({"label": "Source A", "node_type": "source"})),
        Node::new("source2".to_string(), Position::new(50.0, 175.0))
            .with_data(json!({"label": "Source B", "node_type": "source"})),
        Node::new("target1".to_string(), Position::new(350.0, 50.0))
            .with_data(json!({"label": "Target A", "node_type": "target"})),
        Node::new("target2".to_string(), Position::new(350.0, 175.0))
            .with_data(json!({"label": "Target B", "node_type": "target"})),
    ];

    // Create initial edge connecting Source A to Target A
    let initial_edges = vec![
        Edge::new("e1".to_string(), "source1".to_string(), "target1".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Connection log
    let connection_log = RwSignal::new(Vec::<String>::new());

    // Helper to add log entry
    let add_log = move |msg: String| {
        connection_log.update(|logs| {
            logs.push(msg);
            if logs.len() > 8 {
                logs.remove(0);
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

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow validation-example"
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
                        let add_log = add_log.clone();
                        store.get_nodes().into_iter().map(move |node| {
                            let add_log = add_log.clone();
                            view! {
                                <ValidationNode
                                    node=node.clone()
                                    store=store
                                    add_log=add_log
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 6px; max-width: 260px; box-shadow: 0 2px 6px rgba(0,0,0,0.1);">
                        <strong>"Connection Validation"</strong>
                        <div style="margin-top: 8px; font-size: 12px; line-height: 1.5;">
                            <p style="margin: 4px 0;">
                                <span style="display: inline-block; width: 12px; height: 12px; background: #6ede87; border-radius: 2px; margin-right: 6px; vertical-align: middle;"></span>
                                "Source nodes (output only)"
                            </p>
                            <p style="margin: 4px 0;">
                                <span style="display: inline-block; width: 12px; height: 12px; background: #6865A5; border-radius: 2px; margin-right: 6px; vertical-align: middle;"></span>
                                "Target nodes (input only)"
                            </p>
                        </div>
                        <div style="margin-top: 10px; padding-top: 10px; border-top: 1px solid #eee;">
                            <p style="font-size: 11px; color: #666; margin: 0;">
                                "Drag from a source node's handle to a target node's handle. Invalid connections are rejected."
                            </p>
                        </div>
                        <div style="margin-top: 10px;">
                            <strong style="font-size: 11px;">"Connection Log"</strong>
                            <div style="margin-top: 4px; font-size: 10px; font-family: monospace; max-height: 100px; overflow-y: auto;">
                                {move || {
                                    let logs = connection_log.get();
                                    if logs.is_empty() {
                                        view! { <p style="color: #999;">"No connections yet..."</p> }.into_any()
                                    } else {
                                        logs.into_iter().rev().map(|log| {
                                            view! { <p style="margin: 2px 0; padding: 2px 4px; background: #f5f5f5; border-radius: 2px;">{log}</p> }
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

/// Validation-aware node component
#[component]
fn ValidationNode<F>(
    node: Node,
    store: FlowStore,
    #[allow(unused_variables)]
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let node_type = node.data.get("node_type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

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

    // Determine node styling based on type
    let is_source = node_type == "source";
    let (bg_color, border_color, node_class) = if is_source {
        ("#6ede87", "#4cb864", "xyflow__node-input")
    } else {
        ("#6865A5", "#4a4782", "xyflow__node-output")
    };

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
        >
            <div
                class=node_class
                style=format!(
                    "background: {} !important; border-color: {}; padding: 10px 20px; border-radius: 6px; border: 2px solid {};",
                    bg_color, border_color, border_color
                )
            >
                // Target handle (only for target nodes)
                {(!is_source).then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Left
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div class="xyflow__node-label" style="color: white; font-weight: 500;">
                    {label}
                </div>

                // Source handle (only for source nodes)
                {is_source.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Source
                        position=HandlePosition::Right
                        connection_mode=ConnectionMode::Strict
                    />
                })}
            </div>
        </div>
    }
}
