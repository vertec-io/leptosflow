//! A11y (Accessibility) Example
//!
//! Demonstrates accessibility features for keyboard and screen reader users:
//! - ARIA labels on nodes and edges
//! - Keyboard navigation between nodes (Tab, Arrow keys)
//! - Screen reader announcements for interactions
//! - Focus indicators

use leptos::prelude::*;
use leptos::serde_json::json;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Global drag state for A11y example
static A11Y_DRAG_STATE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

fn get_a11y_drag_signal() -> RwSignal<Option<DragState>> {
    *A11Y_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Focused node ID
static FOCUSED_NODE: std::sync::OnceLock<RwSignal<Option<String>>> = std::sync::OnceLock::new();

fn get_focused_node_signal() -> RwSignal<Option<String>> {
    *FOCUSED_NODE.get_or_init(|| RwSignal::new(None))
}

/// Screen reader announcement queue
static ANNOUNCEMENTS: std::sync::OnceLock<RwSignal<Vec<String>>> = std::sync::OnceLock::new();

fn get_announcements_signal() -> RwSignal<Vec<String>> {
    *ANNOUNCEMENTS.get_or_init(|| RwSignal::new(vec![]))
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Add a screen reader announcement
fn announce(message: &str) {
    let announcements = get_announcements_signal();
    announcements.update(|list| {
        list.push(message.to_string());
        if list.len() > 10 {
            list.remove(0);
        }
    });
}

/// Find adjacent node in a direction
fn find_adjacent_node(
    nodes: &[Node],
    current_id: &str,
    direction: &str,
) -> Option<String> {
    let current = nodes.iter().find(|n| n.id == current_id)?;
    let current_x = current.position.x;
    let current_y = current.position.y;

    // Find nodes in the specified direction
    let candidates: Vec<&Node> = match direction {
        "up" => nodes.iter()
            .filter(|n| n.id != current_id && n.position.y < current_y)
            .collect(),
        "down" => nodes.iter()
            .filter(|n| n.id != current_id && n.position.y > current_y)
            .collect(),
        "left" => nodes.iter()
            .filter(|n| n.id != current_id && n.position.x < current_x)
            .collect(),
        "right" => nodes.iter()
            .filter(|n| n.id != current_id && n.position.x > current_x)
            .collect(),
        _ => return None,
    };

    if candidates.is_empty() {
        return None;
    }

    // Find the closest node in that direction
    let closest = match direction {
        "up" => candidates.into_iter()
            .min_by(|a, b| {
                let dist_a = ((current_x - a.position.x).powi(2) + (current_y - a.position.y).powi(2)).sqrt();
                let dist_b = ((current_x - b.position.x).powi(2) + (current_y - b.position.y).powi(2)).sqrt();
                dist_a.partial_cmp(&dist_b).unwrap()
            }),
        "down" => candidates.into_iter()
            .min_by(|a, b| {
                let dist_a = ((current_x - a.position.x).powi(2) + (current_y - a.position.y).powi(2)).sqrt();
                let dist_b = ((current_x - b.position.x).powi(2) + (current_y - b.position.y).powi(2)).sqrt();
                dist_a.partial_cmp(&dist_b).unwrap()
            }),
        "left" => candidates.into_iter()
            .min_by(|a, b| {
                let dist_a = ((current_x - a.position.x).powi(2) + (current_y - a.position.y).powi(2)).sqrt();
                let dist_b = ((current_x - b.position.x).powi(2) + (current_y - b.position.y).powi(2)).sqrt();
                dist_a.partial_cmp(&dist_b).unwrap()
            }),
        "right" => candidates.into_iter()
            .min_by(|a, b| {
                let dist_a = ((current_x - a.position.x).powi(2) + (current_y - a.position.y).powi(2)).sqrt();
                let dist_b = ((current_x - b.position.x).powi(2) + (current_y - b.position.y).powi(2)).sqrt();
                dist_a.partial_cmp(&dist_b).unwrap()
            }),
        _ => None,
    };

    closest.map(|n| n.id.clone())
}

// ============================================================================
// A11y Example Component
// ============================================================================

/// A11y example - Accessibility features for keyboard and screen reader users
#[component]
pub fn A11yExample() -> impl IntoView {
    // Create initial nodes with descriptive labels
    let initial_nodes = vec![
        Node::new("start".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({
                "label": "Start Node",
                "description": "This is the starting point of the workflow",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(140.0, 60.0),
        Node::new("process-1".to_string(), Position::new(300.0, 50.0))
            .with_data(json!({
                "label": "Validate Input",
                "description": "Validates user input before processing",
                "type": "process",
                "color": "#6366f1"
            }))
            .with_dimensions(140.0, 60.0),
        Node::new("process-2".to_string(), Position::new(300.0, 180.0))
            .with_data(json!({
                "label": "Process Data",
                "description": "Processes the validated data",
                "type": "process",
                "color": "#6366f1"
            }))
            .with_dimensions(140.0, 60.0),
        Node::new("decision".to_string(), Position::new(500.0, 100.0))
            .with_data(json!({
                "label": "Decision Point",
                "description": "Determines the next step based on results",
                "type": "decision",
                "color": "#f59e0b"
            }))
            .with_dimensions(140.0, 60.0),
        Node::new("end".to_string(), Position::new(700.0, 100.0))
            .with_data(json!({
                "label": "End Node",
                "description": "This is the final destination of the workflow",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(140.0, 60.0),
    ];

    // Create edges with labels
    let initial_edges = vec![
        Edge::new("e-start-validate".to_string(), "start".to_string(), "process-1".to_string())
            .with_label("to validation".to_string()),
        Edge::new("e-start-process".to_string(), "start".to_string(), "process-2".to_string())
            .with_label("to processing".to_string()),
        Edge::new("e-validate-decision".to_string(), "process-1".to_string(), "decision".to_string())
            .with_label("validated".to_string()),
        Edge::new("e-process-decision".to_string(), "process-2".to_string(), "decision".to_string())
            .with_label("processed".to_string()),
        Edge::new("e-decision-end".to_string(), "decision".to_string(), "end".to_string())
            .with_label("complete".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Provide context
    provide_context(store);

    // Get signals
    let drag_signal = get_a11y_drag_signal();
    let focused_node = get_focused_node_signal();
    let announcements = get_announcements_signal();

    // Selected nodes tracking
    let selected_nodes = RwSignal::new(std::collections::HashSet::<String>::new());

    // Keyboard event log
    let event_log = RwSignal::new(Vec::<String>::new());

    let add_event = move |event: &str| {
        event_log.update(|log| {
            log.insert(0, event.to_string());
            if log.len() > 8 {
                log.pop();
            }
        });
    };

    // Set up keyboard event handling
    Effect::new(move |_| {
        let document = leptos::web_sys::window()
            .and_then(|w| w.document());

        if let Some(doc) = document {
            let handler = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                let key = ev.key();
                let key_str = key.as_str();

                // Check if we're focused on the flow container
                let target = ev.target();
                let is_flow_focused = if let Some(el) = target {
                    if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                        let class_list = html_el.class_list();
                        class_list.contains("a11y-flow") ||
                        class_list.contains("a11y-node") ||
                        html_el.closest(".a11y-flow").is_ok()
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !is_flow_focused {
                    return;
                }

                match key_str {
                    "Tab" => {
                        ev.prevent_default();
                        let nodes = store.get_nodes();
                        let current_focused = focused_node.get();

                        // Tab through nodes in order
                        if ev.shift_key() {
                            // Reverse tab
                            if let Some(current_id) = current_focused {
                                let current_idx = nodes.iter().position(|n| n.id == current_id).unwrap_or(0);
                                let new_idx = if current_idx == 0 { nodes.len() - 1 } else { current_idx - 1 };
                                if let Some(node) = nodes.get(new_idx) {
                                    focused_node.set(Some(node.id.clone()));
                                    let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                    announce(&format!("Focused on {}", label));
                                    add_event(&format!("Tab: focus {}", label));
                                }
                            } else if let Some(node) = nodes.last() {
                                focused_node.set(Some(node.id.clone()));
                                let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                announce(&format!("Focused on {}", label));
                                add_event(&format!("Tab: focus {}", label));
                            }
                        } else {
                            // Forward tab
                            if let Some(current_id) = current_focused {
                                let current_idx = nodes.iter().position(|n| n.id == current_id).unwrap_or(0);
                                let new_idx = (current_idx + 1) % nodes.len();
                                if let Some(node) = nodes.get(new_idx) {
                                    focused_node.set(Some(node.id.clone()));
                                    let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                    announce(&format!("Focused on {}", label));
                                    add_event(&format!("Tab: focus {}", label));
                                }
                            } else if let Some(node) = nodes.first() {
                                focused_node.set(Some(node.id.clone()));
                                let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                announce(&format!("Focused on {}", label));
                                add_event(&format!("Tab: focus {}", label));
                            }
                        }
                    }
                    "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
                        ev.prevent_default();
                        if let Some(current_id) = focused_node.get() {
                            let nodes = store.get_nodes();
                            let direction = match key_str {
                                "ArrowUp" => "up",
                                "ArrowDown" => "down",
                                "ArrowLeft" => "left",
                                "ArrowRight" => "right",
                                _ => return,
                            };

                            if let Some(next_id) = find_adjacent_node(&nodes, &current_id, direction) {
                                focused_node.set(Some(next_id.clone()));
                                if let Some(node) = nodes.iter().find(|n| n.id == next_id) {
                                    let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                    announce(&format!("Moved {} to {}", direction, label));
                                    add_event(&format!("Arrow {}: {}", direction, label));
                                }
                            } else {
                                announce(&format!("No node {} from current position", direction));
                                add_event(&format!("Arrow {}: no node", direction));
                            }
                        }
                    }
                    "Enter" | " " => {
                        ev.prevent_default();
                        if let Some(current_id) = focused_node.get() {
                            selected_nodes.update(|sel| {
                                if sel.contains(&current_id) {
                                    sel.remove(&current_id);
                                    let nodes = store.get_nodes();
                                    if let Some(node) = nodes.iter().find(|n| n.id == current_id) {
                                        let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                        announce(&format!("{} deselected", label));
                                        add_event(&format!("Deselect: {}", label));
                                    }
                                } else {
                                    sel.insert(current_id.clone());
                                    let nodes = store.get_nodes();
                                    if let Some(node) = nodes.iter().find(|n| n.id == current_id) {
                                        let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                                        announce(&format!("{} selected", label));
                                        add_event(&format!("Select: {}", label));
                                    }
                                }
                            });
                        }
                    }
                    "Escape" => {
                        ev.prevent_default();
                        selected_nodes.set(std::collections::HashSet::new());
                        focused_node.set(None);
                        announce("Selection cleared, focus removed");
                        add_event("Escape: clear all");
                    }
                    "Delete" | "Backspace" => {
                        ev.prevent_default();
                        let sel = selected_nodes.get();
                        if !sel.is_empty() {
                            let count = sel.len();
                            announce(&format!("Would delete {} selected node(s)", count));
                            add_event(&format!("Delete: {} nodes", count));
                        }
                    }
                    _ => {}
                }
            });

            let _ = doc.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
            handler.forget();
        }
    });

    // Mouse handlers
    let on_mousemove = move |ev: leptos::ev::MouseEvent| {
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

    let on_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            store.update_node(&drag_state.node_id, |n| {
                n.dragging = false;
            });
            let nodes = store.get_nodes();
            if let Some(node) = nodes.iter().find(|n| n.id == drag_state.node_id) {
                let label = node.data.get("label").and_then(|v| v.as_str()).unwrap_or("Node");
                announce(&format!("{} moved to position {:.0}, {:.0}", label, node.position.x, node.position.y));
            }
            drag_signal.set(None);
        }
    };

    // Clear announcements periodically
    let clear_announcement = move |_| {
        announcements.update(|list| {
            if list.len() > 1 {
                list.remove(0);
            }
        });
    };

    view! {
        <div
            class="example-container a11y-flow"
            tabindex="0"
            role="application"
            aria-label="Accessible flow diagram with 5 nodes and 5 edges. Use Tab to navigate between nodes, Arrow keys to move spatially, Enter or Space to select."
            on:mousemove=on_mousemove
            on:mouseup=on_mouseup
        >
            <div class="xyflow leptos-flow"
                 style="width: 100%; height: 100%; position: relative;"
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Screen reader live region for announcements
                <div
                    role="status"
                    aria-live="polite"
                    aria-atomic="true"
                    style="position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); border: 0;"
                >
                    {move || announcements.get().last().cloned().unwrap_or_default()}
                </div>

                // Main flow container
                <FlowViewport store=store>
                    // Edge renderer with ARIA labels
                    <A11yEdgeRenderer store=store />

                    // Connection line
                    <ConnectionLine />

                    // Render nodes with ARIA attributes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <A11yNode
                                    node=node.clone()
                                    store=store
                                    focused_node=focused_node
                                    selected_nodes=selected_nodes
                                />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // A11y badge
                <div style="position: absolute; top: 10px; left: 10px; background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%); color: white; padding: 8px 12px; border-radius: 8px; font-size: 11px; font-weight: 600; box-shadow: 0 2px 8px rgba(0,0,0,0.2);">
                    "Accessibility Features"
                </div>

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.15); width: 300px;">
                        <strong style="display: block; margin-bottom: 10px; font-size: 14px;">"Accessibility Demo"</strong>

                        // Keyboard shortcuts
                        <div style="background: #f0f9ff; padding: 10px; border-radius: 6px; margin-bottom: 12px; font-size: 11px; color: #0369a1; line-height: 1.6;">
                            <div style="font-weight: 600; margin-bottom: 6px;">"Keyboard Navigation:"</div>
                            <ul style="margin: 0; padding-left: 16px;">
                                <li><kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Tab"</kbd>" / "<kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Shift+Tab"</kbd>" - Navigate nodes"</li>
                                <li><kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Arrow keys"</kbd>" - Move spatially"</li>
                                <li><kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Enter"</kbd>" / "<kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Space"</kbd>" - Select/deselect"</li>
                                <li><kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Escape"</kbd>" - Clear selection"</li>
                                <li><kbd style="background: #e0e7ff; padding: 1px 4px; border-radius: 3px; font-size: 10px;">"Delete"</kbd>" - Delete selected"</li>
                            </ul>
                        </div>

                        // Current focus
                        <div style="background: #f8fafc; padding: 12px; border-radius: 8px; margin-bottom: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Focus State"</div>
                            <div style="display: flex; gap: 8px;">
                                <div style="flex: 1; background: #dbeafe; padding: 10px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 18px; font-weight: 700; color: #2563eb;">
                                        {move || focused_node.get().map(|id| {
                                            let nodes = store.get_nodes();
                                            nodes.iter().find(|n| n.id == id)
                                                .and_then(|n| n.data.get("label"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("Unknown")
                                                .to_string()
                                        }).unwrap_or_else(|| "None".to_string())}
                                    </div>
                                    <div style="font-size: 10px; color: #3b82f6; font-weight: 500;">"Focused"</div>
                                </div>
                                <div style="flex: 1; background: #f3e8ff; padding: 10px; border-radius: 6px; text-align: center;">
                                    <div style="font-size: 18px; font-weight: 700; color: #7c3aed;">
                                        {move || selected_nodes.get().len()}
                                    </div>
                                    <div style="font-size: 10px; color: #8b5cf6; font-weight: 500;">"Selected"</div>
                                </div>
                            </div>
                        </div>

                        // Announcements panel
                        <div style="border-top: 1px solid #eee; padding-top: 12px; margin-bottom: 12px;">
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                                <div style="font-size: 11px; font-weight: 600; color: #333;">"Screen Reader Announcements"</div>
                                <button
                                    style="font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                                    on:click=clear_announcement
                                >
                                    "Clear"
                                </button>
                            </div>
                            <div style="background: #f8f9fa; border-radius: 6px; padding: 8px; max-height: 80px; overflow-y: auto;">
                                {move || {
                                    let anns = announcements.get();
                                    if anns.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                                "Announcements appear here"
                                            </div>
                                        }.into_any()
                                    } else {
                                        anns.iter().rev().map(|ann| {
                                            view! {
                                                <div style="font-size: 10px; color: #333; padding: 4px 6px; background: #e0e7ff; border-radius: 4px; margin-bottom: 4px;">
                                                    {ann.clone()}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // Keyboard event log
                        <div style="border-top: 1px solid #eee; padding-top: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Keyboard Events"</div>
                            <div style="background: #f8f9fa; border-radius: 6px; padding: 8px; max-height: 100px; overflow-y: auto;">
                                {move || {
                                    let log = event_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="font-size: 10px; color: #999; font-style: italic; text-align: center;">
                                                "Click on the flow and use keyboard"
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.iter().enumerate().map(|(idx, event)| {
                                            let bg = if idx == 0 { "#eef2ff" } else { "transparent" };
                                            view! {
                                                <div style=format!("font-size: 10px; color: #333; padding: 4px 6px; background: {}; border-radius: 4px; margin-bottom: 2px;", bg)>
                                                    {event.clone()}
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>

                        // A11y features checklist
                        <div style="border-top: 1px solid #eee; padding-top: 12px; margin-top: 12px;">
                            <div style="font-size: 11px; font-weight: 600; color: #333; margin-bottom: 8px;">"Accessibility Features"</div>
                            <div style="font-size: 10px; color: #666; line-height: 1.6;">
                                <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 4px;">
                                    <span style="color: #10b981; font-weight: bold;">"✓"</span>
                                    " ARIA labels on nodes and edges"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 4px;">
                                    <span style="color: #10b981; font-weight: bold;">"✓"</span>
                                    " Keyboard navigation (Tab, Arrows)"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 4px;">
                                    <span style="color: #10b981; font-weight: bold;">"✓"</span>
                                    " Screen reader announcements"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 4px;">
                                    <span style="color: #10b981; font-weight: bold;">"✓"</span>
                                    " Visible focus indicators"
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px;">
                                    <span style="color: #10b981; font-weight: bold;">"✓"</span>
                                    " Role attributes for structure"
                                </div>
                            </div>
                        </div>
                    </div>
                </Panel>
            </div>
        </div>
    }
}

// ============================================================================
// A11y Node Component
// ============================================================================

/// Accessible node with ARIA attributes and focus handling
#[component]
fn A11yNode(
    node: Node,
    store: FlowStore,
    focused_node: RwSignal<Option<String>>,
    selected_nodes: RwSignal<std::collections::HashSet<String>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_mousedown = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_focus = node.id.clone();
    let node_id_for_click = node.id.clone();
    let node_id_for_pressed = node.id.clone();
    let node_id_for_selected_indicator = node.id.clone();
    let node_id_for_handles = node.id.clone();
    let node_id_for_focus_indicator = node.id.clone();
    let node_id_for_describedby = node.id.clone();

    let drag_signal = get_a11y_drag_signal();

    // Get node data for ARIA
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let label_for_mousedown = label.clone();
    let label_for_display = label.clone();
    let description = node.data.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let description_for_display = description.clone();
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let node_type_for_display = node_type.clone();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6366f1")
        .to_string();
    let color_for_display = color.clone();

    // Build ARIA label
    let aria_label = format!("{} node. Type: {}. {}", label, node_type, description);

    // Mouse down handler
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Set focus
        focused_node.set(Some(node_id_for_mousedown.clone()));
        announce(&format!("Focused on {}", label_for_mousedown));

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
        }
    };

    // Click handler for selection
    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();

        // Toggle selection
        selected_nodes.update(|sel| {
            if sel.contains(&node_id_for_click) {
                sel.remove(&node_id_for_click);
            } else {
                sel.insert(node_id_for_click.clone());
            }
        });
    };

    view! {
        <div
            class="xyflow__node a11y-node"
            role="button"
            aria-label=aria_label.clone()
            aria-describedby=format!("node-desc-{}", node_id_for_describedby)
            aria-pressed=move || selected_nodes.get().contains(&node_id_for_pressed).to_string()
            tabindex="-1"
            style=move || {
                let nodes = store.get_nodes();
                let is_focused = focused_node.get().as_ref() == Some(&node_id_for_focus);
                let is_selected = selected_nodes.get().contains(&node_id_for_style);

                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_style) {
                    // Focus ring: high-contrast blue outline
                    let outline = if is_focused {
                        "3px solid #2563eb"
                    } else {
                        "none"
                    };

                    // Selection indicator
                    let border = if is_selected {
                        format!("3px solid {}", color)
                    } else {
                        format!("2px solid {}60", color)
                    };

                    let box_shadow = if is_focused {
                        "0 0 0 4px rgba(37, 99, 235, 0.3), 0 4px 12px rgba(0,0,0,0.15)"
                    } else if is_selected {
                        "0 0 0 2px rgba(99, 102, 241, 0.3), 0 4px 12px rgba(0,0,0,0.15)"
                    } else {
                        "0 2px 6px rgba(0,0,0,0.1)"
                    };

                    // Background based on node type
                    let background = match node_type.as_str() {
                        "input" => format!("linear-gradient(135deg, {}15 0%, {}30 100%)", color, color),
                        "output" => format!("linear-gradient(135deg, {}15 0%, {}30 100%)", color, color),
                        "decision" => format!("linear-gradient(135deg, {}15 0%, {}30 100%)", color, color),
                        _ => "white".to_string(),
                    };

                    format!(
                        "position: absolute; transform: translate({}px, {}px); width: {}px; height: {}px; \
                         background: {}; border: {}; outline: {}; outline-offset: 2px; border-radius: 8px; \
                         box-shadow: {}; cursor: grab; \
                         display: flex; flex-direction: column; justify-content: center; align-items: center; \
                         padding: 8px; box-sizing: border-box; transition: box-shadow 0.15s, outline 0.15s, border 0.15s;",
                        n.position.x, n.position.y,
                        n.width.unwrap_or(140.0), n.height.unwrap_or(60.0),
                        background, border, outline, box_shadow
                    )
                } else {
                    String::new()
                }
            }
            on:mousedown=on_mousedown
            on:click=on_click
        >
            // Hidden description for screen readers
            <span
                id=format!("node-desc-{}", node_id)
                style="position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); border: 0;"
            >
                {description_for_display.clone()}
            </span>

            // Node type icon
            <div style=format!("font-size: 16px; margin-bottom: 4px; color: {};", color_for_display)>
                {match node_type_for_display.as_str() {
                    "input" => "▶",
                    "output" => "◼",
                    "decision" => "◆",
                    _ => "●",
                }}
            </div>

            // Node label
            <div style=format!("font-weight: 600; font-size: 12px; color: {}; text-align: center;", color_for_display)>
                {label_for_display.clone()}
            </div>

            // Focus indicator badge
            {move || {
                let is_focused = focused_node.get().as_ref() == Some(&node_id_for_focus_indicator);
                is_focused.then(|| view! {
                    <div style="position: absolute; top: -8px; right: -8px; width: 16px; height: 16px; \
                                background: #2563eb; border-radius: 50%; border: 2px solid white; \
                                display: flex; align-items: center; justify-content: center; \
                                font-size: 10px; color: white; box-shadow: 0 2px 4px rgba(0,0,0,0.2);">
                        "⌨"
                    </div>
                })
            }}

            // Selection indicator
            {move || {
                let is_selected = selected_nodes.get().contains(&node_id_for_selected_indicator);
                is_selected.then(|| view! {
                    <div style="position: absolute; top: -8px; left: -8px; width: 16px; height: 16px; \
                                background: #10b981; border-radius: 50%; border: 2px solid white; \
                                display: flex; align-items: center; justify-content: center; \
                                font-size: 10px; color: white; box-shadow: 0 2px 4px rgba(0,0,0,0.2);">
                        "✓"
                    </div>
                })
            }}

            // Handles with ARIA
            {move || {
                let nodes = store.get_nodes();
                if let Some(n) = nodes.iter().find(|n| n.id == node_id_for_handles) {
                    let node_type = n.data.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("default");
                    let has_source = node_type != "output";
                    let has_target = node_type != "input";
                    let handle_node_id = node_id_for_handles.clone();

                    view! {
                        <>
                            {has_target.then(|| view! {
                                <Handle
                                    node_id=handle_node_id.clone()
                                    r#type=HandleType::Target
                                    position=HandlePosition::Left
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
                                />
                            })}
                            {has_source.then(|| view! {
                                <Handle
                                    node_id=handle_node_id.clone()
                                    r#type=HandleType::Source
                                    position=HandlePosition::Right
                                    connection_mode=ConnectionMode::Strict
                                    style="background: #888; width: 10px; height: 10px; border: 2px solid white; box-shadow: 0 1px 4px rgba(0,0,0,0.2);".to_string()
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
// A11y Edge Renderer
// ============================================================================

/// Accessible edge renderer with ARIA labels
#[component]
fn A11yEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
            role="group"
            aria-label="Flow connections between nodes"
        >
            <defs>
                <linearGradient id="a11y-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#6366f1" />
                    <stop offset="100%" stop-color="#8b5cf6" />
                </linearGradient>
                <marker
                    id="a11y-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="6"
                    markerHeight="6"
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

                    // Get labels for ARIA
                    let source_label = source_node.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Source");
                    let target_label = target_node.data.get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Target");

                    // Calculate edge path (horizontal layout)
                    let sx = source_node.position.x + source_node.width.unwrap_or(140.0);
                    let sy = source_node.position.y + source_node.height.unwrap_or(60.0) / 2.0;
                    let tx = target_node.position.x;
                    let ty = target_node.position.y + target_node.height.unwrap_or(60.0) / 2.0;

                    let offset = (tx - sx).abs() * 0.4;
                    let path = format!(
                        "M {} {} C {} {}, {} {}, {} {}",
                        sx, sy,
                        sx + offset, sy,
                        tx - offset, ty,
                        tx, ty
                    );

                    // Calculate midpoint for label
                    let mid_x = (sx + tx) / 2.0;
                    let mid_y = (sy + ty) / 2.0;

                    let label = edge.label.clone().unwrap_or_default();

                    // Build ARIA label for edge
                    let edge_aria = format!(
                        "Connection from {} to {}{}",
                        source_label,
                        target_label,
                        if label.is_empty() { String::new() } else { format!(": {}", label) }
                    );

                    Some(view! {
                        <g
                            class="xyflow__edge"
                            role="img"
                            aria-label=edge_aria
                        >
                            // Shadow/glow
                            <path
                                d=path.clone()
                                stroke="#6366f140"
                                stroke-width="6"
                                fill="none"
                            />
                            // Main edge
                            <path
                                d=path.clone()
                                stroke="url(#a11y-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#a11y-arrow)"
                            />
                            // Label
                            {(!label.is_empty()).then(|| view! {
                                <g transform=format!("translate({}, {})", mid_x, mid_y)>
                                    <rect
                                        x="-32"
                                        y="-10"
                                        width="64"
                                        height="20"
                                        fill="white"
                                        stroke="#e5e7eb"
                                        stroke-width="1"
                                        rx="4"
                                    />
                                    <text
                                        x="0"
                                        y="4"
                                        text-anchor="middle"
                                        font-size="9"
                                        fill="#666"
                                    >
                                        {label}
                                    </text>
                                </g>
                            })}
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
