//! Change events for describing updates to nodes and edges

use serde::{Deserialize, Serialize};
use super::Position;

/// Describes a change to the flow state
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Change {
    /// Select or deselect a node
    SelectNode {
        /// Node ID
        id: String,
        /// Whether the node should be selected
        selected: bool,
    },

    /// Add nodes to selection
    AddSelectedNodes {
        /// Node IDs to select
        ids: Vec<String>,
    },

    /// Remove nodes from selection
    RemoveSelectedNodes {
        /// Node IDs to deselect
        ids: Vec<String>,
    },

    /// Replace the selected nodes
    SetSelectedNodes {
        /// Node IDs that should be selected
        ids: Vec<String>,
    },

    /// Move a single node
    PositionNode {
        /// Node ID
        id: String,
        /// New position
        position: Position,
        /// Whether the drag is starting
        dragging: bool,
    },

    /// Update node dimensions
    UpdateNodeDimensions {
        /// Node ID
        id: String,
        /// New width
        width: Option<f64>,
        /// New height
        height: Option<f64>,
    },

    /// Expand or collapse a node (for grouped/nested flows)
    ExpandNode {
        /// Node ID
        id: String,
        /// Whether the node should be expanded
        expanded: bool,
    },

    /// Hide or show a node
    HideNode {
        /// Node ID
        id: String,
        /// Whether the node should be hidden
        hidden: bool,
    },

    /// Select or deselect an edge
    SelectEdge {
        /// Edge ID
        id: String,
        /// Whether the edge should be selected
        selected: bool,
    },

    /// Add edges to selection
    AddSelectedEdges {
        /// Edge IDs to select
        ids: Vec<String>,
    },

    /// Remove edges from selection
    RemoveSelectedEdges {
        /// Edge IDs to deselect
        ids: Vec<String>,
    },

    /// Replace the selected edges
    SetSelectedEdges {
        /// Edge IDs that should be selected
        ids: Vec<String>,
    },

    /// Hide or show an edge
    HideEdge {
        /// Edge ID
        id: String,
        /// Whether the edge should be hidden
        hidden: bool,
    },

    /// Reset the flow (clear all selections, etc.)
    Reset,
}

impl Change {
    /// Get the node ID if this change affects a node
    pub fn node_id(&self) -> Option<&str> {
        match self {
            Change::SelectNode { id, .. }
            | Change::PositionNode { id, .. }
            | Change::UpdateNodeDimensions { id, .. }
            | Change::ExpandNode { id, .. }
            | Change::HideNode { id, .. } => Some(id),
            _ => None,
        }
    }

    /// Get the edge ID if this change affects an edge
    pub fn edge_id(&self) -> Option<&str> {
        match self {
            Change::SelectEdge { id, .. } | Change::HideEdge { id, .. } => Some(id),
            _ => None,
        }
    }

    /// Get the IDs if this change affects multiple nodes
    pub fn node_ids(&self) -> Option<Vec<&str>> {
        match self {
            Change::AddSelectedNodes { ids }
            | Change::RemoveSelectedNodes { ids }
            | Change::SetSelectedNodes { ids } => Some(ids.iter().map(|s| s.as_str()).collect()),
            _ => None,
        }
    }

    /// Get the IDs if this change affects multiple edges
    pub fn edge_ids(&self) -> Option<Vec<&str>> {
        match self {
            Change::AddSelectedEdges { ids }
            | Change::RemoveSelectedEdges { ids }
            | Change::SetSelectedEdges { ids } => Some(ids.iter().map(|s| s.as_str()).collect()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_node_change() {
        let change = Change::SelectNode {
            id: "node1".to_string(),
            selected: true,
        };
        assert_eq!(change.node_id(), Some("node1"));
        assert!(matches!(change, Change::SelectNode { .. }));
    }

    #[test]
    fn test_position_node_change() {
        let change = Change::PositionNode {
            id: "node1".to_string(),
            position: Position::new(10.0, 20.0),
            dragging: true,
        };
        assert_eq!(change.node_id(), Some("node1"));
    }

    #[test]
    fn test_add_selected_nodes() {
        let ids = vec!["n1".to_string(), "n2".to_string()];
        let change = Change::AddSelectedNodes { ids: ids.clone() };
        let node_ids = change.node_ids();
        assert_eq!(node_ids, Some(vec!["n1", "n2"]));
    }

    #[test]
    fn test_select_edge_change() {
        let change = Change::SelectEdge {
            id: "e1".to_string(),
            selected: true,
        };
        assert_eq!(change.edge_id(), Some("e1"));
    }

    #[test]
    fn test_serialization() {
        let change = Change::SelectNode {
            id: "node1".to_string(),
            selected: true,
        };
        let json = serde_json::to_string(&change).unwrap();
        assert!(json.contains("SelectNode"));
        assert!(json.contains("node1"));
    }
}
