//! Type definitions for XYFlow

pub mod node;
pub mod edge;
pub mod handle;
pub mod position;
pub mod viewport;
pub mod changes;
pub mod handle_bounds;
pub mod connection;

pub use node::Node;
pub use edge::Edge;
pub use handle::{Handle, HandleType};
pub use position::Position;
pub use viewport::Viewport;
pub use changes::Change;
pub use handle_bounds::{HandleBound, HandleBounds, HandlePosition as HandleBoundPosition, NodeInternals};
pub use connection::{Connection, ConnectionMode, IsValidConnection, validate_connection, always_valid};

/// Bounds of a rectangular area
#[derive(Clone, Copy, Debug, Default)]
pub struct Bounds {
    /// Minimum x coordinate
    pub x: f64,
    /// Minimum y coordinate
    pub y: f64,
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

impl Bounds {
    /// Create a new bounds
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Bounds { x, y, width, height }
    }

    /// Get the center point of the bounds
    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Check if a point is within the bounds
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Expand bounds to include another bounds
    pub fn expand(&self, other: &Bounds) -> Bounds {
        let min_x = self.x.min(other.x);
        let min_y = self.y.min(other.y);
        let max_x = (self.x + self.width).max(other.x + other.width);
        let max_y = (self.y + self.height).max(other.y + other.height);

        Bounds {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

/// Unique identifier for nodes and edges
pub type Id = String;

/// Generic data container (defaults to JSON value)
pub type Data = serde_json::Value;
