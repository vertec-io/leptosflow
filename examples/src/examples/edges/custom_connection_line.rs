//! Custom Connection Line Example
//!
//! Demonstrates how to customize the connection preview line while dragging.
//! Shows different connection line styles:
//! - Dashed animated line
//! - Gradient colored line
//! - Custom path styles

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

use crate::shared::{get_drag_signal, DraggableNode};

/// Style options for the connection line
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionLineStyle {
    /// Default solid line
    Default,
    /// Dashed animated line
    Dashed,
    /// Gradient colored line
    Gradient,
    /// Thick glow line
    Glow,
}

impl ConnectionLineStyle {
    fn label(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Dashed => "Dashed",
            Self::Gradient => "Gradient",
            Self::Glow => "Glow",
        }
    }
}

/// Custom connection line example
#[component]
pub fn CustomConnectionLineExample() -> impl IntoView {
    // Create initial nodes with handles
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(50.0, 100.0))
            .with_data(json!({"label": "Source 1", "type": "input", "class": "light"})),
        Node::new("2".to_string(), Position::new(50.0, 250.0))
            .with_data(json!({"label": "Source 2", "type": "input", "class": "light"})),
        Node::new("3".to_string(), Position::new(350.0, 100.0))
            .with_data(json!({"label": "Target 1", "type": "output", "class": "light"})),
        Node::new("4".to_string(), Position::new(350.0, 250.0))
            .with_data(json!({"label": "Target 2", "type": "output", "class": "light"})),
    ];

    // Create one edge to show existing connection
    let initial_edges = vec![
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string())
            .with_label("Existing".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide the store to child components
    provide_context(store);

    // Connection line style selector
    let connection_style = RwSignal::new(ConnectionLineStyle::Default);

    // Connection log
    let connection_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        connection_log.update(|log| {
            log.insert(0, format!("{}", msg));
            if log.len() > 10 {
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

    // Handle background click to deselect
    let on_background_click = move |_ev: leptos::ev::MouseEvent| {
        store.clear_node_selection();
        store.clear_edge_selection();
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
                    // Standard edge renderer
                    <EdgeRenderer />

                    // Custom connection line - reactive to style changes
                    {move || {
                        let style = connection_style.get();
                        view! {
                            <CustomConnectionLineRenderer
                                store=store
                                style=style
                                add_log=add_log
                            />
                        }
                    }}

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

                // Controls (zoom buttons)
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); min-width: 220px;">
                        <strong style="display: block; margin-bottom: 8px;">"Custom Connection Line"</strong>
                        <p style="margin: 0 0 12px 0; font-size: 12px; color: #666;">
                            "Drag from a handle to see the custom connection line"
                        </p>

                        // Style selector
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Connection Line Style:"</div>
                            <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                                {[ConnectionLineStyle::Default, ConnectionLineStyle::Dashed, ConnectionLineStyle::Gradient, ConnectionLineStyle::Glow]
                                    .into_iter()
                                    .map(|style| {
                                        let style_for_click = style;
                                        let style_for_class = style;
                                        view! {
                                            <button
                                                style=move || {
                                                    let is_active = connection_style.get() == style_for_class;
                                                    format!(
                                                        "padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer; transition: all 0.2s; {}",
                                                        if is_active {
                                                            "background: #667eea; color: white; border: 1px solid #667eea;"
                                                        } else {
                                                            "background: white; color: #333; border: 1px solid #ddd;"
                                                        }
                                                    )
                                                }
                                                on:click=move |_| connection_style.set(style_for_click)
                                            >
                                                {style.label()}
                                            </button>
                                        }
                                    }).collect_view()
                                }
                            </div>
                        </div>

                        // Style preview
                        <div style="margin-bottom: 12px; padding: 8px; background: #f8f9fa; border-radius: 4px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 6px;">"Preview:"</div>
                            <svg width="200" height="40" style="display: block;">
                                <defs>
                                    <linearGradient id="preview-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                                        <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                                        <stop offset="100%" style="stop-color:#f093fb;stop-opacity:1" />
                                    </linearGradient>
                                </defs>
                                {move || {
                                    let path = "M 10 20 C 60 20, 140 20, 190 20";
                                    match connection_style.get() {
                                        ConnectionLineStyle::Default => view! {
                                            <path d=path fill="none" stroke="#667eea" stroke-width="2" />
                                        }.into_any(),
                                        ConnectionLineStyle::Dashed => view! {
                                            <path d=path fill="none" stroke="#667eea" stroke-width="2" stroke-dasharray="8,4" class="animated-dash-preview" />
                                        }.into_any(),
                                        ConnectionLineStyle::Gradient => view! {
                                            <path d=path fill="none" stroke="url(#preview-gradient)" stroke-width="2.5" />
                                        }.into_any(),
                                        ConnectionLineStyle::Glow => view! {
                                            <g>
                                                <path d=path fill="none" stroke="#667eea" stroke-width="8" stroke-opacity="0.3" stroke-linecap="round" />
                                                <path d=path fill="none" stroke="#667eea" stroke-width="2" />
                                            </g>
                                        }.into_any(),
                                    }
                                }}
                            </svg>
                        </div>

                        // Instructions
                        <div style="padding: 8px; background: #e3f2fd; border-radius: 4px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"How to use:"</div>
                            <ol style="font-size: 10px; margin: 0; padding-left: 16px; color: #666;">
                                <li>"Select a connection line style above"</li>
                                <li>"Drag from a source handle (bottom of source nodes)"</li>
                                <li>"See the custom styled connection preview"</li>
                                <li>"Drop on a target handle to create edge"</li>
                            </ol>
                        </div>

                        // Connection log
                        <div style="font-size: 11px; font-weight: 600; margin-bottom: 4px;">"Connection Log:"</div>
                        <div style="max-height: 80px; overflow-y: auto; font-size: 10px; color: #666;">
                            {move || {
                                let log = connection_log.get();
                                if log.is_empty() {
                                    view! { <div style="color: #999;">"Drag from a handle to start..."</div> }.into_any()
                                } else {
                                    log.iter().map(|entry| {
                                        view! { <div style="padding: 2px 0; border-bottom: 1px solid #eee;">{entry.clone()}</div> }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </Panel>

                // CSS for animated dashed line
                <style>
                    {"
                    @keyframes dash-animation-preview {
                        from { stroke-dashoffset: 24; }
                        to { stroke-dashoffset: 0; }
                    }
                    .animated-dash-preview {
                        animation: dash-animation-preview 0.5s linear infinite;
                    }
                    "}
                </style>
            </div>
        </div>
    }
}

/// Generate a bezier curve path for connection line
fn generate_connection_path(from: Position, to: Position) -> String {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    // Calculate control point offset based on distance
    let offset = (dx.abs() + dy.abs()) / 3.0;
    let offset = offset.max(50.0).min(150.0);

    // Bezier curve control points
    let ctrl1_x = from.x;
    let ctrl1_y = from.y + offset;
    let ctrl2_x = to.x;
    let ctrl2_y = to.y - offset;

    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y,
        ctrl1_x, ctrl1_y,
        ctrl2_x, ctrl2_y,
        to.x, to.y
    )
}

/// Custom connection line renderer component
#[component]
fn CustomConnectionLineRenderer<F>(
    store: FlowStore,
    style: ConnectionLineStyle,
    add_log: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + Sync + 'static,
{
    // Track if we've logged the connection start
    let logged_start = RwSignal::new(false);
    let add_log_clone = add_log.clone();

    // Get connection state from store
    let connection = move || store.state.connection_in_progress.get();

    // Reset logged_start when connection ends
    Effect::new(move |_| {
        if connection().is_none() {
            logged_start.set(false);
        }
    });

    view! {
        {move || {
            if let Some(conn) = connection() {
                // Log connection start once
                if !logged_start.get() {
                    logged_start.set(true);
                    add_log_clone(format!("Connection started from node {}", conn.from_node));
                }

                let path = generate_connection_path(conn.from_position, conn.to_position);

                // Render custom connection line based on style
                let content = match style {
                    ConnectionLineStyle::Default => view! {
                        <path
                            d=path.clone()
                            class="custom-connection-line"
                            fill="none"
                            stroke="#667eea"
                            stroke-width="2"
                            stroke-linecap="round"
                        />
                    }.into_any(),

                    ConnectionLineStyle::Dashed => view! {
                        <path
                            d=path.clone()
                            class="custom-connection-line animated-connection"
                            fill="none"
                            stroke="#667eea"
                            stroke-width="2"
                            stroke-dasharray="8,4"
                            stroke-linecap="round"
                        />
                    }.into_any(),

                    ConnectionLineStyle::Gradient => view! {
                        <path
                            d=path.clone()
                            class="custom-connection-line"
                            fill="none"
                            stroke="url(#connection-gradient)"
                            stroke-width="2.5"
                            stroke-linecap="round"
                        />
                    }.into_any(),

                    ConnectionLineStyle::Glow => view! {
                        <g class="custom-connection-line">
                            // Outer glow
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="#667eea"
                                stroke-width="10"
                                stroke-opacity="0.2"
                                stroke-linecap="round"
                            />
                            // Middle glow
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="#667eea"
                                stroke-width="5"
                                stroke-opacity="0.4"
                                stroke-linecap="round"
                            />
                            // Core line
                            <path
                                d=path.clone()
                                fill="none"
                                stroke="#667eea"
                                stroke-width="2"
                                stroke-linecap="round"
                            />
                        </g>
                    }.into_any(),
                };

                // Validity indicator
                let validity_indicator = if conn.is_valid {
                    view! {
                        <circle
                            cx=conn.to_position.x
                            cy=conn.to_position.y
                            r="6"
                            fill="#4ade80"
                            stroke="white"
                            stroke-width="2"
                        />
                    }.into_any()
                } else {
                    view! {
                        <circle
                            cx=conn.to_position.x
                            cy=conn.to_position.y
                            r="6"
                            fill="#f87171"
                            stroke="white"
                            stroke-width="2"
                        />
                    }.into_any()
                };

                view! {
                    <svg class="xyflow__custom-connectionline" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000; overflow: visible;">
                        <defs>
                            // Gradient for gradient style
                            <linearGradient id="connection-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
                                <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                                <stop offset="50%" style="stop-color:#764ba2;stop-opacity:1" />
                                <stop offset="100%" style="stop-color:#f093fb;stop-opacity:1" />
                            </linearGradient>
                        </defs>

                        // CSS for animation
                        <style>
                            {"
                            @keyframes connection-dash {
                                from { stroke-dashoffset: 24; }
                                to { stroke-dashoffset: 0; }
                            }
                            .animated-connection {
                                animation: connection-dash 0.5s linear infinite;
                            }
                            "}
                        </style>

                        <g class="xyflow__connection-line-group">
                            {content}
                            {validity_indicator}
                        </g>
                    </svg>
                }.into_any()
            } else {
                view! {}.into_any()
            }
        }}
    }
}
