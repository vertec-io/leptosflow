//! Handle utilities for connection creation

use crate::types::{
    Node, Position, HandleBound, HandleType, HandleBoundPosition,
    Connection, ConnectionMode,
};

/// Connection radius for finding nearby handles (in pixels)
/// This is the distance from the mouse cursor to a handle center
/// within which the connection will "snap" to that handle.
pub const CONNECTION_RADIUS: f64 = 30.0;

/// Additional distance for searching nodes
const ADDITIONAL_DISTANCE: f64 = 250.0;

/// Result of finding the closest handle
#[derive(Clone, Debug)]
pub struct ClosestHandle {
    /// The handle that was found
    pub handle: HandleBound,
    /// The node ID that owns this handle
    pub node_id: String,
}

/// Get nodes within a certain distance of a position
fn get_nodes_within_distance(
    position: Position,
    nodes: &[Node],
    distance: f64,
) -> Vec<&Node> {
    let mut result = Vec::new();
    let rect_x = position.x - distance;
    let rect_y = position.y - distance;
    let rect_width = distance * 2.0;
    let rect_height = distance * 2.0;
    
    for node in nodes {
        let node_x = node.position.x;
        let node_y = node.position.y;
        let node_width = node.width.unwrap_or(150.0);
        let node_height = node.height.unwrap_or(40.0);
        
        // Check if rectangles overlap
        if rect_x < node_x + node_width &&
           rect_x + rect_width > node_x &&
           rect_y < node_y + node_height &&
           rect_y + rect_height > node_y {
            result.push(node);
        }
    }
    
    result
}

/// Find the closest handle to a position
///
/// This is the core function for connection creation - it finds the nearest
/// valid handle within the connection radius.
pub fn get_closest_handle(
    position: Position,
    connection_radius: f64,
    nodes: &[Node],
    from_node_id: &str,
    from_handle_id: Option<&str>,
    from_handle_type: HandleType,
) -> Option<ClosestHandle> {
    let mut closest_handles: Vec<(HandleBound, String)> = Vec::new();
    let mut min_distance = f64::INFINITY;

    let close_nodes = get_nodes_within_distance(
        position,
        nodes,
        connection_radius + ADDITIONAL_DISTANCE,
    );

    for node in close_nodes {
        // Skip if no handle bounds measured yet
        let Some(handle_bounds) = &node.internals.handle_bounds else {
            continue;
        };

        // Check all handles on this node
        for handle in handle_bounds.all_handles() {
            // Skip if this is the same handle we're dragging from
            if node.id == from_node_id &&
               handle.handle_type == from_handle_type &&
               handle.id.as_deref() == from_handle_id {
                continue;
            }

            // Calculate distance to handle center
            // IMPORTANT: Use center_absolute to get flow coordinates, not node-relative coordinates
            let handle_center = handle.center_absolute(&node.position);
            let dx = handle_center.x - position.x;
            let dy = handle_center.y - position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > connection_radius {
                continue;
            }

            if distance < min_distance {
                closest_handles = vec![(handle.clone(), node.id.clone())];
                min_distance = distance;
            } else if (distance - min_distance).abs() < 0.01 {
                // Multiple handles at same distance
                closest_handles.push((handle.clone(), node.id.clone()));
            }
        }
    }

    if closest_handles.is_empty() {
        return None;
    }

    // If multiple handles overlap, prefer the opposite type
    if closest_handles.len() > 1 {
        let opposite_type = match from_handle_type {
            HandleType::Source => HandleType::Target,
            HandleType::Target => HandleType::Source,
        };

        if let Some((handle, node_id)) = closest_handles.iter()
            .find(|(h, _)| h.handle_type == opposite_type) {
            return Some(ClosestHandle {
                handle: handle.clone(),
                node_id: node_id.clone(),
            });
        }
    }

    let (handle, node_id) = &closest_handles[0];
    Some(ClosestHandle {
        handle: handle.clone(),
        node_id: node_id.clone(),
    })
}

/// Calculate the absolute position of a handle on a node
pub fn calculate_handle_position(
    node: &Node,
    handle_position: HandleBoundPosition,
) -> Position {
    let node_x = node.position.x;
    let node_y = node.position.y;
    let node_width = node.width.unwrap_or(150.0);
    let node_height = node.height.unwrap_or(40.0);

    match handle_position {
        HandleBoundPosition::Top => Position::new(
            node_x + node_width / 2.0,
            node_y,
        ),
        HandleBoundPosition::Right => Position::new(
            node_x + node_width,
            node_y + node_height / 2.0,
        ),
        HandleBoundPosition::Bottom => Position::new(
            node_x + node_width / 2.0,
            node_y + node_height,
        ),
        HandleBoundPosition::Left => Position::new(
            node_x,
            node_y + node_height / 2.0,
        ),
    }
}

/// Validate if a handle connection is valid based on handle types and connection mode
///
/// This implements the strict/loose mode logic from React Flow:
/// - Strict: only source-to-target connections allowed
/// - Loose: source-to-source and target-to-target also allowed
pub fn is_valid_handle_connection(
    from_handle_type: HandleType,
    to_handle_type: HandleType,
    connection_mode: ConnectionMode,
) -> bool {
    match connection_mode {
        ConnectionMode::Strict => {
            // In strict mode, must be opposite types
            (from_handle_type == HandleType::Source && to_handle_type == HandleType::Target) ||
            (from_handle_type == HandleType::Target && to_handle_type == HandleType::Source)
        }
        ConnectionMode::Loose => {
            // In loose mode, any combination is allowed
            true
        }
    }
}

/// Create a connection object from handle information
pub fn create_connection(
    from_node_id: &str,
    from_handle_id: Option<&str>,
    from_handle_type: HandleType,
    to_node_id: &str,
    to_handle: &HandleBound,
) -> Connection {
    // Determine which is source and which is target based on handle types
    let (source, source_handle, target, target_handle) = if from_handle_type == HandleType::Source {
        (
            from_node_id.to_string(),
            from_handle_id.map(|s| s.to_string()),
            to_node_id.to_string(),
            to_handle.id.clone(),
        )
    } else {
        (
            to_node_id.to_string(),
            to_handle.id.clone(),
            from_node_id.to_string(),
            from_handle_id.map(|s| s.to_string()),
        )
    };

    Connection::new(source, target, source_handle, target_handle)
}

