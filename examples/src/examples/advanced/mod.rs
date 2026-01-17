//! Advanced examples
//!
//! Examples demonstrating advanced patterns:
//! - Figma: Figma-like selection and interaction
//! - Undirectional: Restricting connection direction
//! - Subflow: Nested flow graphs
//! - MultiFlows: Multiple independent flow instances
//! - Provider: FlowStore context pattern
//! - A11y: Accessibility features for keyboard and screen reader users
//! - Stress: Performance with many nodes

mod a11y;
mod figma;
mod multi_flows;
mod provider;
mod stress;
mod subflow;
mod undirectional;

pub use a11y::A11yExample;
pub use figma::FigmaExample;
pub use multi_flows::MultiFlowsExample;
pub use provider::ProviderExample;
pub use stress::StressExample;
pub use subflow::SubflowExample;
pub use undirectional::UndirectionalExample;
