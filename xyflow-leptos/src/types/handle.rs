//! Handle type definitions (connection points on nodes)

use serde::{Deserialize, Serialize};
use super::Position;

/// Type of handle (source or target)
///
/// This matches React Flow's terminology:
/// - Source: outgoing connections (like Output)
/// - Target: incoming connections (like Input)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandleType {
    /// Handle for outgoing connections (source of edges)
    Source,
    /// Handle for incoming connections (target of edges)
    Target,
}

/// A handle is a connection point on a node
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Handle {
    /// Unique identifier for the handle (optional, derived from node_id + position if not set)
    pub id: Option<String>,

    /// ID of the node this handle belongs to
    pub node_id: String,

    /// Position of the handle within the node
    pub position: Position,

    /// Type of handle (input or output)
    pub handle_type: HandleType,

    /// Whether this handle can be connected to
    pub is_connectable: bool,
}

impl Handle {
    /// Create a new handle
    pub fn new(node_id: String, handle_type: HandleType) -> Self {
        Handle {
            id: None,
            node_id,
            position: Position::default(),
            handle_type,
            is_connectable: true,
        }
    }

    /// Set the position of the handle
    pub fn with_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Set whether the handle is connectable
    pub fn with_connectable(mut self, connectable: bool) -> Self {
        self.is_connectable = connectable;
        self
    }

    /// Set a custom ID for the handle
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Get the full identifier combining node ID and handle ID
    pub fn full_id(&self) -> String {
        if let Some(id) = &self.id {
            format!("{}-{}", self.node_id, id)
        } else {
            self.node_id.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation() {
        let handle = Handle::new("node1".to_string(), HandleType::Target);
        assert_eq!(handle.node_id, "node1");
        assert_eq!(handle.handle_type, HandleType::Target);
        assert!(handle.is_connectable);
    }

    #[test]
    fn test_handle_full_id() {
        let handle = Handle::new("node1".to_string(), HandleType::Target)
            .with_id("input1".to_string());
        assert_eq!(handle.full_id(), "node1-input1");
    }

    #[test]
    fn test_handle_builder() {
        let pos = Position::new(10.0, 20.0);
        let handle = Handle::new("node1".to_string(), HandleType::Source)
            .with_position(pos)
            .with_connectable(true)
            .with_id("output1".to_string());

        assert_eq!(handle.position.x, 10.0);
        assert_eq!(handle.position.y, 20.0);
        assert_eq!(handle.handle_type, HandleType::Source);
        assert_eq!(handle.id, Some("output1".to_string()));
    }
}
