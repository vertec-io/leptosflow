//! Shared components and utilities for examples
//!
//! This module contains reusable components used across multiple examples.

use leptos::prelude::*;
use xyflow_leptos::*;

// ============================================================================
// Example Metadata
// ============================================================================

/// Example metadata for the navigation
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ExampleMeta {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

/// Get all examples organized by category
pub fn get_all_examples() -> Vec<(&'static str, Vec<ExampleMeta>)> {
    vec![
        ("Basic", vec![
            ExampleMeta { id: "basic", name: "Basic", description: "Draggable nodes, pan/zoom, background, minimap, controls", category: "Basic" },
            ExampleMeta { id: "empty", name: "Empty", description: "Minimal starting point with empty flow canvas", category: "Basic" },
            ExampleMeta { id: "default-nodes", name: "Default Nodes", description: "Input, default, and output node types", category: "Basic" },
            ExampleMeta { id: "switch", name: "Switch", description: "Switch between different flow configurations at runtime", category: "Basic" },
        ]),
        ("Nodes", vec![
            ExampleMeta { id: "custom-node", name: "Custom Nodes", description: "User-defined node components with colors", category: "Nodes" },
            ExampleMeta { id: "default-node-overwrite", name: "Default Node Overwrite", description: "Customize the default node component", category: "Nodes" },
            ExampleMeta { id: "node-resizer", name: "Node Resizer", description: "Resizable nodes with handles", category: "Nodes" },
            ExampleMeta { id: "drag-handle", name: "Drag Handle", description: "Limit drag area to specific region", category: "Nodes" },
            ExampleMeta { id: "moving-handles", name: "Moving Handles", description: "Handles that change position dynamically", category: "Nodes" },
            ExampleMeta { id: "detached-handle", name: "Detached Handle", description: "Handles positioned outside node body", category: "Nodes" },
            ExampleMeta { id: "node-type-change", name: "Node Type Change", description: "Dynamically change node type at runtime", category: "Nodes" },
            ExampleMeta { id: "node-types-object-change", name: "Node Types Object Change", description: "Dynamically change node type definitions", category: "Nodes" },
            ExampleMeta { id: "update-node", name: "Update Node", description: "Update node properties programmatically", category: "Nodes" },
            ExampleMeta { id: "use-update-node-internals", name: "Use Update Node Internals", description: "Force re-measurement of node internals", category: "Nodes" },
            ExampleMeta { id: "broken-nodes", name: "Broken Nodes", description: "Graceful handling of invalid node configurations", category: "Nodes" },
            ExampleMeta { id: "node-toolbar", name: "Node Toolbar", description: "Context toolbar on nodes", category: "Nodes" },
            ExampleMeta { id: "use-nodes-init", name: "Use Nodes Init", description: "Lifecycle hook for when nodes are initialized", category: "Nodes" },
        ]),
        ("Edges", vec![
            ExampleMeta { id: "edge-types", name: "Edge Types", description: "Bezier, step, and straight edge styles", category: "Edges" },
            ExampleMeta { id: "default-edge-overwrite", name: "Default Edge Overwrite", description: "Customize the default edge component", category: "Edges" },
            ExampleMeta { id: "custom-edges", name: "Custom Edges", description: "Fully custom edge components", category: "Edges" },
            ExampleMeta { id: "custom-connection-line", name: "Custom Connection Line", description: "Customize connection preview line", category: "Edges" },
            ExampleMeta { id: "floating-edges", name: "Floating Edges", description: "Edges that connect to nodes dynamically", category: "Edges" },
            ExampleMeta { id: "easy-connect", name: "Easy Connect", description: "Click-based connection creation", category: "Edges" },
            ExampleMeta { id: "edge-renderer", name: "Edge Renderer", description: "Custom edge layer rendering with z-index", category: "Edges" },
            ExampleMeta { id: "edge-toolbar", name: "Edge Toolbar", description: "Toolbars on edges", category: "Edges" },
            ExampleMeta { id: "edge-routing", name: "Edge Routing", description: "Advanced edge routing with obstacle avoidance", category: "Edges" },
        ]),
        ("Connections", vec![
            ExampleMeta { id: "validation", name: "Validation", description: "Connection validation rules", category: "Connections" },
            ExampleMeta { id: "use-connection", name: "Use Connection", description: "Connection hook for custom behavior", category: "Connections" },
            ExampleMeta { id: "cancel-connection", name: "Cancel Connection", description: "Cancel connections in progress", category: "Connections" },
            ExampleMeta { id: "reconnect-edge", name: "Reconnect Edge", description: "Reconnect existing edges to different handles", category: "Connections" },
            ExampleMeta { id: "add-node-on-edge-drop", name: "Add Node on Edge Drop", description: "Create new node when dropping connection", category: "Connections" },
        ]),
        ("Interactions", vec![
            ExampleMeta { id: "interactions", name: "Interactions", description: "Selection, multi-select, and deletion", category: "Interactions" },
            ExampleMeta { id: "use-on-selection-change", name: "Selection Change", description: "React to selection changes", category: "Interactions" },
            ExampleMeta { id: "use-node-connections", name: "Node Connections", description: "Get all connections for a specific node", category: "Interactions" },
            ExampleMeta { id: "click-distance", name: "Click Distance", description: "Distinguish between clicks and drags", category: "Interactions" },
            ExampleMeta { id: "touch-device", name: "Touch Device", description: "Touch-optimized interactions", category: "Interactions" },
            ExampleMeta { id: "multi-set-nodes", name: "Multi-Set Nodes", description: "Multiple disconnected node groups", category: "Interactions" },
        ]),
        ("Viewport", vec![
            ExampleMeta { id: "controlled-viewport", name: "Controlled Viewport", description: "Programmatically control viewport", category: "Viewport" },
            ExampleMeta { id: "controlled-uncontrolled", name: "Controlled/Uncontrolled", description: "Compare controlled vs uncontrolled modes", category: "Viewport" },
            ExampleMeta { id: "intersection", name: "Intersection", description: "Detect nodes in viewport", category: "Viewport" },
            ExampleMeta { id: "layouting", name: "Layouting", description: "Automatic graph layout algorithms", category: "Viewport" },
        ]),
        ("MiniMap", vec![
            ExampleMeta { id: "custom-minimap-node", name: "Custom MiniMap Node", description: "Customize minimap node appearance", category: "MiniMap" },
            ExampleMeta { id: "interactive-minimap", name: "Interactive MiniMap", description: "Click and drag to pan viewport", category: "MiniMap" },
            ExampleMeta { id: "overview", name: "Overview", description: "Toggle minimap visibility with animation", category: "MiniMap" },
        ]),
        ("Styling", vec![
            ExampleMeta { id: "backgrounds", name: "Backgrounds", description: "Dots, lines, and cross patterns", category: "Styling" },
            ExampleMeta { id: "color-mode", name: "Color Mode", description: "Light and dark mode switching", category: "Styling" },
            ExampleMeta { id: "hidden", name: "Hidden", description: "Hide and show flow elements", category: "Styling" },
        ]),
        ("State", vec![
            ExampleMeta { id: "save-restore", name: "Save/Restore", description: "Serialize and deserialize flow state", category: "State" },
            ExampleMeta { id: "use-nodes-data", name: "Use Nodes Data", description: "Reactively access node data", category: "State" },
            ExampleMeta { id: "set-nodes-batching", name: "Set Nodes Batching", description: "Batch multiple node updates efficiently", category: "State" },
            ExampleMeta { id: "reactive-stores", name: "Reactive Stores", description: "Integration with Leptos reactive_stores (#[derive(Store)])", category: "State" },
            ExampleMeta { id: "middlewares", name: "Middlewares", description: "Custom middleware/hooks for state operations", category: "State" },
        ]),
        ("Advanced", vec![
            ExampleMeta { id: "figma", name: "Figma", description: "Figma-like selection and interaction", category: "Advanced" },
            ExampleMeta { id: "undirectional", name: "Undirectional", description: "Restrict connection direction (left to right only)", category: "Advanced" },
            ExampleMeta { id: "subflow", name: "Subflow", description: "Nested flow graphs", category: "Advanced" },
            ExampleMeta { id: "multi-flows", name: "Multi Flows", description: "Multiple independent flow instances", category: "Advanced" },
            ExampleMeta { id: "provider", name: "Provider", description: "FlowStore context pattern for sibling components", category: "Advanced" },
            ExampleMeta { id: "a11y", name: "Accessibility", description: "Keyboard navigation and screen reader support", category: "Advanced" },
            ExampleMeta { id: "stress", name: "Stress Test", description: "Performance with many nodes", category: "Advanced" },
        ]),
        ("Hooks", vec![
            ExampleMeta { id: "use-svelte-flow", name: "useSvelteFlow", description: "Main FlowStore API for programmatic control", category: "Hooks" },
            ExampleMeta { id: "use-key-press", name: "useKeyPress", description: "Keyboard shortcut handling", category: "Hooks" },
            ExampleMeta { id: "drag-n-drop", name: "Drag & Drop", description: "Drag nodes from sidebar onto canvas", category: "Hooks" },
            ExampleMeta { id: "dev-tools", name: "Dev Tools", description: "Debug panel showing flow internals", category: "Hooks" },
            ExampleMeta { id: "z-index-mode", name: "Z-Index Mode", description: "Node stacking order controls", category: "Hooks" },
        ]),
    ]
}

// ============================================================================
// Custom Node Component
// ============================================================================

/// Custom node component matching React Flow's default node appearance
#[component]
pub fn CustomNode(
    /// Node ID
    id: String,
    /// Node label
    label: String,
    /// CSS class for light/dark mode
    #[prop(default = "light".to_string())]
    class: String,
    /// Node type: "input" (source only), "output" (target only), or "default" (both)
    #[prop(default = "default".to_string())]
    node_type: String,
    /// Whether the node is selected
    #[prop(default = false)]
    selected: bool,
    /// Whether the node is being dragged
    #[prop(default = false)]
    dragging: bool,
) -> impl IntoView {
    let has_source = node_type != "output";
    let has_target = node_type != "input";

    // Determine node class based on type
    let node_class = match node_type.as_str() {
        "input" => "xyflow__node-input",
        "output" => "xyflow__node-output",
        _ => "xyflow__node-default",
    };

    let selected_class = if selected { " selected" } else { "" };
    let dragging_class = if dragging { " dragging" } else { "" };

    view! {
        <div class=format!("{} {}{}{}", node_class, class, selected_class, dragging_class)>
            {has_target.then(|| view! {
                <Handle
                    node_id=id.clone()
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
                    node_id=id.clone()
                    r#type=HandleType::Source
                    position=HandlePosition::Bottom
                    connection_mode=ConnectionMode::Strict
                />
            })}
        </div>
    }
}

// ============================================================================
// Drag State
// ============================================================================

/// Global drag state - stored globally to avoid per-node document listeners
static DRAGGING_NODE: std::sync::OnceLock<RwSignal<Option<DragState>>> = std::sync::OnceLock::new();

#[derive(Clone, Debug)]
pub struct DragState {
    pub node_id: String,
    pub start_mouse: (f64, f64),
    pub start_pos: (f64, f64),
}

/// Get or initialize the global drag state signal
pub fn get_drag_signal() -> RwSignal<Option<DragState>> {
    *DRAGGING_NODE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Draggable Node Wrapper
// ============================================================================

/// Draggable node wrapper
#[component]
pub fn DraggableNode(
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
    let class = node.data.get("class")
        .and_then(|v| v.as_str())
        .unwrap_or("light")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down - start dragging
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Get current node position
        let nodes = store.get_nodes();
        if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(DragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (node.position.x, node.position.y),
            }));

            // Mark node as dragging
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

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
        >
            <CustomNode
                id=node.id.clone()
                label=label
                node_type=node_type
                class=class
                selected=node.selected
                dragging=node.dragging
            />
        </div>
    }
}

// ============================================================================
// Placeholder Example
// ============================================================================

/// Placeholder example for unimplemented routes
#[component]
pub fn PlaceholderExample(
    #[prop(into)] name: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class="example-container placeholder-example">
            <div class="placeholder-content">
                <h2>{name}</h2>
                <p>{description}</p>
                <p class="placeholder-note">"This example is coming soon!"</p>
            </div>
        </div>
    }
}

// ============================================================================
// Source Code Viewer
// ============================================================================

use leptos::web_sys;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

/// Copy text to clipboard using JavaScript eval
fn copy_text_to_clipboard(text: &str) -> bool {
    // Escape the text for JavaScript (handle quotes and newlines)
    let escaped = text
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('$', "\\$");

    // Use eval to run clipboard API
    let js_code = format!(
        r#"(function() {{
            const text = `{}`;
            const textarea = document.createElement('textarea');
            textarea.value = text;
            textarea.style.position = 'fixed';
            textarea.style.left = '-9999px';
            textarea.style.top = '0';
            document.body.appendChild(textarea);
            textarea.select();
            try {{
                document.execCommand('copy');
                document.body.removeChild(textarea);
                return true;
            }} catch (e) {{
                document.body.removeChild(textarea);
                return false;
            }}
        }})()"#,
        escaped
    );

    match js_sys::eval(&js_code) {
        Ok(result) => result.as_bool().unwrap_or(false),
        Err(_) => false,
    }
}

/// Source code viewer component with syntax highlighting and copy button
#[component]
pub fn SourceCodeViewer(
    /// The source code to display
    #[prop(into)]
    source: String,
    /// The title/name of the example
    #[prop(into)]
    title: String,
) -> impl IntoView {
    let is_expanded = RwSignal::new(false);
    let copy_status = RwSignal::new("Copy");
    let source_for_highlight = source.clone();
    let source_for_copy = source.clone();

    let toggle_expanded = move |_| {
        is_expanded.update(|v| *v = !*v);
    };

    let copy_to_clipboard = move |_| {
        let source_clone = source_for_copy.clone();
        if copy_text_to_clipboard(&source_clone) {
            copy_status.set("Copied!");

            // Reset status after 2 seconds
            if let Some(window) = web_sys::window() {
                let status_signal = copy_status;
                let timeout_callback = Closure::wrap(Box::new(move || {
                    status_signal.set("Copy");
                }) as Box<dyn Fn()>);

                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_callback.as_ref().unchecked_ref(),
                    2000,
                );
                timeout_callback.forget();
            }
        }
    };

    // Apply basic syntax highlighting
    let highlighted_code = move || {
        highlight_rust_code(&source_for_highlight)
    };

    view! {
        <div class="source-code-viewer">
            <button
                class="source-toggle-button"
                on:click=toggle_expanded
            >
                <svg class="source-toggle-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="16 18 22 12 16 6"></polyline>
                    <polyline points="8 6 2 12 8 18"></polyline>
                </svg>
                {move || if is_expanded.get() { "Hide Source" } else { "View Source" }}
            </button>

            <div
                class="source-code-panel"
                style=move || if is_expanded.get() { "display: block;" } else { "display: none;" }
            >
                <div class="source-code-header">
                    <span class="source-code-title">{title.clone()}</span>
                    <button
                        class="source-copy-button"
                        on:click=copy_to_clipboard
                    >
                        <svg class="source-copy-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                        {move || copy_status.get()}
                    </button>
                </div>
                <pre class="source-code-content"><code inner_html=highlighted_code /></pre>
            </div>
        </div>
    }
}

/// Apply basic syntax highlighting to Rust code
fn highlight_rust_code(code: &str) -> String {
    let mut result = String::new();
    let mut chars = code.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Handle strings
            '"' => {
                let mut string_content = String::from("\"");
                while let Some(&next) = chars.peek() {
                    chars.next();
                    string_content.push(next);
                    if next == '"' {
                        break;
                    }
                    if next == '\\' {
                        if let Some(&escaped) = chars.peek() {
                            chars.next();
                            string_content.push(escaped);
                        }
                    }
                }
                result.push_str(&format!("<span class=\"hl-string\">{}</span>", escape_html(&string_content)));
            }
            // Handle line comments
            '/' if chars.peek() == Some(&'/') => {
                let mut comment = String::from("/");
                chars.next();
                comment.push('/');
                while let Some(&next) = chars.peek() {
                    if next == '\n' {
                        break;
                    }
                    chars.next();
                    comment.push(next);
                }
                result.push_str(&format!("<span class=\"hl-comment\">{}</span>", escape_html(&comment)));
            }
            // Handle block comments
            '/' if chars.peek() == Some(&'*') => {
                let mut comment = String::from("/");
                chars.next();
                comment.push('*');
                let mut depth = 1;
                while depth > 0 {
                    if let Some(next) = chars.next() {
                        comment.push(next);
                        if next == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            comment.push('/');
                            depth -= 1;
                        } else if next == '/' && chars.peek() == Some(&'*') {
                            chars.next();
                            comment.push('*');
                            depth += 1;
                        }
                    } else {
                        break;
                    }
                }
                result.push_str(&format!("<span class=\"hl-comment\">{}</span>", escape_html(&comment)));
            }
            // Handle characters
            '\'' => {
                let mut char_content = String::from("'");
                // Check if it's a lifetime or char literal
                let mut is_lifetime = true;
                let mut temp_chars = Vec::new();

                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' || next == '\\' {
                        chars.next();
                        temp_chars.push(next);
                        if next == '\'' {
                            is_lifetime = false;
                            break;
                        }
                    } else if next == '\'' {
                        chars.next();
                        temp_chars.push(next);
                        is_lifetime = false;
                        break;
                    } else {
                        break;
                    }
                }

                for tc in temp_chars {
                    char_content.push(tc);
                }

                if is_lifetime && char_content.len() > 1 {
                    result.push_str(&format!("<span class=\"hl-lifetime\">{}</span>", escape_html(&char_content)));
                } else if !is_lifetime {
                    result.push_str(&format!("<span class=\"hl-string\">{}</span>", escape_html(&char_content)));
                } else {
                    result.push_str(&escape_html(&char_content));
                }
            }
            // Handle numbers
            '0'..='9' => {
                let mut number = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' || next == '.' {
                        chars.next();
                        number.push(next);
                    } else {
                        break;
                    }
                }
                result.push_str(&format!("<span class=\"hl-number\">{}</span>", escape_html(&number)));
            }
            // Handle identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        chars.next();
                        word.push(next);
                    } else {
                        break;
                    }
                }

                let class = match word.as_str() {
                    // Keywords
                    "as" | "async" | "await" | "break" | "const" | "continue" | "crate" |
                    "dyn" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if" |
                    "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" |
                    "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" |
                    "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" |
                    "while" => "hl-keyword",
                    // Built-in types
                    "bool" | "char" | "str" | "u8" | "u16" | "u32" | "u64" | "u128" |
                    "usize" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "f32" |
                    "f64" | "String" | "Vec" | "Option" | "Result" | "Box" | "Rc" |
                    "Arc" | "Cell" | "RefCell" | "HashMap" | "HashSet" | "Some" | "None" |
                    "Ok" | "Err" => "hl-type",
                    // Macros (check if followed by !)
                    _ if chars.peek() == Some(&'!') => "hl-macro",
                    // Attributes
                    _ if word.starts_with("derive") || word == "component" || word == "prop" => "hl-attribute",
                    _ => "",
                };

                if class.is_empty() {
                    result.push_str(&escape_html(&word));
                } else {
                    result.push_str(&format!("<span class=\"{}\">{}</span>", class, escape_html(&word)));
                }
            }
            // Handle attributes
            '#' if chars.peek() == Some(&'[') => {
                let mut attr = String::from("#");
                chars.next();
                attr.push('[');
                let mut depth = 1;
                while depth > 0 {
                    if let Some(next) = chars.next() {
                        attr.push(next);
                        if next == '[' {
                            depth += 1;
                        } else if next == ']' {
                            depth -= 1;
                        }
                    } else {
                        break;
                    }
                }
                result.push_str(&format!("<span class=\"hl-attribute\">{}</span>", escape_html(&attr)));
            }
            // Default: escape HTML entities
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            _ => result.push(c),
        }
    }

    result
}

/// Escape HTML special characters
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
