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

    /// Zoom by `factor`, keeping the flow point under `(point_x, point_y)`
    /// stationary on screen.
    ///
    /// `point_x`/`point_y` are in screen pixels relative to the flow
    /// container's top-left corner (i.e. the same space the viewport's CSS
    /// `translate(x, y) scale(zoom)` transform maps into). The new zoom is
    /// clamped to `[min_zoom, max_zoom]`; the pan is derived from the zoom
    /// that was actually applied, so clamping never shifts the view.
    ///
    /// Derivation: the flow point under the cursor is
    /// `f = (p - pan) / zoom`. Requiring `p = f * zoom' + pan'` gives
    /// `pan' = p - (p - pan) * zoom' / zoom`.
    pub fn zoom_at_point(
        &self,
        factor: f64,
        point_x: f64,
        point_y: f64,
        min_zoom: f64,
        max_zoom: f64,
    ) -> Viewport {
        let new_zoom = (self.zoom * factor).clamp(min_zoom, max_zoom);
        if self.zoom <= 0.0 || !self.zoom.is_finite() {
            // Degenerate current zoom: just recover to the clamped zoom.
            return Viewport::new(self.x, self.y, new_zoom);
        }
        let applied = new_zoom / self.zoom;
        Viewport {
            x: point_x - (point_x - self.x) * applied,
            y: point_y - (point_y - self.y) * applied,
            zoom: new_zoom,
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
    fn test_zoom_at_point_keeps_cursor_point_stationary() {
        let vp = Viewport::new(37.0, -12.0, 1.3);
        let (px, py) = (211.0, 143.0);

        // Flow point under the cursor before the zoom
        let (fx, fy) = vp.screen_to_viewport(px, py);

        let zoomed = vp.zoom_at_point(1.4, px, py, 0.2, 4.0);

        // Same flow point must map back to the same screen point
        let (sx, sy) = zoomed.viewport_to_screen(fx, fy);
        assert!((sx - px).abs() < 1e-9, "x drifted: {} vs {}", sx, px);
        assert!((sy - py).abs() < 1e-9, "y drifted: {} vs {}", sy, py);
        assert!((zoomed.zoom - 1.3 * 1.4).abs() < 1e-9);
    }

    #[test]
    fn test_zoom_at_point_zoom_out_keeps_cursor_point_stationary() {
        let vp = Viewport::new(-80.0, 55.0, 2.0);
        let (px, py) = (10.0, 480.0);
        let (fx, fy) = vp.screen_to_viewport(px, py);

        let zoomed = vp.zoom_at_point(0.5, px, py, 0.2, 4.0);

        let (sx, sy) = zoomed.viewport_to_screen(fx, fy);
        assert!((sx - px).abs() < 1e-9);
        assert!((sy - py).abs() < 1e-9);
        assert!((zoomed.zoom - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_zoom_at_point_identity_factor_is_noop() {
        let vp = Viewport::new(12.0, 34.0, 1.5);
        let zoomed = vp.zoom_at_point(1.0, 100.0, 200.0, 0.2, 4.0);
        assert_eq!(zoomed.x, vp.x);
        assert_eq!(zoomed.y, vp.y);
        assert_eq!(zoomed.zoom, vp.zoom);
    }

    #[test]
    fn test_zoom_at_point_clamps_to_bounds() {
        let vp = Viewport::new(0.0, 0.0, 1.0);

        let maxed = vp.zoom_at_point(100.0, 50.0, 50.0, 0.2, 4.0);
        assert_eq!(maxed.zoom, 4.0);

        let minned = vp.zoom_at_point(0.001, 50.0, 50.0, 0.2, 4.0);
        assert_eq!(minned.zoom, 0.2);

        // Pan must be consistent with the zoom that was actually applied:
        // the flow point under the cursor stays put even when clamped.
        let (fx, fy) = vp.screen_to_viewport(50.0, 50.0);
        let (sx, sy) = maxed.viewport_to_screen(fx, fy);
        assert!((sx - 50.0).abs() < 1e-9);
        assert!((sy - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_zoom_at_point_at_max_zoom_is_stable() {
        let vp = Viewport::new(-5.0, 9.0, 4.0);
        let zoomed = vp.zoom_at_point(2.0, 123.0, 456.0, 0.2, 4.0);
        // Already at max: nothing moves.
        assert_eq!(zoomed.zoom, 4.0);
        assert_eq!(zoomed.x, vp.x);
        assert_eq!(zoomed.y, vp.y);
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
