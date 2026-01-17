//! Edge path generation utilities

use crate::types::Position;

/// Edge path type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgePathType {
    /// Straight line
    Straight,
    /// Bezier curve
    Bezier,
    /// Step/L-shaped path
    Step,
    /// Smooth step
    SmoothStep,
    /// Simple bezier
    SimpleBezier,
}

/// Generate an SVG path string for an edge
pub fn generate_edge_path(
    from: Position,
    to: Position,
    path_type: EdgePathType,
) -> String {
    match path_type {
        EdgePathType::Straight => generate_straight_path(from, to),
        EdgePathType::Bezier => generate_bezier_path(from, to),
        EdgePathType::Step => generate_step_path(from, to),
        EdgePathType::SmoothStep => generate_smooth_step_path(from, to),
        EdgePathType::SimpleBezier => generate_simple_bezier_path(from, to),
    }
}

/// Generate a straight line path
fn generate_straight_path(from: Position, to: Position) -> String {
    format!("M {} {} L {} {}", from.x, from.y, to.x, to.y)
}

/// Generate a bezier curve path
fn generate_bezier_path(from: Position, to: Position) -> String {
    let mid_x = (from.x + to.x) / 2.0;
    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y, mid_x, from.y, mid_x, to.y, to.x, to.y
    )
}

/// Generate a step path (L-shaped)
fn generate_step_path(from: Position, to: Position) -> String {
    let mid_x = (from.x + to.x) / 2.0;
    format!(
        "M {} {} L {} {} L {} {}",
        from.x, from.y, mid_x, from.y, mid_x, to.y
    )
}

/// Generate a smooth step path
fn generate_smooth_step_path(from: Position, to: Position) -> String {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let distance = (dx * dx + dy * dy).sqrt();
    let control_distance = (distance / 3.0).min(50.0);

    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x,
        from.y,
        from.x + control_distance,
        from.y,
        to.x - control_distance,
        to.y,
        to.x,
        to.y
    )
}

/// Generate a simple bezier path with two control points
fn generate_simple_bezier_path(from: Position, to: Position) -> String {
    let ctrl1_x = from.x + (to.x - from.x) * 0.5;
    let ctrl1_y = from.y;
    let ctrl2_x = to.x - (to.x - from.x) * 0.5;
    let ctrl2_y = to.y;

    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y, ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, to.x, to.y
    )
}

/// Calculate the label position (midpoint) for an edge
pub fn calculate_label_position(from: Position, to: Position, path_type: EdgePathType) -> Position {
    match path_type {
        EdgePathType::Straight | EdgePathType::Step => {
            // For straight and step paths, use simple midpoint
            Position::new(
                (from.x + to.x) / 2.0,
                (from.y + to.y) / 2.0,
            )
        }
        EdgePathType::Bezier | EdgePathType::SmoothStep | EdgePathType::SimpleBezier => {
            // For bezier curves, use midpoint (could be improved with actual curve calculation)
            Position::new(
                (from.x + to.x) / 2.0,
                (from.y + to.y) / 2.0,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_path() {
        let path = generate_straight_path(Position::new(0.0, 0.0), Position::new(100.0, 100.0));
        assert!(path.contains("M 0"));
        assert!(path.contains("L 100"));
    }

    #[test]
    fn test_bezier_path() {
        let path = generate_bezier_path(Position::new(0.0, 0.0), Position::new(100.0, 100.0));
        assert!(path.contains("M 0"));
        assert!(path.contains("C"));
    }

    #[test]
    fn test_step_path() {
        let path = generate_step_path(Position::new(0.0, 0.0), Position::new(100.0, 100.0));
        assert!(path.contains("M 0"));
        assert_eq!(path.matches("L").count(), 2);
    }
}
