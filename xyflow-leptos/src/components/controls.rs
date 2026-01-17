//! Controls component for zoom and viewport controls

use leptos::prelude::*;
use leptos::ev;
use crate::components::panel::{Panel, PanelPosition};
use crate::events::zoom::use_zoom_controls;
use crate::hooks::use_flow_store;

/// Orientation of the controls panel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlsOrientation {
    /// Vertical layout (default)
    Vertical,
    /// Horizontal layout
    Horizontal,
}

impl Default for ControlsOrientation {
    fn default() -> Self {
        ControlsOrientation::Vertical
    }
}

/// Controls component that renders zoom and viewport control buttons
///
/// The Controls component provides convenient buttons to zoom in, zoom out,
/// fit the view, and lock/unlock the viewport.
///
/// # Example
///
/// ```ignore
/// use xyflow_leptos::{SvelteFlow, Controls};
///
/// view! {
///     <SvelteFlow nodes edges>
///         <Controls />
///     </SvelteFlow>
/// }
/// ```
#[component]
pub fn Controls(
    /// Position of the controls panel
    #[prop(default = PanelPosition::BottomLeft)]
    position: PanelPosition,
    
    /// Orientation of the controls
    #[prop(default = ControlsOrientation::Vertical)]
    orientation: ControlsOrientation,
    
    /// Show zoom in/out buttons
    #[prop(default = true)]
    show_zoom: bool,
    
    /// Show fit view button
    #[prop(default = true)]
    show_fit_view: bool,
    
    /// Show lock/unlock button
    #[prop(default = true)]
    show_interactive: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<String>,
    
    /// Inline styles
    #[prop(optional)]
    style: Option<String>,
    
    /// ARIA label for the controls
    #[prop(default = "Flow controls".to_string())]
    aria_label: String,
    
    /// Child components (custom buttons)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let store = use_flow_store();
    let (zoom_in, zoom_out, _zoom_to) = use_zoom_controls();
    
    // Get current state for button states
    let min_zoom_reached = move || {
        let viewport = store.get_viewport();
        let min_zoom = store.state.min_zoom.get();
        viewport.zoom <= min_zoom
    };

    let max_zoom_reached = move || {
        let viewport = store.get_viewport();
        let max_zoom = store.state.max_zoom.get();
        viewport.zoom >= max_zoom
    };

    // TODO: Add interactivity state to FlowState
    let is_interactive = RwSignal::new(true);
    
    let orientation_class = match orientation {
        ControlsOrientation::Horizontal => "horizontal",
        ControlsOrientation::Vertical => "vertical",
    };
    
    let classes = move || {
        let mut result = format!("xyflow__controls leptos-flow__controls {}", orientation_class);
        if let Some(ref c) = class {
            result.push(' ');
            result.push_str(c);
        }
        result
    };

    view! {
        <Panel position=position class=classes() style=style.unwrap_or_default()>
            <div
                class="xyflow__controls-inner"
                aria-label=aria_label
                role="group"
            >
                {move || show_zoom.then(|| {
                    let zoom_in_clone = zoom_in.clone();
                    let zoom_out_clone = zoom_out.clone();
                    view! {
                        <>
                            <ControlButton
                                on_click=move |_| zoom_in_clone()
                                disabled=max_zoom_reached()
                                title="Zoom in".to_string()
                                aria_label="Zoom in".to_string()
                                class="xyflow__controls-zoomin".to_string()
                            >
                                <PlusIcon />
                            </ControlButton>
                            <ControlButton
                                on_click=move |_| zoom_out_clone()
                                disabled=min_zoom_reached()
                                title="Zoom out".to_string()
                                aria_label="Zoom out".to_string()
                                class="xyflow__controls-zoomout".to_string()
                            >
                                <MinusIcon />
                            </ControlButton>
                        </>
                    }
                })}

                {move || show_fit_view.then(|| {
                    let store_clone = store;
                    view! {
                        <ControlButton
                            on_click=move |_| {
                                // TODO: Implement fit_view in store
                                // For now, just reset to default zoom
                                let mut viewport = store_clone.get_viewport();
                                viewport.zoom = 1.0;
                                viewport.x = 0.0;
                                viewport.y = 0.0;
                                store_clone.set_viewport(viewport);
                            }
                            disabled=false
                            title="Fit view".to_string()
                            aria_label="Fit view".to_string()
                            class="xyflow__controls-fitview".to_string()
                        >
                            <FitViewIcon />
                        </ControlButton>
                    }
                })}

                {move || show_interactive.then(|| {
                    let interactive = is_interactive.get();
                    let title_text = if interactive { "Lock".to_string() } else { "Unlock".to_string() };
                    let aria_text = if interactive { "Lock viewport".to_string() } else { "Unlock viewport".to_string() };

                    view! {
                        <ControlButton
                            on_click=move |_| {
                                is_interactive.update(|v| *v = !*v);
                            }
                            disabled=false
                            title=title_text
                            aria_label=aria_text
                            class="xyflow__controls-interactive".to_string()
                        >
                            {if interactive {
                                view! { <UnlockIcon /> }.into_any()
                            } else {
                                view! { <LockIcon /> }.into_any()
                            }}
                        </ControlButton>
                    }
                })}

                {children.map(|children| children())}
            </div>
        </Panel>
    }
}

/// Control button component
#[component]
fn ControlButton(
    /// Click handler
    on_click: impl Fn(ev::MouseEvent) + 'static,

    /// Whether the button is disabled
    #[prop(default = false)]
    disabled: bool,

    /// Button title (tooltip)
    title: String,

    /// ARIA label
    aria_label: String,

    /// Additional CSS classes
    class: String,

    /// Child components (icon)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let classes = format!("xyflow__controls-button {}", class);

    view! {
        <button
            type="button"
            class=classes
            on:click=on_click
            disabled=disabled
            title=title
            aria-label=aria_label
        >
            {children.map(|children| children())}
        </button>
    }
}

// SVG Icons

#[component]
fn PlusIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
            <path d="M32 18.133H18.133V32h-4.266V18.133H0v-4.266h13.867V0h4.266v13.867H32z" />
        </svg>
    }
}

#[component]
fn MinusIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 5">
            <path d="M0 0h32v4.2H0z" />
        </svg>
    }
}

#[component]
fn FitViewIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 30">
            <path d="M3.692 4.63c0-.53.4-.938.939-.938h5.215V0H4.708C2.13 0 0 2.054 0 4.63v5.216h3.692V4.631zM27.354 0h-5.2v3.692h5.17c.53 0 .984.4.984.939v5.215H32V4.631A4.624 4.624 0 0027.354 0zm.954 24.83c0 .532-.4.94-.939.94h-5.215v3.768h5.215c2.577 0 4.631-2.13 4.631-4.707v-5.139h-3.692v5.139zm-23.677.94c-.531 0-.939-.4-.939-.94v-5.138H0v5.139c0 2.577 2.13 4.707 4.708 4.707h5.138V25.77H4.631z" />
        </svg>
    }
}

#[component]
fn LockIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 25 32">
            <path d="M21.333 10.667H19.81V7.619C19.81 3.429 16.38 0 12.19 0 8 0 4.571 3.429 4.571 7.619v3.048H3.048A3.056 3.056 0 000 13.714v15.238A3.056 3.056 0 003.048 32h18.285a3.056 3.056 0 003.048-3.048V13.714a3.056 3.056 0 00-3.048-3.047zM12.19 24.533a3.056 3.056 0 01-3.047-3.047 3.056 3.056 0 013.047-3.048 3.056 3.056 0 013.048 3.048 3.056 3.056 0 01-3.048 3.047zm4.724-13.866H7.467V7.619c0-2.59 2.133-4.724 4.723-4.724 2.591 0 4.724 2.133 4.724 4.724v3.048z" />
        </svg>
    }
}

#[component]
fn UnlockIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 25 32">
            <path d="M21.333 10.667H19.81V7.619C19.81 3.429 16.38 0 12.19 0c-4.114 1.828-1.37 2.133.305 2.438 1.676.305 4.42 2.59 4.42 5.181v3.048H3.047A3.056 3.056 0 000 13.714v15.238A3.056 3.056 0 003.048 32h18.285a3.056 3.056 0 003.048-3.048V13.714a3.056 3.056 0 00-3.048-3.047zM12.19 24.533a3.056 3.056 0 01-3.047-3.047 3.056 3.056 0 013.047-3.048 3.056 3.056 0 013.048 3.048 3.056 3.056 0 01-3.048 3.047z" />
        </svg>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_controls_component_exists() {
        // Placeholder test - real tests need browser environment
        assert!(true);
    }
}

