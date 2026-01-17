//! Node dragging event handlers

use leptos::prelude::*;
use leptos::ev;
use crate::hooks::use_flow_store;
use crate::types::Position;

/// State for tracking drag operations
#[derive(Clone, Debug)]
struct DragState {
    /// Whether we're currently dragging
    is_dragging: bool,
    /// ID of the node being dragged
    node_id: Option<String>,
    /// Starting mouse position (screen coordinates)
    start_mouse_pos: Position,
    /// Starting node position (flow coordinates)
    start_node_pos: Position,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            node_id: None,
            start_mouse_pos: Position::new(0.0, 0.0),
            start_node_pos: Position::new(0.0, 0.0),
        }
    }
}

/// Hook for node dragging functionality
///
/// Returns event handlers for mouse down, move, and up events
pub fn use_node_drag_handlers(
    node_id: String,
) -> (
    impl Fn(ev::MouseEvent) + Clone,  // on_mouse_down
    impl Fn(ev::MouseEvent) + Clone,  // on_mouse_move
    impl Fn(ev::MouseEvent) + Clone,  // on_mouse_up
) {
    let store = use_flow_store();
    
    // Drag state signal
    let drag_state = RwSignal::new(DragState::default());
    
    // Mouse down handler - start drag
    let on_mouse_down = {
        let node_id = node_id.clone();
        let store = store.clone();
        move |ev: ev::MouseEvent| {
            ev.prevent_default();
            ev.stop_propagation();
            
            // Get current node position
            let nodes = store.get_nodes();
            if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                drag_state.update(|state| {
                    state.is_dragging = true;
                    state.node_id = Some(node_id.clone());
                    state.start_mouse_pos = Position::new(ev.client_x() as f64, ev.client_y() as f64);
                    state.start_node_pos = node.position;
                });
                
                // Mark node as dragging
                store.update_node(&node_id, |node| {
                    node.dragging = true;
                });
            }
        }
    };
    
    // Mouse move handler - update position
    let on_mouse_move = {
        let store = store.clone();
        move |ev: ev::MouseEvent| {
            let state = drag_state.get();
            if !state.is_dragging {
                return;
            }
            
            ev.prevent_default();
            
            // Calculate delta in screen coordinates
            let current_mouse_pos = Position::new(ev.client_x() as f64, ev.client_y() as f64);
            let delta_screen = Position::new(
                current_mouse_pos.x - state.start_mouse_pos.x,
                current_mouse_pos.y - state.start_mouse_pos.y,
            );
            
            // Convert delta to flow coordinates (accounting for zoom)
            let viewport = store.get_viewport();
            let delta_flow = Position::new(
                delta_screen.x / viewport.zoom,
                delta_screen.y / viewport.zoom,
            );
            
            // Calculate new position
            let new_position = Position::new(
                state.start_node_pos.x + delta_flow.x,
                state.start_node_pos.y + delta_flow.y,
            );
            
            // Update node position
            if let Some(ref node_id) = state.node_id {
                store.update_node(node_id, |node| {
                    node.position = new_position;
                });
            }
        }
    };
    
    // Mouse up handler - end drag
    let on_mouse_up = {
        let store = store.clone();
        move |ev: ev::MouseEvent| {
            let state = drag_state.get();
            if !state.is_dragging {
                return;
            }
            
            ev.prevent_default();
            
            // Mark node as not dragging
            if let Some(ref node_id) = state.node_id {
                store.update_node(node_id, |node| {
                    node.dragging = false;
                });
            }
            
            // Reset drag state
            drag_state.set(DragState::default());
        }
    };
    
    (on_mouse_down, on_mouse_move, on_mouse_up)
}

