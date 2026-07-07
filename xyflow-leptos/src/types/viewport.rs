//! Viewport state (pan and zoom)

use serde::{Deserialize, Serialize};

/// Represents the viewport state (pan and zoom)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Viewport {
    /// X offset (pan)
    pub x: f64,
    /// Y offset (pan)
    pub y: f64,
    /// Zoom level
    pub zoom: f64,
}

impl Viewport {
    /// Create a new viewport
    pub fn new(x: f64, y: f64, zoom: f64) -> Self {
        Viewport { x, y, zoom }
    }

    /// Create a default viewport (no pan, 1x zoom)
    pub fn default() -> Self {
        Viewport {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }

    /// Pan the viewport by an offset
    pub fn pan_by(&self, dx: f64, dy: f64) -> Viewport {
        Viewport {
            x: self.x + dx,
            y: self.y + dy,
            zoom: self.zoom,
        }
    }

    /// Zoom the viewport
    pub fn zoom_by(&self, factor: f64) -> Viewport {
        Viewport {
            x: self.x,
            y: self.y,
            zoom: (self.zoom * factor).max(0.1).min(10.0), // Clamp zoom to reasonable bounds
        }
    }

    /// Get the SVG transform string for applying viewport transformations
    pub fn transform_string(&self) -> String {
        format!("translate({} {}) scale({})", self.x, self.y, self.zoom)
    }

    /// Convert screen coordinates to flow coordinates
    ///
    /// Inverts the CSS transform `translate(x, y) scale(zoom)` applied by the
    /// viewport element: `flow = (screen - pan) / zoom`.
    pub fn screen_to_viewport(&self, screen_x: f64, screen_y: f64) -> (f64, f64) {
        (
            (screen_x - self.x) / self.zoom,
            (screen_y - self.y) / self.zoom,
        )
    }

    /// Convert flow coordinates to screen coordinates
    ///
    /// Matches the CSS transform `translate(x, y) scale(zoom)`:
    /// `screen = flow * zoom + pan`.
    pub fn viewport_to_screen(&self, viewport_x: f64, viewport_y: f64) -> (f64, f64) {
        (
            viewport_x * self.zoom + self.x,
            viewport_y * self.zoom + self.y,
        )
    }

    /// Check if a point is within the viewport bounds
    pub fn contains(&self, x: f64, y: f64, width: f64, height: f64) -> bool {
        // Transform viewport coordinates to screen
        let (screen_x1, screen_y1) = self.viewport_to_screen(x, y);
        let (screen_x2, screen_y2) =
            self.viewport_to_screen(x + width, y + height);

        // Check if transformed bounds are visible
        screen_x2 > 0.0 && screen_x1 < width && screen_y2 > 0.0 && screen_y1 < height
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_creation() {
        let vp = Viewport::new(10.0, 20.0, 1.5);
        assert_eq!(vp.x, 10.0);
        assert_eq!(vp.y, 20.0);
        assert_eq!(vp.zoom, 1.5);
    }

    #[test]
    fn test_pan_by() {
        let vp = Viewport::new(10.0, 20.0, 1.0);
        let new_vp = vp.pan_by(5.0, 10.0);
        assert_eq!(new_vp.x, 15.0);
        assert_eq!(new_vp.y, 30.0);
        assert_eq!(new_vp.zoom, 1.0);
    }

    #[test]
    fn test_zoom_by() {
        let vp = Viewport::new(0.0, 0.0, 1.0);
        let new_vp = vp.zoom_by(2.0);
        assert_eq!(new_vp.zoom, 2.0);
    }

    #[test]
    fn test_zoom_clamping() {
        let vp = Viewport::default();
        let very_small = vp.zoom_by(0.05); // 0.05 * 1.0 = 0.05
        assert_eq!(very_small.zoom, 0.1); // Clamped to 0.1

        let very_large = vp.zoom_by(15.0); // 15.0 * 1.0 = 15.0
        assert_eq!(very_large.zoom, 10.0); // Clamped to 10.0
    }

    #[test]
    fn test_transform_string() {
        let vp = Viewport::new(10.0, 20.0, 1.5);
        assert_eq!(vp.transform_string(), "translate(10 20) scale(1.5)");
    }

    #[test]
    fn test_coordinate_conversion() {
        let vp = Viewport::new(0.0, 0.0, 2.0);
        let (vp_x, vp_y) = vp.screen_to_viewport(100.0, 200.0);
        assert_eq!(vp_x, 50.0);
        assert_eq!(vp_y, 100.0);

        // Reverse
        let (screen_x, screen_y) = vp.viewport_to_screen(vp_x, vp_y);
        assert_eq!(screen_x, 100.0);
        assert_eq!(screen_y, 200.0);
    }
}
