# XYFlow Leptos

A highly customizable Rust/WASM library for building node-based editors, workflow systems, diagrams and more with Leptos.

## Quick Start

```rust,ignore
use leptos::prelude::*;
use xyflow_leptos::{SvelteFlow, Node, Position};

#[component]
fn App() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        Node::new("1".to_string(), Position::new(0.0, 0.0)),
    ]);

    let edges = RwSignal::new(vec![]);

    view! {
        <SvelteFlow nodes edges />
    }
}
```

## Features

- **Fully Customizable**: Every element can be customized
- **Nodes & Edges**: Flexible node and edge types
- **Pan & Zoom**: Smooth viewport navigation
- **Selection**: Select and manipulate multiple elements
- **Connections**: Intuitive edge creation
- **Type-Safe**: Full Rust type safety with generics

## Connecting nodes

Dragging from a connectable `Handle` starts an interactive connection drag:
the connection line follows the cursor, snaps to nearby connectable handles
(hit-tested against the measured handle bounds in the store), and dropping on
a valid handle completes the connection. Dropping anywhere else cancels
cleanly. The whole lifecycle is pointer-capture based — no document-level
listeners.

```rust,ignore
use leptos::prelude::*;
use xyflow_leptos::{Connection, FlowStore};

let store = FlowStore::new(nodes, edges);

// Host-level validity, enforced while dragging and on drop. May capture
// state (port types, existing bindings, ...) — invalid targets cannot
// complete, and the snapped handle gets an `invalid` CSS class.
store.set_is_valid_connection(Callback::new(move |conn: Connection| {
    is_compatible(&conn.source, conn.source_handle.as_deref(),
                  &conn.target, conn.target_handle.as_deref())
}));

// Completion callback. While registered, the crate does NOT insert an edge
// itself — the host receives the Connection (source/target node + handle
// ids, always ordered source→target) and decides what to create, matching
// xyflow's `onConnect`. Without it, a default edge is added.
store.set_on_connect(Callback::new(move |conn: Connection| {
    create_my_edge_or_binding(conn);
}));
```

Per-handle control on the `Handle` component:

- `is_connectable=false` locks the handle: a drag cannot start from it
  (not-allowed cursor) and it can never be a drop target — hovering it
  during a drag still shows the `invalid` state, so the refusal is visible
  rather than a silent no-op
- `is_connectable_start` / `is_connectable_end` gate the two directions
  individually (non-startable handles also show the not-allowed cursor)
- `is_valid_connection` accepts a plain `fn(&Connection) -> bool` for
  capture-free validation local to that handle

Styling during a drag: the fixed end carries a `connectingfrom` class; the
handle currently snapped carries `connectingto` plus `valid` or `invalid`.
For richer effects (e.g. dimming every incompatible handle on both sides),
read the in-flight state reactively with `use_connection()` — it exposes the
fixed end (`from_node`/`from_handle`/`from_handle_type`), the current snap
`candidate`, and `is_valid`; derive a per-handle `Memo` from it and bind a
class. See `examples/src/examples/connections/validation.rs`.

## Selection, deletion & context menus

Clicking an edge selects it (the invisible wide interaction stroke makes
this forgiving); clicking a node selects the node. The flow container is
focusable, so once clicked, `Delete`/`Backspace` requests deletion of the
selection and `Escape` cancels an in-flight connection drag. Keys typed in
inputs inside custom nodes are ignored.

```rust,ignore
use xyflow_leptos::{ContextMenuEvent, DeleteRequest, FlowStore};

// Host-owned deletion: while registered, the crate deletes NOTHING itself —
// you receive the selected node/edge ids and decide. Without it, the store
// removes the selection directly (deleting a node also drops its edges).
store.set_on_delete_requested(Callback::new(move |req: DeleteRequest| {
    delete_my_bindings(&req.edges);           // host bookkeeping
    store.delete_elements(&req.nodes, &req.edges); // then apply visually
}));

// Right-click callbacks. The container classifies the target by walking up
// to the closest `.xyflow__edge` / `.xyflow__node` (works for custom node
// and edge views — they already carry those classes + data-id). The native
// browser menu is suppressed only where a callback is registered.
store.set_on_edge_context_menu(Callback::new(move |ev: ContextMenuEvent| {
    // ev.id = Some(edge_id); ev.screen_x/y for positioning a fixed menu;
    // ev.flow_x/y for placing things on the canvas
    open_edge_menu(ev);
}));
store.set_on_node_context_menu(Callback::new(move |ev: ContextMenuEvent| { /* ... */ }));
store.set_on_pane_context_menu(Callback::new(move |ev: ContextMenuEvent| { /* ev.id == None */ }));
```

`FlowStore::request_delete_selection()` triggers the same delete flow
programmatically (e.g. from a menu item or toolbar button).

## Architecture

XYFlow Leptos is built on top of:

- **Leptos**: A full-stack, isomorphic Rust web framework
- **Fine-grained Reactivity**: Signals for efficient updates
- **Web APIs**: Direct DOM access via wasm-bindgen

## Status

🚧 **Phase 1: Core Foundation** (In Progress)

- ✅ Type definitions
- ✅ Store architecture
- ⏳ Basic components
- ⏳ Event handling
- ⏳ Documentation

## Learn More

- [XYFlow Main Repository](https://github.com/xyflow/xyflow)
- [Leptos Framework](https://leptos.dev/)
- [Documentation](https://xyflow.dev/)

## License

MIT License - See LICENSE file for details

---

**Note**: This is an early-stage port of the XYFlow library to Leptos. Check the GitHub issues and discussions for current status and contributing guidelines.
