//! Edge type definition

use serde::{Deserialize, Serialize};
use super::Data;

/// An edge connecting two nodes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    /// Unique identifier for the edge
    pub id: String,

    /// ID of the source node
    pub source: String,

    /// ID of the target node
    pub target: String,

    /// ID of the source handle (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_handle: Option<String>,

    /// ID of the target handle (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_handle: Option<String>,

    /// User-defined data for the edge
    #[serde(default)]
    pub data: Data,

    /// Custom edge type (e.g., "default", "straight", "bezier", "step")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_type: Option<String>,

    /// Whether the edge is selected
    #[serde(default)]
    pub selected: bool,

    /// Whether the edge is animated
    #[serde(default)]
    pub animated: bool,

    /// Whether the edge is hidden
    #[serde(default)]
    pub hidden: bool,

    /// Label for the edge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Whether the label is animated
    #[serde(default)]
    pub label_animated: bool,

    /// Custom class name for styling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// Whether the edge is a no-selection edge
    #[serde(default)]
    pub no_selection: bool,
}

impl Edge {
    /// Create a new edge connecting two nodes
    pub fn new(id: String, source: String, target: String) -> Self {
        Edge {
            id,
            source,
            target,
            source_handle: None,
            target_handle: None,
            data: Data::Object(serde_json::Map::new()),
            edge_type: None,
            selected: false,
            animated: false,
            hidden: false,
            label: None,
            label_animated: false,
            class_name: None,
            no_selection: false,
        }
    }

    /// Set the edge type
    pub fn with_type(mut self, edge_type: String) -> Self {
        self.edge_type = Some(edge_type);
        self
    }

    /// Set the edge data
    pub fn with_data(mut self, data: Data) -> Self {
        self.data = data;
        self
    }

    /// Set the handle IDs
    pub fn with_handles(mut self, source_handle: String, target_handle: String) -> Self {
        self.source_handle = Some(source_handle);
        self.target_handle = Some(target_handle);
        self
    }

    /// Set the source handle ID
    pub fn with_source_handle(mut self, source_handle: Option<String>) -> Self {
        self.source_handle = source_handle;
        self
    }

    /// Set the target handle ID
    pub fn with_target_handle(mut self, target_handle: Option<String>) -> Self {
        self.target_handle = target_handle;
        self
    }

    /// Set the edge label
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Set whether the edge is animated
    pub fn with_animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Set custom class name for styling
    pub fn with_class(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    /// Toggle selection state
    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Check if this edge is a self-loop
    pub fn is_self_loop(&self) -> bool {
        self.source == self.target
    }

    /// Get the full identifier for the source handle
    pub fn source_handle_id(&self) -> String {
        if let Some(handle) = &self.source_handle {
            format!("{}-{}", self.source, handle)
        } else {
            self.source.clone()
        }
    }

    /// Get the full identifier for the target handle
    pub fn target_handle_id(&self) -> String {
        if let Some(handle) = &self.target_handle {
            format!("{}-{}", self.target, handle)
        } else {
            self.target.clone()
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Edge {}

impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new("e1".to_string(), "node1".to_string(), "node2".to_string());
        assert_eq!(edge.id, "e1");
        assert_eq!(edge.source, "node1");
        assert_eq!(edge.target, "node2");
        assert!(!edge.selected);
        assert!(!edge.animated);
    }

    #[test]
    fn test_edge_with_handles() {
        let edge = Edge::new("e1".to_string(), "node1".to_string(), "node2".to_string())
            .with_handles("out".to_string(), "in".to_string());
        assert_eq!(edge.source_handle, Some("out".to_string()));
        assert_eq!(edge.target_handle, Some("in".to_string()));
    }

    #[test]
    fn test_edge_with_type() {
        let edge =
            Edge::new("e1".to_string(), "node1".to_string(), "node2".to_string())
                .with_type("bezier".to_string());
        assert_eq!(edge.edge_type, Some("bezier".to_string()));
    }

    #[test]
    fn test_edge_self_loop() {
        let edge = Edge::new("e1".to_string(), "node1".to_string(), "node1".to_string());
        assert!(edge.is_self_loop());
    }

    #[test]
    fn test_edge_handle_ids() {
        let edge = Edge::new("e1".to_string(), "node1".to_string(), "node2".to_string())
            .with_handles("out".to_string(), "in".to_string());
        assert_eq!(edge.source_handle_id(), "node1-out");
        assert_eq!(edge.target_handle_id(), "node2-in");
    }

    #[test]
    fn test_edge_selection() {
        let mut edge = Edge::new("e1".to_string(), "node1".to_string(), "node2".to_string());
        assert!(!edge.selected);
        edge.set_selected(true);
        assert!(edge.selected);
        edge.toggle_selected();
        assert!(!edge.selected);
    }
}
