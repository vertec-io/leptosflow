//! MiniMap component for showing a miniature overview of the flow

use leptos::prelude::*;
use crate::hooks::use_flow_store;
use crate::components::PanelPosition;

/// MiniMap component that shows a miniature overview of the flow
///
/// This component renders a small map showing all nodes and the current viewport position.
#[component]
pub fn MiniMap(
    /// Position of the minimap
    #[prop(default = PanelPosition::BottomRight)]
    position: PanelPosition,
    /// Width of the minimap in pixels
    #[prop(default = 200)]
    width: u32,
    /// Height of the minimap in pixels
    #[prop(default = 150)]
    height: u32,
    /// Whether the minimap is pannable
    #[prop(default = false)]
    pannable: bool,
    /// Whether the minimap is zoomable
    #[prop(default = false)]
    zoomable: bool,
) -> impl IntoView {
    let store = use_flow_store();
    
    // Calculate bounds of all nodes
    let bounds = move || {
        let nodes = store.get_nodes();
        if nodes.is_empty() {
            return (0.0, 0.0, 500.0, 500.0);
        }
        
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        
        for node in &nodes {
            let x = node.position.x;
            let y = node.position.y;
            // Assume default node size of 150x40
            let node_width = node.width.unwrap_or(150.0);
            let node_height = node.height.unwrap_or(40.0);
            
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + node_width);
            max_y = max_y.max(y + node_height);
        }
        
        // Add padding
        let padding = 50.0;
        (min_x - padding, min_y - padding, max_x + padding, max_y + padding)
    };
    
    // Calculate scale to fit all nodes in minimap
    let scale = move || {
        let (min_x, min_y, max_x, max_y) = bounds();
        let content_width = max_x - min_x;
        let content_height = max_y - min_y;
        
        let scale_x = width as f64 / content_width;
        let scale_y = height as f64 / content_height;
        
        scale_x.min(scale_y).min(1.0)
    };
    
    // Position class
    let position_class = match position {
        PanelPosition::TopLeft => "top left",
        PanelPosition::TopRight => "top right",
        PanelPosition::TopCenter => "top center",
        PanelPosition::BottomLeft => "bottom left",
        PanelPosition::BottomRight => "bottom right",
        PanelPosition::BottomCenter => "bottom center",
        PanelPosition::CenterLeft => "center left",
        PanelPosition::CenterRight => "center right",
    };
    
    view! {
        <div
            class=format!("xyflow__minimap xyflow__panel {}", position_class)
            style=format!("width: {}px; height: {}px;", width, height)
        >
            <svg
                width=width
                height=height
                viewBox=move || {
                    let (min_x, min_y, max_x, max_y) = bounds();
                    format!("{} {} {} {}", min_x, min_y, max_x - min_x, max_y - min_y)
                }
            >
                // Render nodes as rectangles
                {move || {
                    store.get_nodes().into_iter().map(|node| {
                        let x = node.position.x;
                        let y = node.position.y;
                        let w = node.width.unwrap_or(150.0);
                        let h = node.height.unwrap_or(40.0);
                        
                        view! {
                            <rect
                                class="xyflow__minimap-node"
                                x=x
                                y=y
                                width=w
                                height=h
                                rx="3"
                            />
                        }
                    }).collect_view()
                }}
                
                // Render viewport mask
                {move || {
                    let viewport = store.get_viewport();
                    let (min_x, min_y, max_x, max_y) = bounds();
                    let content_width = max_x - min_x;
                    let content_height = max_y - min_y;
                    
                    // Calculate visible area in flow coordinates
                    let visible_x = -viewport.x / viewport.zoom;
                    let visible_y = -viewport.y / viewport.zoom;
                    let visible_width = 800.0 / viewport.zoom; // Assume 800px viewport
                    let visible_height = 600.0 / viewport.zoom; // Assume 600px viewport
                    
                    view! {
                        <rect
                            class="xyflow__minimap-mask"
                            x=visible_x
                            y=visible_y
                            width=visible_width
                            height=visible_height
                            fill="none"
                            stroke="#555"
                            stroke-width="2"
                        />
                    }
                }}
            </svg>
        </div>
    }
}

