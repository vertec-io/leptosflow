# LeptosFlow

A highly customizable Rust/WASM library for building node-based editors, workflow systems, and interactive diagrams with [Leptos](https://leptos.dev/).

Inspired by [xyflow/xyflow](https://github.com/xyflow/xyflow) (React Flow / Svelte Flow).

## Features

- **Fully Customizable** - Every node, edge, and component can be customized
- **Pan & Zoom** - Smooth viewport navigation with mouse and touch support
- **Connections** - Intuitive edge creation with validation support
- **Selection** - Select and manipulate multiple elements
- **MiniMap** - Overview navigation component
- **Controls** - Zoom and fit-view controls
- **Backgrounds** - Dots, lines, and cross patterns
- **Type-Safe** - Full Rust type safety

## Quick Start

```rust
use leptos::prelude::*;
use xyflow_leptos::{Flow, Node, Edge, Position, Background, Controls, MiniMap};

#[component]
fn App() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        Node::new("1", Position::new(0.0, 0.0))
            .with_data("label", "Node 1"),
        Node::new("2", Position::new(200.0, 100.0))
            .with_data("label", "Node 2"),
    ]);

    let edges = RwSignal::new(vec![
        Edge::new("e1-2", "1", "2"),
    ]);

    view! {
        <Flow nodes edges>
            <Background />
            <Controls />
            <MiniMap />
        </Flow>
    }
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xyflow-leptos = { git = "https://github.com/vertec-io/leptosflow" }
```

## Project Structure

```
leptosflow/
├── xyflow-leptos/      # Core library
│   ├── src/
│   │   ├── components/ # Flow, Handle, Background, Controls, MiniMap, etc.
│   │   ├── hooks/      # use_nodes, use_edges, use_viewport, etc.
│   │   ├── store/      # FlowStore state management
│   │   ├── types/      # Node, Edge, Position, Viewport, etc.
│   │   └── utils/      # Math, DOM, edge path utilities
│   └── docs/           # Library documentation
└── examples/           # 62 interactive examples
    └── src/
        └── examples/   # Organized by category
```

## Running the Examples

### Prerequisites

1. Install Rust with WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Install Trunk:
   ```bash
   cargo install trunk
   ```

### Development Server

```bash
cd examples
trunk serve
```

Open http://localhost:8080 to browse the examples.

## Example Categories

| Category | Examples | Description |
|----------|----------|-------------|
| Basic | 3 | Getting started, empty flow, default nodes |
| Nodes | 13 | Custom nodes, resizing, handles, toolbars |
| Edges | 9 | Edge types, custom edges, floating edges |
| Connections | 5 | Validation, reconnection, drop handling |
| Interactions | 6 | Selection, drag handling, touch support |
| Viewport | 4 | Controlled viewport, layouts, intersection |
| MiniMap | 3 | Custom nodes, interactive navigation |
| Styling | 3 | Backgrounds, themes, visibility |
| State | 5 | Save/restore, batching, middleware |
| Advanced | 7 | Subflows, multi-flows, accessibility |
| Hooks | 5 | FlowStore API, keyboard shortcuts, drag-and-drop |

## Documentation

- [Quickstart Guide](xyflow-leptos/docs/QUICKSTART.md)
- [Connection System](xyflow-leptos/docs/CONNECTION_SYSTEM.md)
- [Coordinate System](xyflow-leptos/COORDINATE_SYSTEM.md)
- [Examples README](examples/README.md)

## License

MIT License
