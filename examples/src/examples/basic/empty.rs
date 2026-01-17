//! Empty Example
//!
//! A minimal starting point with an empty flow canvas.

use leptos::prelude::*;
use xyflow_leptos::*;
use crate::shared::SourceCodeViewer;

/// Empty canvas example
#[component]
pub fn EmptyExample() -> impl IntoView {
    let nodes = RwSignal::new(vec![]);
    let edges = RwSignal::new(vec![]);

    view! {
        <div class="example-container">
            <SvelteFlow nodes=nodes edges=edges>
                <Background variant=BackgroundVariant::Dots />
                <Controls position=PanelPosition::BottomLeft />
                <Panel position=PanelPosition::TopCenter>
                    <div style="background: white; padding: 10px; border-radius: 4px; text-align: center;">
                        <strong>"Empty Canvas"</strong>
                        <p style="margin: 5px 0; font-size: 12px;">"Start with a clean slate"</p>
                    </div>
                </Panel>
            </SvelteFlow>
            <SourceCodeViewer
                source=include_str!("empty.rs")
                title="empty.rs"
            />
        </div>
    }
}
