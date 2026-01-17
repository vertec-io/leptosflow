//! Connection examples
//!
//! Examples demonstrating connection handling:
//! - Validation: Connection validation rules
//! - UseConnection: Connection state hook
//! - CancelConnection: Cancel connections in progress
//! - ReconnectEdge: Reconnect existing edges to different handles
//! - AddNodeOnEdgeDrop: Create new node when dropping connection

mod validation;
mod use_connection;
mod cancel_connection;
mod reconnect_edge;
mod add_node_on_edge_drop;

pub use validation::ValidationExample;
pub use use_connection::UseConnectionExample;
pub use cancel_connection::CancelConnectionExample;
pub use reconnect_edge::ReconnectEdgeExample;
pub use add_node_on_edge_drop::AddNodeOnEdgeDropExample;
