//! Handle bounds and position tracking

use crate::types::{Position, HandleType};
use serde::{Deserialize, Serialize};

/// Position of a handle on a node
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandlePosition {
    /// Top center
    Top,
    /// Right center
    Right,
    /// Bottom center
    Bottom,
    /// Left center
    Left,
}

impl HandlePosition {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            HandlePosition::Top => "top",
            HandlePosition::Right => "right",
            HandlePosition::Bottom => "bottom",
            HandlePosition::Left => "left",
        }
    }
}

/// Handle bounds with position relative to the node
///
/// NOTE: The x and y coordinates are relative to the node's top-left corner,
/// not absolute flow coordinates. To get absolute coordinates, add the node's position.
/// This matches React Flow's implementation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandleBound {
    /// Handle ID (None for default handle)
    pub id: Option<String>,
    /// Handle type (source or target)
    pub handle_type: HandleType,
    /// Position on the node
    pub position: HandlePosition,
    /// X coordinate relative to the node's top-left corner
    pub x: f64,
    /// Y coordinate relative to the node's top-left corner
    pub y: f64,
    /// Width of the handle
    pub width: f64,
    /// Height of the handle
    pub height: f64,
    /// Whether a connection may END on this handle
    /// (`is_connectable && is_connectable_end` on the `Handle` component).
    /// Non-connectable handles are skipped by connection hit-testing.
    #[serde(default = "default_connectable")]
    pub connectable: bool,
}

fn default_connectable() -> bool {
    true
}

impl HandleBound {
    /// Create a new handle bound (connectable by default)
    pub fn new(
        id: Option<String>,
        handle_type: HandleType,
        position: HandlePosition,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Self {
        HandleBound {
            id,
            handle_type,
            position,
            x,
            y,
            width,
            height,
            connectable: true,
        }
    }

    /// Set whether connections may end on this handle
    pub fn with_connectable(mut self, connectable: bool) -> Self {
        self.connectable = connectable;
        self
    }

    /// Get the center position of the handle relative to the node
    ///
    /// NOTE: This returns coordinates relative to the node's top-left corner.
    /// To get absolute flow coordinates, use `center_absolute()` instead.
    pub fn center(&self) -> Position {
        Position::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }

    /// Get the absolute center position of the handle in flow space
    ///
    /// This adds the node's position to get absolute flow coordinates.
    /// This is what you want to use for distance calculations.
    pub fn center_absolute(&self, node_position: &Position) -> Position {
        Position::new(
            node_position.x + self.x + self.width / 2.0,
            node_position.y + self.y + self.height / 2.0,
        )
    }
}

/// Collection of handle bounds for a node
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HandleBounds {
    /// Source handles
    pub source: Vec<HandleBound>,
    /// Target handles
    pub target: Vec<HandleBound>,
}

impl HandleBounds {
    /// Create empty handle bounds
    pub fn new() -> Self {
        HandleBounds {
            source: Vec::new(),
            target: Vec::new(),
        }
    }
    
    /// Add a source handle
    pub fn add_source(&mut self, handle: HandleBound) {
        self.source.push(handle);
    }
    
    /// Add a target handle
    pub fn add_target(&mut self, handle: HandleBound) {
        self.target.push(handle);
    }
    
    /// Get all handles (source and target)
    pub fn all_handles(&self) -> impl Iterator<Item = &HandleBound> {
        self.source.iter().chain(self.target.iter())
    }
    
    /// Find a handle by ID and type
    pub fn find_handle(&self, id: Option<&str>, handle_type: HandleType) -> Option<&HandleBound> {
        let handles = match handle_type {
            HandleType::Source => &self.source,
            HandleType::Target => &self.target,
        };
        
        match id {
            Some(id) => handles.iter().find(|h| h.id.as_deref() == Some(id)),
            None => handles.first(),
        }
    }
}

/// Node internals for tracking computed properties
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeInternals {
    /// Handle bounds (measured from DOM)
    pub handle_bounds: Option<HandleBounds>,
    /// Whether the node has been measured
    pub measured: bool,
}

impl NodeInternals {
    /// Create new node internals
    pub fn new() -> Self {
        NodeInternals {
            handle_bounds: None,
            measured: false,
        }
    }
    
    /// Set handle bounds
    pub fn set_handle_bounds(&mut self, bounds: HandleBounds) {
        self.handle_bounds = Some(bounds);
        self.measured = true;
    }
}

