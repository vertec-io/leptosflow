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
    // Connectable handles win; non-connectable (locked) handles are still
    // reported — as a last resort — so the UI can show a visible refusal
    // (`invalid` state) instead of silently ignoring the hover/drop. They
    // can never complete: `evaluate_connection_candidate` marks them invalid.
    let mut closest_handles: Vec<(HandleBound, String)> = Vec::new();
    let mut min_distance = f64::INFINITY;
    let mut closest_locked: Vec<(HandleBound, String)> = Vec::new();
    let mut min_locked_distance = f64::INFINITY;

    let close_nodes = get_nodes_within_distance(
        position,
        nodes,
        connection_radius + ADDITIONAL_DISTANCE,
    );

    // Direct hover beats proximity: when the pointer is physically inside a
    // handle's bounds, THAT handle is the candidate — even a locked one —
    // so feedback always matches the handle the user is pointing at (a
    // nearby connectable handle within the snap radius must not steal the
    // hover from a locked handle under the cursor).
    let mut hovered: Option<(HandleBound, String, f64)> = None;

    for node in &close_nodes {
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

            let handle_x = node.position.x + handle.x;
            let handle_y = node.position.y + handle.y;
            let contains_pointer = position.x >= handle_x
                && position.x <= handle_x + handle.width
                && position.y >= handle_y
                && position.y <= handle_y + handle.height;
            if contains_pointer
                && hovered
                    .as_ref()
                    .is_none_or(|(_, _, best)| distance < *best)
            {
                hovered = Some((handle.clone(), node.id.clone(), distance));
            }

            if distance > connection_radius {
                continue;
            }

            // Locked handles never shadow connectable ones: separate bucket,
            // consulted only when no connectable handle is in range.
            let (bucket, bucket_min) = if handle.connectable {
                (&mut closest_handles, &mut min_distance)
            } else {
                (&mut closest_locked, &mut min_locked_distance)
            };

            if distance < *bucket_min {
                *bucket = vec![(handle.clone(), node.id.clone())];
                *bucket_min = distance;
            } else if (distance - *bucket_min).abs() < 0.01 {
                // Multiple handles at same distance
                bucket.push((handle.clone(), node.id.clone()));
            }
        }
    }

    // The handle directly under the pointer always wins
    if let Some((handle, node_id, _)) = hovered {
        return Some(ClosestHandle { handle, node_id });
    }

    if closest_handles.is_empty() {
        closest_handles = closest_locked;
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

/// Everything the pointer handlers need to know about the current drop
/// candidate, produced by [`evaluate_connection_candidate`].
#[derive(Clone, Debug)]
pub struct CandidateEvaluation {
    /// Closest handle within the connection radius, if any. Present even
    /// when the resulting connection would be invalid — including locked
    /// (non-connectable) handles — so the UI can snap to it and style it
    /// as invalid rather than ignoring the gesture.
    pub candidate: Option<ClosestHandle>,
    /// The connection the candidate would form (source/target ordered by
    /// handle type). `None` when there is no candidate.
    pub connection: Option<Connection>,
    /// Whether completing on the candidate would produce a valid connection.
    pub is_valid: bool,
}

/// Hit-test the pointer position against the measured handle bounds and
/// evaluate whether the snapped handle would form a valid connection.
///
/// Validity layers, all of which must pass:
/// 1. the candidate handle accepts connections (`connectable` — locked
///    handles are reported as candidates for styling but never validate)
/// 2. connection-mode check (strict: opposite handle types only)
/// 3. built-in [`validate_connection`] rules (no same-handle; no self-loop
///    in strict mode) plus the per-`Handle` `is_valid_connection` fn
/// 4. the host-level predicate (`FlowStore::set_is_valid_connection`),
///    passed here as `extra_validator`
///
/// Pure function (no DOM, no signals) so the state machine is testable
/// headlessly.
#[allow(clippy::too_many_arguments)]
pub fn evaluate_connection_candidate(
    nodes: &[Node],
    position: Position,
    connection_radius: f64,
    from_node_id: &str,
    from_handle_id: Option<&str>,
    from_handle_type: HandleType,
    connection_mode: ConnectionMode,
    is_valid_connection: Option<crate::types::IsValidConnection>,
    extra_validator: Option<&dyn Fn(&Connection) -> bool>,
) -> CandidateEvaluation {
    let Some(closest) = get_closest_handle(
        position,
        connection_radius,
        nodes,
        from_node_id,
        from_handle_id,
        from_handle_type,
    ) else {
        return CandidateEvaluation {
            candidate: None,
            connection: None,
            is_valid: false,
        };
    };

    let connection = create_connection(
        from_node_id,
        from_handle_id,
        from_handle_type,
        &closest.node_id,
        &closest.handle,
    );

    // Locked (non-connectable) handles are reported as candidates for
    // visible-refusal styling, but can never form a valid connection.
    let is_valid = closest.handle.connectable
        && is_valid_handle_connection(
            from_handle_type,
            closest.handle.handle_type,
            connection_mode,
        )
        && crate::types::validate_connection(&connection, connection_mode, is_valid_connection)
        && extra_validator.map_or(true, |validate| validate(&connection));

    CandidateEvaluation {
        candidate: Some(closest),
        connection: Some(connection),
        is_valid,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HandleBounds;

    /// A node with one source handle on the right and one target handle on
    /// the left, both 8x8, measured (node-relative) like the Handle
    /// component would register them.
    fn node_with_handles(id: &str, x: f64, y: f64, connectable: bool) -> Node {
        let mut node = Node::new(id.to_string(), Position::new(x, y))
            .with_dimensions(100.0, 40.0);
        let mut bounds = HandleBounds::new();
        bounds.add_source(
            HandleBound::new(
                Some("out".to_string()),
                HandleType::Source,
                HandleBoundPosition::Right,
                96.0,
                16.0,
                8.0,
                8.0,
            )
            .with_connectable(connectable),
        );
        bounds.add_target(
            HandleBound::new(
                Some("in".to_string()),
                HandleType::Target,
                HandleBoundPosition::Left,
                -4.0,
                16.0,
                8.0,
                8.0,
            )
            .with_connectable(connectable),
        );
        node.internals.set_handle_bounds(bounds);
        node
    }

    fn nodes() -> Vec<Node> {
        vec![
            node_with_handles("n1", 0.0, 0.0, true),
            node_with_handles("n2", 300.0, 0.0, true),
        ]
    }

    /// Position on n2's target handle center (300 - 4 + 4, 16 + 4)
    fn over_n2_target() -> Position {
        Position::new(300.0, 20.0)
    }

    #[test]
    fn test_closest_handle_within_radius() {
        let nodes = nodes();
        let closest = get_closest_handle(
            over_n2_target(),
            CONNECTION_RADIUS,
            &nodes,
            "n1",
            Some("out"),
            HandleType::Source,
        )
        .expect("handle in range");
        assert_eq!(closest.node_id, "n2");
        assert_eq!(closest.handle.id.as_deref(), Some("in"));
        assert_eq!(closest.handle.handle_type, HandleType::Target);
    }

    #[test]
    fn test_closest_handle_out_of_radius_is_none() {
        let nodes = nodes();
        let far = Position::new(200.0, 200.0);
        assert!(get_closest_handle(
            far,
            CONNECTION_RADIUS,
            &nodes,
            "n1",
            Some("out"),
            HandleType::Source,
        )
        .is_none());
    }

    #[test]
    fn test_locked_handle_is_reported_but_never_valid() {
        let nodes = vec![
            node_with_handles("n1", 0.0, 0.0, true),
            node_with_handles("n2", 300.0, 0.0, false), // locked (anchors only)
        ];
        // Reported as candidate: refusal must be visible, not a silent no-op
        let eval = evaluate_connection_candidate(
            &nodes,
            over_n2_target(),
            CONNECTION_RADIUS,
            "n1",
            Some("out"),
            HandleType::Source,
            ConnectionMode::Strict,
            None,
            None,
        );
        let candidate = eval.candidate.expect("locked handle reported for styling");
        assert!(!candidate.handle.connectable);
        // ...but it can never complete
        assert!(!eval.is_valid);
    }

    /// n2 with its usual connectable handles plus a locked target handle
    /// just below the connectable "in" (rect y 26..34, center (300, 30)).
    fn nodes_with_extra_locked_target() -> Vec<Node> {
        let mut n2 = node_with_handles("n2", 300.0, 0.0, true);
        let bounds = n2.internals.handle_bounds.as_mut().unwrap();
        bounds.add_target(
            HandleBound::new(
                Some("locked_in".to_string()),
                HandleType::Target,
                HandleBoundPosition::Left,
                -4.0,
                26.0,
                8.0,
                8.0,
            )
            .with_connectable(false),
        );
        vec![node_with_handles("n1", 0.0, 0.0, true), n2]
    }

    #[test]
    fn test_locked_handle_never_shadows_connectable_one_by_proximity() {
        // Probe between the two handles, slightly CLOSER to the locked one
        // but not inside either rect ("in" rect ends at y=24, locked starts
        // at y=26): the connectable handle must still win the snap.
        let nodes = nodes_with_extra_locked_target();
        let closest = get_closest_handle(
            Position::new(300.0, 25.5),
            CONNECTION_RADIUS,
            &nodes,
            "n1",
            Some("out"),
            HandleType::Source,
        )
        .expect("handle in range");
        assert_eq!(closest.handle.id.as_deref(), Some("in"));
        assert!(closest.handle.connectable);
    }

    #[test]
    fn test_direct_hover_on_locked_handle_wins_over_nearby_connectable() {
        // Pointer physically INSIDE the locked handle's bounds: the locked
        // handle is the candidate (visible refusal), even though the
        // connectable "in" handle is within the snap radius.
        let nodes = nodes_with_extra_locked_target();
        let closest = get_closest_handle(
            Position::new(300.0, 30.0),
            CONNECTION_RADIUS,
            &nodes,
            "n1",
            Some("out"),
            HandleType::Source,
        )
        .expect("handle in range");
        assert_eq!(closest.handle.id.as_deref(), Some("locked_in"));
        assert!(!closest.handle.connectable);
    }

    #[test]
    fn test_evaluate_valid_source_to_target() {
        let nodes = nodes();
        let eval = evaluate_connection_candidate(
            &nodes,
            over_n2_target(),
            CONNECTION_RADIUS,
            "n1",
            Some("out"),
            HandleType::Source,
            ConnectionMode::Strict,
            None,
            None,
        );
        assert!(eval.is_valid);
        let connection = eval.connection.expect("connection for candidate");
        assert_eq!(connection.source, "n1");
        assert_eq!(connection.target, "n2");
        assert_eq!(connection.target_handle.as_deref(), Some("in"));
    }

    #[test]
    fn test_evaluate_strict_rejects_source_to_source_but_reports_candidate() {
        let nodes = nodes();
        // Over n2's SOURCE handle center: (300 + 96 + 4, 16 + 4)
        let over_n2_source = Position::new(400.0, 20.0);
        let eval = evaluate_connection_candidate(
            &nodes,
            over_n2_source,
            CONNECTION_RADIUS,
            "n1",
            Some("out"),
            HandleType::Source,
            ConnectionMode::Strict,
            None,
            None,
        );
        // Candidate still reported (for invalid-hover styling and snapping)…
        let candidate = eval.candidate.expect("candidate reported");
        assert_eq!(candidate.handle.handle_type, HandleType::Source);
        // …but the connection is not valid in strict mode
        assert!(!eval.is_valid);
    }

    #[test]
    fn test_evaluate_enforces_handle_level_validator() {
        fn reject_n2(connection: &Connection) -> bool {
            connection.target != "n2"
        }
        let nodes = nodes();
        let eval = evaluate_connection_candidate(
            &nodes,
            over_n2_target(),
            CONNECTION_RADIUS,
            "n1",
            Some("out"),
            HandleType::Source,
            ConnectionMode::Strict,
            Some(reject_n2),
            None,
        );
        assert!(eval.candidate.is_some());
        assert!(!eval.is_valid);
    }

    #[test]
    fn test_evaluate_enforces_host_level_validator() {
        let nodes = nodes();
        let reject_all = |_: &Connection| false;
        let eval = evaluate_connection_candidate(
            &nodes,
            over_n2_target(),
            CONNECTION_RADIUS,
            "n1",
            Some("out"),
            HandleType::Source,
            ConnectionMode::Strict,
            None,
            Some(&reject_all),
        );
        assert!(eval.candidate.is_some());
        assert!(!eval.is_valid);
    }
}

