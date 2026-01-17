//! Panel component for positioning UI elements on the flow

use leptos::prelude::*;

/// Position of a panel on the flow viewport
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
    /// Center left
    CenterLeft,
    /// Center right
    CenterRight,
}

impl Default for PanelPosition {
    fn default() -> Self {
        PanelPosition::TopLeft
    }
}

impl PanelPosition {
    /// Get CSS class names for this position
    pub fn class_names(&self) -> Vec<&'static str> {
        match self {
            PanelPosition::TopLeft => vec!["top", "left"],
            PanelPosition::TopCenter => vec!["top", "center"],
            PanelPosition::TopRight => vec!["top", "right"],
            PanelPosition::BottomLeft => vec!["bottom", "left"],
            PanelPosition::BottomCenter => vec!["bottom", "center"],
            PanelPosition::BottomRight => vec!["bottom", "right"],
            PanelPosition::CenterLeft => vec!["center", "left"],
            PanelPosition::CenterRight => vec!["center", "right"],
        }
    }
}

/// Panel component for positioning UI elements on the flow
///
/// The Panel component helps position content above the viewport.
/// It is used by Controls, MiniMap, and other overlay components.
///
/// # Example
///
/// ```ignore
/// use xyflow_leptos::{Panel, PanelPosition};
///
/// view! {
///     <Panel position=PanelPosition::TopLeft>
///         "Custom content"
///     </Panel>
/// }
/// ```
#[component]
pub fn Panel(
    /// Position of the panel
    #[prop(default = PanelPosition::TopLeft)]
    position: PanelPosition,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
    
    /// Child components
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let position_classes = position.class_names().join(" ");
    let classes = move || {
        let mut result = format!("xyflow__panel leptos-flow__panel {}", position_classes);
        if let Some(ref c) = class {
            result.push(' ');
            result.push_str(c);
        }
        result
    };

    view! {
        <div
            class=classes
            style=move || style.clone().unwrap_or_default()
        >
            {children.map(|children| children())}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_position_classes() {
        assert_eq!(PanelPosition::TopLeft.class_names(), vec!["top", "left"]);
        assert_eq!(PanelPosition::BottomRight.class_names(), vec!["bottom", "right"]);
        assert_eq!(PanelPosition::TopCenter.class_names(), vec!["top", "center"]);
    }

    #[test]
    fn test_panel_position_default() {
        assert_eq!(PanelPosition::default(), PanelPosition::TopLeft);
    }
}

