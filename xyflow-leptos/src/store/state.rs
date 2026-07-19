//! Flow state definition using Leptos signals

use std::collections::HashSet;
use leptos::prelude::*;
use leptos::html;
use crate::types::{Node, Edge, Viewport, Position, Connection, HandleType};

/// How a plain (no-modifier) wheel/trackpad scroll drives the viewport.
///
/// A `ctrl`/`meta` wheel — trackpad pinch, ctrl+scroll — always zooms at the
/// cursor regardless of this setting; the mode only decides what a *plain*
/// scroll does. Matches react-flow's `zoomOnScroll` / `panOnScroll` pair;
/// the default is [`WheelMode::ZoomOnScroll`] (react-flow's default).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WheelMode {
    /// Plain scroll zooms at the cursor (default).
    #[default]
    ZoomOnScroll,
    /// Plain scroll pans by `(deltaX, deltaY)`.
    PanOnScroll,
}

/// The handle the in-flight connection is currently snapped to.
///
/// Present whenever the pointer is within the connection radius of a
/// connectable handle — even when that handle would form an INVALID
/// connection (`ConnectionState::is_valid` says which), so consumers can
/// style the hovered handle as valid or invalid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionCandidate {
    /// Node owning the candidate handle
    pub node_id: String,
    /// Candidate handle ID (None for default handle)
    pub handle_id: Option<String>,
    /// Candidate handle type
    pub handle_type: HandleType,
}

/// Connection state while dragging from a handle
///
/// Exposed reactively via `FlowState::connection_in_progress` (or the
/// [`use_connection`](crate::hooks::use_connection) hook) so hosts can style
/// handles during the drag: the `from_*` fields identify the fixed end,
/// `candidate`/`is_valid` describe the handle currently under the cursor.
#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionState {
    /// Source node ID
    pub from_node: String,
    /// Source handle ID (None for default handle)
    pub from_handle: Option<String>,
    /// Source handle type (Source or Target)
    pub from_handle_type: crate::types::HandleType,
    /// Source handle position in flow coordinates
    pub from_position: Position,
    /// Current free end position in flow coordinates (snapped to the
    /// candidate handle center when one is in range, else the cursor)
    pub to_position: Position,
    /// Handle currently snapped to (None while over empty space)
    pub candidate: Option<ConnectionCandidate>,
    /// Whether completing on the current candidate would be valid
    pub is_valid: bool,
}

impl ConnectionState {
    /// The `Connection` this drag would create if completed on the current
    /// candidate, with source/target ordered by handle type (dragging out of
    /// a Target handle still produces a source→target connection).
    ///
    /// `None` while no candidate handle is snapped.
    pub fn to_connection(&self) -> Option<Connection> {
        let candidate = self.candidate.as_ref()?;
        Some(if self.from_handle_type == HandleType::Source {
            Connection::new(
                self.from_node.clone(),
                candidate.node_id.clone(),
                self.from_handle.clone(),
                candidate.handle_id.clone(),
            )
        } else {
            Connection::new(
                candidate.node_id.clone(),
                self.from_node.clone(),
                candidate.handle_id.clone(),
                self.from_handle.clone(),
            )
        })
    }
}

/// The complete state of the flow
///
/// Phase 2: Now uses Leptos RwSignal for full reactivity.
/// Each field is a signal that can be read and written independently.
#[derive(Clone, Copy)]
pub struct FlowState {
    /// All nodes in the flow
    pub nodes: RwSignal<Vec<Node>>,

    /// All edges in the flow
    pub edges: RwSignal<Vec<Edge>>,

    /// The viewport state (pan and zoom)
    pub viewport: RwSignal<Viewport>,

    /// IDs of selected nodes
    pub selected_nodes: RwSignal<HashSet<String>>,

    /// IDs of selected edges
    pub selected_edges: RwSignal<HashSet<String>>,

    /// Currently dragging node IDs
    pub dragging_nodes: RwSignal<HashSet<String>>,

    /// Minimum zoom level
    pub min_zoom: RwSignal<f64>,

    /// Maximum zoom level
    pub max_zoom: RwSignal<f64>,

    /// Whether panning on drag is enabled
    pub pan_on_drag: RwSignal<bool>,

    /// How a plain (no-modifier) wheel scroll drives the viewport
    /// (zoom-at-cursor vs pan). See [`WheelMode`].
    pub wheel_mode: RwSignal<WheelMode>,

    /// Connection in progress (when dragging from a handle)
    pub connection_in_progress: RwSignal<Option<ConnectionState>>,

    /// NodeRef to the flow container element (for coordinate conversion)
    pub container_ref: NodeRef<html::Div>,

    /// Called when a node drag finishes, with `(node_id, final_position)`.
    /// Consumers register this to persist node positions.
    pub on_node_drag_end: RwSignal<Option<Callback<(String, Position)>>>,

    /// Called when a connection drag completes on a valid handle.
    ///
    /// When registered, the crate does NOT insert an edge itself — the host
    /// receives the `Connection` and decides what to create (matching xyflow's
    /// `onConnect` semantics). When absent, a default edge is added.
    pub on_connect: RwSignal<Option<Callback<Connection>>>,

    /// Host-supplied connection validity predicate, applied to every
    /// candidate while dragging and enforced on completion. Unlike the
    /// per-`Handle` `is_valid_connection` fn pointer, this is a `Callback`,
    /// so it may capture host state (port types, existing bindings, ...).
    pub is_valid_connection: RwSignal<Option<Callback<Connection, bool>>>,

    /// Called when the user requests deletion of the current selection
    /// (Delete/Backspace with the flow focused, or
    /// `FlowStore::request_delete_selection`).
    ///
    /// When registered, the crate does NOT delete anything itself — the host
    /// receives the request and decides (matching `on_connect` semantics).
    /// When absent, the selection is removed from the store directly.
    pub on_delete_requested: RwSignal<Option<Callback<DeleteRequest>>>,

    /// Called on right-click over a node (built-in or custom — resolved via
    /// the `.xyflow__node` ancestor and its `data-id`). While registered,
    /// the native browser menu is suppressed for that target.
    pub on_node_context_menu: RwSignal<Option<Callback<ContextMenuEvent>>>,

    /// Called on right-click over an edge (resolved via the `.xyflow__edge`
    /// group and its `data-id`). While registered, the native browser menu
    /// is suppressed for that target.
    pub on_edge_context_menu: RwSignal<Option<Callback<ContextMenuEvent>>>,

    /// Called on right-click over the empty pane (`ContextMenuEvent::id` is
    /// `None`). While registered, the native browser menu is suppressed.
    pub on_pane_context_menu: RwSignal<Option<Callback<ContextMenuEvent>>>,
}

/// The selection the user asked to delete. Passed to `on_delete_requested`
/// (host-owned deletion) or applied directly by the store when no callback
/// is registered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeleteRequest {
    /// Selected node IDs (deleting a node also implies its attached edges)
    pub nodes: Vec<String>,
    /// Selected edge IDs
    pub edges: Vec<String>,
}

impl DeleteRequest {
    /// Whether the request contains anything to delete
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

/// Payload for the node/edge/pane context-menu callbacks.
#[derive(Clone, Debug, PartialEq)]
pub struct ContextMenuEvent {
    /// The node or edge id under the cursor (`None` for the pane)
    pub id: Option<String>,
    /// Pointer position in screen (client) coordinates — position fixed
    /// menus with these
    pub screen_x: f64,
    /// Pointer position in screen (client) coordinates
    pub screen_y: f64,
    /// Pointer position in flow coordinates — use these to place elements
    /// on the canvas (e.g. "add node here")
    pub flow_x: f64,
    /// Pointer position in flow coordinates
    pub flow_y: f64,
}

impl FlowState {
    /// Create a new flow state with initial nodes and edges
    pub fn new(initial_nodes: Vec<Node>, initial_edges: Vec<Edge>) -> Self {
        FlowState {
            nodes: RwSignal::new(initial_nodes),
            edges: RwSignal::new(initial_edges),
            viewport: RwSignal::new(Viewport::default()),
            selected_nodes: RwSignal::new(HashSet::new()),
            selected_edges: RwSignal::new(HashSet::new()),
            dragging_nodes: RwSignal::new(HashSet::new()),
            min_zoom: RwSignal::new(0.2),
            max_zoom: RwSignal::new(4.0),
            pan_on_drag: RwSignal::new(true),
            wheel_mode: RwSignal::new(WheelMode::default()),
            connection_in_progress: RwSignal::new(None),
            container_ref: NodeRef::new(),
            on_node_drag_end: RwSignal::new(None),
            on_connect: RwSignal::new(None),
            is_valid_connection: RwSignal::new(None),
            on_delete_requested: RwSignal::new(None),
            on_node_context_menu: RwSignal::new(None),
            on_edge_context_menu: RwSignal::new(None),
            on_pane_context_menu: RwSignal::new(None),
        }
    }

    /// Create a new flow state from existing signals
    pub fn from_signals(nodes: RwSignal<Vec<Node>>, edges: RwSignal<Vec<Edge>>) -> Self {
        FlowState {
            nodes,
            edges,
            viewport: RwSignal::new(Viewport::default()),
            selected_nodes: RwSignal::new(HashSet::new()),
            selected_edges: RwSignal::new(HashSet::new()),
            dragging_nodes: RwSignal::new(HashSet::new()),
            min_zoom: RwSignal::new(0.2),
            max_zoom: RwSignal::new(4.0),
            pan_on_drag: RwSignal::new(true),
            wheel_mode: RwSignal::new(WheelMode::default()),
            connection_in_progress: RwSignal::new(None),
            container_ref: NodeRef::new(),
            on_node_drag_end: RwSignal::new(None),
            on_connect: RwSignal::new(None),
            is_valid_connection: RwSignal::new(None),
            on_delete_requested: RwSignal::new(None),
            on_node_context_menu: RwSignal::new(None),
            on_edge_context_menu: RwSignal::new(None),
            on_pane_context_menu: RwSignal::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Position;

    #[test]
    fn test_flow_state_creation() {
        let nodes = vec![
            Node::new("n1".to_string(), Position::new(0.0, 0.0)),
            Node::new("n2".to_string(), Position::new(100.0, 0.0)),
        ];
        let edges = vec![];

        let state = FlowState::new(nodes, edges);

        assert_eq!(state.nodes.get().len(), 2);
        assert_eq!(state.edges.get().len(), 0);
    }
}
