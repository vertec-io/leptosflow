//! Edge Types Example
//!
//! Demonstrates different edge styles: bezier, step, and straight.

use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

/// Edge types example showing bezier, step, and straight edges
#[component]
pub fn EdgeTypesExample() -> impl IntoView {
    // Create nodes for edge type demonstration
    let nodes = RwSignal::new(vec![
        Node::new("1".to_string(), Position::new(0.0, 0.0))
            .with_data(json!({"label": "Bezier Edge", "type": "input", "class": "light"})),
        Node::new("2".to_string(), Position::new(250.0, 0.0))
            .with_data(json!({"label": "Step Edge", "type": "input", "class": "light"})),
        Node::new("3".to_string(), Position::new(500.0, 0.0))
            .with_data(json!({"label": "Straight Edge", "type": "input", "class": "light"})),
        Node::new("4".to_string(), Position::new(0.0, 150.0))
            .with_data(json!({"label": "Target 1", "type": "output", "class": "light"})),
        Node::new("5".to_string(), Position::new(250.0, 150.0))
            .with_data(json!({"label": "Target 2", "type": "output", "class": "light"})),
        Node::new("6".to_string(), Position::new(500.0, 150.0))
            .with_data(json!({"label": "Target 3", "type": "output", "class": "light"})),
    ]);

    let edges = RwSignal::new(vec![
        Edge::new("e1-4".to_string(), "1".to_string(), "4".to_string())
            .with_type("bezier".to_string()),
        Edge::new("e2-5".to_string(), "2".to_string(), "5".to_string())
            .with_type("step".to_string()),
        Edge::new("e3-6".to_string(), "3".to_string(), "6".to_string())
            .with_type("straight".to_string()),
    ]);

    view! {
        <div class="example-container">
            <SvelteFlow nodes=nodes edges=edges>
                <Background variant=BackgroundVariant::Dots />
                <Controls position=PanelPosition::BottomLeft />
                <MiniMap position=PanelPosition::BottomRight />
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 10px; border-radius: 4px;">
                        <strong>"Edge Types Demo"</strong>
                        <p style="margin: 5px 0; font-size: 12px;">"Bezier, Step, and Straight edges"</p>
                    </div>
                </Panel>
            </SvelteFlow>
        </div>
    }
}
