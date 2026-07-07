//! Coordinate system utilities

use leptos::prelude::*;
use leptos::html;
use crate::types::{Viewport, Position};

/// Utilities for coordinate transformations
pub struct CoordinateSystem;

impl CoordinateSystem {
    /// Convert screen coordinates to flow coordinates
    ///
    /// The viewport applies the CSS transform `translate(x, y) scale(zoom)`,
    /// i.e. `screen = flow * zoom + pan + container_origin`. This inverts it.
    pub fn screen_to_flow(
        screen_x: f64,
        screen_y: f64,
        viewport: Viewport,
        svg_rect_x: f64,
        svg_rect_y: f64,
    ) -> (f64, f64) {
        let adjusted_x = (screen_x - svg_rect_x - viewport.x) / viewport.zoom;
        let adjusted_y = (screen_y - svg_rect_y - viewport.y) / viewport.zoom;
        (adjusted_x, adjusted_y)
    }

    /// Convert flow coordinates to screen coordinates
    ///
    /// Matches the CSS transform `translate(x, y) scale(zoom)` applied by the
    /// viewport: `screen = flow * zoom + pan + container_origin`.
    pub fn flow_to_screen(
        flow_x: f64,
        flow_y: f64,
        viewport: Viewport,
        svg_rect_x: f64,
        svg_rect_y: f64,
    ) -> (f64, f64) {
        let screen_x = flow_x * viewport.zoom + viewport.x + svg_rect_x;
        let screen_y = flow_y * viewport.zoom + viewport.y + svg_rect_y;
        (screen_x, screen_y)
    }

    /// Calculate zoom delta from mouse wheel
    pub fn calculate_zoom_delta(wheel_delta: f64, zoom_speed: f64) -> f64 {
        if wheel_delta > 0.0 {
            1.0 + zoom_speed
        } else {
            1.0 / (1.0 + zoom_speed)
        }
    }
}

/// Convert screen coordinates to flow position
///
/// This function accounts for the flow container's position on the screen.
/// It queries the DOM for the flow container to get its bounding rect.
pub fn screen_to_flow_position(screen_x: f64, screen_y: f64, viewport: &Viewport) -> Position {
    use wasm_bindgen::JsCast;

    // Try to get the flow container's position
    // We look for elements with common flow container classes
    let (container_x, container_y) = if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            // Try to find the flow container by common class names or data attributes
            let container = document.query_selector("[data-xyflow-container]").ok().flatten()
                .or_else(|| document.query_selector(".xyflow").ok().flatten())
                .or_else(|| document.query_selector(".leptos-flow").ok().flatten())
                .or_else(|| document.query_selector("[style*='position: relative']").ok().flatten());

            if let Some(element) = container {
                if let Ok(html_element) = element.dyn_into::<web_sys::HtmlElement>() {
                    let rect = html_element.get_bounding_client_rect();
                    (rect.left(), rect.top())
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        }
    } else {
        (0.0, 0.0)
    };

    let (x, y) = CoordinateSystem::screen_to_flow(screen_x, screen_y, *viewport, container_x, container_y);
    Position::new(x, y)
}

/// Convert screen coordinates to flow position using a NodeRef
///
/// This is the preferred method that uses Leptos's NodeRef for efficient access
/// to the container element without DOM queries.
pub fn screen_to_flow_position_with_ref(
    screen_x: f64,
    screen_y: f64,
    viewport: &Viewport,
    container_ref: NodeRef<html::Div>,
) -> Position {
    let (container_x, container_y) = if let Some(element) = container_ref.get() {
        let rect = element.get_bounding_client_rect();
        (rect.left(), rect.top())
    } else {
        // Fallback to (0, 0) if container not yet mounted
        (0.0, 0.0)
    };

    let (x, y) = CoordinateSystem::screen_to_flow(screen_x, screen_y, *viewport, container_x, container_y);
    Position::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_to_flow() {
        let viewport = Viewport::new(0.0, 0.0, 2.0);
        let (flow_x, flow_y) = CoordinateSystem::screen_to_flow(100.0, 200.0, viewport, 0.0, 0.0);
        assert_eq!(flow_x, 50.0);
        assert_eq!(flow_y, 100.0);
    }

    #[test]
    fn test_flow_to_screen() {
        let viewport = Viewport::new(0.0, 0.0, 2.0);
        let (screen_x, screen_y) = CoordinateSystem::flow_to_screen(50.0, 100.0, viewport, 0.0, 0.0);
        assert_eq!(screen_x, 100.0);
        assert_eq!(screen_y, 200.0);
    }

    #[test]
    fn test_coordinate_round_trip() {
        let viewport = Viewport::new(10.0, 20.0, 1.5);
        let original = (50.0, 100.0);

        let screen = CoordinateSystem::flow_to_screen(original.0, original.1, viewport, 0.0, 0.0);
        let flow = CoordinateSystem::screen_to_flow(screen.0, screen.1, viewport, 0.0, 0.0);

        assert!((flow.0 - original.0).abs() < 0.01);
        assert!((flow.1 - original.1).abs() < 0.01);
    }
}
