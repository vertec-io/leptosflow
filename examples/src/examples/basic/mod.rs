//! Basic examples
//!
//! Foundational examples demonstrating core XYFlow functionality:
//! - Basic: Draggable nodes, pan/zoom, background, minimap, controls
//! - Empty: Minimal starting point with empty canvas
//! - DefaultNodes: Input, default, and output node types
//! - Switch: Swap between different flow configurations at runtime

mod basic;
mod empty;
mod default_nodes;
mod switch;

pub use basic::BasicExample;
pub use empty::EmptyExample;
pub use default_nodes::DefaultNodesExample;
pub use switch::SwitchExample;
