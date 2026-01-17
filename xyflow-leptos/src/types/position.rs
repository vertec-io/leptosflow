//! Position type for node and handle positioning

use serde::{Deserialize, Serialize};

/// A position in 2D space
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl Position {
    /// Create a new position
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    /// Calculate distance to another position
    pub fn distance_to(&self, other: Position) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Calculate midpoint between two positions
    pub fn midpoint(p1: Position, p2: Position) -> Position {
        Position {
            x: (p1.x + p2.x) / 2.0,
            y: (p1.y + p2.y) / 2.0,
        }
    }

    /// Add another position (as a vector)
    pub fn add(&self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract another position (as a vector)
    pub fn subtract(&self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Multiply by a scalar
    pub fn multiply(&self, scalar: f64) -> Position {
        Position {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0.0, y: 0.0 }
    }
}

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Position { x, y }
    }
}

impl Into<(f64, f64)> for Position {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        assert_eq!(p1.distance_to(p2), 5.0);
    }

    #[test]
    fn test_midpoint() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(10.0, 10.0);
        let mid = Position::midpoint(p1, p2);
        assert_eq!(mid.x, 5.0);
        assert_eq!(mid.y, 5.0);
    }

    #[test]
    fn test_from_tuple() {
        let pos: Position = (5.0, 10.0).into();
        assert_eq!(pos.x, 5.0);
        assert_eq!(pos.y, 10.0);
    }
}
