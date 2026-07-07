//! Node type definition

use serde::{Deserialize, Serialize};
use super::{Position, Data, Bounds, NodeInternals};

/// A node in the flow
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for the node
    pub id: String,

    /// Position of the node in flow space
    pub position: Position,

    /// User-defined data for the node
    #[serde(default)]
    pub data: Data,

    /// Custom node type (e.g., "input", "output", "default")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,

    /// Whether the node is selected
    #[serde(default)]
    pub selected: bool,

    /// Whether the node is currently being dragged
    #[serde(default, skip_serializing)]
    pub dragging: bool,

    /// Whether the node is hidden
    #[serde(default)]
    pub hidden: bool,

    /// Width of the node (optional, calculated if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Height of the node (optional, calculated if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// ID of the parent node (for nested flows)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    /// Whether the node is expandable/has children
    #[serde(default)]
    pub expandable: bool,

    /// Custom class name for styling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// Metadata for the node (not serialized)
    #[serde(skip)]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// Internal computed properties (handle bounds, etc.)
    #[serde(skip)]
    pub internals: NodeInternals,
}

impl Node {
    /// Create a new node with an ID and position
    pub fn new(id: String, position: Position) -> Self {
        Node {
            id,
            position,
            data: Data::Object(serde_json::Map::new()),
            node_type: None,
            selected: false,
            dragging: false,
            hidden: false,
            width: None,
            height: None,
            parent_id: None,
            expandable: false,
            class_name: None,
            metadata: None,
            internals: NodeInternals::new(),
        }
    }

    /// Create a node from a position tuple
    pub fn from_position(id: String, (x, y): (f64, f64)) -> Self {
        Self::new(id, Position::new(x, y))
    }

    /// Set the node type
    pub fn with_type(mut self, node_type: String) -> Self {
        self.node_type = Some(node_type);
        self
    }

    /// Set the node data
    pub fn with_data(mut self, data: Data) -> Self {
        self.data = data;
        self
    }

    /// Set the node dimensions
    pub fn with_dimensions(mut self, width: f64, height: f64) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set the parent node ID (for nested flows)
    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Set custom class name for styling
    pub fn with_class(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    /// Get the bounding box of the node
    ///
    /// Unmeasured nodes fall back to the default footprint (150 x 40),
    /// consistent with the edge renderer and fit-view.
    pub fn bounds(&self) -> Bounds {
        Bounds {
            x: self.position.x,
            y: self.position.y,
            width: self.width.unwrap_or(150.0),
            height: self.height.unwrap_or(40.0),
        }
    }

    /// Check if this node is a parent node
    pub fn is_parent(&self) -> bool {
        self.expandable || self.parent_id.is_none()
    }

    /// Toggle selection state
    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Set dragging state
    pub fn set_dragging(&mut self, dragging: bool) {
        self.dragging = dragging;
    }

    /// Move the node by a delta
    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.position.x += dx;
        self.position.y += dy;
    }

    /// Set the position directly
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new("node1".to_string(), Position::new(10.0, 20.0));
        assert_eq!(node.id, "node1");
        assert_eq!(node.position.x, 10.0);
        assert_eq!(node.position.y, 20.0);
        assert!(!node.selected);
        assert!(!node.dragging);
    }

    #[test]
    fn test_node_with_type() {
        let node = Node::new("node1".to_string(), Position::default())
            .with_type("input".to_string());
        assert_eq!(node.node_type, Some("input".to_string()));
    }

    #[test]
    fn test_node_with_dimensions() {
        let node =
            Node::new("node1".to_string(), Position::default()).with_dimensions(200.0, 100.0);
        assert_eq!(node.width, Some(200.0));
        assert_eq!(node.height, Some(100.0));
    }

    #[test]
    fn test_node_bounds() {
        let node =
            Node::new("node1".to_string(), Position::new(10.0, 20.0)).with_dimensions(50.0, 60.0);
        let bounds = node.bounds();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 50.0);
        assert_eq!(bounds.height, 60.0);
    }

    #[test]
    fn test_node_move() {
        let mut node = Node::new("node1".to_string(), Position::new(10.0, 20.0));
        node.move_by(5.0, 10.0);
        assert_eq!(node.position.x, 15.0);
        assert_eq!(node.position.y, 30.0);
    }

    #[test]
    fn test_node_selection() {
        let mut node = Node::new("node1".to_string(), Position::default());
        assert!(!node.selected);
        node.set_selected(true);
        assert!(node.selected);
        node.toggle_selected();
        assert!(!node.selected);
    }
}
