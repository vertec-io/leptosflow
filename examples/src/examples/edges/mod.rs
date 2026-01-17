//! Edge examples
//!
//! Examples demonstrating edge customization:
//! - EdgeTypes: Bezier, step, and straight edge styles
//! - DefaultEdgeOverwrite: Customize the default edge component
//! - CustomEdges: Fully custom edge components
//! - CustomConnectionLine: Customize connection preview line
//! - FloatingEdges: Edges that connect to nodes dynamically
//! - EasyConnect: Click-based connection creation
//! - EdgeRenderer: Custom edge layer rendering with z-index
//! - EdgeToolbar: Toolbars on edges
//! - EdgeRouting: Advanced edge routing with obstacle avoidance

mod edge_types;
mod default_edge_overwrite;
mod custom_edges;
mod custom_connection_line;
mod floating_edges;
mod easy_connect;
mod edge_renderer;
mod edge_toolbar;
mod edge_routing;

pub use edge_types::EdgeTypesExample;
pub use default_edge_overwrite::DefaultEdgeOverwriteExample;
pub use custom_edges::CustomEdgesExample;
pub use custom_connection_line::CustomConnectionLineExample;
pub use floating_edges::FloatingEdgesExample;
pub use easy_connect::EasyConnectExample;
pub use edge_renderer::EdgeRendererExample;
pub use edge_toolbar::EdgeToolbarExample;
pub use edge_routing::EdgeRoutingExample;
