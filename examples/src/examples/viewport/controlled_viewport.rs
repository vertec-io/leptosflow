//! Controlled Viewport Example
//!
//! Demonstrates how to programmatically control the viewport:
//! - External state controls viewport (x, y, zoom)
//! - UI controls to set viewport values
//! - Two-way binding (dragging updates external state)

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static CONTROLLED_VIEWPORT_DRAG_STATE: OnceLock<RwSignal<Option<ControlledViewportDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct ControlledViewportDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_drag_signal() -> RwSignal<Option<ControlledViewportDragState>> {
    *CONTROLLED_VIEWPORT_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Action Log
// ============================================================================

#[derive(Clone, Debug)]
struct ViewportAction {
    timestamp: f64,
    action_type: String,
    details: String,
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Controlled Viewport Example
#[component]
pub fn ControlledViewportExample() -> impl IntoView {
    // External viewport state - this controls the viewport
    let external_x = RwSignal::new(0.0_f64);
    let external_y = RwSignal::new(0.0_f64);
    let external_zoom = RwSignal::new(1.0_f64);

    // Action log
    let action_log = RwSignal::new(Vec::<ViewportAction>::new());

    // Change counter
    let change_count = RwSignal::new(0_i32);

    // Helper to add log entry
    let add_log = move |action_type: String, details: String| {
        action_log.update(|logs| {
            logs.push(ViewportAction {
                timestamp: js_sys::Date::now(),
                action_type,
                details,
            });
            // Keep last 10 entries
            if logs.len() > 10 {
                logs.remove(0);
            }
        });
        change_count.update(|c| *c += 1);
    };

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({"label": "Node A", "type": "input", "color": "#6ede87"})),
        Node::new("2".to_string(), Position::new(100.0, 200.0))
            .with_data(json!({"label": "Node B", "type": "default", "color": "#6865A5"})),
        Node::new("3".to_string(), Position::new(300.0, 125.0))
            .with_data(json!({"label": "Node C", "type": "default", "color": "#6865A5"})),
        Node::new("4".to_string(), Position::new(300.0, 275.0))
            .with_data(json!({"label": "Node D", "type": "output", "color": "#ff6b6b"})),
        Node::new("5".to_string(), Position::new(500.0, 200.0))
            .with_data(json!({"label": "Node E", "type": "output", "color": "#ff6b6b"})),
    ];

    // Create initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
        Edge::new("e1-3".to_string(), "1".to_string(), "3".to_string()),
        Edge::new("e2-4".to_string(), "2".to_string(), "4".to_string()),
        Edge::new("e3-4".to_string(), "3".to_string(), "4".to_string()),
        Edge::new("e3-5".to_string(), "3".to_string(), "5".to_string()),
        Edge::new("e4-5".to_string(), "4".to_string(), "5".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Sync external state to store viewport
    Effect::new(move |_| {
        let x = external_x.get();
        let y = external_y.get();
        let zoom = external_zoom.get();
        store.set_viewport(Viewport { x, y, zoom });
    });

    // Drag signal
    let drag_signal = get_drag_signal();

    // Pan state for viewport dragging
    let is_panning = RwSignal::new(false);
    let pan_start = RwSignal::new((0.0_f64, 0.0_f64));
    let pan_start_viewport = RwSignal::new((0.0_f64, 0.0_f64));

    // Mouse down handler for canvas (start panning)
    let add_log_for_pan = add_log.clone();
    let on_canvas_mousedown = move |ev: leptos::ev::MouseEvent| {
        // Only pan on middle-click or when clicking empty space (not on nodes)
        if ev.button() == 1 {
            // Middle mouse button - always pan
            ev.prevent_default();
            is_panning.set(true);
            pan_start.set((ev.client_x() as f64, ev.client_y() as f64));
            pan_start_viewport.set((external_x.get(), external_y.get()));
            add_log_for_pan(
                "pan_start".to_string(),
                "Middle-click pan initiated".to_string(),
            );
        }
    };

    // Mouse move handler
    let on_canvas_mousemove = move |ev: leptos::ev::MouseEvent| {
        // Handle node drag
        if let Some(drag_state) = drag_signal.get() {
            let zoom = external_zoom.get();
            let dx = (ev.client_x() as f64 - drag_state.start_mouse.0) / zoom;
            let dy = (ev.client_y() as f64 - drag_state.start_mouse.1) / zoom;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(drag_state.start_pos.0 + dx, drag_state.start_pos.1 + dy);
            });
        }

        // Handle panning
        if is_panning.get() {
            let (start_x, start_y) = pan_start.get();
            let (vp_start_x, vp_start_y) = pan_start_viewport.get();

            let dx = ev.client_x() as f64 - start_x;
            let dy = ev.client_y() as f64 - start_y;

            external_x.set(vp_start_x + dx);
            external_y.set(vp_start_y + dy);
        }
    };

    // Mouse up handler
    let add_log_for_up = add_log.clone();
    let on_canvas_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            store.update_node(&drag_state.node_id, |n| {
                n.dragging = false;
            });
            drag_signal.set(None);
        }

        if is_panning.get() {
            is_panning.set(false);
            let vp = store.get_viewport();
            add_log_for_up(
                "pan_end".to_string(),
                format!("Position: ({:.0}, {:.0})", vp.x, vp.y),
            );
        }
    };

    // Mouse wheel handler for zoom
    let add_log_for_wheel = add_log.clone();
    let on_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();

        let delta = if ev.delta_y() > 0.0 { -0.1 } else { 0.1 };
        let new_zoom = (external_zoom.get() + delta).clamp(0.1, 4.0);
        external_zoom.set(new_zoom);

        add_log_for_wheel(
            "zoom".to_string(),
            format!("Zoom: {:.2}x", new_zoom),
        );
    };

    // Preset viewport positions
    let add_log_preset = add_log.clone();
    let set_preset = move |name: &str, x: f64, y: f64, zoom: f64| {
        external_x.set(x);
        external_y.set(y);
        external_zoom.set(zoom);
        add_log_preset(
            "preset".to_string(),
            format!("{}: ({:.0}, {:.0}) @ {:.1}x", name, x, y, zoom),
        );
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow controlled-viewport-example"
                 style="width: 100%; height: 100%; position: relative;"
                 on:mousedown=on_canvas_mousedown
                 on:mousemove=on_canvas_mousemove
                 on:mouseup=on_canvas_mouseup
                 on:mouseleave=move |_| {
                     if is_panning.get() {
                         is_panning.set(false);
                     }
                     if drag_signal.get().is_some() {
                         if let Some(ds) = drag_signal.get() {
                             store.update_node(&ds.node_id, |n| n.dragging = false);
                         }
                         drag_signal.set(None);
                     }
                 }
                 on:wheel=on_wheel
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Render edges
                    <ControlledViewportEdgeRenderer store=store />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <ControlledViewportNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Viewport Control Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; max-width: 340px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333; display: flex; align-items: center; gap: 8px;">
                            <span style="display: inline-block; width: 8px; height: 8px; background: #667eea; border-radius: 50%;"></span>
                            "Controlled Viewport"
                        </h3>

                        // Current viewport state display
                        <div style="margin-bottom: 16px; padding: 12px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px; color: white;">
                            <div style="font-size: 11px; opacity: 0.8; margin-bottom: 6px;">"Current Viewport State"</div>
                            <div style="font-family: monospace; font-size: 13px; display: grid; grid-template-columns: repeat(3, 1fr); gap: 8px;">
                                <div>
                                    <div style="font-size: 10px; opacity: 0.7;">"X"</div>
                                    <div style="font-weight: 600;">{move || format!("{:.0}", external_x.get())}</div>
                                </div>
                                <div>
                                    <div style="font-size: 10px; opacity: 0.7;">"Y"</div>
                                    <div style="font-weight: 600;">{move || format!("{:.0}", external_y.get())}</div>
                                </div>
                                <div>
                                    <div style="font-size: 10px; opacity: 0.7;">"Zoom"</div>
                                    <div style="font-weight: 600;">{move || format!("{:.2}x", external_zoom.get())}</div>
                                </div>
                            </div>
                        </div>

                        // Position controls
                        <div style="margin-bottom: 12px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Position Controls"</div>
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                                <div>
                                    <label style="font-size: 11px; color: #777; display: block; margin-bottom: 4px;">"X Position"</label>
                                    <input
                                        type="number"
                                        step="10"
                                        style="width: 100%; padding: 6px 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 13px;"
                                        prop:value=move || external_x.get()
                                        on:input={
                                            let add_log = add_log.clone();
                                            move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    external_x.set(val);
                                                    add_log("input".to_string(), format!("X set to {:.0}", val));
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <div>
                                    <label style="font-size: 11px; color: #777; display: block; margin-bottom: 4px;">"Y Position"</label>
                                    <input
                                        type="number"
                                        step="10"
                                        style="width: 100%; padding: 6px 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 13px;"
                                        prop:value=move || external_y.get()
                                        on:input={
                                            let add_log = add_log.clone();
                                            move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                                    external_y.set(val);
                                                    add_log("input".to_string(), format!("Y set to {:.0}", val));
                                                }
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        </div>

                        // Zoom control
                        <div style="margin-bottom: 12px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
                                <label style="font-size: 11px; color: #777;">"Zoom Level"</label>
                                <span style="font-size: 12px; font-weight: 600; color: #667eea;">{move || format!("{:.2}x", external_zoom.get())}</span>
                            </div>
                            <input
                                type="range"
                                min="0.1"
                                max="4"
                                step="0.1"
                                style="width: 100%; cursor: pointer;"
                                prop:value=move || external_zoom.get()
                                on:input={
                                    let add_log = add_log.clone();
                                    move |ev| {
                                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                            external_zoom.set(val);
                                            add_log("slider".to_string(), format!("Zoom set to {:.2}x", val));
                                        }
                                    }
                                }
                            />
                            <div style="display: flex; justify-content: space-between; font-size: 10px; color: #999;">
                                <span>"0.1x"</span>
                                <span>"1x"</span>
                                <span>"4x"</span>
                            </div>
                        </div>

                        // Quick zoom buttons
                        <div style="display: flex; gap: 6px; margin-bottom: 16px;">
                            <button
                                style="flex: 1; padding: 6px; font-size: 11px; cursor: pointer; background: #f5f5f5; border: 1px solid #ddd; border-radius: 4px;"
                                on:click={
                                    let add_log = add_log.clone();
                                    move |_| {
                                        external_zoom.set(0.5);
                                        add_log("button".to_string(), "Zoom: 0.5x".to_string());
                                    }
                                }
                            >
                                "0.5x"
                            </button>
                            <button
                                style="flex: 1; padding: 6px; font-size: 11px; cursor: pointer; background: #667eea; color: white; border: none; border-radius: 4px;"
                                on:click={
                                    let add_log = add_log.clone();
                                    move |_| {
                                        external_zoom.set(1.0);
                                        add_log("button".to_string(), "Zoom: 1.0x".to_string());
                                    }
                                }
                            >
                                "1x"
                            </button>
                            <button
                                style="flex: 1; padding: 6px; font-size: 11px; cursor: pointer; background: #f5f5f5; border: 1px solid #ddd; border-radius: 4px;"
                                on:click={
                                    let add_log = add_log.clone();
                                    move |_| {
                                        external_zoom.set(1.5);
                                        add_log("button".to_string(), "Zoom: 1.5x".to_string());
                                    }
                                }
                            >
                                "1.5x"
                            </button>
                            <button
                                style="flex: 1; padding: 6px; font-size: 11px; cursor: pointer; background: #f5f5f5; border: 1px solid #ddd; border-radius: 4px;"
                                on:click={
                                    let add_log = add_log.clone();
                                    move |_| {
                                        external_zoom.set(2.0);
                                        add_log("button".to_string(), "Zoom: 2.0x".to_string());
                                    }
                                }
                            >
                                "2x"
                            </button>
                        </div>

                        // Preset positions
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Preset Positions"</div>
                            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 6px;">
                                <button
                                    style="padding: 8px; font-size: 11px; cursor: pointer; background: #e8f5e9; border: 1px solid #c8e6c9; border-radius: 4px; color: #2e7d32;"
                                    on:click=move |_| set_preset("Center", 0.0, 0.0, 1.0)
                                >
                                    "Center (0, 0)"
                                </button>
                                <button
                                    style="padding: 8px; font-size: 11px; cursor: pointer; background: #e3f2fd; border: 1px solid #bbdefb; border-radius: 4px; color: #1565c0;"
                                    on:click=move |_| set_preset("Node A", -50.0, 0.0, 1.5)
                                >
                                    "Focus Node A"
                                </button>
                                <button
                                    style="padding: 8px; font-size: 11px; cursor: pointer; background: #f3e5f5; border: 1px solid #e1bee7; border-radius: 4px; color: #7b1fa2;"
                                    on:click=move |_| set_preset("Node E", -350.0, -100.0, 1.5)
                                >
                                    "Focus Node E"
                                </button>
                                <button
                                    style="padding: 8px; font-size: 11px; cursor: pointer; background: #fff3e0; border: 1px solid #ffe0b2; border-radius: 4px; color: #e65100;"
                                    on:click=move |_| set_preset("Overview", -100.0, -50.0, 0.6)
                                >
                                    "Overview"
                                </button>
                            </div>
                        </div>

                        // Change counter
                        <div style="display: flex; justify-content: space-between; align-items: center; padding: 8px 12px; background: #f8f9fa; border-radius: 6px; margin-bottom: 12px;">
                            <span style="font-size: 12px; color: #666;">"Total Changes"</span>
                            <span style="font-size: 16px; font-weight: 700; color: #667eea;">
                                {move || change_count.get()}
                            </span>
                        </div>

                        // Action Log
                        <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Change Log"</div>
                        <div style="max-height: 120px; overflow-y: auto; font-size: 11px; font-family: monospace; background: #fafafa; border-radius: 4px; padding: 8px;">
                            {move || {
                                let logs = action_log.get();
                                if logs.is_empty() {
                                    view! {
                                        <div style="color: #999; text-align: center; padding: 12px 0;">
                                            "Interact with viewport..."
                                        </div>
                                    }.into_any()
                                } else {
                                    logs.iter().rev().map(|action| {
                                        let (bg, color) = match action.action_type.as_str() {
                                            "input" => ("#e3f2fd", "#1565c0"),
                                            "slider" => ("#f3e5f5", "#7b1fa2"),
                                            "button" => ("#e8f5e9", "#2e7d32"),
                                            "preset" => ("#fff3e0", "#e65100"),
                                            "pan_start" | "pan_end" => ("#ede7f6", "#5e35b1"),
                                            "zoom" => ("#fce4ec", "#c2185b"),
                                            _ => ("#fafafa", "#666"),
                                        };
                                        let action_type = action.action_type.clone();
                                        let details = action.details.clone();
                                        view! {
                                            <div style=format!(
                                                "margin-bottom: 4px; padding: 4px 6px; background: {}; border-radius: 3px;",
                                                bg
                                            )>
                                                <span style=format!("color: {}; text-transform: uppercase; font-size: 9px; font-weight: 600;", color)>
                                                    {action_type}
                                                </span>
                                                <span style="color: #666; margin-left: 6px;">
                                                    {details}
                                                </span>
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }
                            }}
                        </div>
                    </div>
                </Panel>

                // Instructions badge
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 10px 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <div style="color: white; font-size: 11px; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 4px;">"Two-Way Binding"</div>
                            <div style="opacity: 0.9;">"• Scroll to zoom"</div>
                            <div style="opacity: 0.9;">"• Middle-click to pan"</div>
                            <div style="opacity: 0.9;">"• Use controls to update"</div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// Controlled Viewport Node Component
// ============================================================================

#[component]
fn ControlledViewportNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
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
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6865A5")
        .to_string();

    let has_source = node_type != "output";
    let has_target = node_type != "input";

    let drag_signal = get_drag_signal();

    // Mouse down handler
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(ControlledViewportDragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (n.position.x, n.position.y),
            }));

            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
        }
    };

    // Get reactive position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    let color_for_border = color.clone();

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
                class="xyflow__node-default light"
                style=format!(
                    "background: {}; border: 2px solid {}; border-radius: 8px; padding: 12px 18px; min-width: 80px; text-align: center;",
                    color, color_for_border
                )
            >
                // Target handle
                {has_target.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 13px;">
                    {label}
                </div>

                // Source handle
                {has_source.then(|| {
                    let node_id_for_source = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id_for_source
                            r#type=HandleType::Source
                            position=HandlePosition::Bottom
                            connection_mode=ConnectionMode::Strict
                        />
                    }
                })}
            </div>
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

#[component]
fn ControlledViewportEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="controlled-viewport-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="controlled-viewport-arrow"
                    markerWidth="12"
                    markerHeight="12"
                    refX="10"
                    refY="6"
                    orient="auto"
                    markerUnits="userSpaceOnUse"
                >
                    <path d="M2,2 L10,6 L2,10 L4,6 Z" fill="#764ba2" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.iter().map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        let source_x = source.position.x + 60.0;
                        let source_y = source.position.y + 50.0;
                        let target_x = target.position.x + 60.0;
                        let target_y = target.position.y;

                        let ctrl_offset = (target_y - source_y).abs() * 0.5;
                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            source_x, source_y,
                            source_x, source_y + ctrl_offset,
                            target_x, target_y - ctrl_offset,
                            target_x, target_y
                        );

                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke="url(#controlled-viewport-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#controlled-viewport-arrow)"
                            />
                        }.into_any()
                    } else {
                        view! { <g></g> }.into_any()
                    }
                }).collect_view()
            }}
        </svg>
    }
}
