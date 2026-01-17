//! Backgrounds Example
//!
//! Demonstrates different background patterns: dots, lines, and cross.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

/// Backgrounds example with pattern switching
#[component]
pub fn BackgroundsExample() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Node 1", "type": "default", "class": "light"})),
        Node::new("2".to_string(), Position::new(300.0, 100.0))
            .with_data(json!({"label": "Node 2", "type": "default", "class": "light"})),
    ]);

    let edges = RwSignal::new(vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
    ]);

    let bg_variant = RwSignal::new(BackgroundVariant::Dots);

    let set_dots = move |_| bg_variant.set(BackgroundVariant::Dots);
    let set_lines = move |_| bg_variant.set(BackgroundVariant::Lines);
    let set_cross = move |_| bg_variant.set(BackgroundVariant::Cross);

    view! {
        <div class="example-container">
            <SvelteFlow nodes=nodes edges=edges>
                {move || view! { <Background variant=bg_variant.get() gap=25.0 /> }}
                <Controls position=PanelPosition::BottomLeft />
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 10px; border-radius: 4px;">
                        <strong>"Background Patterns"</strong>
                        <div style="margin-top: 8px; display: flex; gap: 4px;">
                            <button on:click=set_dots>"Dots"</button>
                            <button on:click=set_lines>"Lines"</button>
                            <button on:click=set_cross>"Cross"</button>
                        </div>
                    </div>
                </Panel>
            </SvelteFlow>
        </div>
    }
}
