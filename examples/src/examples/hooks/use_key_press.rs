//! Use Key Press Example
//!
//! Demonstrates keyboard shortcut handling using event listeners.
//! Implements common shortcuts: Delete, Ctrl+A, Ctrl+C, Ctrl+V.
//! Shows a key press indicator in the UI.

use leptos::prelude::*;
use leptos::web_sys;
use serde_json::json;
use std::sync::OnceLock;
use xyflow_leptos::*;

use crate::shared::DragState;

// ============================================================================
// Global State
// ============================================================================

/// Drag state for UseKeyPress example
static USE_KEY_PRESS_DRAG_STATE: OnceLock<RwSignal<Option<DragState>>> = OnceLock::new();

fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *USE_KEY_PRESS_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

/// Key press state
#[derive(Clone, Debug)]
pub struct KeyPressState {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyPressState {
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.meta {
            parts.push("Meta");
        }
        parts.push(&self.key);
        parts.join(" + ")
    }
}

static USE_KEY_PRESS_LAST_KEY: OnceLock<RwSignal<Option<KeyPressState>>> = OnceLock::new();

fn get_last_key_signal() -> RwSignal<Option<KeyPressState>> {
    *USE_KEY_PRESS_LAST_KEY.get_or_init(|| RwSignal::new(None))
}

/// Clipboard for copy/paste
static USE_KEY_PRESS_CLIPBOARD: OnceLock<RwSignal<Vec<Node>>> = OnceLock::new();

fn get_clipboard_signal() -> RwSignal<Vec<Node>> {
    *USE_KEY_PRESS_CLIPBOARD.get_or_init(|| RwSignal::new(Vec::new()))
}

/// Action log for keyboard shortcuts
static USE_KEY_PRESS_ACTION_LOG: OnceLock<RwSignal<Vec<String>>> = OnceLock::new();

fn get_action_log() -> RwSignal<Vec<String>> {
    *USE_KEY_PRESS_ACTION_LOG.get_or_init(|| RwSignal::new(vec!["Ready for keyboard input...".to_string()]))
}

fn log_action(action: &str) {
    get_action_log().update(|entries| {
        entries.push(action.to_string());
        if entries.len() > 20 {
            entries.remove(0);
        }
    });
}

// ============================================================================
// Use Key Press Example Component
// ============================================================================

/// UseKeyPress example - Demonstrates keyboard shortcut handling
#[component]
pub fn UseKeyPressExample() -> impl IntoView {
    // Create initial nodes
    let initial_nodes = vec![
        Node::new("a".to_string(), Position::new(100.0, 50.0))
            .with_data(json!({
                "label": "Node A",
                "type": "input",
                "color": "#10b981"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("b".to_string(), Position::new(100.0, 150.0))
            .with_data(json!({
                "label": "Node B",
                "type": "default",
                "color": "#6366f1"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("c".to_string(), Position::new(300.0, 100.0))
            .with_data(json!({
                "label": "Node C",
                "type": "default",
                "color": "#8b5cf6"
            }))
            .with_dimensions(120.0, 50.0),
        Node::new("d".to_string(), Position::new(300.0, 200.0))
            .with_data(json!({
                "label": "Node D",
                "type": "output",
                "color": "#ef4444"
            }))
            .with_dimensions(120.0, 50.0),
    ];

    let initial_edges = vec![
        Edge::new("e-a-b".to_string(), "a".to_string(), "b".to_string()),
        Edge::new("e-a-c".to_string(), "a".to_string(), "c".to_string()),
        Edge::new("e-b-c".to_string(), "b".to_string(), "c".to_string()),
        Edge::new("e-c-d".to_string(), "c".to_string(), "d".to_string()),
    ];

    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store.clone());

    let drag_signal = get_drag_signal();
    let last_key_signal = get_last_key_signal();
    let clipboard = get_clipboard_signal();
    let action_log = get_action_log();

    // Reset state on mount
    last_key_signal.set(None);
    clipboard.set(Vec::new());
    action_log.set(vec!["Ready for keyboard input...".to_string()]);

    // Set up keyboard event listener using Effect
    Effect::new({
        let store = store.clone();
        move |_| {
            use leptos::wasm_bindgen::JsCast;
            use leptos::wasm_bindgen::closure::Closure;

            // Get window for event listener
            let window = leptos::web_sys::window().expect("no window");
            let document = window.document().expect("no document");
            let store = store.clone();

            let handler = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
                    let key = ev.key();
                    let ctrl = ev.ctrl_key() || ev.meta_key();
                    let shift = ev.shift_key();
                    let alt = ev.alt_key();
                    let meta = ev.meta_key();

                    // Update last key pressed
                    get_last_key_signal().set(Some(KeyPressState {
                        key: key.clone(),
                        ctrl,
                        shift,
                        alt,
                        meta,
                    }));

                    // Handle shortcuts
                    match (ctrl, key.as_str()) {
                        // Delete - remove selected nodes
                        (false, "Delete") | (false, "Backspace") => {
                            let selected = store.get_selected_nodes();
                            if !selected.is_empty() {
                                ev.prevent_default();
                                let count = selected.len();
                                for node_id in selected {
                                    store.remove_node(&node_id);
                                }
                                log_action(&format!("Delete: Removed {} node(s)", count));
                            } else {
                                log_action("Delete: No nodes selected");
                            }
                        }

                        // Ctrl+A - select all nodes
                        (true, "a") | (true, "A") => {
                            ev.prevent_default();
                            let nodes = store.get_nodes();
                            for (i, node) in nodes.iter().enumerate() {
                                store.select_node(&node.id, i > 0);
                            }
                            log_action(&format!("Ctrl+A: Selected all {} nodes", nodes.len()));
                        }

                        // Ctrl+C - copy selected nodes
                        (true, "c") | (true, "C") => {
                            ev.prevent_default();
                            let selected = store.get_selected_nodes();
                            let nodes = store.get_nodes();
                            let copied: Vec<Node> = nodes.into_iter()
                                .filter(|n| selected.contains(&n.id))
                                .collect();

                            if !copied.is_empty() {
                                let count = copied.len();
                                get_clipboard_signal().set(copied);
                                log_action(&format!("Ctrl+C: Copied {} node(s)", count));
                            } else {
                                log_action("Ctrl+C: No nodes selected to copy");
                            }
                        }

                        // Ctrl+V - paste copied nodes
                        (true, "v") | (true, "V") => {
                            ev.prevent_default();
                            let clipboard = get_clipboard_signal().get();
                            if !clipboard.is_empty() {
                                let nodes = store.get_nodes();
                                let offset = 30.0;
                                let pasted_count = clipboard.len();

                                // Clear current selection
                                store.clear_node_selection();

                                for node in &clipboard {
                                    let new_id = format!("{}-copy-{}", node.id, nodes.len() + 1);
                                    let new_node = Node::new(new_id.clone(), Position::new(
                                        node.position.x + offset,
                                        node.position.y + offset,
                                    ))
                                    .with_data(node.data.clone())
                                    .with_dimensions(
                                        node.width.unwrap_or(120.0),
                                        node.height.unwrap_or(50.0),
                                    );
                                    store.add_node(new_node);
                                    // Select the pasted node
                                    store.select_node(&new_id, true);
                                }
                                log_action(&format!("Ctrl+V: Pasted {} node(s)", pasted_count));
                            } else {
                                log_action("Ctrl+V: Clipboard is empty");
                            }
                        }

                        // Ctrl+D - duplicate selected nodes
                        (true, "d") | (true, "D") => {
                            ev.prevent_default();
                            let selected = store.get_selected_nodes();
                            let nodes = store.get_nodes();
                            let to_duplicate: Vec<Node> = nodes.into_iter()
                                .filter(|n| selected.contains(&n.id))
                                .collect();

                            if !to_duplicate.is_empty() {
                                store.clear_node_selection();
                                let nodes_count = store.get_nodes().len();

                                for (i, node) in to_duplicate.iter().enumerate() {
                                    let new_id = format!("{}-dup-{}", node.id, nodes_count + i);
                                    let new_node = Node::new(new_id.clone(), Position::new(
                                        node.position.x + 40.0,
                                        node.position.y + 40.0,
                                    ))
                                    .with_data(node.data.clone())
                                    .with_dimensions(
                                        node.width.unwrap_or(120.0),
                                        node.height.unwrap_or(50.0),
                                    );
                                    store.add_node(new_node);
                                    store.select_node(&new_id, true);
                                }
                                log_action(&format!("Ctrl+D: Duplicated {} node(s)", to_duplicate.len()));
                            } else {
                                log_action("Ctrl+D: No nodes selected to duplicate");
                            }
                        }

                        // Escape - clear selection
                        (false, "Escape") => {
                            ev.prevent_default();
                            store.clear_node_selection();
                            store.clear_edge_selection();
                            log_action("Escape: Cleared selection");
                        }

                        // Arrow keys - move selected nodes
                        (false, "ArrowUp") | (false, "ArrowDown") | (false, "ArrowLeft") | (false, "ArrowRight") => {
                            let selected = store.get_selected_nodes();
                            if !selected.is_empty() {
                                ev.prevent_default();
                                let delta = if shift { 20.0 } else { 5.0 };
                                let (dx, dy) = match key.as_str() {
                                    "ArrowUp" => (0.0, -delta),
                                    "ArrowDown" => (0.0, delta),
                                    "ArrowLeft" => (-delta, 0.0),
                                    "ArrowRight" => (delta, 0.0),
                                    _ => (0.0, 0.0),
                                };

                                for node_id in &selected {
                                    store.update_node(node_id, |n| {
                                        n.position.x += dx;
                                        n.position.y += dy;
                                    });
                                }
                                log_action(&format!("{}{}: Moved {} node(s)",
                                    if shift { "Shift+" } else { "" },
                                    key,
                                    selected.len()));
                            }
                        }

                        // Log other key presses
                        _ => {
                            let display = if ctrl {
                                format!("Ctrl+{}", key)
                            } else {
                                key.clone()
                            };
                            log_action(&format!("Key: {}", display));
                        }
                    }
                }) as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
                .expect("failed to add keydown listener");

            // Forget the closure to keep it alive
            handler.forget();
        }
    });

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
            class="example-container"
            style="display: flex; flex-direction: column; height: 100%;"
            tabindex="0"
        >
            // Header
            <div style="padding: 12px; background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); \
                        border-bottom: 1px solid #fbbf24;">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <div style="background: #f59e0b; color: white; padding: 6px 12px; border-radius: 6px; \
                                font-size: 11px; font-weight: 600;">
                        "useKeyPress"
                    </div>
                    <div style="font-size: 12px; color: #92400e;">
                        "Keyboard shortcut handling with event listeners"
                    </div>
                </div>
            </div>

            // Key press indicator bar
            <KeyPressIndicator />

            // Main content
            <div style="display: flex; flex: 1; min-height: 0;">
                // Flow canvas
                <div
                    class="xyflow leptos-flow"
                    style="flex: 1; position: relative; background: #fafafa;"
                    on:mousemove=on_mousemove
                    on:mouseup=on_mouseup
                >
                    <Background variant=BackgroundVariant::Dots />

                    <FlowViewport store=store.clone()>
                        <KeyPressEdgeRenderer store=store.clone() />
                        <ConnectionLine />

                        {move || {
                            store.get_nodes().into_iter().map(|node| {
                                view! {
                                    <KeyPressNode
                                        node=node.clone()
                                        store=store.clone()
                                        drag_signal=drag_signal
                                    />
                                }
                            }).collect_view()
                        }}
                    </FlowViewport>

                    <Controls position=PanelPosition::BottomLeft />
                </div>

                // Control Panel
                <div style="width: 300px; background: #f8fafc; border-left: 1px solid #e2e8f0; \
                            display: flex; flex-direction: column; overflow-y: auto;">
                    <ShortcutsPanel />
                    <ClipboardPanel />
                    <ActionLogPanel />
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// Key Press Indicator Component
// ============================================================================

#[component]
fn KeyPressIndicator() -> impl IntoView {
    let last_key = get_last_key_signal();

    view! {
        <div style="padding: 8px 12px; background: #1a1a2e; display: flex; align-items: center; gap: 12px;">
            <div style="font-size: 10px; color: #a5b4fc; font-weight: 600; text-transform: uppercase; \
                        letter-spacing: 0.5px;">
                "Last Key"
            </div>
            <div style="flex: 1; display: flex; align-items: center; gap: 8px;">
                {move || {
                    if let Some(key_state) = last_key.get() {
                        let parts: Vec<_> = [
                            key_state.ctrl.then_some("Ctrl"),
                            key_state.shift.then_some("Shift"),
                            key_state.alt.then_some("Alt"),
                            key_state.meta.then_some("Meta"),
                        ].into_iter().flatten().collect();

                        view! {
                            <>
                                {parts.into_iter().map(|modifier| {
                                    view! {
                                        <span style="background: #4c1d95; color: #e9d5ff; padding: 3px 8px; \
                                                     border-radius: 4px; font-size: 10px; font-weight: 600;">
                                            {modifier}
                                        </span>
                                    }
                                }).collect_view()}
                                <span style="background: #f59e0b; color: white; padding: 4px 12px; \
                                             border-radius: 4px; font-size: 12px; font-weight: 700; \
                                             font-family: monospace; min-width: 40px; text-align: center;">
                                    {key_state.key}
                                </span>
                            </>
                        }.into_any()
                    } else {
                        view! {
                            <span style="color: #6b7280; font-size: 11px; font-style: italic;">
                                "Press any key..."
                            </span>
                        }.into_any()
                    }
                }}
            </div>
            <div style="font-size: 9px; color: #6b7280;">
                "Click on canvas first to focus"
            </div>
        </div>
    }
}

// ============================================================================
// Shortcuts Panel Component
// ============================================================================

#[component]
fn ShortcutsPanel() -> impl IntoView {
    let shortcuts = [
        ("Delete", "Remove selected nodes", "#ef4444"),
        ("Ctrl+A", "Select all nodes", "#3b82f6"),
        ("Ctrl+C", "Copy selected nodes", "#10b981"),
        ("Ctrl+V", "Paste copied nodes", "#10b981"),
        ("Ctrl+D", "Duplicate selected", "#8b5cf6"),
        ("Escape", "Clear selection", "#6b7280"),
        ("Arrow", "Move selected (5px)", "#f59e0b"),
        ("Shift+Arrow", "Move selected (20px)", "#f59e0b"),
    ];

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #f59e0b; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"KEYS"</span>
                "Keyboard Shortcuts"
            </div>

            <div style="display: flex; flex-direction: column; gap: 4px;">
                {shortcuts.into_iter().map(|(key, desc, color)| {
                    view! {
                        <div style="display: flex; align-items: center; gap: 8px; padding: 4px 0;">
                            <span style=format!(
                                "background: {}20; color: {}; padding: 2px 8px; border-radius: 4px; \
                                 font-size: 10px; font-weight: 600; font-family: monospace; \
                                 min-width: 80px; text-align: center; border: 1px solid {}40;",
                                color, color, color
                            )>
                                {key}
                            </span>
                            <span style="font-size: 10px; color: #666;">{desc}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

// ============================================================================
// Clipboard Panel Component
// ============================================================================

#[component]
fn ClipboardPanel() -> impl IntoView {
    let clipboard = get_clipboard_signal();

    view! {
        <div style="padding: 12px; border-bottom: 1px solid #e2e8f0;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #10b981; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"CLIP"</span>
                "Clipboard"
            </div>

            {move || {
                let items = clipboard.get();
                if items.is_empty() {
                    view! {
                        <div style="font-size: 10px; color: #9ca3af; font-style: italic; \
                                    padding: 8px; background: #f3f4f6; border-radius: 6px; \
                                    text-align: center;">
                            "Clipboard empty"
                            <br />
                            <span style="font-size: 9px;">"Select nodes and press Ctrl+C"</span>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style="background: #f0fdf4; padding: 8px; border-radius: 6px; \
                                    border: 1px solid #86efac;">
                            <div style="display: flex; align-items: center; gap: 6px; margin-bottom: 6px;">
                                <span style="font-size: 16px; font-weight: 700; color: #166534;">
                                    {items.len()}
                                </span>
                                <span style="font-size: 10px; color: #15803d;">
                                    "node(s) in clipboard"
                                </span>
                            </div>
                            <div style="display: flex; flex-wrap: wrap; gap: 4px;">
                                {items.into_iter().map(|node| {
                                    let label = node.data.get("label")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or(&node.id)
                                        .to_string();
                                    let color = node.data.get("color")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("#6366f1")
                                        .to_string();
                                    view! {
                                        <span style=format!(
                                            "background: {}30; color: {}; padding: 2px 6px; \
                                             border-radius: 3px; font-size: 9px; font-weight: 500;",
                                            color, color
                                        )>
                                            {label}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

// ============================================================================
// Action Log Panel Component
// ============================================================================

#[component]
fn ActionLogPanel() -> impl IntoView {
    let action_log = get_action_log();

    view! {
        <div style="padding: 12px; flex: 1; display: flex; flex-direction: column;">
            <div style="font-size: 12px; font-weight: 600; color: #333; margin-bottom: 10px; \
                        display: flex; align-items: center; gap: 8px;">
                <span style="background: #6366f1; color: white; padding: 2px 6px; border-radius: 4px; \
                             font-size: 9px;">"LOG"</span>
                "Key Actions"
                <button
                    style="margin-left: auto; font-size: 9px; padding: 2px 6px; border: 1px solid #ddd; \
                           border-radius: 3px; background: white; cursor: pointer; color: #666;"
                    on:click=move |_| action_log.set(vec!["Log cleared".to_string()])
                >
                    "Clear"
                </button>
            </div>

            <div style="flex: 1; background: #1a1a2e; border-radius: 6px; padding: 8px; \
                        overflow-y: auto; font-family: monospace; font-size: 9px;">
                {move || {
                    let entries = action_log.get();
                    entries.into_iter().rev().enumerate().map(|(i, entry)| {
                        let color = if entry.contains("Delete") || entry.contains("Removed") {
                            "#ef4444"
                        } else if entry.contains("Ctrl+A") || entry.contains("Selected") {
                            "#3b82f6"
                        } else if entry.contains("Ctrl+C") || entry.contains("Copied") {
                            "#10b981"
                        } else if entry.contains("Ctrl+V") || entry.contains("Pasted") {
                            "#22c55e"
                        } else if entry.contains("Ctrl+D") || entry.contains("Duplicated") {
                            "#8b5cf6"
                        } else if entry.contains("Arrow") || entry.contains("Moved") {
                            "#f59e0b"
                        } else if entry.contains("Escape") || entry.contains("Cleared") {
                            "#6b7280"
                        } else if entry.contains("Key:") {
                            "#a5b4fc"
                        } else {
                            "#9ca3af"
                        };
                        view! {
                            <div style=format!(
                                "color: {}; padding: 2px 0; border-bottom: 1px solid #2a2a4e; \
                                 opacity: {};",
                                color,
                                if i < 5 { 1.0 } else { 0.7 - (i as f64 - 5.0) * 0.05 }
                            )>
                                {entry}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ============================================================================
// Node Component
// ============================================================================

#[component]
fn KeyPressNode(
    node: Node,
    store: FlowStore,
    drag_signal: RwSignal<Option<DragState>>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_drag = node.id.clone();
    let node_id_for_style = node.id.clone();
    let node_id_for_label = node.id.clone();

    // Mouse down - start drag and select
    let on_mousedown = {
        let store = store.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();

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
                if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
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
// Edge Renderer Component
// ============================================================================

#[component]
fn KeyPressEdgeRenderer(store: FlowStore) -> impl IntoView {
    view! {
        <svg
            class="xyflow__edges"
            style="position: absolute; width: 100%; height: 100%; overflow: visible; pointer-events: none;"
        >
            <defs>
                <linearGradient id="key-press-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" stop-color="#f59e0b" />
                    <stop offset="100%" stop-color="#f97316" />
                </linearGradient>
                <marker
                    id="key-press-edge-arrow"
                    viewBox="0 0 10 10"
                    refX="8"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="#f59e0b" />
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
                                stroke="url(#key-press-edge-gradient)"
                                stroke-width="2"
                                fill="none"
                                marker-end="url(#key-press-edge-arrow)"
                            />
                        </g>
                    })
                }).collect_view()
            }}
        </svg>
    }
}
