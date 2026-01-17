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
}
