//! Node examples
//!
//! Examples demonstrating node customization:
//! - CustomNodes: User-defined node components with colors
//! - DefaultNodeOverwrite: Customize the default node component
//! - NodeResizer: Resizable nodes with handles
//! - DragHandle: Limit drag area to specific region
//! - MovingHandles: Handles that change position dynamically
//! - DetachedHandle: Handles positioned outside node body
//! - NodeTypeChange: Dynamically change node type at runtime
//! - NodeTypesObjectChange: Dynamically change node type definitions
//! - UpdateNode: Update node properties programmatically
//! - UseUpdateNodeInternals: Force re-measurement of node internals
//! - BrokenNodes: Graceful handling of nodes without proper handles
//! - NodeToolbar: Context toolbar on nodes
//! - UseNodesInit: Lifecycle hook for when nodes are initialized

mod custom_node;
mod default_node_overwrite;
mod node_resizer;
mod drag_handle;
mod moving_handles;
mod detached_handle;
mod node_type_change;
mod node_types_object_change;
mod update_node;
mod use_update_node_internals;
mod broken_nodes;
mod node_toolbar;
mod use_nodes_init;

pub use custom_node::CustomNodesExample;
pub use default_node_overwrite::DefaultNodeOverwriteExample;
pub use node_resizer::NodeResizerExample;
pub use drag_handle::DragHandleExample;
pub use moving_handles::MovingHandlesExample;
pub use detached_handle::DetachedHandleExample;
pub use node_type_change::NodeTypeChangeExample;
pub use node_types_object_change::NodeTypesObjectChangeExample;
pub use update_node::UpdateNodeExample;
pub use use_update_node_internals::UseUpdateNodeInternalsExample;
pub use broken_nodes::BrokenNodesExample;
pub use node_toolbar::NodeToolbarExample;
pub use use_nodes_init::UseNodesInitExample;

