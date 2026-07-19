//! Handle component for node connection points

use leptos::prelude::*;
use leptos::html;
use leptos::wasm_bindgen::{closure::Closure, JsCast};
use crate::hooks::use_flow_store;
use crate::types::{Connection, HandleType, ConnectionMode, IsValidConnection};
use crate::events::use_connection_handlers;
use crate::utils::dom::measure_handle_bound;
use crate::utils::handle::is_valid_handle_connection;

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
/// Dragging from a connectable handle starts an interactive connection: the
/// connection line follows the cursor, snaps to nearby connectable handles,
/// and completing on a valid handle fires the store's `on_connect` callback
/// (or inserts a default edge when none is registered). See
/// [`crate::events::use_connection_handlers`] for the drag lifecycle and
/// [`crate::FlowStore::set_is_valid_connection`] for host-level validation.
///
/// While a connection is in flight the handle carries reactive state classes
/// consumers can style:
/// - `connectingfrom` — this handle is the drag's fixed end
/// - `connectingto` — this handle is the current snap candidate
/// - `valid` / `invalid` — whether dropping on this candidate would connect
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

    /// Whether this handle is connectable at all. When false the handle is
    /// purely a measured edge anchor: drags cannot start from it and
    /// connection hit-testing skips it.
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

    /// Custom connection validation function, enforced while dragging and on
    /// drop. For validators that need captured state, use
    /// [`crate::FlowStore::set_is_valid_connection`] instead.
    #[prop(optional)]
    is_valid_connection: Option<IsValidConnection>,

    /// Custom CSS class. Accepts a plain string or a reactive
    /// `Signal<String>` — bind a signal to restyle the handle during a
    /// connection drag (e.g. dim handles incompatible with the in-flight
    /// connection, derived from [`crate::hooks::use_connection`]).
    #[prop(optional, into)]
    class: MaybeProp<String>,

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

    // Reactive so a signal-backed `class` prop restyles the handle live
    // (e.g. validity dimming during a connection drag).
    let classes = move || {
        format!(
            "xyflow__handle xyflow__handle-{} {} {}",
            position_str,
            handle_type_str,
            class.get().unwrap_or_default()
        )
    };

    // Effective gates: `is_connectable` masters both directions
    let connectable_start = is_connectable && is_connectable_start;
    let connectable_end = is_connectable && is_connectable_end;

    // Build data attributes for handle identification
    let handle_id_attr = id.clone().unwrap_or_else(|| "null".to_string());

    let store = use_flow_store();
    let handle_ref: NodeRef<html::Div> = NodeRef::new();

    // Measure this handle relative to its parent `.xyflow__node` element once
    // it is mounted, and register the bound in the store. Bounds are stored
    // node-relative, so they remain correct while the node is dragged and are
    // zoom-invariant. EdgeRenderer picks them up keyed by (node_id, handle_id)
    // and connection hit-testing uses them as drop targets (gated by
    // `connectable`).
    {
        let node_id = node_id.clone();
        // Keep the pending frame CANCELLABLE. The measurement runs one frame
        // after mount; if the handle is unmounted before then (a rapid
        // re-render removed its node), an orphaned `forget()`-ed frame would
        // still fire and read the disposed store `viewport`, which panics —
        // and a panic inside the reactive graph poisons it for the entire app
        // (cascading "already disposed" / "already borrowed" until the tab is
        // dead). We cancel on cleanup AND read the viewport fallibly.
        let raf: StoredValue<Option<(i32, Closure<dyn FnMut()>)>, LocalStorage> =
            StoredValue::new_local(None);
        Effect::new(move |_| {
            let Some(element) = handle_ref.get() else {
                return;
            };
            let store = store;
            let node_id = node_id.clone();
            // Wait one frame so layout (and stylesheets) have settled.
            let closure = Closure::once(move || {
                // The store may have been disposed between scheduling this
                // frame and it firing; bail instead of panicking.
                let Some(viewport) = store.try_get_viewport_untracked() else {
                    return;
                };
                if let Some(bound) = measure_handle_bound(&element, viewport.zoom) {
                    store.register_handle(&node_id, bound.with_connectable(connectable_end));
                }
            });
            if let Some(window) = web_sys::window() {
                // Cancel any previously scheduled (not-yet-fired) frame.
                raf.update_value(|slot| {
                    if let Some((id, _)) = slot.take() {
                        let _ = window.cancel_animation_frame(id);
                    }
                });
                if let Ok(id) =
                    window.request_animation_frame(closure.as_ref().unchecked_ref())
                {
                    raf.set_value(Some((id, closure)));
                }
            }
        });
        on_cleanup(move || {
            raf.update_value(|slot| {
                if let (Some(window), Some((id, _))) = (web_sys::window(), slot.take()) {
                    let _ = window.cancel_animation_frame(id);
                }
            });
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

    // Pointer-capture connection drag (start gated by `connectable_start`;
    // even non-connectable handles keep pointerdown from starting a node
    // drag underneath).
    let (on_pointer_down, on_pointer_move, on_pointer_up) = use_connection_handlers(
        node_id.clone(),
        id.clone(),
        r#type,
        connection_mode,
        is_valid_connection,
        connectable_start,
    );

    // Reactive in-flight connection state for this handle, so hosts can
    // style the drag: `connectingfrom` on the fixed end, `connectingto` +
    // `valid`/`invalid` on the current snap candidate.
    let connection_signal = store.state.connection_in_progress;
    let is_connecting_from = {
        let node_id = node_id.clone();
        let handle_id = id.clone();
        Memo::new(move |_| {
            connection_signal.with(|conn| {
                conn.as_ref().is_some_and(|c| {
                    c.from_node == node_id
                        && c.from_handle == handle_id
                        && c.from_handle_type == r#type
                })
            })
        })
    };
    // Some(is_valid) while this handle is the snap candidate, None otherwise
    let candidate_validity = {
        let node_id = node_id.clone();
        let handle_id = id.clone();
        Memo::new(move |_| {
            connection_signal.with(|conn| {
                conn.as_ref().and_then(|c| {
                    c.candidate
                        .as_ref()
                        .filter(|cand| {
                            cand.node_id == node_id
                                && cand.handle_id == handle_id
                                && cand.handle_type == r#type
                        })
                        .map(|_| c.is_valid)
                })
            })
        })
    };
    // While a connection is being dragged, every handle that could NOT accept
    // it dims so the valid drop targets stand out. Works from either drag
    // direction (source-start or target-start): the oriented connection is
    // built the same way `create_connection`/`to_connection` do, then run
    // through the same gates a real drop would face. The fixed end and every
    // valid target stay full-opacity.
    let dimmed = {
        let node_id = node_id.clone();
        let handle_id = id.clone();
        Memo::new(move |_| {
            connection_signal.with(|conn| {
                let Some(c) = conn.as_ref() else {
                    return false; // no drag in flight — nothing dims
                };
                // The drag's fixed end is never dimmed.
                if c.from_node == node_id
                    && c.from_handle == handle_id
                    && c.from_handle_type == r#type
                {
                    return false;
                }
                // Must be able to receive a connection AND be handle-type
                // compatible for the current mode.
                if !connectable_end
                    || !is_valid_handle_connection(c.from_handle_type, r#type, connection_mode)
                {
                    return true;
                }
                // Orient source/target by the fixed end's handle type, then ask
                // the host validator (reads its own state untracked).
                let candidate = if c.from_handle_type == HandleType::Source {
                    Connection::new(
                        c.from_node.clone(),
                        node_id.clone(),
                        c.from_handle.clone(),
                        handle_id.clone(),
                    )
                } else {
                    Connection::new(
                        node_id.clone(),
                        c.from_node.clone(),
                        handle_id.clone(),
                        c.from_handle.clone(),
                    )
                };
                let host_ok = store
                    .state
                    .is_valid_connection
                    .get_untracked()
                    .map_or(true, |cb| cb.run(candidate));
                !host_ok
            })
        })
    };

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
            class:connectablestart=connectable_start
            class:connectableend=connectable_end
            class:connectingfrom=move || is_connecting_from.get()
            class:connectingto=move || candidate_validity.get().is_some()
            class:valid=move || candidate_validity.get() == Some(true)
            class:invalid=move || candidate_validity.get() == Some(false)
            class:dimmed=move || dimmed.get()
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up.clone()
            on:pointercancel=on_pointer_up
        >
            {children.map(|c| c())}
        </div>
    }
}
