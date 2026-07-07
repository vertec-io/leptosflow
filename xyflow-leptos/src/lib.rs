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

pub mod types;
pub mod store;
pub mod hooks;
pub mod components;
pub mod utils;
pub mod error;
pub mod events;

// Re-export common types and functions
pub use types::{Node, Edge, Handle as HandleData, Viewport, Position, Change, HandleType, ConnectionMode, Connection, HandleBound, HandleBounds, HandleBoundPosition};
pub use store::{FlowStore, FlowState, ConnectionState};
pub use hooks::{use_flow_store, use_nodes, use_edges, use_viewport};
pub use components::{
    SvelteFlow, Handle, HandlePosition, Background, BackgroundVariant,
    Panel, PanelPosition, Controls, ControlsOrientation, ConnectionLine,
    EdgeRenderer, NodeRenderer, Viewport, Viewport as FlowViewport, MiniMap,
};
pub use error::{FlowError, Result};
pub use events::{use_connection_handlers, calculate_handle_position};
pub use utils::fit_view::{fit_view, fit_view_with_options, fit_bounds_with_options, FitViewOptions};
