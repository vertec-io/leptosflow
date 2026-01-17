//! Interaction examples
//!
//! Examples demonstrating user interactions:
//! - Interactions: Selection, multi-select, and deletion
//! - UseOnSelectionChange: React to selection changes
//! - UseNodeConnections: Get all connections for a node
//! - ClickDistance: Distinguish between clicks and drags
//! - TouchDevice: Touch-optimized interactions
//! - MultiSetNodes: Multiple disconnected node groups

mod interactions;
mod use_on_selection_change;
mod use_node_connections;
mod click_distance;
mod touch_device;
mod multi_set_nodes;

pub use interactions::InteractionsExample;
pub use use_on_selection_change::UseOnSelectionChangeExample;
pub use use_node_connections::UseNodeConnectionsExample;
pub use click_distance::ClickDistanceExample;
pub use touch_device::TouchDeviceExample;
pub use multi_set_nodes::MultiSetNodesExample;
