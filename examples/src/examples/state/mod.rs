//! State management examples
//!
//! Examples demonstrating state management:
//! - SaveRestore: Serialize and deserialize flow state
//! - UseNodesData: Reactively access node data
//! - SetNodesBatching: Batch multiple node updates efficiently
//! - ReactiveStores: Integration with Leptos reactive_stores (Redux-like)
//! - Middlewares: Custom middleware/hooks for state operations

mod middlewares;
mod reactive_stores;
mod save_restore;
mod set_nodes_batching;
mod use_nodes_data;

pub use middlewares::MiddlewaresExample;
pub use reactive_stores::ReactiveStoresExample;
pub use save_restore::SaveRestoreExample;
pub use set_nodes_batching::SetNodesBatchingExample;
pub use use_nodes_data::UseNodesDataExample;
