//! Edge path generation utilities
//!
//! All path generators come in two flavors:
//!
//! * [`generate_edge_path_oriented`] — takes the source/target handle
//!   orientation ([`HandleBoundPosition`]) so control points and step
//!   segments leave/enter each node along the axis the handle faces.
//!   This is what [`crate::EdgeRenderer`] uses.
//! * [`generate_edge_path`] — orientation-free compatibility wrapper that
//!   assumes a horizontal flow (source faces right, target faces left).

use crate::types::{HandleBoundPosition, Position};

/// Default curvature for bezier control offsets (matches React Flow)
const DEFAULT_CURVATURE: f64 = 0.25;

/// Corner radius used by smooth-step paths
const STEP_BORDER_RADIUS: f64 = 5.0;

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

/// Generate an SVG path string for an edge, honoring handle orientation.
///
/// `source_position`/`target_position` describe which side of the node the
/// handle sits on; paths leave the source and enter the target along that
/// axis instead of assuming horizontal flow.
pub fn generate_edge_path_oriented(
    from: Position,
    source_position: HandleBoundPosition,
    to: Position,
    target_position: HandleBoundPosition,
    path_type: EdgePathType,
) -> String {
    match path_type {
        EdgePathType::Straight => generate_straight_path(from, to),
        EdgePathType::Bezier => {
            let (c1, c2) = bezier_control_points(from, source_position, to, target_position);
            cubic_path(from, c1, c2, to)
        }
        EdgePathType::SimpleBezier => {
            let (c1, c2) = simple_bezier_control_points(from, source_position, to, target_position);
            cubic_path(from, c1, c2, to)
        }
        EdgePathType::Step => {
            let points = step_points(from, source_position, to, target_position);
            polyline_path(&points)
        }
        EdgePathType::SmoothStep => {
            let points = step_points(from, source_position, to, target_position);
            rounded_polyline_path(&points, STEP_BORDER_RADIUS)
        }
    }
}

/// Generate an SVG path string for an edge (orientation-free compatibility API).
///
/// Assumes a horizontal flow: the source handle faces right and the target
/// handle faces left. Prefer [`generate_edge_path_oriented`] when the handle
/// orientation is known.
pub fn generate_edge_path(
    from: Position,
    to: Position,
    path_type: EdgePathType,
) -> String {
    generate_edge_path_oriented(
        from,
        HandleBoundPosition::Right,
        to,
        HandleBoundPosition::Left,
        path_type,
    )
}

/// Generate a straight line path
fn generate_straight_path(from: Position, to: Position) -> String {
    format!("M {} {} L {} {}", from.x, from.y, to.x, to.y)
}

/// How far a bezier control point extends from its endpoint.
///
/// Mirrors React Flow's `calculateControlOffset`: half the distance when the
/// edge flows in the natural direction, a square-root falloff when it has to
/// double back.
fn control_offset(distance: f64, curvature: f64) -> f64 {
    if distance >= 0.0 {
        0.5 * distance
    } else {
        curvature * 25.0 * (-distance).sqrt()
    }
}

/// Control point for an endpoint, extending along the axis its handle faces.
fn control_point(
    point: Position,
    handle_position: HandleBoundPosition,
    other: Position,
    curvature: f64,
) -> Position {
    match handle_position {
        HandleBoundPosition::Left => Position::new(
            point.x - control_offset(point.x - other.x, curvature),
            point.y,
        ),
        HandleBoundPosition::Right => Position::new(
            point.x + control_offset(other.x - point.x, curvature),
            point.y,
        ),
        HandleBoundPosition::Top => Position::new(
            point.x,
            point.y - control_offset(point.y - other.y, curvature),
        ),
        HandleBoundPosition::Bottom => Position::new(
            point.x,
            point.y + control_offset(other.y - point.y, curvature),
        ),
    }
}

fn bezier_control_points(
    from: Position,
    source_position: HandleBoundPosition,
    to: Position,
    target_position: HandleBoundPosition,
) -> (Position, Position) {
    (
        control_point(from, source_position, to, DEFAULT_CURVATURE),
        control_point(to, target_position, from, DEFAULT_CURVATURE),
    )
}

/// Simple bezier: control points sit at the midpoint projected on the handle axis.
fn simple_bezier_control_points(
    from: Position,
    source_position: HandleBoundPosition,
    to: Position,
    target_position: HandleBoundPosition,
) -> (Position, Position) {
    let mid_x = (from.x + to.x) / 2.0;
    let mid_y = (from.y + to.y) / 2.0;

    let project = |point: Position, handle_position: HandleBoundPosition| match handle_position {
        HandleBoundPosition::Left | HandleBoundPosition::Right => Position::new(mid_x, point.y),
        HandleBoundPosition::Top | HandleBoundPosition::Bottom => Position::new(point.x, mid_y),
    };

    (project(from, source_position), project(to, target_position))
}

fn cubic_path(from: Position, c1: Position, c2: Position, to: Position) -> String {
    format!(
        "M {} {} C {} {}, {} {}, {} {}",
        from.x, from.y, c1.x, c1.y, c2.x, c2.y, to.x, to.y
    )
}

/// Whether a handle faces horizontally (left/right)
fn is_horizontal(position: HandleBoundPosition) -> bool {
    matches!(
        position,
        HandleBoundPosition::Left | HandleBoundPosition::Right
    )
}

/// Waypoints for an orthogonal (step) route from `from` to `to`.
///
/// The route leaves the source along its handle axis and enters the target
/// along its handle axis, always terminating at the target point.
fn step_points(
    from: Position,
    source_position: HandleBoundPosition,
    to: Position,
    target_position: HandleBoundPosition,
) -> Vec<Position> {
    let source_horizontal = is_horizontal(source_position);
    let target_horizontal = is_horizontal(target_position);

    let mut points = vec![from];

    match (source_horizontal, target_horizontal) {
        (true, true) => {
            // Both horizontal: route via vertical mid line
            let mid_x = (from.x + to.x) / 2.0;
            points.push(Position::new(mid_x, from.y));
            points.push(Position::new(mid_x, to.y));
        }
        (false, false) => {
            // Both vertical: route via horizontal mid line
            let mid_y = (from.y + to.y) / 2.0;
            points.push(Position::new(from.x, mid_y));
            points.push(Position::new(to.x, mid_y));
        }
        (true, false) => {
            // Horizontal out of source, vertical into target: single corner
            points.push(Position::new(to.x, from.y));
        }
        (false, true) => {
            // Vertical out of source, horizontal into target: single corner
            points.push(Position::new(from.x, to.y));
        }
    }

    points.push(to);
    // Drop consecutive duplicates (collinear endpoints)
    points.dedup_by(|a, b| (a.x - b.x).abs() < f64::EPSILON && (a.y - b.y).abs() < f64::EPSILON);
    points
}

fn polyline_path(points: &[Position]) -> String {
    let mut path = String::new();
    for (i, p) in points.iter().enumerate() {
        if i == 0 {
            path.push_str(&format!("M {} {}", p.x, p.y));
        } else {
            path.push_str(&format!(" L {} {}", p.x, p.y));
        }
    }
    path
}

/// Polyline with corners rounded by quadratic beziers of at most `radius`.
fn rounded_polyline_path(points: &[Position], radius: f64) -> String {
    if points.len() < 3 {
        return polyline_path(points);
    }

    let mut path = format!("M {} {}", points[0].x, points[0].y);

    for i in 1..points.len() - 1 {
        let prev = points[i - 1];
        let corner = points[i];
        let next = points[i + 1];

        let d_in = ((corner.x - prev.x).powi(2) + (corner.y - prev.y).powi(2)).sqrt();
        let d_out = ((next.x - corner.x).powi(2) + (next.y - corner.y).powi(2)).sqrt();
        let r = radius.min(d_in / 2.0).min(d_out / 2.0);

        if r < f64::EPSILON || d_in < f64::EPSILON || d_out < f64::EPSILON {
            path.push_str(&format!(" L {} {}", corner.x, corner.y));
            continue;
        }

        // Point on the incoming segment r before the corner
        let in_x = corner.x - (corner.x - prev.x) / d_in * r;
        let in_y = corner.y - (corner.y - prev.y) / d_in * r;
        // Point on the outgoing segment r after the corner
        let out_x = corner.x + (next.x - corner.x) / d_out * r;
        let out_y = corner.y + (next.y - corner.y) / d_out * r;

        path.push_str(&format!(
            " L {} {} Q {} {} {} {}",
            in_x, in_y, corner.x, corner.y, out_x, out_y
        ));
    }

    let last = points[points.len() - 1];
    path.push_str(&format!(" L {} {}", last.x, last.y));
    path
}

/// Calculate the label position for an edge, honoring handle orientation.
pub fn calculate_label_position_oriented(
    from: Position,
    source_position: HandleBoundPosition,
    to: Position,
    target_position: HandleBoundPosition,
    path_type: EdgePathType,
) -> Position {
    match path_type {
        EdgePathType::Straight => Position::midpoint(from, to),
        EdgePathType::Step | EdgePathType::SmoothStep => {
            // Midpoint along the polyline route
            let points = step_points(from, source_position, to, target_position);
            polyline_midpoint(&points)
        }
        EdgePathType::Bezier => {
            let (c1, c2) = bezier_control_points(from, source_position, to, target_position);
            cubic_point_at_half(from, c1, c2, to)
        }
        EdgePathType::SimpleBezier => {
            let (c1, c2) = simple_bezier_control_points(from, source_position, to, target_position);
            cubic_point_at_half(from, c1, c2, to)
        }
    }
}

/// Calculate the label position (midpoint) for an edge
/// (orientation-free compatibility API).
pub fn calculate_label_position(from: Position, to: Position, path_type: EdgePathType) -> Position {
    calculate_label_position_oriented(
        from,
        HandleBoundPosition::Right,
        to,
        HandleBoundPosition::Left,
        path_type,
    )
}

/// Point at t = 0.5 on a cubic bezier: (p0 + 3*c1 + 3*c2 + p1) / 8
fn cubic_point_at_half(p0: Position, c1: Position, c2: Position, p1: Position) -> Position {
    Position::new(
        (p0.x + 3.0 * c1.x + 3.0 * c2.x + p1.x) / 8.0,
        (p0.y + 3.0 * c1.y + 3.0 * c2.y + p1.y) / 8.0,
    )
}

/// Point halfway along a polyline (by arc length)
fn polyline_midpoint(points: &[Position]) -> Position {
    if points.is_empty() {
        return Position::default();
    }
    if points.len() == 1 {
        return points[0];
    }

    let total: f64 = points
        .windows(2)
        .map(|w| w[0].distance_to(w[1]))
        .sum();
    let mut remaining = total / 2.0;

    for w in points.windows(2) {
        let seg = w[0].distance_to(w[1]);
        if seg >= remaining && seg > 0.0 {
            let t = remaining / seg;
            return Position::new(
                w[0].x + (w[1].x - w[0].x) * t,
                w[0].y + (w[1].y - w[0].y) * t,
            );
        }
        remaining -= seg;
    }

    points[points.len() - 1]
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
    fn test_bezier_path_horizontal() {
        let path = generate_edge_path_oriented(
            Position::new(0.0, 0.0),
            HandleBoundPosition::Right,
            Position::new(100.0, 100.0),
            HandleBoundPosition::Left,
            EdgePathType::Bezier,
        );
        // Control points extend horizontally: source control at (50, 0), target at (50, 100)
        assert_eq!(path, "M 0 0 C 50 0, 50 100, 100 100");
    }

    #[test]
    fn test_bezier_path_vertical() {
        let path = generate_edge_path_oriented(
            Position::new(0.0, 0.0),
            HandleBoundPosition::Bottom,
            Position::new(100.0, 100.0),
            HandleBoundPosition::Top,
            EdgePathType::Bezier,
        );
        // Control points extend vertically: source control at (0, 50), target at (100, 50)
        assert_eq!(path, "M 0 0 C 0 50, 100 50, 100 100");
    }

    #[test]
    fn test_step_path_reaches_target() {
        let from = Position::new(0.0, 0.0);
        let to = Position::new(100.0, 100.0);
        let path = generate_edge_path_oriented(
            from,
            HandleBoundPosition::Right,
            to,
            HandleBoundPosition::Left,
            EdgePathType::Step,
        );
        // Must terminate at the target point
        assert!(path.ends_with("L 100 100"), "path was: {path}");
        // Route via the vertical mid line
        assert_eq!(path, "M 0 0 L 50 0 L 50 100 L 100 100");
    }

    #[test]
    fn test_step_path_mixed_orientation() {
        let path = generate_edge_path_oriented(
            Position::new(0.0, 0.0),
            HandleBoundPosition::Bottom,
            Position::new(100.0, 100.0),
            HandleBoundPosition::Left,
            EdgePathType::Step,
        );
        // Vertical out of source, horizontal into target: corner at (0, 100)
        assert_eq!(path, "M 0 0 L 0 100 L 100 100");
    }

    #[test]
    fn test_smooth_step_reaches_target() {
        let path = generate_edge_path_oriented(
            Position::new(0.0, 0.0),
            HandleBoundPosition::Right,
            Position::new(100.0, 100.0),
            HandleBoundPosition::Left,
            EdgePathType::SmoothStep,
        );
        assert!(path.ends_with("L 100 100"), "path was: {path}");
        assert!(path.contains('Q'), "smooth step should round corners: {path}");
    }

    #[test]
    fn test_compat_wrapper_is_horizontal() {
        let a = generate_edge_path(
            Position::new(0.0, 0.0),
            Position::new(100.0, 100.0),
            EdgePathType::Bezier,
        );
        let b = generate_edge_path_oriented(
            Position::new(0.0, 0.0),
            HandleBoundPosition::Right,
            Position::new(100.0, 100.0),
            HandleBoundPosition::Left,
            EdgePathType::Bezier,
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_label_position_step() {
        let pos = calculate_label_position_oriented(
            Position::new(0.0, 0.0),
            HandleBoundPosition::Right,
            Position::new(100.0, 100.0),
            HandleBoundPosition::Left,
            EdgePathType::Step,
        );
        // Halfway along M 0 0 L 50 0 L 50 100 L 100 100 (length 200) => (50, 50)
        assert_eq!(pos, Position::new(50.0, 50.0));
    }

    #[test]
    fn test_backwards_edge_doubles_back() {
        // Target is to the LEFT of the source: control points must still
        // extend outward (right from source, left from... i.e. doubling back)
        let path = generate_edge_path_oriented(
            Position::new(100.0, 0.0),
            HandleBoundPosition::Right,
            Position::new(0.0, 50.0),
            HandleBoundPosition::Left,
            EdgePathType::Bezier,
        );
        // Source control x must be > 100 (extends right), target control x < 0
        let parts: Vec<&str> = path.split(&[' ', ','][..]).filter(|s| !s.is_empty()).collect();
        // "M x y C c1x c1y c2x c2y x y"
        let c1x: f64 = parts[4].parse().unwrap();
        let c2x: f64 = parts[6].parse().unwrap();
        assert!(c1x > 100.0, "source control should extend right: {path}");
        assert!(c2x < 0.0, "target control should extend left: {path}");
    }
}
