//! Event handling for XYFlow
//!
//! This module contains event handlers for pan, zoom, drag, connection
//! creation, context menus, and keyboard shortcuts.

pub mod pan;
pub mod zoom;
pub mod drag;
pub mod connection;
pub mod context_menu;
pub mod keyboard;

pub use pan::use_pane_pan_handlers;
#[allow(deprecated)]
pub use pan::use_pan_handlers;
pub use zoom::use_wheel_handler;
#[allow(deprecated)]
pub use zoom::use_zoom_handler;
pub use drag::use_node_drag_handlers;
pub use connection::{use_connection_handlers, calculate_handle_position};
pub use context_menu::use_context_menu_handler;
pub use keyboard::use_flow_keydown_handler;

