//! Store module (state management)
//!
//! The store manages the state of the flow using Leptos signals.
//! It provides reactive state and action methods.

pub mod state;

pub use state::FlowState;

use std::collections::HashSet;
use leptos::prelude::*;
use crate::types::{Node, Edge, Viewport, Position, Connection};
pub use state::{ConnectionState, ConnectionCandidate, ContextMenuEvent, DeleteRequest, WheelMode};

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

    /// Get the current viewport non-reactively, returning `None` if the
    /// store's reactive scope has already been disposed.
    ///
    /// Deferred callbacks (a `requestAnimationFrame` scheduled on mount, a
    /// queued microtask) can fire *after* the owning component was unmounted
    /// — e.g. a handle whose node was removed by a rapid re-render. Reading a
    /// disposed signal with [`get_viewport_untracked`](Self::get_viewport_untracked)
    /// panics, and a panic inside the reactive graph poisons it for the whole
    /// app (every subsequent signal access then fails). Async/deferred reads
    /// must use this fallible variant and bail when it returns `None`.
    pub fn try_get_viewport_untracked(&self) -> Option<Viewport> {
        self.state.viewport.try_get_untracked()
    }

    /// Get selected node IDs (reactive)
    pub fn get_selected_nodes(&self) -> HashSet<String> {
        self.state.selected_nodes.get()
    }

    /// Get selected edge IDs (reactive)
    pub fn get_selected_edges(&self) -> HashSet<String> {
        self.state.selected_edges.get()
    }

    /// Get the wheel mode (non-reactive, for use in the wheel event handler)
    pub fn get_wheel_mode_untracked(&self) -> WheelMode {
        self.state.wheel_mode.get_untracked()
    }

    /// Set how a plain wheel scroll drives the viewport (zoom vs pan).
    pub fn set_wheel_mode(&self, mode: WheelMode) {
        self.state.wheel_mode.set(mode);
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
    ///
    /// Reads untracked and no-ops when nothing is selected: a tracked read
    /// followed by a write to the same signal makes any calling effect
    /// subscribe to and then invalidate itself — an infinite loop.
    pub fn clear_node_selection(&self) {
        let selected = self.state.selected_nodes.get_untracked();
        if selected.is_empty() {
            return;
        }
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
    ///
    /// Untracked + empty-guard for the same reason as
    /// [`clear_node_selection`](Self::clear_node_selection).
    pub fn clear_edge_selection(&self) {
        let selected = self.state.selected_edges.get_untracked();
        if selected.is_empty() {
            return;
        }
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

    /// Zoom by `factor`, keeping the flow point under `(point_x, point_y)`
    /// stationary on screen.
    ///
    /// The point is in screen pixels relative to the flow container's
    /// top-left corner (e.g. `clientX - containerRect.left`). The zoom is
    /// clamped to the store's `min_zoom`/`max_zoom`.
    ///
    /// All reads are untracked: this is called from event handlers and must
    /// never leave reactive subscriptions in the caller's scope.
    pub fn zoom_at(&self, factor: f64, point_x: f64, point_y: f64) {
        let min_zoom = self.state.min_zoom.get_untracked();
        let max_zoom = self.state.max_zoom.get_untracked();
        self.state.viewport.update(|viewport| {
            *viewport = viewport.zoom_at_point(factor, point_x, point_y, min_zoom, max_zoom);
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

    // ===== Drag Callbacks =====

    /// Register a callback fired when a node drag finishes, with
    /// `(node_id, final_position)`. Use this to persist node positions.
    pub fn set_on_node_drag_end(&self, callback: Callback<(String, Position)>) {
        self.state.on_node_drag_end.set(Some(callback));
    }

    /// Remove the drag-end callback.
    pub fn clear_on_node_drag_end(&self) {
        self.state.on_node_drag_end.set(None);
    }

    // ===== Connection Callbacks =====

    /// Register a callback fired when a connection drag completes on a valid
    /// handle. The callback receives the `Connection` (source node/handle,
    /// target node/handle, already ordered source→target).
    ///
    /// While an `on_connect` callback is registered the crate does NOT insert
    /// an edge itself — the host decides what the connection becomes
    /// (matching xyflow's `onConnect` semantics). Without one,
    /// [`complete_connection`](Self::complete_connection) adds a default edge.
    pub fn set_on_connect(&self, callback: Callback<Connection>) {
        self.state.on_connect.set(Some(callback));
    }

    /// Remove the connect callback (restores default-edge insertion).
    pub fn clear_on_connect(&self) {
        self.state.on_connect.set(None);
    }

    /// Register a host-level validity predicate enforced on every candidate
    /// while dragging and again on completion. Runs in addition to the
    /// connection-mode check and any per-`Handle` `is_valid_connection`.
    pub fn set_is_valid_connection(&self, callback: Callback<Connection, bool>) {
        self.state.is_valid_connection.set(Some(callback));
    }

    /// Remove the host-level validity predicate.
    pub fn clear_is_valid_connection(&self) {
        self.state.is_valid_connection.set(None);
    }

    // ===== Connection Actions =====
    //
    // State machine: `start_connection` → `update_connection`* →
    // `complete_connection` | `cancel_connection`. Driven by the pointer
    // handlers in [`crate::events::connection`]; the in-flight state is
    // readable via `state.connection_in_progress` / `use_connection`.

    /// Start a connection from a handle
    pub fn start_connection(&self, from_node: String, from_handle: Option<String>, from_handle_type: crate::types::HandleType, from_position: Position) {
        let connection = ConnectionState {
            from_node,
            from_handle,
            from_handle_type,
            from_position,
            to_position: from_position,
            candidate: None,
            is_valid: false,
        };
        self.state.connection_in_progress.set(Some(connection));
    }

    /// Update the in-flight connection: free-end position, the handle it is
    /// currently snapped to (if any), and whether completing there would be
    /// valid. No-op when no connection is in flight.
    pub fn update_connection(
        &self,
        to_position: Position,
        candidate: Option<ConnectionCandidate>,
        is_valid: bool,
    ) {
        self.state.connection_in_progress.update(|conn| {
            if let Some(connection) = conn {
                connection.to_position = to_position;
                connection.candidate = candidate;
                connection.is_valid = is_valid;
            }
        });
    }

    /// Complete the in-flight connection on its current candidate handle.
    ///
    /// Succeeds only when a candidate is snapped AND the last
    /// [`update_connection`](Self::update_connection) marked it valid;
    /// otherwise this behaves like [`cancel_connection`](Self::cancel_connection)
    /// and returns `None`. Either way the in-flight state is cleared — a drop
    /// can never leave a stuck connection line.
    ///
    /// On success the `Connection` is passed to the registered `on_connect`
    /// callback; if none is registered, a default edge is added instead.
    pub fn complete_connection(&self) -> Option<Connection> {
        let in_flight = self.state.connection_in_progress.get_untracked();
        self.state.connection_in_progress.set(None);

        let conn = in_flight?;
        if !conn.is_valid {
            return None;
        }
        let connection = conn.to_connection()?;

        if let Some(callback) = self.state.on_connect.get_untracked() {
            // Host-owned completion: the host creates its own edge/binding.
            callback.run(connection.clone());
        } else {
            let edge_id = format!(
                "e-{}{}-{}{}",
                connection.source,
                connection
                    .source_handle
                    .as_deref()
                    .map(|h| format!(":{h}"))
                    .unwrap_or_default(),
                connection.target,
                connection
                    .target_handle
                    .as_deref()
                    .map(|h| format!(":{h}"))
                    .unwrap_or_default(),
            );
            let edge = Edge::new(
                edge_id,
                connection.source.clone(),
                connection.target.clone(),
            )
            .with_source_handle(connection.source_handle.clone())
            .with_target_handle(connection.target_handle.clone());
            self.add_edge(edge);
        }

        Some(connection)
    }

    /// Cancel the connection in progress
    pub fn cancel_connection(&self) {
        self.state.connection_in_progress.set(None);
    }

    /// Focus the flow container so its keyboard shortcuts (Delete/Backspace,
    /// Escape) work right away. Pointer handlers that `prevent_default()` on
    /// pointerdown (connection + node drags) suppress the browser's native
    /// focus-on-click, so they call this explicitly.
    pub fn focus_container(&self) {
        if let Some(container) = self.state.container_ref.get_untracked() {
            let _ = container.focus();
        }
    }

    // ===== Deletion =====

    /// Register a callback fired when the user requests deletion of the
    /// current selection (Delete/Backspace with the flow focused, or a
    /// direct [`request_delete_selection`](Self::request_delete_selection)
    /// call).
    ///
    /// While registered the crate does NOT delete anything itself — the host
    /// receives the [`DeleteRequest`] and decides (matching `on_connect`
    /// semantics). Without one, the selection is removed from the store.
    pub fn set_on_delete_requested(&self, callback: Callback<DeleteRequest>) {
        self.state.on_delete_requested.set(Some(callback));
    }

    /// Remove the delete callback (restores store-owned deletion).
    pub fn clear_on_delete_requested(&self) {
        self.state.on_delete_requested.set(None);
    }

    /// Request deletion of the currently selected nodes and edges.
    ///
    /// No-op returning `None` when nothing is selected. Otherwise the
    /// [`DeleteRequest`] goes to the registered `on_delete_requested`
    /// callback (host-owned deletion) or, when none is registered, is
    /// applied directly via [`delete_elements`](Self::delete_elements).
    pub fn request_delete_selection(&self) -> Option<DeleteRequest> {
        let mut nodes: Vec<String> = self
            .state
            .selected_nodes
            .get_untracked()
            .into_iter()
            .collect();
        let mut edges: Vec<String> = self
            .state
            .selected_edges
            .get_untracked()
            .into_iter()
            .collect();
        // Deterministic order (HashSet iteration is not)
        nodes.sort();
        edges.sort();

        let request = DeleteRequest { nodes, edges };
        if request.is_empty() {
            return None;
        }

        if let Some(callback) = self.state.on_delete_requested.get_untracked() {
            // Host-owned deletion: the host mutates its own state.
            callback.run(request.clone());
        } else {
            self.delete_elements(&request.nodes, &request.edges);
        }
        Some(request)
    }

    /// Remove the given nodes and edges from the store.
    ///
    /// Deleting a node also removes every edge attached to it (xyflow
    /// semantics). Selection entries for removed elements are cleared.
    /// Hosts using `on_delete_requested` can call this to apply a request
    /// after their own bookkeeping.
    pub fn delete_elements(&self, nodes: &[String], edges: &[String]) {
        if nodes.is_empty() && edges.is_empty() {
            return;
        }

        self.state.edges.update(|all_edges| {
            all_edges.retain(|e| {
                !edges.contains(&e.id)
                    && !nodes.contains(&e.source)
                    && !nodes.contains(&e.target)
            });
        });
        if !nodes.is_empty() {
            self.state.nodes.update(|all_nodes| {
                all_nodes.retain(|n| !nodes.contains(&n.id));
            });
        }

        // Drop stale selection entries (untracked read + guarded write,
        // per the self-invalidating-effect rule)
        let selected_nodes = self.state.selected_nodes.get_untracked();
        if selected_nodes.iter().any(|id| nodes.contains(id)) {
            self.state.selected_nodes.update(|selected| {
                selected.retain(|id| !nodes.contains(id));
            });
        }
        let selected_edges = self.state.selected_edges.get_untracked();
        if selected_edges.iter().any(|id| edges.contains(id)) {
            self.state.selected_edges.update(|selected| {
                selected.retain(|id| !edges.contains(id));
            });
        }
    }

    // ===== Context Menus =====

    /// Register a callback for right-clicks on nodes (built-in or custom —
    /// resolved through the `.xyflow__node` ancestor's `data-id`). While
    /// registered, the native browser menu is suppressed over nodes.
    pub fn set_on_node_context_menu(&self, callback: Callback<ContextMenuEvent>) {
        self.state.on_node_context_menu.set(Some(callback));
    }

    /// Remove the node context-menu callback.
    pub fn clear_on_node_context_menu(&self) {
        self.state.on_node_context_menu.set(None);
    }

    /// Register a callback for right-clicks on edges (resolved through the
    /// `.xyflow__edge` group's `data-id`). While registered, the native
    /// browser menu is suppressed over edges.
    pub fn set_on_edge_context_menu(&self, callback: Callback<ContextMenuEvent>) {
        self.state.on_edge_context_menu.set(Some(callback));
    }

    /// Remove the edge context-menu callback.
    pub fn clear_on_edge_context_menu(&self) {
        self.state.on_edge_context_menu.set(None);
    }

    /// Register a callback for right-clicks on the empty pane
    /// (`ContextMenuEvent::id` is `None`). While registered, the native
    /// browser menu is suppressed on the pane.
    pub fn set_on_pane_context_menu(&self, callback: Callback<ContextMenuEvent>) {
        self.state.on_pane_context_menu.set(Some(callback));
    }

    /// Remove the pane context-menu callback.
    pub fn clear_on_pane_context_menu(&self) {
        self.state.on_pane_context_menu.set(None);
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

    // ===== Connection state machine =====

    use crate::types::HandleType;

    fn start_test_connection(store: &FlowStore) {
        store.start_connection(
            "n1".to_string(),
            Some("out".to_string()),
            HandleType::Source,
            Position::new(10.0, 10.0),
        );
    }

    fn candidate(node: &str, handle: Option<&str>, handle_type: HandleType) -> ConnectionCandidate {
        ConnectionCandidate {
            node_id: node.to_string(),
            handle_id: handle.map(str::to_string),
            handle_type,
        }
    }

    #[test]
    fn test_connection_start_update_complete() {
        let store = FlowStore::new(vec![], vec![]);
        start_test_connection(&store);

        let in_flight = store.state.connection_in_progress.get_untracked().unwrap();
        assert_eq!(in_flight.from_node, "n1");
        assert_eq!(in_flight.candidate, None);
        assert!(!in_flight.is_valid);

        // Cursor over empty space: position tracks, no candidate
        store.update_connection(Position::new(50.0, 50.0), None, false);
        let in_flight = store.state.connection_in_progress.get_untracked().unwrap();
        assert_eq!(in_flight.to_position, Position::new(50.0, 50.0));
        assert!(in_flight.candidate.is_none());

        // Snap to a valid target handle and complete
        store.update_connection(
            Position::new(100.0, 20.0),
            Some(candidate("n2", Some("in"), HandleType::Target)),
            true,
        );
        let connection = store.complete_connection().expect("valid completion");
        assert_eq!(connection.source, "n1");
        assert_eq!(connection.source_handle.as_deref(), Some("out"));
        assert_eq!(connection.target, "n2");
        assert_eq!(connection.target_handle.as_deref(), Some("in"));

        // No on_connect registered: default edge added, in-flight cleared
        assert_eq!(store.get_edges().len(), 1);
        assert_eq!(store.get_edges()[0].source, "n1");
        assert_eq!(store.get_edges()[0].target, "n2");
        assert!(store.state.connection_in_progress.get_untracked().is_none());
    }

    #[test]
    fn test_connection_invalid_candidate_cannot_complete() {
        let store = FlowStore::new(vec![], vec![]);
        start_test_connection(&store);

        // Candidate snapped but marked invalid (validators rejected it)
        store.update_connection(
            Position::new(100.0, 20.0),
            Some(candidate("n2", Some("in"), HandleType::Target)),
            false,
        );

        assert!(store.complete_connection().is_none());
        assert_eq!(store.get_edges().len(), 0);
        // No stuck in-flight state after a failed drop
        assert!(store.state.connection_in_progress.get_untracked().is_none());
    }

    #[test]
    fn test_connection_drop_on_empty_space_cancels() {
        let store = FlowStore::new(vec![], vec![]);
        start_test_connection(&store);
        store.update_connection(Position::new(500.0, 500.0), None, false);

        assert!(store.complete_connection().is_none());
        assert_eq!(store.get_edges().len(), 0);
        assert!(store.state.connection_in_progress.get_untracked().is_none());
    }

    #[test]
    fn test_cancel_connection_clears_state() {
        let store = FlowStore::new(vec![], vec![]);
        start_test_connection(&store);
        store.cancel_connection();
        assert!(store.state.connection_in_progress.get_untracked().is_none());
        // Completing after cancel is a no-op
        assert!(store.complete_connection().is_none());
    }

    #[test]
    fn test_update_and_complete_without_start_are_noops() {
        let store = FlowStore::new(vec![], vec![]);
        store.update_connection(Position::new(1.0, 1.0), None, true);
        assert!(store.state.connection_in_progress.get_untracked().is_none());
        assert!(store.complete_connection().is_none());
    }

    #[test]
    fn test_on_connect_hands_completion_to_host() {
        let store = FlowStore::new(vec![], vec![]);
        let received: RwSignal<Option<crate::types::Connection>> = RwSignal::new(None);
        store.set_on_connect(Callback::new(move |conn| received.set(Some(conn))));

        start_test_connection(&store);
        store.update_connection(
            Position::new(100.0, 20.0),
            Some(candidate("n2", Some("in"), HandleType::Target)),
            true,
        );
        let connection = store.complete_connection().expect("valid completion");

        // Host received the connection...
        let got = received.get_untracked().expect("on_connect fired");
        assert_eq!(got, connection);
        // ...and the crate did NOT insert an edge (host owns edge creation)
        assert_eq!(store.get_edges().len(), 0);
    }

    #[test]
    fn test_drag_from_target_handle_orders_source_and_target() {
        let store = FlowStore::new(vec![], vec![]);
        // Drag out of a TARGET handle, drop on a SOURCE handle
        store.start_connection(
            "n2".to_string(),
            Some("in".to_string()),
            HandleType::Target,
            Position::new(0.0, 0.0),
        );
        store.update_connection(
            Position::new(10.0, 10.0),
            Some(candidate("n1", Some("out"), HandleType::Source)),
            true,
        );
        let connection = store.complete_connection().expect("valid completion");

        // Connection is still ordered source -> target
        assert_eq!(connection.source, "n1");
        assert_eq!(connection.source_handle.as_deref(), Some("out"));
        assert_eq!(connection.target, "n2");
        assert_eq!(connection.target_handle.as_deref(), Some("in"));
    }

    // ===== Deletion =====

    fn store_with_two_nodes_and_edge() -> FlowStore {
        FlowStore::new(
            vec![
                Node::new("n1".to_string(), Position::new(0.0, 0.0)),
                Node::new("n2".to_string(), Position::new(100.0, 0.0)),
                Node::new("n3".to_string(), Position::new(200.0, 0.0)),
            ],
            vec![
                Edge::new("e1".to_string(), "n1".to_string(), "n2".to_string()),
                Edge::new("e2".to_string(), "n2".to_string(), "n3".to_string()),
            ],
        )
    }

    #[test]
    fn test_request_delete_with_no_selection_is_noop() {
        let store = store_with_two_nodes_and_edge();
        assert!(store.request_delete_selection().is_none());
        assert_eq!(store.get_nodes().len(), 3);
        assert_eq!(store.get_edges().len(), 2);
    }

    #[test]
    fn test_request_delete_selected_edge_removes_it_by_default() {
        let store = store_with_two_nodes_and_edge();
        store.select_edge("e1", false);

        let request = store.request_delete_selection().expect("selection exists");
        assert_eq!(request.edges, vec!["e1".to_string()]);
        assert!(request.nodes.is_empty());

        // No callback registered: store-owned deletion applied
        assert_eq!(store.get_edges().len(), 1);
        assert_eq!(store.get_edges()[0].id, "e2");
        assert!(store.get_selected_edges().is_empty());
        assert_eq!(store.get_nodes().len(), 3);
    }

    #[test]
    fn test_request_delete_selected_node_removes_attached_edges() {
        let store = store_with_two_nodes_and_edge();
        store.select_node("n2", false);

        store.request_delete_selection().expect("selection exists");

        // n2 gone, and BOTH edges touching n2 gone with it
        assert_eq!(store.get_nodes().len(), 2);
        assert!(store.get_nodes().iter().all(|n| n.id != "n2"));
        assert_eq!(store.get_edges().len(), 0);
        assert!(store.get_selected_nodes().is_empty());
    }

    #[test]
    fn test_on_delete_requested_hands_deletion_to_host() {
        let store = store_with_two_nodes_and_edge();
        let received: RwSignal<Option<DeleteRequest>> = RwSignal::new(None);
        store.set_on_delete_requested(Callback::new(move |req| received.set(Some(req))));

        store.select_edge("e1", false);
        store.select_node("n1", true);
        let request = store.request_delete_selection().expect("selection exists");

        // Host received the request...
        let got = received.get_untracked().expect("callback fired");
        assert_eq!(got, request);
        assert_eq!(got.nodes, vec!["n1".to_string()]);
        assert_eq!(got.edges, vec!["e1".to_string()]);
        // ...and the crate did NOT mutate the store (host owns deletion)
        assert_eq!(store.get_nodes().len(), 3);
        assert_eq!(store.get_edges().len(), 2);
    }

    #[test]
    fn test_delete_elements_direct() {
        let store = store_with_two_nodes_and_edge();
        store.delete_elements(&[], &["e2".to_string()]);
        assert_eq!(store.get_edges().len(), 1);
        assert_eq!(store.get_edges()[0].id, "e1");

        store.delete_elements(&["n1".to_string()], &[]);
        assert_eq!(store.get_nodes().len(), 2);
        assert_eq!(store.get_edges().len(), 0); // e1 was attached to n1
    }

    #[test]
    fn test_zoom_at_respects_store_limits_and_anchors_cursor() {
        let store = FlowStore::new(vec![], vec![]);
        let min = store.state.min_zoom.get_untracked();
        let max = store.state.max_zoom.get_untracked();

        // A huge factor clamps to the store's max zoom
        store.zoom_at(1000.0, 100.0, 50.0);
        assert_eq!(store.get_viewport().zoom, max);

        // A tiny factor clamps to the store's min zoom
        store.zoom_at(1e-6, 100.0, 50.0);
        assert_eq!(store.get_viewport().zoom, min);

        // Within bounds: the flow point under the cursor stays put
        store.set_viewport(Viewport::new(20.0, -10.0, 1.0));
        let before = store.get_viewport();
        let (fx, fy) = before.screen_to_viewport(150.0, 90.0);
        store.zoom_at(1.5, 150.0, 90.0);
        let after = store.get_viewport();
        let (sx, sy) = after.viewport_to_screen(fx, fy);
        assert!((sx - 150.0).abs() < 1e-9);
        assert!((sy - 90.0).abs() < 1e-9);
        assert!((after.zoom - 1.5).abs() < 1e-9);
    }
}
