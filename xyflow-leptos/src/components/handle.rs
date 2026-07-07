//! Handle component for node connection points

use leptos::prelude::*;
use leptos::html;
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use crate::hooks::use_flow_store;
use crate::types::{HandleType, ConnectionMode, IsValidConnection};
use crate::events::use_connection_handlers;
use crate::utils::dom::measure_handle_bound;

/// Position of a handle on a node
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlePosition {
    /// Top of the node
    Top,
    /// Right side of the node
    Right,
    /// Bottom of the node
    Bottom,
    /// Left side of the node
    Left,
}

impl Default for HandlePosition {
    fn default() -> Self {
        HandlePosition::Top
    }
}

/// Handle component for node connection points
///
/// Handles are the connection points on nodes where edges can be attached.
/// They can be inputs (where edges end) or outputs (where edges start).
///
/// # Example
///
/// ```ignore
/// use xyflow_leptos::{Handle, HandleType, HandlePosition, ConnectionMode};
///
/// #[component]
/// fn CustomNode() -> impl IntoView {
///     view! {
///         <div class="custom-node">
///             <Handle
///                 node_id="node-1".to_string()
///                 r#type=HandleType::Target
///                 position=HandlePosition::Left
///             />
///             <div>"Node Content"</div>
///             <Handle
///                 node_id="node-1".to_string()
///                 r#type=HandleType::Source
///                 position=HandlePosition::Right
///             />
///         </div>
///     }
/// }
/// ```
#[component]
pub fn Handle(
    /// Node ID (required for connection creation)
    node_id: String,

    /// Optional handle ID
    #[prop(optional)]
    id: Option<String>,

    /// Handle type (source or target)
    #[prop(default = HandleType::Source)]
    r#type: HandleType,

    /// Position on the node
    #[prop(default = HandlePosition::Top)]
    position: HandlePosition,

    /// Whether this handle is connectable
    #[prop(default = true)]
    is_connectable: bool,

    /// Whether connections can start from this handle
    #[prop(default = true)]
    is_connectable_start: bool,

    /// Whether connections can end at this handle
    #[prop(default = true)]
    is_connectable_end: bool,

    /// Connection mode (Strict or Loose)
    #[prop(default = ConnectionMode::Strict)]
    connection_mode: ConnectionMode,

    /// Custom connection validation function
    #[prop(optional)]
    is_valid_connection: Option<IsValidConnection>,

    /// Custom CSS class
    #[prop(optional)]
    class: Option<String>,

    /// Custom inline style
    #[prop(optional)]
    style: Option<String>,

    /// Child elements
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // Build CSS classes
    let handle_type_str = match r#type {
        HandleType::Target => "target",
        HandleType::Source => "source",
    };

    let position_str = match position {
        HandlePosition::Top => "top",
        HandlePosition::Right => "right",
        HandlePosition::Bottom => "bottom",
        HandlePosition::Left => "left",
    };

    let classes = format!(
        "xyflow__handle xyflow__handle-{} {} {}",
        position_str,
        handle_type_str,
        class.unwrap_or_default()
    );

    // Build data attributes for handle identification
    let handle_id_attr = id.clone().unwrap_or_else(|| "null".to_string());

    let store = use_flow_store();
    let handle_ref: NodeRef<html::Div> = NodeRef::new();

    // Measure this handle relative to its parent `.xyflow__node` element once
    // it is mounted, and register the bound in the store. Bounds are stored
    // node-relative, so they remain correct while the node is dragged and are
    // zoom-invariant. EdgeRenderer picks them up keyed by (node_id, handle_id).
    {
        let node_id = node_id.clone();
        Effect::new(move |_| {
            let Some(element) = handle_ref.get() else {
                return;
            };
            let store = store;
            let node_id = node_id.clone();
            // Wait one frame so layout (and stylesheets) have settled.
            let closure = Closure::once(move || {
                let zoom = store.get_viewport_untracked().zoom;
                if let Some(bound) = measure_handle_bound(&element, zoom) {
                    store.register_handle(&node_id, bound);
                }
            });
            if let Some(window) = web_sys::window() {
                let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
            }
            closure.forget();
        });
    }

    // Drop the registered bound when the handle unmounts, so edges do not
    // anchor to handles that no longer exist.
    {
        let node_id = node_id.clone();
        let handle_id = id.clone();
        on_cleanup(move || {
            store.unregister_handle(&node_id, handle_id.as_deref(), r#type);
        });
    }

    // Placeholder start position; the connection handler measures the actual
    // handle center from the DOM on mousedown.
    let handle_pos = crate::types::Position::new(0.0, 0.0);

    // Create connection handler
    let on_mouse_down = use_connection_handlers(
        node_id.clone(),
        id.clone(),
        handle_pos,
        r#type,
        connection_mode,
        is_valid_connection,
    );

    // Keep pointerdown on a handle from starting a node drag (the node's
    // drag handler calls prevent_default, which would also suppress the
    // compatibility mousedown this handle's connection handler listens for).
    let on_pointer_down = |ev: leptos::ev::PointerEvent| {
        ev.stop_propagation();
    };

    // Only attach handlers if connectable
    if is_connectable_start {
        view! {
            <div
                node_ref=handle_ref
                class=classes
                data-handleid=handle_id_attr
                data-handlepos=position_str
                data-nodeid=node_id
                data-handletype=handle_type_str
                style=style.unwrap_or_default()
                class:connectable=is_connectable
                class:connectablestart=is_connectable_start
                class:connectableend=is_connectable_end
                on:pointerdown=on_pointer_down
                on:mousedown=on_mouse_down
            >
                {children.map(|c| c())}
            </div>
        }.into_any()
    } else {
        view! {
            <div
                node_ref=handle_ref
                class=classes
                data-handleid=handle_id_attr
                data-handlepos=position_str
                data-nodeid=node_id
                data-handletype=handle_type_str
                style=style.unwrap_or_default()
                class:connectable=is_connectable
                class:connectablestart=is_connectable_start
                class:connectableend=is_connectable_end
                on:pointerdown=on_pointer_down
            >
                {children.map(|c| c())}
            </div>
        }.into_any()
    }
}

