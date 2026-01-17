//! Drag Handle Example
//!
//! Demonstrates how to limit the drag area on a node to a specific region,
//! such as a title bar. The rest of the node can be used for other interactions.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::DragState;

/// Global drag state for drag handle nodes
static DRAG_HANDLE_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

/// Get or initialize the drag handle state signal
fn get_drag_handle_signal() -> RwSignal<Option<DragState>> {
    *DRAG_HANDLE_STATE.get_or_init(|| RwSignal::new(None))
}

/// Drag handle example showing nodes with designated drag regions
#[component]
pub fn DragHandleExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({
                "label": "Card Node 1",
                "content": "Click the title bar to drag this node. The content area can be used for other interactions.",
                "type": "card"
            }))
            .with_dimensions(220.0, 140.0),
        Node::new("2".to_string(), Position::new(100.0, 250.0))
            .with_data(json!({
                "label": "Card Node 2",
                "content": "Try clicking the button below - it won't trigger a drag!",
                "type": "card",
                "hasButton": true
            }))
            .with_dimensions(220.0, 160.0),
        Node::new("3".to_string(), Position::new(380.0, 120.0))
            .with_data(json!({
                "label": "Card Node 3",
                "content": "Drag only works on the green title bar.",
                "type": "card"
            }))
            .with_dimensions(200.0, 120.0),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components via context
    provide_context(store);

    // Interaction log
    let log_entries = RwSignal::new(Vec::<String>::new());

    let add_log = move |msg: String| {
        log_entries.update(|entries| {
            entries.insert(0, msg);
            if entries.len() > 5 {
                entries.pop();
            }
        });
    };

    // Global drag handlers
    let drag_signal = get_drag_handle_signal();

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

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousemove=on_global_mousemove
                 on:mouseup=on_global_mouseup
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Main flow container with pan/zoom
                <FlowViewport store=store>
                    // Render edges
                    <EdgeRenderer />

                    // Render connection line while dragging
                    <ConnectionLine />

                    // Render card nodes with drag handles
                    {move || {
                        store.get_nodes().into_iter().map(move |node| {
                            let add_log = add_log.clone();
                            view! {
                                <CardNode
                                    node=node.clone()
                                    store=store
                                    on_button_click=move |node_id: String| {
                                        add_log(format!("Button clicked on {}", node_id));
                                    }
                                    on_content_click=move |node_id: String| {
                                        add_log(format!("Content clicked on {}", node_id));
                                    }
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
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); max-width: 240px;">
                        <strong style="display: block; margin-bottom: 8px;">"Drag Handle"</strong>
                        <p style="margin: 0 0 8px 0; font-size: 12px; color: #666;">
                            "Nodes can only be dragged by clicking the title bar (green area)"
                        </p>
                        <div style="font-size: 11px; color: #888; margin-bottom: 8px;">
                            <div style="margin: 4px 0;">"• Green title bar = drag handle"</div>
                            <div style="margin: 4px 0;">"• Content area = clickable"</div>
                            <div style="margin: 4px 0;">"• Buttons work independently"</div>
                        </div>

                        <div style="border-top: 1px solid #eee; padding-top: 8px; margin-top: 8px;">
                            <strong style="font-size: 11px;">"Interaction Log:"</strong>
                            <div style="font-size: 10px; color: #666; max-height: 80px; overflow-y: auto;">
                                {move || {
                                    let entries = log_entries.get();
                                    if entries.is_empty() {
                                        view! { <div style="color: #999; font-style: italic;">"No interactions yet"</div> }.into_any()
                                    } else {
                                        entries.iter().map(|entry| {
                                            view! { <div style="margin: 2px 0;">{entry.clone()}</div> }
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

/// Card node component with a drag handle title bar
#[component]
fn CardNode<F1, F2>(
    node: Node,
    store: FlowStore,
    on_button_click: F1,
    on_content_click: F2,
) -> impl IntoView
where
    F1: Fn(String) + Clone + 'static,
    F2: Fn(String) + Clone + 'static,
{
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_button = node.id.clone();
    let node_id_for_content = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Card")
        .to_string();
    let content = node.data.get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let has_button = node.data.get("hasButton")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let drag_signal = get_drag_handle_signal();

    // Mouse down on title bar ONLY - start dragging
    let on_title_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node
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

    // Content click handler (not drag)
    let on_content_click_handler = {
        let on_content_click = on_content_click.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.stop_propagation();
            on_content_click(node_id_for_content.clone());
        }
    };

    // Button click handler
    let on_button_click_handler = {
        let on_button_click = on_button_click.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();
            on_button_click(node_id_for_button.clone());
        }
    };

    // Get reactive node state
    let node_state = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| (n.position, n.width.unwrap_or(200.0), n.height.unwrap_or(120.0)))
            .unwrap_or((Position::new(0.0, 0.0), 200.0, 120.0))
    };

    view! {
        <div
            class="xyflow__node card-node"
            style=move || {
                let (pos, width, height) = node_state();
                format!(
                    "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                     background: white; border: 1px solid #ddd; border-radius: 8px; \
                     box-shadow: 0 2px 8px rgba(0,0,0,0.1); overflow: hidden;",
                    pos.x, pos.y, width, height
                )
            }
        >
            // Title bar - THIS IS THE DRAG HANDLE
            <div
                class="card-title-bar drag-handle"
                style="
                    background: linear-gradient(135deg, #4caf50 0%, #45a049 100%);
                    color: white;
                    padding: 8px 12px;
                    font-weight: 600;
                    font-size: 13px;
                    cursor: grab;
                    user-select: none;
                    display: flex;
                    align-items: center;
                    gap: 6px;
                "
                on:mousedown=on_title_mousedown
            >
                <span style="font-size: 14px;">"⋮⋮"</span>
                <span>{label}</span>
            </div>

            // Content area - clickable but NOT draggable
            <div
                class="card-content"
                style="
                    padding: 12px;
                    font-size: 12px;
                    color: #555;
                    cursor: pointer;
                    flex: 1;
                "
                on:click=on_content_click_handler
            >
                {content}

                // Optional button
                {has_button.then(|| {
                    view! {
                        <button
                            style="
                                margin-top: 10px;
                                padding: 6px 12px;
                                background: #2196f3;
                                color: white;
                                border: none;
                                border-radius: 4px;
                                cursor: pointer;
                                font-size: 11px;
                            "
                            on:click=on_button_click_handler
                        >
                            "Click Me!"
                        </button>
                    }
                })}
            </div>

            // Target handle (top of title bar)
            <Handle
                node_id=node_id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Top
                connection_mode=ConnectionMode::Strict
            />

            // Source handle (bottom)
            <Handle
                node_id=node_id.clone()
                r#type=HandleType::Source
                position=HandlePosition::Bottom
                connection_mode=ConnectionMode::Strict
            />
        </div>
    }
}
