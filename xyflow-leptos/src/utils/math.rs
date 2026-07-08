//! Math utilities for XYFlow

/// Clamp a value between min and max
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Linear interpolation between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Normalize a value between min and max to [0, 1]
pub fn normalize(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    (value - min) / (max - min)
}

/// Check if two floats are approximately equal
pub fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Pixels represented by one `wheel` delta unit for a given `deltaMode`.
///
/// * `0` (`DOM_DELTA_PIXEL`) — deltas are already pixels (trackpads,
///   pixel-precise mice).
/// * `1` (`DOM_DELTA_LINE`) — deltas are text lines (classic mouse wheels on
///   Firefox); a line is treated as 16 px, matching xyflow conventions.
/// * `2` (`DOM_DELTA_PAGE`) — deltas are pages; approximated as 800 px.
pub fn wheel_delta_scale(delta_mode: u32) -> f64 {
    match delta_mode {
        1 => 16.0,
        2 => 800.0,
        _ => 1.0,
    }
}

/// Exponential zoom factor for a normalized (pixel-space) wheel deltaY.
///
/// Scrolling up / pinching out (negative delta) zooms in. The exponential
/// form composes multiplicatively across events, so zoom speed feels the
/// same at every zoom level, and equal-and-opposite deltas cancel exactly.
pub fn wheel_zoom_factor(delta_y_px: f64) -> f64 {
    (-delta_y_px * 0.01).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(5.0, 0.0, 10.0), 0.5);
        assert_eq!(normalize(0.0, 0.0, 10.0), 0.0);
        assert_eq!(normalize(10.0, 0.0, 10.0), 1.0);
    }

    #[test]
    fn test_approx_equal() {
        assert!(approx_equal(1.0, 1.0, 0.01));
        assert!(approx_equal(1.0, 1.001, 0.01));
        assert!(!approx_equal(1.0, 1.1, 0.01));
    }

    #[test]
    fn test_wheel_delta_scale() {
        assert_eq!(wheel_delta_scale(0), 1.0); // DOM_DELTA_PIXEL
        assert_eq!(wheel_delta_scale(1), 16.0); // DOM_DELTA_LINE
        assert_eq!(wheel_delta_scale(2), 800.0); // DOM_DELTA_PAGE
        assert_eq!(wheel_delta_scale(99), 1.0); // unknown -> pixels
    }

    #[test]
    fn test_wheel_zoom_factor() {
        // Negative delta (scroll up / pinch out) zooms in
        assert!(wheel_zoom_factor(-100.0) > 1.0);
        // Positive delta zooms out
        assert!(wheel_zoom_factor(100.0) < 1.0);
        // Zero delta is identity
        assert!(approx_equal(wheel_zoom_factor(0.0), 1.0, 1e-12));
        // Equal-and-opposite deltas cancel exactly
        let round_trip = wheel_zoom_factor(-42.0) * wheel_zoom_factor(42.0);
        assert!(approx_equal(round_trip, 1.0, 1e-12));
    }
}
