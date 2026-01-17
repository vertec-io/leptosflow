//! Connection types and validation

use serde::{Deserialize, Serialize};

/// Connection mode determines how handles can be connected
///
/// This matches React Flow's ConnectionMode enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionMode {
    /// Strict mode only allows source-to-target connections (default)
    ///
    /// - Source handles can only connect to Target handles
    /// - Target handles can only connect to Source handles
    /// - No source-to-source or target-to-target connections
    Strict,
    
    /// Loose mode allows more flexible connections
    ///
    /// - Source can connect to Source
    /// - Target can connect to Target
    /// - Still prevents connecting a handle to itself
    Loose,
}

impl Default for ConnectionMode {
    fn default() -> Self {
        ConnectionMode::Strict
    }
}

/// Represents a connection between two handles
///
/// This is used during connection creation and validation
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    /// ID of the source node
    pub source: String,
    
    /// ID of the source handle (optional if node has only one source handle)
    pub source_handle: Option<String>,
    
    /// ID of the target node
    pub target: String,
    
    /// ID of the target handle (optional if node has only one target handle)
    pub target_handle: Option<String>,
}

impl Connection {
    /// Create a new connection
    pub fn new(
        source: String,
        target: String,
        source_handle: Option<String>,
        target_handle: Option<String>,
    ) -> Self {
        Connection {
            source,
            target,
            source_handle,
            target_handle,
        }
    }
    
    /// Check if this connection is a self-loop (same node)
    pub fn is_self_loop(&self) -> bool {
        self.source == self.target
    }
    
    /// Check if this connection connects the same handle to itself
    pub fn is_same_handle(&self) -> bool {
        self.source == self.target && self.source_handle == self.target_handle
    }
}

/// Type alias for connection validation callback
///
/// Returns true if the connection is valid, false otherwise
pub type IsValidConnection = fn(&Connection) -> bool;

/// Default validation that always returns true
pub fn always_valid(_connection: &Connection) -> bool {
    true
}

/// Validate a connection based on connection mode and custom validation
///
/// This is the core validation logic ported from React Flow's XYHandle
pub fn validate_connection(
    connection: &Connection,
    connection_mode: ConnectionMode,
    is_valid_connection: Option<IsValidConnection>,
) -> bool {
    // Check if connecting handle to itself
    if connection.is_same_handle() {
        return false;
    }
    
    // In strict mode, prevent same-node connections
    // In loose mode, allow them (but not same handle)
    if connection_mode == ConnectionMode::Strict && connection.is_self_loop() {
        return false;
    }
    
    // Apply custom validation if provided
    if let Some(validator) = is_valid_connection {
        if !validator(connection) {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_creation() {
        let conn = Connection::new(
            "node1".to_string(),
            "node2".to_string(),
            Some("out1".to_string()),
            Some("in1".to_string()),
        );
        
        assert_eq!(conn.source, "node1");
        assert_eq!(conn.target, "node2");
        assert_eq!(conn.source_handle, Some("out1".to_string()));
        assert_eq!(conn.target_handle, Some("in1".to_string()));
    }

    #[test]
    fn test_self_loop_detection() {
        let conn = Connection::new(
            "node1".to_string(),
            "node1".to_string(),
            Some("out1".to_string()),
            Some("in1".to_string()),
        );
        
        assert!(conn.is_self_loop());
    }

    #[test]
    fn test_same_handle_detection() {
        let conn = Connection::new(
            "node1".to_string(),
            "node1".to_string(),
            Some("handle1".to_string()),
            Some("handle1".to_string()),
        );

        assert!(conn.is_same_handle());
    }

    #[test]
    fn test_validate_strict_mode_prevents_self_loop() {
        let conn = Connection::new(
            "node1".to_string(),
            "node1".to_string(),
            Some("out1".to_string()),
            Some("in1".to_string()),
        );

        assert!(!validate_connection(&conn, ConnectionMode::Strict, None));
    }

    #[test]
    fn test_validate_loose_mode_allows_self_loop() {
        let conn = Connection::new(
            "node1".to_string(),
            "node1".to_string(),
            Some("out1".to_string()),
            Some("in1".to_string()),
        );

        assert!(validate_connection(&conn, ConnectionMode::Loose, None));
    }

    #[test]
    fn test_validate_prevents_same_handle() {
        let conn = Connection::new(
            "node1".to_string(),
            "node1".to_string(),
            Some("handle1".to_string()),
            Some("handle1".to_string()),
        );

        // Both modes should prevent same handle connection
        assert!(!validate_connection(&conn, ConnectionMode::Strict, None));
        assert!(!validate_connection(&conn, ConnectionMode::Loose, None));
    }

    #[test]
    fn test_validate_with_custom_validator() {
        let conn = Connection::new(
            "node1".to_string(),
            "node2".to_string(),
            None,
            None,
        );

        // Custom validator that only allows connections to "node2"
        fn only_node2(connection: &Connection) -> bool {
            connection.target == "node2"
        }

        assert!(validate_connection(&conn, ConnectionMode::Strict, Some(only_node2)));

        let invalid_conn = Connection::new(
            "node1".to_string(),
            "node3".to_string(),
            None,
            None,
        );

        assert!(!validate_connection(&invalid_conn, ConnectionMode::Strict, Some(only_node2)));
    }
}

