//! Store module (state management)
//!
//! The store manages the state of the flow using Leptos signals.
//! It provides reactive state and action methods.

pub mod state;

pub use state::FlowState;

use std::collections::HashSet;
use leptos::prelude::*;
use crate::types::{Node, Edge, Viewport, Position};
pub use state::ConnectionState;

/// The main flow store containing all state and actions
///
/// Phase 2: Now uses Leptos RwSignal for full reactivity.
/// The store is Copy because FlowState is Copy (all signals are Copy).
#[derive(Clone, Copy)]
pub struct FlowStore {
    /// Current flow state
    pub state: FlowState,
}

impl FlowStore {
    /// Create a new store with initial nodes and edges
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        FlowStore {
            state: FlowState::new(nodes, edges),
        }
    }

    /// Create a new store from existing signals
    pub fn from_signals(nodes: RwSignal<Vec<Node>>, edges: RwSignal<Vec<Edge>>) -> Self {
        FlowStore {
            state: FlowState::from_signals(nodes, edges),
        }
    }

    // ===== Getters =====

    /// Get all nodes (reactive)
    pub fn get_nodes(&self) -> Vec<Node> {
        self.state.nodes.get()
    }

    /// Get all nodes (non-reactive, for use in event handlers)
    pub fn get_nodes_untracked(&self) -> Vec<Node> {
        self.state.nodes.get_untracked()
    }

    /// Get all edges (reactive)
    pub fn get_edges(&self) -> Vec<Edge> {
        self.state.edges.get()
    }

    /// Get all edges (non-reactive, for use in event handlers)
    pub fn get_edges_untracked(&self) -> Vec<Edge> {
        self.state.edges.get_untracked()
    }

    /// Get the current viewport (reactive)
    pub fn get_viewport(&self) -> Viewport {
        self.state.viewport.get()
    }

    /// Get the current viewport (non-reactive, for use in event handlers)
    pub fn get_viewport_untracked(&self) -> Viewport {
        self.state.viewport.get_untracked()
    }

    /// Get selected node IDs (reactive)
    pub fn get_selected_nodes(&self) -> HashSet<String> {
        self.state.selected_nodes.get()
    }

    /// Get selected edge IDs (reactive)
    pub fn get_selected_edges(&self) -> HashSet<String> {
        self.state.selected_edges.get()
    }

    // ===== Node Actions =====

    /// Set all nodes
    pub fn set_nodes(&self, nodes: Vec<Node>) {
        self.state.nodes.set(nodes);
    }

    /// Add a node to the flow
    pub fn add_node(&self, node: Node) {
        self.state.nodes.update(|nodes| {
            nodes.push(node);
        });
    }

    /// Remove a node by ID
    pub fn remove_node(&self, id: &str) -> bool {
        let mut found = false;
        self.state.nodes.update(|nodes| {
            if let Some(pos) = nodes.iter().position(|n| n.id == id) {
                nodes.remove(pos);
                found = true;
            }
        });
        found
    }

    /// Update a node by ID
    pub fn update_node<F>(&self, id: &str, update_fn: F) -> bool
    where
        F: FnOnce(&mut Node),
    {
        let mut found = false;
        self.state.nodes.update(|nodes| {
            if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                update_fn(node);
                found = true;
            }
        });
        found
    }

    // ===== Handle Registration =====

    /// Register (or refresh) a measured handle bound on a node.
    ///
    /// Called by the `Handle` component after measuring itself in the DOM.
    /// Bounds are stored relative to the node's top-left corner, so they stay
    /// valid while the node is dragged; `EdgeRenderer` resolves the absolute
    /// anchor as `node.position + bound.center()`.
    pub fn register_handle(&self, node_id: &str, bound: crate::types::HandleBound) {
        self.state.nodes.update(|nodes| {
            if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
                let bounds = node
                    .internals
                    .handle_bounds
                    .get_or_insert_with(crate::types::HandleBounds::new);
                let list = match bound.handle_type {
                    crate::types::HandleType::Source => &mut bounds.source,
                    crate::types::HandleType::Target => &mut bounds.target,
                };
                if let Some(existing) = list.iter_mut().find(|h| h.id == bound.id) {
                    *existing = bound;
                } else {
                    list.push(bound);
                }
                node.internals.measured = true;
            }
        });
    }

    /// Remove a previously registered handle bound (called on handle unmount).
    pub fn unregister_handle(
        &self,
        node_id: &str,
        handle_id: Option<&str>,
        handle_type: crate::types::HandleType,
    ) {
        self.state.nodes.update(|nodes| {
            if let Some(node) = nodes.iter_mut().find(|n| n.id == node_id) {
                if let Some(bounds) = node.internals.handle_bounds.as_mut() {
                    let list = match handle_type {
                        crate::types::HandleType::Source => &mut bounds.source,
                        crate::types::HandleType::Target => &mut bounds.target,
                    };
                    list.retain(|h| h.id.as_deref() != handle_id);
                }
            }
        });
    }

    // ===== Edge Actions =====

    /// Set all edges
    pub fn set_edges(&self, edges: Vec<Edge>) {
        self.state.edges.set(edges);
    }

    /// Add an edge to the flow
    pub fn add_edge(&self, edge: Edge) {
        self.state.edges.update(|edges| {
            edges.push(edge);
        });
    }

    /// Remove an edge by ID
    pub fn remove_edge(&self, id: &str) -> bool {
        let mut found = false;
        self.state.edges.update(|edges| {
            if let Some(pos) = edges.iter().position(|e| e.id == id) {
                edges.remove(pos);
                found = true;
            }
        });
        found
    }

    // ===== Selection Actions =====

    /// Select a node
    /// If multi_select is false, deselects all other nodes and edges first
    pub fn select_node(&self, id: &str, multi_select: bool) {
        // If not multi-select, clear all selections first
        if !multi_select {
            self.clear_node_selection();
            self.clear_edge_selection();
        }

        self.state.selected_nodes.update(|selected| {
            selected.insert(id.to_string());
        });
        self.update_node(id, |node| node.selected = true);
    }

    /// Deselect a node
    pub fn deselect_node(&self, id: &str) {
        self.state.selected_nodes.update(|selected| {
            selected.remove(id);
        });
        self.update_node(id, |node| node.selected = false);
    }

    /// Clear all node selections
    pub fn clear_node_selection(&self) {
        let selected = self.state.selected_nodes.get();
        for id in selected {
            self.update_node(&id, |node| node.selected = false);
        }
        self.state.selected_nodes.update(|selected| {
            selected.clear();
        });
    }

    /// Select an edge
    /// If multi_select is false, deselects all other nodes and edges first
    pub fn select_edge(&self, id: &str, multi_select: bool) {
        // If not multi-select, clear all selections first
        if !multi_select {
            self.clear_node_selection();
            self.clear_edge_selection();
        }

        self.state.selected_edges.update(|selected| {
            selected.insert(id.to_string());
        });
        self.state.edges.update(|edges| {
            if let Some(edge) = edges.iter_mut().find(|e| e.id == id) {
                edge.selected = true;
            }
        });
    }

    /// Deselect an edge
    pub fn deselect_edge(&self, id: &str) {
        self.state.selected_edges.update(|selected| {
            selected.remove(id);
        });
        self.state.edges.update(|edges| {
            if let Some(edge) = edges.iter_mut().find(|e| e.id == id) {
                edge.selected = false;
            }
        });
    }

    /// Clear all edge selections
    pub fn clear_edge_selection(&self) {
        let selected = self.state.selected_edges.get();
        self.state.edges.update(|edges| {
            for id in &selected {
                if let Some(edge) = edges.iter_mut().find(|e| &e.id == id) {
                    edge.selected = false;
                }
            }
        });
        self.state.selected_edges.update(|selected| {
            selected.clear();
        });
    }

    // ===== Viewport Actions =====

    /// Set the viewport
    pub fn set_viewport(&self, viewport: Viewport) {
        self.state.viewport.set(viewport);
    }

    /// Pan the viewport by a delta
    pub fn pan_by(&self, dx: f64, dy: f64) {
        self.state.viewport.update(|viewport| {
            *viewport = viewport.pan_by(dx, dy);
        });
    }

    /// Zoom the viewport
    pub fn zoom_by(&self, factor: f64) {
        self.state.viewport.update(|viewport| {
            *viewport = viewport.zoom_by(factor);
        });
    }

    /// Fit all (visible) nodes into the container with 10% padding.
    ///
    /// See [`crate::utils::fit_view::fit_view_with_options`] for custom padding
    /// or zoom clamps.
    pub fn fit_view(&self) {
        crate::utils::fit_view::fit_view(self);
    }

    /// Frame an arbitrary flow-coordinate rectangle in the container.
    pub fn fit_bounds(&self, bounds: crate::types::Bounds) {
        crate::utils::fit_view::fit_bounds_with_options(
            self,
            bounds,
            crate::utils::fit_view::FitViewOptions::default(),
        );
    }

    /// Zoom the viewport to frame the currently selected nodes.
    ///
    /// Falls back to fitting all nodes when nothing is selected.
    pub fn zoom_to_selection(&self) {
        let selected = self.state.selected_nodes.get_untracked();
        if selected.is_empty() {
            self.fit_view();
            return;
        }
        let nodes: Vec<crate::types::Node> = self
            .get_nodes_untracked()
            .into_iter()
            .filter(|n| selected.contains(&n.id))
            .collect();
        if let Some(bounds) = crate::utils::fit_view::nodes_bounds(&nodes) {
            self.fit_bounds(bounds);
        }
    }

    // ===== Connection Actions =====

    /// Start a connection from a handle
    pub fn start_connection(&self, from_node: String, from_handle: Option<String>, from_handle_type: crate::types::HandleType, from_position: Position) {
        let connection = ConnectionState {
            from_node,
            from_handle,
            from_handle_type,
            from_position,
            to_position: from_position,
            is_valid: false,
        };
        self.state.connection_in_progress.set(Some(connection));
    }

    /// Update the connection target position
    pub fn update_connection(&self, to_position: Position, is_valid: bool) {
        self.state.connection_in_progress.update(|conn| {
            if let Some(connection) = conn {
                connection.to_position = to_position;
                connection.is_valid = is_valid;
            }
        });
    }

    /// Complete the connection and create an edge
    pub fn complete_connection(&self, target_node: String, target_handle: Option<String>) -> Option<Edge> {
        let connection = self.state.connection_in_progress.get();
        self.state.connection_in_progress.set(None);

        if let Some(conn) = connection {
            if conn.is_valid {
                // Generate edge ID
                let edge_id = format!("e-{}-{}", conn.from_node, target_node);
                let edge = Edge::new(edge_id, conn.from_node, target_node)
                    .with_source_handle(conn.from_handle)
                    .with_target_handle(target_handle);

                self.add_edge(edge.clone());
                return Some(edge);
            }
        }
        None
    }

    /// Cancel the connection in progress
    pub fn cancel_connection(&self) {
        self.state.connection_in_progress.set(None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Position;

    #[test]
    fn test_store_creation() {
        let nodes = vec![
            Node::new("n1".to_string(), Position::new(0.0, 0.0)),
            Node::new("n2".to_string(), Position::new(100.0, 0.0)),
        ];
        let edges = vec![];

        let store = FlowStore::new(nodes, edges);

        assert_eq!(store.get_nodes().len(), 2);
        assert_eq!(store.get_edges().len(), 0);
    }

    #[test]
    fn test_add_node() {
        let store = FlowStore::new(vec![], vec![]);
        let node = Node::new("n1".to_string(), Position::new(0.0, 0.0));

        store.add_node(node);

        assert_eq!(store.get_nodes().len(), 1);
        assert_eq!(store.get_nodes()[0].id, "n1");
    }

    #[test]
    fn test_remove_node() {
        let nodes = vec![
            Node::new("n1".to_string(), Position::new(0.0, 0.0)),
            Node::new("n2".to_string(), Position::new(100.0, 0.0)),
        ];
        let store = FlowStore::new(nodes, vec![]);

        assert!(store.remove_node("n1"));
        assert_eq!(store.get_nodes().len(), 1);
        assert_eq!(store.get_nodes()[0].id, "n2");

        assert!(!store.remove_node("n3")); // Non-existent node
    }

    #[test]
    fn test_update_node() {
        let nodes = vec![Node::new("n1".to_string(), Position::new(0.0, 0.0))];
        let store = FlowStore::new(nodes, vec![]);

        assert!(store.update_node("n1", |node| {
            node.position.x = 50.0;
        }));

        assert_eq!(store.get_nodes()[0].position.x, 50.0);
    }

    #[test]
    fn test_node_selection() {
        let nodes = vec![
            Node::new("n1".to_string(), Position::new(0.0, 0.0)),
            Node::new("n2".to_string(), Position::new(100.0, 0.0)),
        ];
        let store = FlowStore::new(nodes, vec![]);

        // Single select
        store.select_node("n1", false);
        assert_eq!(store.get_selected_nodes().len(), 1);
        assert!(store.get_selected_nodes().contains("n1"));
        assert!(store.get_nodes()[0].selected);

        store.select_node("n2", true);
        assert_eq!(store.get_selected_nodes().len(), 2);

        store.deselect_node("n1");
        assert_eq!(store.get_selected_nodes().len(), 1);
        assert!(!store.get_nodes()[0].selected);

        store.clear_node_selection();
        assert_eq!(store.get_selected_nodes().len(), 0);
    }

    #[test]
    fn test_add_edge() {
        let store = FlowStore::new(vec![], vec![]);
        let edge = Edge::new("e1".to_string(), "n1".to_string(), "n2".to_string());

        store.add_edge(edge);

        assert_eq!(store.get_edges().len(), 1);
        assert_eq!(store.get_edges()[0].id, "e1");
    }

    #[test]
    fn test_remove_edge() {
        let edges = vec![
            Edge::new("e1".to_string(), "n1".to_string(), "n2".to_string()),
            Edge::new("e2".to_string(), "n2".to_string(), "n3".to_string()),
        ];
        let store = FlowStore::new(vec![], edges);

        assert!(store.remove_edge("e1"));
        assert_eq!(store.get_edges().len(), 1);
        assert_eq!(store.get_edges()[0].id, "e2");

        assert!(!store.remove_edge("e3")); // Non-existent edge
    }

    #[test]
    fn test_viewport_actions() {
        let store = FlowStore::new(vec![], vec![]);

        // Test pan
        store.pan_by(10.0, 20.0);
        let viewport = store.get_viewport();
        assert_eq!(viewport.x, 10.0);
        assert_eq!(viewport.y, 20.0);

        // Test zoom
        store.zoom_by(2.0);
        let viewport = store.get_viewport();
        assert_eq!(viewport.zoom, 2.0);
    }
}
