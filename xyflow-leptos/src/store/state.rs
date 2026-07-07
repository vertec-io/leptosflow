//! Flow state definition using Leptos signals

use std::collections::HashSet;
use leptos::prelude::*;
use leptos::html;
use crate::types::{Node, Edge, Viewport, Position};

/// Connection state while dragging from a handle
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
    /// Current mouse position in flow coordinates
    pub to_position: Position,
    /// Whether the connection is currently valid
    pub is_valid: bool,
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

    /// Connection in progress (when dragging from a handle)
    pub connection_in_progress: RwSignal<Option<ConnectionState>>,

    /// NodeRef to the flow container element (for coordinate conversion)
    pub container_ref: NodeRef<html::Div>,

    /// Called when a node drag finishes, with `(node_id, final_position)`.
    /// Consumers register this to persist node positions.
    pub on_node_drag_end: RwSignal<Option<Callback<(String, Position)>>>,
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
            min_zoom: RwSignal::new(0.5),
            max_zoom: RwSignal::new(2.0),
            pan_on_drag: RwSignal::new(true),
            connection_in_progress: RwSignal::new(None),
            container_ref: NodeRef::new(),
            on_node_drag_end: RwSignal::new(None),
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
            min_zoom: RwSignal::new(0.5),
            max_zoom: RwSignal::new(2.0),
            pan_on_drag: RwSignal::new(true),
            connection_in_progress: RwSignal::new(None),
            container_ref: NodeRef::new(),
            on_node_drag_end: RwSignal::new(None),
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
