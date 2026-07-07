//! Hardening Example
//!
//! Exercises the library-native paths hardened for production use:
//!
//! - Custom node components with typed, colored port handles — handle
//!   anchors are measured automatically by the `Handle` component (no
//!   manual measurement effects in the consumer).
//! - Edges attach to specific (node_id, handle_id) port anchors with
//!   orientation-aware paths, and follow nodes while dragging.
//! - Node dragging via the library's pointer-capture hook
//!   (`use_node_drag_handlers`), with positions persisted through
//!   `FlowStore::set_on_node_drag_end`.
//! - Real fit-view: the Controls button and programmatic
//!   `store.fit_view()` / `store.zoom_to_selection()` calls.

use leptos::prelude::*;
use xyflow_leptos::events::use_node_drag_handlers;
use xyflow_leptos::*;

use crate::shared::SourceCodeViewer;

/// Example-local styling for the device nodes (library styling comes from
/// `xyflow_leptos::STYLES` / the host stylesheet).
const DEVICE_NODE_CSS: &str = r#"
.device-node {
    width: 170px;
    border: 1px solid #d0d0d7;
    border-radius: 6px;
    background: var(--xy-node-background-color, #fff);
    font-size: 11px;
}
.device-node.selected {
    box-shadow: 0 0 0 1.5px #7c5cff;
}
.device-node__header {
    color: #fff;
    font-weight: 600;
    padding: 4px 8px;
    border-radius: 5px 5px 0 0;
}
.device-node__body {
    position: relative;
    padding: 4px 0;
}
.device-node__port {
    position: absolute;
    font-size: 10px;
    color: #666;
    pointer-events: none;
}
.device-node__port--in { left: 10px; }
.device-node__port--out { right: 10px; }
.edge-power { --xy-edge-stroke: #2fbf71; --xy-edge-stroke-width: 2; }
.edge-drive { --xy-edge-stroke: #ff8c42; --xy-edge-stroke-width: 2; }
"#;

/// A device-style custom node with typed left/right ports
#[component]
fn DeviceNode(
    /// Node ID
    node_id: String,
    /// Title shown in the node header
    title: String,
    /// Accent color for the header + ports
    accent: String,
    /// Input port IDs (left side, target handles)
    inputs: Vec<String>,
    /// Output port IDs (right side, source handles)
    outputs: Vec<String>,
) -> impl IntoView {
    let store = use_flow_store();

    // Library drag hook: pointer capture, zoom-aware, updates the store
    let (on_pointer_down, on_pointer_move, on_pointer_up) =
        use_node_drag_handlers(node_id.clone());

    // Click-to-select
    let on_click = {
        let node_id = node_id.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.stop_propagation();
            let multi = ev.ctrl_key() || ev.meta_key();
            store.select_node(&node_id, multi);
        }
    };

    // Reactive position from the store (edges follow via handle bounds)
    let style = {
        let node_id = node_id.clone();
        move || {
            let nodes = store.get_nodes();
            let Some(node) = nodes.iter().find(|n| n.id == node_id) else {
                return "display: none;".to_string();
            };
            format!(
                "transform: translate({}px, {}px);",
                node.position.x, node.position.y
            )
        }
    };

    // Reactive selection state
    let class = {
        let node_id = node_id.clone();
        move || {
            let selected = store
                .get_nodes()
                .iter()
                .find(|n| n.id == node_id)
                .map(|n| n.selected)
                .unwrap_or(false);
            format!(
                "xyflow__node device-node{}",
                if selected { " selected" } else { "" }
            )
        }
    };

    let n_rows = inputs.len().max(outputs.len()).max(1);

    view! {
        <div
            class=class
            style=style
            data-id=node_id.clone()
            on:click=on_click
            on:pointerdown=on_pointer_down
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up.clone()
            on:pointercancel=on_pointer_up
        >
            <div class="device-node__header" style=format!("background: {};", accent)>
                {title}
            </div>
            <div class="device-node__body" style=format!("min-height: {}px;", n_rows * 22)>
                {inputs
                    .into_iter()
                    .enumerate()
                    .map(|(i, port)| {
                        view! {
                            <Handle
                                node_id=node_id.clone()
                                id=port.clone()
                                r#type=HandleType::Target
                                position=HandlePosition::Left
                                style=format!("top: {}px; background: {};", 14 + i * 22, accent)
                            />
                            <div
                                class="device-node__port device-node__port--in"
                                style=format!("top: {}px;", 6 + i * 22)
                            >
                                {port}
                            </div>
                        }
                    })
                    .collect_view()}
                {outputs
                    .into_iter()
                    .enumerate()
                    .map(|(i, port)| {
                        view! {
                            <Handle
                                node_id=node_id.clone()
                                id=port.clone()
                                r#type=HandleType::Source
                                position=HandlePosition::Right
                                style=format!("top: {}px; background: {};", 14 + i * 22, accent)
                            />
                            <div
                                class="device-node__port device-node__port--out"
                                style=format!("top: {}px;", 6 + i * 22)
                            >
                                {port}
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// Hardening demo: custom port nodes, anchored edges, library drag, fit-view
#[component]
pub fn HardeningExample() -> impl IntoView {
    let initial_nodes = vec![
        Node::new("plc".to_string(), Position::new(40.0, 60.0)).with_dimensions(170.0, 96.0),
        Node::new("extruder".to_string(), Position::new(360.0, 40.0)).with_dimensions(170.0, 74.0),
        Node::new("microwave".to_string(), Position::new(360.0, 220.0))
            .with_dimensions(170.0, 52.0),
    ];

    // Edges between specific ports; paths are orientation-aware and terminate
    // at the measured handle anchors.
    let initial_edges = vec![
        Edge::new("e-ao1".to_string(), "plc".to_string(), "extruder".to_string())
            .with_source_handle(Some("ao_1".to_string()))
            .with_target_handle(Some("drive".to_string()))
            .with_label("analog_out".to_string())
            .with_class("edge-drive".to_string()),
        Edge::new("e-ao2".to_string(), "plc".to_string(), "microwave".to_string())
            .with_source_handle(Some("ao_2".to_string()))
            .with_target_handle(Some("power".to_string()))
            .with_type("smoothstep".to_string())
            .with_label("power_setpoint".to_string())
            .with_class("edge-power".to_string()),
        Edge::new("e-do1".to_string(), "plc".to_string(), "extruder".to_string())
            .with_source_handle(Some("do_1".to_string()))
            .with_target_handle(Some("enable".to_string()))
            .with_type("step".to_string()),
    ];

    // Consumer-owned store: keeps a handle for fit_view / callbacks
    let store = FlowStore::new(initial_nodes, initial_edges);

    // Persist positions on drag end (a real consumer would write to disk/server)
    let last_persisted = RwSignal::new(String::from("(drag a node)"));
    store.set_on_node_drag_end(Callback::new(move |(node_id, pos): (String, Position)| {
        last_persisted.set(format!("{node_id} @ ({:.0}, {:.0})", pos.x, pos.y));
    }));

    let fit = move |_| store.fit_view();
    let zoom_selection = move |_| store.zoom_to_selection();

    view! {
        <div class="example-container">
            <style>{DEVICE_NODE_CSS}</style>
            <SvelteFlow store=store>
                <Background variant=BackgroundVariant::Dots />
                <FlowViewport>
                    <EdgeRenderer />
                    <ConnectionLine />
                    <DeviceNode
                        node_id="plc".to_string()
                        title="PLC".to_string()
                        accent="#7c5cff".to_string()
                        inputs=Vec::new()
                        outputs=vec!["ao_1".to_string(), "ao_2".to_string(), "do_1".to_string()]
                    />
                    <DeviceNode
                        node_id="extruder".to_string()
                        title="Extruder".to_string()
                        accent="#ff8c42".to_string()
                        inputs=vec!["drive".to_string(), "enable".to_string()]
                        outputs=Vec::new()
                    />
                    <DeviceNode
                        node_id="microwave".to_string()
                        title="Microwave".to_string()
                        accent="#2fbf71".to_string()
                        inputs=vec!["power".to_string()]
                        outputs=Vec::new()
                    />
                </FlowViewport>
                <Controls position=PanelPosition::BottomLeft />
                <Panel position=PanelPosition::TopRight>
                    <button on:click=fit>"fit view"</button>
                    <button on:click=zoom_selection>"zoom to selection"</button>
                    <span style="font-size: 11px; margin-left: 6px;">
                        "last drag end: " {move || last_persisted.get()}
                    </span>
                </Panel>
            </SvelteFlow>

            <SourceCodeViewer source=include_str!("hardening.rs") title="hardening.rs" />
        </div>
    }
}
