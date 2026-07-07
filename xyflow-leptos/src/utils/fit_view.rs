//! Fit-view: compute a viewport that frames a set of nodes.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::store::FlowStore;
use crate::types::{Bounds, Node, Viewport};

/// Options controlling [`fit_view`] framing.
#[derive(Clone, Copy, Debug)]
pub struct FitViewOptions {
    /// Fraction of the container to leave as padding around the content
    /// (0.1 = 10% on every side, matching React Flow's default).
    pub padding: f64,
    /// Lower zoom clamp. `None` uses the store's `min_zoom`.
    pub min_zoom: Option<f64>,
    /// Upper zoom clamp. `None` uses the store's `max_zoom`.
    pub max_zoom: Option<f64>,
}

impl Default for FitViewOptions {
    fn default() -> Self {
        FitViewOptions {
            padding: 0.1,
            min_zoom: None,
            max_zoom: None,
        }
    }
}

/// Compute the union bounds of a set of nodes (in flow coordinates).
///
/// Returns `None` for an empty slice. Unmeasured nodes fall back to the
/// default node footprint (150 x 40).
pub fn nodes_bounds(nodes: &[Node]) -> Option<Bounds> {
    let mut iter = nodes.iter().filter(|n| !n.hidden);
    let first = iter.next()?;
    let node_bounds = |n: &Node| {
        Bounds::new(
            n.position.x,
            n.position.y,
            n.width.unwrap_or(150.0),
            n.height.unwrap_or(40.0),
        )
    };
    let mut bounds = node_bounds(first);
    for node in iter {
        bounds = bounds.expand(&node_bounds(node));
    }
    Some(bounds)
}

/// Compute the viewport that frames `bounds` inside a container of the given
/// size with fractional `padding`, clamped to `[min_zoom, max_zoom]`.
pub fn viewport_for_bounds(
    bounds: Bounds,
    container_width: f64,
    container_height: f64,
    min_zoom: f64,
    max_zoom: f64,
    padding: f64,
) -> Viewport {
    let width = bounds.width.max(1.0);
    let height = bounds.height.max(1.0);
    let usable = (1.0 - 2.0 * padding.clamp(0.0, 0.45)).max(0.1);

    let zoom = ((container_width * usable) / width)
        .min((container_height * usable) / height)
        .clamp(min_zoom, max_zoom);

    // Center the content: screen = flow * zoom + pan
    let (center_x, center_y) = bounds.center();
    Viewport {
        x: container_width / 2.0 - center_x * zoom,
        y: container_height / 2.0 - center_y * zoom,
        zoom,
    }
}

/// Resolve the flow container's size in CSS pixels.
///
/// Prefers the store's `container_ref` (attached by [`crate::SvelteFlow`]);
/// falls back to querying the document for a `.xyflow`/`.leptos-flow`/
/// `.svelte-flow` element, then to the window size.
pub fn container_size(store: &FlowStore) -> (f64, f64) {
    if let Some(element) = store.state.container_ref.get() {
        let rect = element.get_bounding_client_rect();
        if rect.width() > 0.0 && rect.height() > 0.0 {
            return (rect.width(), rect.height());
        }
    }

    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        for selector in [".xyflow", ".leptos-flow", ".svelte-flow"] {
            if let Ok(Some(element)) = document.query_selector(selector) {
                if let Ok(html) = element.dyn_into::<web_sys::HtmlElement>() {
                    let rect = html.get_bounding_client_rect();
                    if rect.width() > 0.0 && rect.height() > 0.0 {
                        return (rect.width(), rect.height());
                    }
                }
            }
        }
    }

    let fallback = web_sys::window()
        .map(|w| {
            (
                w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0),
                w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0),
            )
        })
        .unwrap_or((800.0, 600.0));
    fallback
}

/// Fit all (visible) nodes into the container with default options.
///
/// Sets the store's viewport so every node is visible with 10% padding.
/// No-op when the flow contains no nodes.
pub fn fit_view(store: &FlowStore) {
    fit_view_with_options(store, FitViewOptions::default());
}

/// Fit all (visible) nodes into the container.
pub fn fit_view_with_options(store: &FlowStore, options: FitViewOptions) {
    let nodes = store.get_nodes_untracked();
    let Some(bounds) = nodes_bounds(&nodes) else {
        return;
    };
    fit_bounds_with_options(store, bounds, options);
}

/// Frame an arbitrary flow-coordinate rectangle in the container.
pub fn fit_bounds_with_options(store: &FlowStore, bounds: Bounds, options: FitViewOptions) {
    let (width, height) = container_size(store);
    let min_zoom = options
        .min_zoom
        .unwrap_or_else(|| store.state.min_zoom.get_untracked());
    let max_zoom = options
        .max_zoom
        .unwrap_or_else(|| store.state.max_zoom.get_untracked());
    let viewport = viewport_for_bounds(bounds, width, height, min_zoom, max_zoom, options.padding);
    store.set_viewport(viewport);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Position;

    #[test]
    fn test_nodes_bounds_empty() {
        assert!(nodes_bounds(&[]).is_none());
    }

    #[test]
    fn test_nodes_bounds_union() {
        let nodes = vec![
            Node::new("a".into(), Position::new(0.0, 0.0)).with_dimensions(100.0, 50.0),
            Node::new("b".into(), Position::new(200.0, 300.0)).with_dimensions(100.0, 50.0),
        ];
        let b = nodes_bounds(&nodes).unwrap();
        assert_eq!(b.x, 0.0);
        assert_eq!(b.y, 0.0);
        assert_eq!(b.width, 300.0);
        assert_eq!(b.height, 350.0);
    }

    #[test]
    fn test_nodes_bounds_skips_hidden() {
        let mut far = Node::new("far".into(), Position::new(10_000.0, 0.0));
        far.hidden = true;
        let nodes = vec![
            Node::new("a".into(), Position::new(0.0, 0.0)).with_dimensions(100.0, 50.0),
            far,
        ];
        let b = nodes_bounds(&nodes).unwrap();
        assert_eq!(b.width, 100.0);
    }

    #[test]
    fn test_viewport_for_bounds_centers_content() {
        // 200x100 content in an 800x600 container, no padding, zoom clamped to 1.0
        let bounds = Bounds::new(100.0, 100.0, 200.0, 100.0);
        let vp = viewport_for_bounds(bounds, 800.0, 600.0, 0.1, 1.0, 0.0);
        assert_eq!(vp.zoom, 1.0);
        // Content center (200, 150) should map to container center (400, 300)
        assert_eq!(vp.x + 200.0 * vp.zoom, 400.0);
        assert_eq!(vp.y + 150.0 * vp.zoom, 300.0);
    }

    #[test]
    fn test_viewport_for_bounds_zooms_to_fit() {
        // 1600x1200 content in an 800x600 container: zoom 0.5 without padding
        let bounds = Bounds::new(0.0, 0.0, 1600.0, 1200.0);
        let vp = viewport_for_bounds(bounds, 800.0, 600.0, 0.1, 2.0, 0.0);
        assert!((vp.zoom - 0.5).abs() < 1e-9);
        // Fully framed: top-left maps to (0, 0), bottom-right to (800, 600)
        assert!((vp.x - 0.0).abs() < 1e-9);
        assert!((vp.y - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_viewport_for_bounds_respects_padding() {
        let bounds = Bounds::new(0.0, 0.0, 800.0, 600.0);
        let vp = viewport_for_bounds(bounds, 800.0, 600.0, 0.1, 2.0, 0.1);
        assert!((vp.zoom - 0.8).abs() < 1e-9);
    }
}
