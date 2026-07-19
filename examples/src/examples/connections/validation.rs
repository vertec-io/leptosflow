//! Validation Example
//!
//! Interactive drag-connect with a validity predicate, exercising the
//! library-native connection engine end to end:
//!
//! - Drag from a port handle → live connection line → drop on a handle.
//! - Host-level validity via `FlowStore::set_is_valid_connection`: ports are
//!   typed (analog vs digital) and only matching kinds may connect. Invalid
//!   targets cannot complete and show the `invalid` handle state.
//! - Host-owned completion via `FlowStore::set_on_connect`: the crate does
//!   not insert an edge itself — this example creates its own (colored,
//!   de-duplicated) edge and logs the event.
//! - Per-handle connectable gating: the "locked" port
//!   (`is_connectable=false`) shows a not-allowed cursor, cannot start a
//!   drag, and turns red (`invalid`) when hovered as a drop candidate —
//!   refusal is always visible, never a silent no-op.
//! - Both-sides feedback while dragging: every handle that could not accept
//!   the in-flight connection is dimmed, derived from `use_connection()`.
//! - Edge interactions: click selects an edge; right-click opens a demo
//!   context menu with Delete (`set_on_edge_context_menu`); Delete/Backspace
//!   removes the selection through the host-owned `set_on_delete_requested`.

use leptos::prelude::*;
use xyflow_leptos::events::use_node_drag_handlers;
use xyflow_leptos::*;

const VALIDATION_CSS: &str = r#"
.typed-node {
    width: 170px;
    border: 1px solid #d0d0d7;
    border-radius: 6px;
    background: var(--xy-node-background-color, #fff);
    font-size: 11px;
}
.typed-node.selected { box-shadow: 0 0 0 1.5px #7c5cff; }
.typed-node__header {
    color: #fff;
    font-weight: 600;
    padding: 4px 8px;
    border-radius: 5px 5px 0 0;
}
.typed-node__body { position: relative; padding: 4px 0; }
.typed-node__port {
    position: absolute;
    font-size: 10px;
    color: #666;
    pointer-events: none;
}
.typed-node__port--in { left: 10px; }
.typed-node__port--out { right: 10px; }
/* Handles that cannot accept the in-flight connection are dimmed —
 * except the current snap candidate, whose valid/invalid state must
 * stay fully visible. */
.typed-node .xyflow__handle.port-dim:not(.connectingto) { opacity: 0.2; }
.edge-analog { --xy-edge-stroke: #ff8c42; --xy-edge-stroke-width: 2; }
.edge-digital { --xy-edge-stroke: #2fbf71; --xy-edge-stroke-width: 2; }
/* Demo context menu (positioned at ContextMenuEvent screen coords) */
.demo-menu {
    position: fixed;
    z-index: 2000;
    background: white;
    border: 1px solid #d0d0d7;
    border-radius: 6px;
    box-shadow: 0 4px 16px rgba(0,0,0,0.15);
    font-size: 12px;
    min-width: 140px;
    overflow: hidden;
}
.demo-menu__title {
    padding: 6px 10px;
    color: #888;
    font-size: 10px;
    border-bottom: 1px solid #eee;
    font-family: monospace;
}
.demo-menu button {
    display: block;
    width: 100%;
    padding: 7px 10px;
    border: none;
    background: none;
    text-align: left;
    cursor: pointer;
}
.demo-menu button:hover { background: #f5f5f5; }
.demo-menu button.danger { color: #dc2626; }
"#;

/// Port kind from the handle ID prefix: `a_*` = analog, `d_*` = digital
fn port_kind(handle_id: Option<&str>) -> Option<char> {
    handle_id.and_then(|id| id.chars().next())
}

fn port_accent(port_id: &str) -> &'static str {
    match port_kind(Some(port_id)) {
        Some('a') => "#ff8c42",
        Some('d') => "#2fbf71",
        _ => "#9a9aa5",
    }
}

/// One typed port row: measured handle + label, with a reactive dim class
/// while an incompatible connection is being dragged.
#[component]
fn PortRow(
    node_id: String,
    port_id: String,
    handle_type: HandleType,
    row: usize,
    /// false = anchor only (no connect start, no drop target)
    connectable: bool,
) -> impl IntoView {
    // Both-sides validity styling: while a connection is in flight, dim this
    // handle unless it could accept the drag (opposite handle type, matching
    // port kind, connectable) or it IS the drag's fixed end.
    let connection = use_connection();
    let dim = {
        let node_id = node_id.clone();
        let port_id = port_id.clone();
        Memo::new(move |_| {
            connection.get().is_some_and(|conn| {
                let is_fixed_end = conn.from_node == node_id
                    && conn.from_handle.as_deref() == Some(port_id.as_str())
                    && conn.from_handle_type == handle_type;
                if is_fixed_end {
                    return false;
                }
                let accepts = connectable
                    && handle_type != conn.from_handle_type
                    && port_kind(Some(&port_id)) == port_kind(conn.from_handle.as_deref());
                !accepts
            })
        })
    };

    let (position, side_class) = match handle_type {
        HandleType::Target => (HandlePosition::Left, "typed-node__port--in"),
        HandleType::Source => (HandlePosition::Right, "typed-node__port--out"),
    };
    let label = if connectable {
        port_id.clone()
    } else {
        format!("{port_id} (locked)")
    };

    view! {
        <Handle
            node_id=node_id
            id=port_id.clone()
            r#type=handle_type
            position=position
            is_connectable=connectable
            class=Signal::derive(move || {
                if dim.get() { "port-dim".to_string() } else { String::new() }
            })
            style=format!(
                "top: {}px; background: {};",
                14 + row * 22,
                port_accent(&port_id),
            )
        />
        <div class=format!("typed-node__port {side_class}") style=format!("top: {}px;", 6 + row * 22)>
            {label}
        </div>
    }
}

/// A node with typed left (target) / right (source) ports
#[component]
fn TypedNode(
    node_id: String,
    title: String,
    accent: String,
    /// (port_id, connectable) — left side, target handles
    inputs: Vec<(String, bool)>,
    /// (port_id, connectable) — right side, source handles
    outputs: Vec<(String, bool)>,
) -> impl IntoView {
    let store = use_flow_store();
    let (on_pointer_down, on_pointer_move, on_pointer_up) =
        use_node_drag_handlers(node_id.clone());

    let on_click = {
        let node_id = node_id.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.stop_propagation();
            store.select_node(&node_id, ev.ctrl_key() || ev.meta_key());
        }
    };

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
                "xyflow__node typed-node{}",
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
            <div class="typed-node__header" style=format!("background: {};", accent)>
                {title}
            </div>
            <div class="typed-node__body" style=format!("min-height: {}px;", n_rows * 22)>
                {inputs
                    .into_iter()
                    .enumerate()
                    .map(|(i, (port, connectable))| {
                        view! {
                            <PortRow
                                node_id=node_id.clone()
                                port_id=port
                                handle_type=HandleType::Target
                                row=i
                                connectable=connectable
                            />
                        }
                    })
                    .collect_view()}
                {outputs
                    .into_iter()
                    .enumerate()
                    .map(|(i, (port, connectable))| {
                        view! {
                            <PortRow
                                node_id=node_id.clone()
                                port_id=port
                                handle_type=HandleType::Source
                                row=i
                                connectable=connectable
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// Connection validation example: typed ports + host-owned edge creation
#[component]
pub fn ValidationExample() -> impl IntoView {
    let initial_nodes = vec![
        Node::new("sensors".to_string(), Position::new(40.0, 60.0)).with_dimensions(170.0, 96.0),
        Node::new("controller".to_string(), Position::new(360.0, 40.0))
            .with_dimensions(170.0, 74.0),
        Node::new("logger".to_string(), Position::new(360.0, 200.0)).with_dimensions(170.0, 52.0),
    ];

    let store = FlowStore::new(initial_nodes, vec![]);

    // Connection event log
    let connection_log = RwSignal::new(Vec::<String>::new());
    let add_log = move |msg: String| {
        connection_log.update(|logs| {
            logs.insert(0, msg);
            if logs.len() > 10 {
                logs.pop();
            }
        });
    };

    // Host-level validity: port kinds must match (analog↔analog,
    // digital↔digital). Enforced while dragging (invalid handles show the
    // `invalid` state and are dimmed) and again on drop.
    store.set_is_valid_connection(Callback::new(move |conn: Connection| {
        port_kind(conn.source_handle.as_deref()) == port_kind(conn.target_handle.as_deref())
    }));

    // Host-owned completion: the crate does not add an edge — we decide.
    // Here: de-duplicate, then add a kind-colored edge and log it.
    store.set_on_connect(Callback::new(move |conn: Connection| {
        let duplicate = store.get_edges_untracked().iter().any(|e| {
            e.source == conn.source
                && e.target == conn.target
                && e.source_handle == conn.source_handle
                && e.target_handle == conn.target_handle
        });
        let source_handle = conn.source_handle.clone().unwrap_or_default();
        let target_handle = conn.target_handle.clone().unwrap_or_default();
        if duplicate {
            add_log(format!(
                "duplicate ignored: {}:{} → {}:{}",
                conn.source, source_handle, conn.target, target_handle
            ));
            return;
        }
        let class = match port_kind(conn.source_handle.as_deref()) {
            Some('a') => "edge-analog",
            _ => "edge-digital",
        };
        store.add_edge(
            Edge::new(
                format!(
                    "e-{}:{}-{}:{}",
                    conn.source, source_handle, conn.target, target_handle
                ),
                conn.source.clone(),
                conn.target.clone(),
            )
            .with_source_handle(conn.source_handle.clone())
            .with_target_handle(conn.target_handle.clone())
            .with_class(class.to_string()),
        );
        add_log(format!(
            "connected {}:{} → {}:{}",
            conn.source, source_handle, conn.target, target_handle
        ));
    }));

    // Host-owned deletion: Delete/Backspace (flow focused) routes the
    // selection here — we log and then apply it ourselves.
    store.set_on_delete_requested(Callback::new(move |req: DeleteRequest| {
        add_log(format!(
            "deleted {} edge(s), {} node(s)",
            req.edges.len(),
            req.nodes.len()
        ));
        store.delete_elements(&req.nodes, &req.edges);
    }));

    // Edge context menu: right-click an edge → select it + open a demo menu
    // at the event's screen coordinates.
    let edge_menu: RwSignal<Option<ContextMenuEvent>> = RwSignal::new(None);
    store.set_on_edge_context_menu(Callback::new(move |ev: ContextMenuEvent| {
        if let Some(id) = ev.id.as_deref() {
            store.select_edge(id, false);
        }
        edge_menu.set(Some(ev));
    }));
    // Pane right-click: just log (and close any open menu)
    store.set_on_pane_context_menu(Callback::new(move |ev: ContextMenuEvent| {
        edge_menu.set(None);
        add_log(format!("pane context menu @ ({:.0}, {:.0})", ev.flow_x, ev.flow_y));
    }));

    let close_menu = move |_ev: leptos::ev::MouseEvent| edge_menu.set(None);
    let delete_menu_edge = move |_ev: leptos::ev::MouseEvent| {
        if let Some(menu) = edge_menu.get_untracked() {
            if let Some(id) = menu.id {
                store.select_edge(&id, false);
                store.request_delete_selection();
            }
        }
        edge_menu.set(None);
    };

    view! {
        <div class="example-container" on:click=close_menu>
            // The crate's canonical stylesheet (connection line, handle
            // valid/invalid states, cursors) + example-local styling.
            <style>{xyflow_leptos::STYLES}</style>
            <style>{VALIDATION_CSS}</style>
            <SvelteFlow store=store>
                <Background variant=BackgroundVariant::Dots />
                <FlowViewport>
                    <EdgeRenderer />
                    <ConnectionLine />
                    <TypedNode
                        node_id="sensors".to_string()
                        title="Sensors".to_string()
                        accent="#7c5cff".to_string()
                        inputs=Vec::new()
                        outputs=vec![
                            ("a_temp".to_string(), true),
                            ("d_alarm".to_string(), true),
                            ("a_raw".to_string(), false),
                        ]
                    />
                    <TypedNode
                        node_id="controller".to_string()
                        title="Controller".to_string()
                        accent="#ff8c42".to_string()
                        inputs=vec![("a_setpoint".to_string(), true), ("d_enable".to_string(), true)]
                        outputs=Vec::new()
                    />
                    <TypedNode
                        node_id="logger".to_string()
                        title="Logger".to_string()
                        accent="#2fbf71".to_string()
                        inputs=vec![("a_trend".to_string(), true)]
                        outputs=Vec::new()
                    />
                </FlowViewport>
                <Controls position=PanelPosition::BottomLeft />
                <MiniMap position=PanelPosition::BottomRight />
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 12px; border-radius: 6px; max-width: 280px; box-shadow: 0 2px 6px rgba(0,0,0,0.1); font-size: 12px;">
                        <strong>"Typed-port validation"</strong>
                        <p style="margin: 6px 0; color: #666; font-size: 11px;">
                            "Drag from a right-side port to a left-side port. "
                            <span style="color: #ff8c42;">"analog (a_*)"</span>
                            " only connects to analog, "
                            <span style="color: #2fbf71;">"digital (d_*)"</span>
                            " only to digital. Incompatible handles dim while you drag; "
                            "the locked port shows a not-allowed cursor and flashes red "
                            "if you try to drop on it."
                        </p>
                        <p style="margin: 6px 0; color: #666; font-size: 11px;">
                            "Click an edge to select it, then press "
                            <kbd style="background: #eee; padding: 1px 4px; border-radius: 2px;">"Delete"</kbd>
                            " — or right-click the edge for a menu."
                        </p>
                        <strong style="font-size: 11px;">"Connection log"</strong>
                        <div style="margin-top: 4px; font-size: 10px; font-family: monospace; max-height: 120px; overflow-y: auto;">
                            {move || {
                                let logs = connection_log.get();
                                if logs.is_empty() {
                                    view! { <p style="color: #999;">"No connections yet"</p> }
                                        .into_any()
                                } else {
                                    logs.into_iter()
                                        .map(|log| {
                                            view! {
                                                <p style="margin: 2px 0; padding: 2px 4px; background: #f5f5f5; border-radius: 2px;">
                                                    {log}
                                                </p>
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>
                </Panel>
            </SvelteFlow>
            // Demo context menu at the ContextMenuEvent's screen coordinates
            {move || {
                edge_menu.get().map(|menu| {
                    let label = menu.id.clone().unwrap_or_default();
                    view! {
                        <div
                            class="demo-menu"
                            style=format!("left: {}px; top: {}px;", menu.screen_x, menu.screen_y)
                        >
                            <div class="demo-menu__title">{label}</div>
                            <button class="danger" on:click=delete_menu_edge>
                                "Delete edge"
                            </button>
                            <button on:click=close_menu>"Cancel"</button>
                        </div>
                    }
                })
            }}
        </div>
    }
}
