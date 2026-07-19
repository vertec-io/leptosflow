//! SVG marker definitions for edge arrow heads

use leptos::prelude::*;

/// Marker type for edge arrow heads
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkerType {
    /// Open arrow (stroke only)
    Arrow,
    /// Closed arrow (filled)
    ArrowClosed,
}

/// Props for a single marker
#[derive(Clone, Debug, PartialEq)]
pub struct MarkerProps {
    /// Unique marker ID
    pub id: String,
    /// Marker type
    pub marker_type: MarkerType,
    /// Marker color (defaults to edge stroke color)
    pub color: Option<String>,
    /// Marker width
    pub width: f64,
    /// Marker height
    pub height: f64,
    /// Marker units
    pub marker_units: String,
    /// Stroke width
    pub stroke_width: f64,
    /// Orient attribute
    pub orient: String,
}

impl Default for MarkerProps {
    fn default() -> Self {
        Self {
            id: String::new(),
            marker_type: MarkerType::ArrowClosed,
            color: None,
            width: 12.5,
            height: 12.5,
            marker_units: "strokeWidth".to_string(),
            stroke_width: 1.0,
            orient: "auto-start-reverse".to_string(),
        }
    }
}

/// Renders a single SVG marker
///
/// Colors are handled via CSS using the `.xyflow__arrowhead` class.
/// The CSS uses `--xy-edge-stroke` variable for consistent edge styling.
#[component]
fn MarkerComponent(
    /// Marker ID
    id: String,
    /// Marker type
    #[prop(default = MarkerType::ArrowClosed)]
    marker_type: MarkerType,
    /// Marker width
    #[prop(default = 12.5)]
    width: f64,
    /// Marker height
    #[prop(default = 12.5)]
    height: f64,
    /// Marker units
    #[prop(default = "strokeWidth".to_string())]
    marker_units: String,
    /// Stroke width
    #[prop(default = 1.0)]
    stroke_width: f64,
    /// Orient attribute
    #[prop(default = "auto-start-reverse".to_string())]
    orient: String,
) -> impl IntoView {
    view! {
        // NOTE: plain attribute names, NOT `attr:` prefixed — on plain
        // elements the view! macro emits `attr:foo` as a literal attribute
        // named "attr:foo" (the prefix is component-spread syntax), which
        // silently breaks SVG presentation attributes.
        <marker
            class="xyflow__arrowhead"
            id=id
            markerWidth=format!("{}", width)
            markerHeight=format!("{}", height)
            viewBox="-10 -10 20 20"
            markerUnits=marker_units
            orient=orient
            refX="0"
            refY="0"
        >
            {match marker_type {
                // Colors are styled via CSS (.xyflow__arrowhead polyline)
                // `context-stroke` paints the arrow with the stroke color of
                // the edge path referencing the marker, so per-edge colors
                // (CSS vars / classes) carry through to the arrowhead.
                MarkerType::Arrow => view! {
                    <polyline
                        class="arrow"
                        fill="none"
                        stroke="context-stroke"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width=format!("{}", stroke_width)
                        points="-5,-4 0,0 -5,4"
                    />
                }.into_any(),
                MarkerType::ArrowClosed => view! {
                    <polyline
                        class="arrowclosed"
                        fill="context-stroke"
                        stroke="context-stroke"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width=format!("{}", stroke_width)
                        points="-5,-4 0,0 -5,4 -5,-4"
                    />
                }.into_any(),
            }}
        </marker>
    }
}

/// Renders all marker definitions for edges
///
/// Marker colors are styled via CSS using the `--xy-edge-stroke` variable.
#[component]
pub fn MarkerDefinitions() -> impl IntoView {
    // Create default markers with CSS-based styling
    let default_marker_id = "xyflow__arrowclosed";

    view! {
        <svg class="xyflow__marker" aria-hidden="true" style="position: absolute; width: 0; height: 0;">
            <defs>
                <MarkerComponent
                    id=default_marker_id.to_string()
                    marker_type=MarkerType::ArrowClosed
                />
            </defs>
        </svg>
    }
}

