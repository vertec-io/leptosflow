#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

//! # XYFlow Leptos
//!
//! A highly customizable Rust/WASM library for building node-based editors,
//! workflow systems, diagrams and more with Leptos.
//!
//! ## Quick Start
//!
//! ```ignore
//! use leptos::prelude::*;
//! use xyflow_leptos::{SvelteFlow, Node, Position};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let nodes = RwSignal::new(vec![
//!         Node::new("1".to_string(), Position::new(0.0, 0.0)),
//!     ]);
//!
//!     let edges = RwSignal::new(vec![]);
//!
//!     view! {
//!         <SvelteFlow nodes edges />
//!     }
//! }
//! ```

/// Base stylesheet for the classes emitted by the components.
///
/// All colors route through `--xy-*` CSS variables with fallbacks, so a host
/// application can retheme by defining those variables without overriding
/// selectors. Inject it once per document, e.g. with `leptos_meta`:
///
/// ```ignore
/// use leptos_meta::Style;
/// view! { <Style>{xyflow_leptos::STYLES}</Style> }
/// ```
///
/// or serve `xyflow-leptos/styles.css` as a static asset.
pub const STYLES: &str = include_str!("../styles.css");

pub mod types;
pub mod store;
pub mod hooks;
pub mod components;
pub mod utils;
pub mod error;
pub mod events;

// Re-export common types and functions
pub use types::{Node, Edge, Handle as HandleData, Viewport, Position, Change, HandleType, ConnectionMode, Connection, IsValidConnection, HandleBound, HandleBounds, HandleBoundPosition};
pub use store::{FlowStore, FlowState, ConnectionState, ConnectionCandidate, ContextMenuEvent, DeleteRequest};
pub use hooks::{use_flow_store, use_nodes, use_edges, use_viewport, use_connection};
pub use components::{
    SvelteFlow, Handle, HandlePosition, Background, BackgroundVariant,
    Panel, PanelPosition, Controls, ControlsOrientation, ConnectionLine,
    EdgeRenderer, NodeRenderer, Viewport, Viewport as FlowViewport, MiniMap,
};
pub use error::{FlowError, Result};
pub use events::{
    use_connection_handlers, calculate_handle_position,
    use_wheel_handler, use_pane_pan_handlers,
    use_context_menu_handler, use_flow_keydown_handler,
};
pub use utils::fit_view::{fit_view, fit_view_with_options, fit_bounds_with_options, FitViewOptions};

#[cfg(test)]
mod styles_tests {
    /// The animated-edge rules are a published contract: hosts set
    /// `Edge::animated` and expect the shipped stylesheet to move the dash,
    /// to leave the invisible interaction path alone, and to stand down when
    /// the operating system asks for reduced motion. `STYLES` is an
    /// `include_str!` of the real file, so asserting on it asserts on what
    /// actually ships.
    #[test]
    fn styles_carry_the_animated_edge_contract() {
        let css = crate::STYLES;

        // The class the EdgeRenderer emits for `Edge::animated` is styled,
        // and styled on the VISIBLE path only. Applying the dash to the wide
        // transparent interaction twin would paint a second ghost track.
        assert!(css.contains(".xyflow__edge.animated .xyflow__edge-path"));
        assert!(!css.contains(".xyflow__edge.animated .xyflow__edge-interaction"));

        // Dash pattern and cycle time are host-tunable; the keyframe walks
        // the offset back to zero so a host can retime it without redefining
        // the animation.
        assert!(css.contains("--xy-edge-animation-dash"));
        assert!(css.contains("--xy-edge-animation-duration"));
        assert!(css.contains("--xy-edge-animation-offset"));

        // Reduced motion stands the animation down entirely.
        assert!(css.contains("prefers-reduced-motion: reduce"));
    }
}
