//! ConnectionLine component for rendering connection in progress

use leptos::prelude::*;
use crate::hooks::use_flow_store;
use crate::utils::edge_path::{generate_edge_path, EdgePathType};

/// Component that renders a connection line while dragging from a handle
///
/// This component is automatically rendered by the SvelteFlow component
/// when a connection is in progress (user is dragging from a handle).
///
/// Uses CSS classes for styling:
/// - `.xyflow__connection-path` - Base styling from CSS variables
/// - `.valid` - Applied when connection is valid (solid line)
/// - Otherwise shows dashed line for invalid/uncertain state
#[component]
pub fn ConnectionLine() -> impl IntoView {
    let store = use_flow_store();

    // Get connection state
    let connection = move || store.state.connection_in_progress.get();

    view! {
        {move || {
            if let Some(conn) = connection() {
                // Log connection coordinates for debugging
                #[cfg(debug_assertions)]
                {
                    web_sys::console::log_1(&format!(
                        "ConnectionLine: from=({:.1}, {:.1}) to=({:.1}, {:.1})",
                        conn.from_position.x, conn.from_position.y,
                        conn.to_position.x, conn.to_position.y
                    ).into());
                }

                // Calculate path from source to current mouse position
                let path = generate_edge_path(
                    conn.from_position,
                    conn.to_position,
                    EdgePathType::Bezier,
                );

                // Use CSS classes for styling, with valid/invalid state
                let class = if conn.is_valid {
                    "xyflow__connection-path valid"
                } else {
                    "xyflow__connection-path"
                };

                view! {
                    <svg class="xyflow__connectionline" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000; overflow: visible;">
                        <g class="xyflow__connection">
                            <path
                                d=path
                                class=class
                            />
                        </g>
                    </svg>
                }.into_any()
            } else {
                view! {}.into_any()
            }
        }}
    }
}

