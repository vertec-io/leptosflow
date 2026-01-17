//! Connection creation event handlers

use leptos::prelude::*;
use leptos::ev;
use crate::hooks::use_flow_store;
use crate::types::{Position, HandleType, ConnectionMode, IsValidConnection};
use crate::utils::coordinate::screen_to_flow_position_with_ref;

/// Hook to create connection drag handlers for a handle
///
/// This hook returns a mousedown handler that starts a connection.
/// The mousemove and mouseup handling is done by the flow's global handlers
/// set up via `setup_connection_listeners`.
///
/// # Arguments
///
/// * `node_id` - ID of the node containing the handle
/// * `handle_id` - Optional handle ID (None for default handle)
/// * `handle_position` - Position of the handle in flow coordinates
/// * `handle_type` - Type of the handle (Source or Target)
///
/// # Example
///
/// ```ignore
/// let on_mouse_down = use_connection_handlers(
///     node_id.clone(),
///     handle_id.clone(),
///     handle_pos,
///     HandleType::Source,
/// );
/// ```
pub fn use_connection_handlers(
    node_id: String,
    handle_id: Option<String>,
    _handle_position: Position,
    handle_type: HandleType,
    _connection_mode: ConnectionMode,
    _is_valid_connection: Option<IsValidConnection>,
) -> impl Fn(ev::MouseEvent) + Clone {
    let store = use_flow_store();

    // Mouse down handler: Start connection
    let on_mouse_down = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        // Debug: Log that handle mousedown was triggered
        web_sys::console::log_1(&format!(
            "Handle mousedown: node={}, handle={:?}, type={:?}",
            node_id, handle_id, handle_type
        ).into());

        let viewport = store.get_viewport_untracked();
        let container_ref = store.state.container_ref;

        // Get the handle element's center position instead of mouse position
        // This ensures the connection line starts exactly from the handle center
        let handle_center_pos = if let Some(target) = ev.target() {
            use leptos::wasm_bindgen::JsCast;
            if let Some(element) = target.dyn_ref::<leptos::web_sys::Element>() {
                let rect = element.get_bounding_client_rect();
                let center_x = rect.left() + rect.width() / 2.0;
                let center_y = rect.top() + rect.height() / 2.0;

                screen_to_flow_position_with_ref(
                    center_x,
                    center_y,
                    &viewport,
                    container_ref,
                )
            } else {
                // Fallback to mouse position if we can't get the element
                screen_to_flow_position_with_ref(
                    ev.client_x() as f64,
                    ev.client_y() as f64,
                    &viewport,
                    container_ref,
                )
            }
        } else {
            // Fallback to mouse position
            screen_to_flow_position_with_ref(
                ev.client_x() as f64,
                ev.client_y() as f64,
                &viewport,
                container_ref,
            )
        };

        web_sys::console::log_1(&format!(
            "Starting connection at position: ({:.1}, {:.1})",
            handle_center_pos.x, handle_center_pos.y
        ).into());

        store.start_connection(
            node_id.clone(),
            handle_id.clone(),
            handle_type,
            handle_center_pos,
        );

        // Verify connection was started
        let conn = store.state.connection_in_progress.get_untracked();
        web_sys::console::log_1(&format!(
            "Connection after start: {:?}",
            conn.is_some()
        ).into());
    };

    on_mouse_down
}

/// Get the position of a handle in flow coordinates
///
/// This calculates the center position of a handle element.
/// In a full implementation, this would use DOM measurements.
///
/// # Arguments
///
/// * `node_position` - Position of the parent node
/// * `node_width` - Width of the parent node
/// * `node_height` - Height of the parent node
/// * `handle_position` - Position of the handle (Top, Right, Bottom, Left)
///
/// # Returns
///
/// Position of the handle center in flow coordinates
pub fn calculate_handle_position(
    node_position: Position,
    node_width: f64,
    node_height: f64,
    handle_position: &str,
) -> Position {
    match handle_position {
        "top" => Position::new(
            node_position.x + node_width / 2.0,
            node_position.y,
        ),
        "right" => Position::new(
            node_position.x + node_width,
            node_position.y + node_height / 2.0,
        ),
        "bottom" => Position::new(
            node_position.x + node_width / 2.0,
            node_position.y + node_height,
        ),
        "left" => Position::new(
            node_position.x,
            node_position.y + node_height / 2.0,
        ),
        _ => Position::new(
            node_position.x + node_width / 2.0,
            node_position.y + node_height / 2.0,
        ),
    }
}

