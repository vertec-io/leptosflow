//! Background component for rendering grid patterns

use leptos::prelude::*;
use crate::hooks::use_flow_store;

/// Background pattern variant
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundVariant {
    /// Dot pattern
    Dots,
    /// Line grid pattern
    Lines,
    /// Cross pattern
    Cross,
}

impl Default for BackgroundVariant {
    fn default() -> Self {
        BackgroundVariant::Dots
    }
}

/// Background component that renders a pattern behind the flow
///
/// The background component renders an SVG pattern that scales with the viewport zoom.
/// It supports three variants: dots, lines, and cross.
///
/// # Example
///
/// ```ignore
/// use xyflow_leptos::{SvelteFlow, Background, BackgroundVariant};
///
/// view! {
///     <SvelteFlow nodes edges>
///         <Background variant=BackgroundVariant::Dots gap=20.0 />
///     </SvelteFlow>
/// }
/// ```
#[component]
pub fn Background(
    /// Unique ID for the pattern (useful when using multiple backgrounds)
    #[prop(optional)]
    id: Option<String>,
    
    /// Pattern variant
    #[prop(default = BackgroundVariant::Dots)]
    variant: BackgroundVariant,
    
    /// Gap between pattern elements
    #[prop(default = 20.0)]
    gap: f64,
    
    /// Size of pattern elements (dots radius or cross size)
    #[prop(optional)]
    size: Option<f64>,
    
    /// Line width for lines and cross patterns
    #[prop(default = 1.0)]
    line_width: f64,
    
    /// Background color
    #[prop(optional)]
    bg_color: Option<String>,
    
    /// Pattern color
    #[prop(optional)]
    pattern_color: Option<String>,
    
    /// Custom CSS class
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let store = use_flow_store();
    
    // Default sizes for each variant
    let default_size = match variant {
        BackgroundVariant::Dots => 1.0,
        BackgroundVariant::Lines => 1.0,
        BackgroundVariant::Cross => 6.0,
    };
    
    let pattern_size = size.unwrap_or(default_size);
    
    // Generate unique pattern ID (not a closure, just a string)
    let pattern_id = format!("background-pattern-{}", id.clone().unwrap_or_else(|| "default".to_string()));
    
    // Reactive calculations based on viewport
    let pattern_props = move || {
        let viewport = store.get_viewport();
        let zoom = viewport.zoom;
        
        let scaled_gap = gap * zoom;
        let scaled_size = pattern_size * zoom;
        
        let is_dots = variant == BackgroundVariant::Dots;
        let is_cross = variant == BackgroundVariant::Cross;
        
        let pattern_dimensions = if is_cross {
            (scaled_size, scaled_size)
        } else {
            (scaled_gap, scaled_gap)
        };
        
        let pattern_offset = if is_dots {
            (scaled_size / 2.0, scaled_size / 2.0)
        } else {
            (pattern_dimensions.0 / 2.0, pattern_dimensions.1 / 2.0)
        };
        
        (viewport.x, viewport.y, scaled_gap, scaled_size, pattern_dimensions, pattern_offset)
    };
    
    let classes = format!(
        "xyflow__background xyflow__container {}",
        class.unwrap_or_default()
    );
    
    view! {
        <svg
            class=classes
            style:--xy-background-color-props=bg_color.clone()
            style:--xy-background-pattern-color-props=pattern_color.clone()
        >
            <defs>
                <pattern
                    id=pattern_id.clone()
                    x=move || pattern_props().0 % pattern_props().2
                    y=move || pattern_props().1 % pattern_props().2
                    width=move || pattern_props().2
                    height=move || pattern_props().2
                    patternUnits="userSpaceOnUse"
                    patternTransform=move || {
                        let offset = pattern_props().5;
                        format!("translate(-{}, -{})", offset.0, offset.1)
                    }
                >
                    {move || {
                        match variant {
                            BackgroundVariant::Dots => {
                                let radius = pattern_props().3 / 2.0;
                                leptos::either::Either::Left(view! {
                                    <circle
                                        cx=radius
                                        cy=radius
                                        r=radius
                                        class="xyflow__background-pattern dots"
                                    />
                                })
                            },
                            _ => {
                                let dims = pattern_props().4;
                                let variant_class = if variant == BackgroundVariant::Cross { "cross" } else { "lines" };
                                leptos::either::Either::Right(view! {
                                    <path
                                        stroke-width=line_width
                                        d=format!("M{} 0 V{} M0 {} H{}", dims.0 / 2.0, dims.1, dims.1 / 2.0, dims.0)
                                        class=format!("xyflow__background-pattern {}", variant_class)
                                    />
                                })
                            }
                        }
                    }}
                </pattern>
            </defs>
            <rect x="0" y="0" width="100%" height="100%" fill=format!("url(#{})", pattern_id.clone()) />
        </svg>
    }
}

