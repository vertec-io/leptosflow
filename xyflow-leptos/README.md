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
