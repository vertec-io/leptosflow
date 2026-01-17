//! Controlled vs Uncontrolled Viewport Example
//!
//! Demonstrates the difference between controlled and uncontrolled viewport modes:
//! - Side-by-side comparison of both modes
//! - Shows when to use each mode
//! - Toggle between modes to see behavior differences

use leptos::prelude::*;
use leptos::serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static CONTROLLED_DRAG_STATE: OnceLock<RwSignal<Option<ControlledDragState>>> = OnceLock::new();
static UNCONTROLLED_DRAG_STATE: OnceLock<RwSignal<Option<UncontrolledDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct ControlledDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

#[derive(Clone, Debug)]
struct UncontrolledDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_controlled_drag_signal() -> RwSignal<Option<ControlledDragState>> {
    *CONTROLLED_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

fn get_uncontrolled_drag_signal() -> RwSignal<Option<UncontrolledDragState>> {
    *UNCONTROLLED_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Event Log
// ============================================================================

#[derive(Clone, Debug)]
struct ViewportEvent {
    timestamp: f64,
    mode: String,
    event_type: String,
    details: String,
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Controlled vs Uncontrolled Viewport Example
#[component]
pub fn ControlledUncontrolledExample() -> impl IntoView {
    // Event log shared between both flows
    let event_log = RwSignal::new(Vec::<ViewportEvent>::new());

    // Helper to add log entry
    let add_log = move |mode: String, event_type: String, details: String| {
        event_log.update(|logs| {
            logs.push(ViewportEvent {
                timestamp: js_sys::Date::now(),
                mode,
                event_type,
                details,
            });
            // Keep last 15 entries
            if logs.len() > 15 {
                logs.remove(0);
            }
        });
    };

    // Current mode being highlighted
    let active_mode = RwSignal::new("both".to_string());

    view! {
        <div class="example-container" style="display: flex; flex-direction: column; height: 100%; position: relative;">
            // Main content area - split view
            <div style="display: flex; flex: 1; gap: 4px; padding: 4px; background: #f0f0f0;">
                // Left panel - Controlled mode
                <div style=move || format!(
                    "flex: 1; display: flex; flex-direction: column; opacity: {}; transition: opacity 0.3s;",
                    if active_mode.get() == "both" || active_mode.get() == "controlled" { "1" } else { "0.4" }
                )>
                    <div style="background: linear-gradient(135deg, #6ede87 0%, #4caf50 100%); color: white; padding: 10px 16px; font-weight: 600; font-size: 13px; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 12px; height: 12px; background: white; border-radius: 3px; display: flex; align-items: center; justify-content: center;">
                            <span style="font-size: 10px; color: #4caf50;">"C"</span>
                        </span>
                        "Controlled Mode"
                    </div>
                    <ControlledFlow add_log=add_log.clone() />
                </div>

                // Right panel - Uncontrolled mode
                <div style=move || format!(
                    "flex: 1; display: flex; flex-direction: column; opacity: {}; transition: opacity 0.3s;",
                    if active_mode.get() == "both" || active_mode.get() == "uncontrolled" { "1" } else { "0.4" }
                )>
                    <div style="background: linear-gradient(135deg, #6865A5 0%, #5c5999 100%); color: white; padding: 10px 16px; font-weight: 600; font-size: 13px; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 12px; height: 12px; background: white; border-radius: 3px; display: flex; align-items: center; justify-content: center;">
                            <span style="font-size: 10px; color: #6865A5;">"U"</span>
                        </span>
                        "Uncontrolled Mode"
                    </div>
                    <UncontrolledFlow add_log=add_log.clone() />
                </div>
            </div>

            // Header with mode explanations (positioned absolutely)
            <div style="position: absolute; top: 12px; left: 12px; z-index: 100;">
                <div style="background: white; padding: 12px 16px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); max-width: 280px;">
                    <h3 style="margin: 0 0 10px 0; font-size: 14px; color: #333; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 8px; height: 8px; background: #667eea; border-radius: 50%;"></span>
                        "Controlled vs Uncontrolled"
                    </h3>

                    <div style="font-size: 11px; color: #666; line-height: 1.5; margin-bottom: 12px;">
                        "Compare how viewport state is managed in each mode. Interact with both flows to see the differences."
                    </div>

                    // Mode highlight buttons
                    <div style="font-size: 11px; font-weight: 600; color: #555; margin-bottom: 6px;">"Highlight Mode"</div>
                    <div style="display: flex; gap: 6px;">
                        <button
                            style=move || format!(
                                "flex: 1; padding: 6px 10px; font-size: 10px; cursor: pointer; border-radius: 4px; border: 1px solid {}; background: {}; color: {};",
                                if active_mode.get() == "both" { "#667eea" } else { "#ddd" },
                                if active_mode.get() == "both" { "#667eea" } else { "#f5f5f5" },
                                if active_mode.get() == "both" { "white" } else { "#666" }
                            )
                            on:click=move |_| active_mode.set("both".to_string())
                        >
                            "Both"
                        </button>
                        <button
                            style=move || format!(
                                "flex: 1; padding: 6px 10px; font-size: 10px; cursor: pointer; border-radius: 4px; border: 1px solid {}; background: {}; color: {};",
                                if active_mode.get() == "controlled" { "#6ede87" } else { "#ddd" },
                                if active_mode.get() == "controlled" { "#6ede87" } else { "#f5f5f5" },
                                if active_mode.get() == "controlled" { "white" } else { "#666" }
                            )
                            on:click=move |_| active_mode.set("controlled".to_string())
                        >
                            "Controlled"
                        </button>
                        <button
                            style=move || format!(
                                "flex: 1; padding: 6px 10px; font-size: 10px; cursor: pointer; border-radius: 4px; border: 1px solid {}; background: {}; color: {};",
                                if active_mode.get() == "uncontrolled" { "#6865A5" } else { "#ddd" },
                                if active_mode.get() == "uncontrolled" { "#6865A5" } else { "#f5f5f5" },
                                if active_mode.get() == "uncontrolled" { "white" } else { "#666" }
                            )
                            on:click=move |_| active_mode.set("uncontrolled".to_string())
                        >
                            "Uncontrolled"
                        </button>
                    </div>
                </div>
            </div>

            // Bottom panel - Event log (positioned absolutely)
            <div style="position: absolute; bottom: 12px; right: 12px; z-index: 100;">
                <div style="background: white; padding: 12px 16px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); max-width: 400px;">
                    <h4 style="margin: 0 0 8px 0; font-size: 12px; color: #333;">"Event Log"</h4>
                    <div style="max-height: 120px; overflow-y: auto; font-size: 10px; font-family: monospace; background: #fafafa; border-radius: 4px; padding: 8px;">
                        {move || {
                            let logs = event_log.get();
                            if logs.is_empty() {
                                view! {
                                    <div style="color: #999; text-align: center; padding: 8px 0;">
                                        "Interact with either flow..."
                                    </div>
                                }.into_any()
                            } else {
                                logs.iter().rev().map(|event| {
                                    let (bg, border_color) = match event.mode.as_str() {
                                        "controlled" => ("#e8f5e9", "#6ede87"),
                                        "uncontrolled" => ("#ede7f6", "#6865A5"),
                                        _ => ("#f5f5f5", "#ddd"),
                                    };
                                    let mode = event.mode.clone();
                                    let event_type = event.event_type.clone();
                                    let details = event.details.clone();
                                    view! {
                                        <div style=format!(
                                            "margin-bottom: 4px; padding: 4px 8px; background: {}; border-left: 3px solid {}; border-radius: 0 3px 3px 0;",
                                            bg, border_color
                                        )>
                                            <span style=format!(
                                                "font-size: 9px; font-weight: 600; text-transform: uppercase; color: {};",
                                                border_color
                                            )>
                                                {mode}
                                            </span>
                                            <span style="color: #444; margin-left: 6px;">
                                                {event_type}
                                            </span>
                                            <span style="color: #888; margin-left: 4px;">
                                                {details}
                                            </span>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            // Comparison panel (positioned absolutely)
            <div style="position: absolute; bottom: 12px; left: 12px; z-index: 100;">
                <div style="background: white; padding: 12px 16px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); max-width: 320px;">
                    <h4 style="margin: 0 0 10px 0; font-size: 12px; color: #333;">"Mode Comparison"</h4>

                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 10px;">
                        // Controlled column
                        <div style="background: #e8f5e9; padding: 10px; border-radius: 6px; border: 1px solid #c8e6c9;">
                            <div style="font-weight: 600; color: #2e7d32; margin-bottom: 6px; display: flex; align-items: center; gap: 4px;">
                                <span style="width: 6px; height: 6px; background: #6ede87; border-radius: 50%;"></span>
                                "Controlled"
                            </div>
                            <div style="color: #555; line-height: 1.5;">
                                <div>"- External state source"</div>
                                <div>"- Predictable updates"</div>
                                <div>"- Full control"</div>
                                <div>"- Best for syncing UIs"</div>
                            </div>
                        </div>

                        // Uncontrolled column
                        <div style="background: #ede7f6; padding: 10px; border-radius: 6px; border: 1px solid #d1c4e9;">
                            <div style="font-weight: 600; color: #5e35b1; margin-bottom: 6px; display: flex; align-items: center; gap: 4px;">
                                <span style="width: 6px; height: 6px; background: #6865A5; border-radius: 50%;"></span>
                                "Uncontrolled"
                            </div>
                            <div style="color: #555; line-height: 1.5;">
                                <div>"- Internal state"</div>
                                <div>"- Simpler setup"</div>
                                <div>"- Less boilerplate"</div>
                                <div>"- Best for standalone"</div>
                            </div>
                        </div>
                    </div>

                    <div style="margin-top: 10px; padding: 8px; background: #fff3e0; border-radius: 4px; font-size: 10px; color: #e65100;">
                        <strong>"Tip:"</strong>" Use controlled when you need to sync viewport state with other UI elements or persist it."
                    </div>
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Controlled Flow Component
// ============================================================================

#[component]
fn ControlledFlow<F>(
    add_log: F,
) -> impl IntoView
where
    F: Fn(String, String, String) + Clone + Send + Sync + 'static,
{
    // CONTROLLED: External viewport state - we manage it
    let external_x = RwSignal::new(0.0_f64);
    let external_y = RwSignal::new(0.0_f64);
    let external_zoom = RwSignal::new(1.0_f64);

    // Create initial nodes
    let initial_nodes = vec![
        Node::new("c1".to_string(), Position::new(50.0, 30.0))
            .with_data(json!({"label": "Input", "type": "input", "color": "#6ede87"})),
        Node::new("c2".to_string(), Position::new(50.0, 130.0))
            .with_data(json!({"label": "Process", "type": "default", "color": "#6ede87"})),
        Node::new("c3".to_string(), Position::new(50.0, 230.0))
            .with_data(json!({"label": "Output", "type": "output", "color": "#6ede87"})),
    ];

    let initial_edges = vec![
        Edge::new("ce1-2".to_string(), "c1".to_string(), "c2".to_string()),
        Edge::new("ce2-3".to_string(), "c2".to_string(), "c3".to_string()),
    ];

    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // CONTROLLED: Effect syncs our external state TO the store
    Effect::new(move |_| {
        let x = external_x.get();
        let y = external_y.get();
        let zoom = external_zoom.get();
        store.set_viewport(Viewport { x, y, zoom });
    });

    let drag_signal = get_controlled_drag_signal();

    // Pan state
    let is_panning = RwSignal::new(false);
    let pan_start = RwSignal::new((0.0_f64, 0.0_f64));
    let pan_start_viewport = RwSignal::new((0.0_f64, 0.0_f64));

    // Mouse handlers for controlled flow
    let add_log_down = add_log.clone();
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        if ev.button() == 1 {
            ev.prevent_default();
            is_panning.set(true);
            pan_start.set((ev.client_x() as f64, ev.client_y() as f64));
            pan_start_viewport.set((external_x.get(), external_y.get()));
            add_log_down("controlled".to_string(), "pan_start".to_string(), "External state initiated".to_string());
        }
    };

    let add_log_move = add_log.clone();
    let on_mousemove = move |ev: leptos::ev::MouseEvent| {
        // Handle node drag
        if let Some(drag_state) = drag_signal.get() {
            let zoom = external_zoom.get();
            let dx = (ev.client_x() as f64 - drag_state.start_mouse.0) / zoom;
            let dy = (ev.client_y() as f64 - drag_state.start_mouse.1) / zoom;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(drag_state.start_pos.0 + dx, drag_state.start_pos.1 + dy);
            });
        }

        // CONTROLLED: Panning updates external state, which triggers Effect
        if is_panning.get() {
            let (start_x, start_y) = pan_start.get();
            let (vp_start_x, vp_start_y) = pan_start_viewport.get();

            let dx = ev.client_x() as f64 - start_x;
            let dy = ev.client_y() as f64 - start_y;

            external_x.set(vp_start_x + dx);
            external_y.set(vp_start_y + dy);
        }
    };

    let add_log_up = add_log.clone();
    let on_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(ds) = drag_signal.get() {
            store.update_node(&ds.node_id, |n| n.dragging = false);
            drag_signal.set(None);
        }

        if is_panning.get() {
            is_panning.set(false);
            add_log_up(
                "controlled".to_string(),
                "pan_end".to_string(),
                format!("({:.0}, {:.0})", external_x.get(), external_y.get()),
            );
        }
    };

    let add_log_wheel = add_log.clone();
    let on_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();
        let delta = if ev.delta_y() > 0.0 { -0.1 } else { 0.1 };
        let new_zoom = (external_zoom.get() + delta).clamp(0.1, 4.0);
        // CONTROLLED: Update external state, Effect propagates to store
        external_zoom.set(new_zoom);
        add_log_wheel("controlled".to_string(), "zoom".to_string(), format!("{:.2}x", new_zoom));
    };

    view! {
        <div
            class="xyflow leptos-flow controlled-flow"
            style="flex: 1; position: relative; background: white;"
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:mouseleave=move |_| {
                if is_panning.get() { is_panning.set(false); }
                if drag_signal.get().is_some() {
                    if let Some(ds) = drag_signal.get() {
                        store.update_node(&ds.node_id, |n| n.dragging = false);
                    }
                    drag_signal.set(None);
                }
            }
            on:wheel=on_wheel
        >
            <Background variant=BackgroundVariant::Dots />

            <FlowViewport store=store>
                <ControlledEdgeRenderer store=store />

                {move || {
                    store.get_nodes().into_iter().map(|node| {
                        view! { <ControlledNode node=node.clone() store=store /> }
                    }).collect_view()
                }}
            </FlowViewport>

            // Viewport state display
            <div style="position: absolute; top: 8px; right: 8px; background: rgba(255,255,255,0.95); padding: 8px 12px; border-radius: 6px; font-size: 10px; font-family: monospace; border: 1px solid #c8e6c9;">
                <div style="font-weight: 600; color: #2e7d32; margin-bottom: 4px;">"External State"</div>
                <div style="color: #555;">
                    "x: " {move || format!("{:.0}", external_x.get())}
                    " y: " {move || format!("{:.0}", external_y.get())}
                </div>
                <div style="color: #555;">
                    "zoom: " {move || format!("{:.2}x", external_zoom.get())}
                </div>
            </div>

            // Quick controls
            <div style="position: absolute; bottom: 8px; right: 8px; display: flex; gap: 4px;">
                <button
                    style="padding: 4px 8px; font-size: 9px; background: #6ede87; color: white; border: none; border-radius: 4px; cursor: pointer;"
                    on:click={
                        let add_log = add_log.clone();
                        move |_| {
                            external_x.set(0.0);
                            external_y.set(0.0);
                            external_zoom.set(1.0);
                            add_log("controlled".to_string(), "reset".to_string(), "State reset".to_string());
                        }
                    }
                >
                    "Reset"
                </button>
                <button
                    style="padding: 4px 8px; font-size: 9px; background: #4caf50; color: white; border: none; border-radius: 4px; cursor: pointer;"
                    on:click={
                        let add_log = add_log.clone();
                        move |_| {
                            external_zoom.set(1.5);
                            add_log("controlled".to_string(), "zoom".to_string(), "1.5x".to_string());
                        }
                    }
                >
                    "1.5x"
                </button>
            </div>
        </div>
    }
}

// ============================================================================
// Uncontrolled Flow Component
// ============================================================================

#[component]
fn UncontrolledFlow<F>(
    add_log: F,
) -> impl IntoView
where
    F: Fn(String, String, String) + Clone + Send + Sync + 'static,
{
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("u1".to_string(), Position::new(50.0, 30.0))
            .with_data(json!({"label": "Input", "type": "input", "color": "#6865A5"})),
        Node::new("u2".to_string(), Position::new(50.0, 130.0))
            .with_data(json!({"label": "Process", "type": "default", "color": "#6865A5"})),
        Node::new("u3".to_string(), Position::new(50.0, 230.0))
            .with_data(json!({"label": "Output", "type": "output", "color": "#6865A5"})),
    ];

    let initial_edges = vec![
        Edge::new("ue1-2".to_string(), "u1".to_string(), "u2".to_string()),
        Edge::new("ue2-3".to_string(), "u2".to_string(), "u3".to_string()),
    ];

    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // UNCONTROLLED: No external state - just read from store when needed
    // Store manages viewport internally

    let drag_signal = get_uncontrolled_drag_signal();

    // Pan state - but updates store directly
    let is_panning = RwSignal::new(false);
    let pan_start = RwSignal::new((0.0_f64, 0.0_f64));
    let pan_start_viewport = RwSignal::new((0.0_f64, 0.0_f64));

    // Mouse handlers for uncontrolled flow
    let add_log_down = add_log.clone();
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        if ev.button() == 1 {
            ev.prevent_default();
            is_panning.set(true);
            pan_start.set((ev.client_x() as f64, ev.client_y() as f64));
            let vp = store.get_viewport();
            pan_start_viewport.set((vp.x, vp.y));
            add_log_down("uncontrolled".to_string(), "pan_start".to_string(), "Direct store update".to_string());
        }
    };

    let on_mousemove = move |ev: leptos::ev::MouseEvent| {
        // Handle node drag
        if let Some(drag_state) = drag_signal.get() {
            let vp = store.get_viewport();
            let dx = (ev.client_x() as f64 - drag_state.start_mouse.0) / vp.zoom;
            let dy = (ev.client_y() as f64 - drag_state.start_mouse.1) / vp.zoom;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(drag_state.start_pos.0 + dx, drag_state.start_pos.1 + dy);
            });
        }

        // UNCONTROLLED: Directly update store viewport
        if is_panning.get() {
            let (start_x, start_y) = pan_start.get();
            let (vp_start_x, vp_start_y) = pan_start_viewport.get();

            let dx = ev.client_x() as f64 - start_x;
            let dy = ev.client_y() as f64 - start_y;

            let current = store.get_viewport();
            store.set_viewport(Viewport {
                x: vp_start_x + dx,
                y: vp_start_y + dy,
                zoom: current.zoom,
            });
        }
    };

    let add_log_up = add_log.clone();
    let on_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(ds) = drag_signal.get() {
            store.update_node(&ds.node_id, |n| n.dragging = false);
            drag_signal.set(None);
        }

        if is_panning.get() {
            is_panning.set(false);
            let vp = store.get_viewport();
            add_log_up(
                "uncontrolled".to_string(),
                "pan_end".to_string(),
                format!("({:.0}, {:.0})", vp.x, vp.y),
            );
        }
    };

    let add_log_wheel = add_log.clone();
    let on_wheel = move |ev: leptos::ev::WheelEvent| {
        ev.prevent_default();
        let current = store.get_viewport();
        let delta = if ev.delta_y() > 0.0 { -0.1 } else { 0.1 };
        let new_zoom = (current.zoom + delta).clamp(0.1, 4.0);
        // UNCONTROLLED: Update store directly
        store.set_viewport(Viewport {
            x: current.x,
            y: current.y,
            zoom: new_zoom,
        });
        add_log_wheel("uncontrolled".to_string(), "zoom".to_string(), format!("{:.2}x", new_zoom));
    };

    view! {
        <div
            class="xyflow leptos-flow uncontrolled-flow"
            style="flex: 1; position: relative; background: white;"
            on:mousedown=on_mousedown
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
            on:mouseleave=move |_| {
                if is_panning.get() { is_panning.set(false); }
                if drag_signal.get().is_some() {
                    if let Some(ds) = drag_signal.get() {
                        store.update_node(&ds.node_id, |n| n.dragging = false);
                    }
                    drag_signal.set(None);
                }
            }
            on:wheel=on_wheel
        >
            <Background variant=BackgroundVariant::Lines />

            <FlowViewport store=store>
                <UncontrolledEdgeRenderer store=store />

                {move || {
                    store.get_nodes().into_iter().map(|node| {
                        view! { <UncontrolledNode node=node.clone() store=store /> }
                    }).collect_view()
                }}
            </FlowViewport>

            // Viewport state display (reads from store)
            <div style="position: absolute; top: 8px; right: 8px; background: rgba(255,255,255,0.95); padding: 8px 12px; border-radius: 6px; font-size: 10px; font-family: monospace; border: 1px solid #d1c4e9;">
                <div style="font-weight: 600; color: #5e35b1; margin-bottom: 4px;">"Store State"</div>
                {move || {
                    let vp = store.get_viewport();
                    view! {
                        <>
                            <div style="color: #555;">
                                "x: " {format!("{:.0}", vp.x)}
                                " y: " {format!("{:.0}", vp.y)}
                            </div>
                            <div style="color: #555;">
                                "zoom: " {format!("{:.2}x", vp.zoom)}
                            </div>
                        </>
                    }
                }}
            </div>

            // Quick controls
            <div style="position: absolute; bottom: 8px; right: 8px; display: flex; gap: 4px;">
                <button
                    style="padding: 4px 8px; font-size: 9px; background: #6865A5; color: white; border: none; border-radius: 4px; cursor: pointer;"
                    on:click={
                        let add_log = add_log.clone();
                        move |_| {
                            store.set_viewport(Viewport { x: 0.0, y: 0.0, zoom: 1.0 });
                            add_log("uncontrolled".to_string(), "reset".to_string(), "Store reset".to_string());
                        }
                    }
                >
                    "Reset"
                </button>
                <button
                    style="padding: 4px 8px; font-size: 9px; background: #5c5999; color: white; border: none; border-radius: 4px; cursor: pointer;"
                    on:click={
                        let add_log = add_log.clone();
                        move |_| {
                            let vp = store.get_viewport();
                            store.set_viewport(Viewport { x: vp.x, y: vp.y, zoom: 1.5 });
                            add_log("uncontrolled".to_string(), "zoom".to_string(), "1.5x".to_string());
                        }
                    }
                >
                    "1.5x"
                </button>
            </div>
        </div>
    }
}

// ============================================================================
// Controlled Node Component
// ============================================================================

#[component]
fn ControlledNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

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
        .unwrap_or("#6ede87")
        .to_string();

    let has_source = node_type != "output";
    let has_target = node_type != "input";

    let drag_signal = get_controlled_drag_signal();

    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(ControlledDragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (n.position.x, n.position.y),
            }));

            store.update_node(&node_id, |n| n.dragging = true);
        }
    };

    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    let color_border = color.clone();

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
                    "background: {}; border: 2px solid {}; border-radius: 6px; padding: 8px 14px; min-width: 60px; text-align: center;",
                    color, color_border
                )
            >
                {has_target.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 11px;">
                    {label}
                </div>

                {has_source.then(|| {
                    let node_id_s = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id_s
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
// Uncontrolled Node Component
// ============================================================================

#[component]
fn UncontrolledNode(
    node: Node,
    store: FlowStore,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

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

    let drag_signal = get_uncontrolled_drag_signal();

    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(UncontrolledDragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (n.position.x, n.position.y),
            }));

            store.update_node(&node_id, |n| n.dragging = true);
        }
    };

    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    let color_border = color.clone();

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
                    "background: {}; border: 2px solid {}; border-radius: 6px; padding: 8px 14px; min-width: 60px; text-align: center;",
                    color, color_border
                )
            >
                {has_target.then(|| view! {
                    <Handle
                        node_id=node.id.clone()
                        r#type=HandleType::Target
                        position=HandlePosition::Top
                        connection_mode=ConnectionMode::Strict
                    />
                })}

                <div style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 11px;">
                    {label}
                </div>

                {has_source.then(|| {
                    let node_id_s = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id_s
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
// Edge Renderers
// ============================================================================

#[component]
fn ControlledEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="controlled-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#6ede87;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#4caf50;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="controlled-arrow"
                    markerWidth="10"
                    markerHeight="10"
                    refX="9"
                    refY="5"
                    orient="auto"
                    markerUnits="userSpaceOnUse"
                >
                    <path d="M2,2 L9,5 L2,8 L4,5 Z" fill="#4caf50" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.iter().map(|edge| {
                    let source = nodes.iter().find(|n| n.id == edge.source);
                    let target = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(s), Some(t)) = (source, target) {
                        let sx = s.position.x + 47.0;
                        let sy = s.position.y + 40.0;
                        let tx = t.position.x + 47.0;
                        let ty = t.position.y;

                        let ctrl = (ty - sy).abs() * 0.5;
                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            sx, sy, sx, sy + ctrl, tx, ty - ctrl, tx, ty
                        );

                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke="url(#controlled-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#controlled-arrow)"
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

#[component]
fn UncontrolledEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="uncontrolled-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#6865A5;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#5c5999;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="uncontrolled-arrow"
                    markerWidth="10"
                    markerHeight="10"
                    refX="9"
                    refY="5"
                    orient="auto"
                    markerUnits="userSpaceOnUse"
                >
                    <path d="M2,2 L9,5 L2,8 L4,5 Z" fill="#5c5999" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();

                edges.iter().map(|edge| {
                    let source = nodes.iter().find(|n| n.id == edge.source);
                    let target = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(s), Some(t)) = (source, target) {
                        let sx = s.position.x + 47.0;
                        let sy = s.position.y + 40.0;
                        let tx = t.position.x + 47.0;
                        let ty = t.position.y;

                        let ctrl = (ty - sy).abs() * 0.5;
                        let path = format!(
                            "M {} {} C {} {}, {} {}, {} {}",
                            sx, sy, sx, sy + ctrl, tx, ty - ctrl, tx, ty
                        );

                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke="url(#uncontrolled-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#uncontrolled-arrow)"
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
