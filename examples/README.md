# XYFlow Leptos Examples

A comprehensive collection of 62 interactive examples demonstrating all features of xyflow-leptos, the Rust/WASM port of XYFlow built with Leptos.

## Architecture Overview

The examples are organized as a single-page application with client-side routing, providing a showcase of all xyflow-leptos features organized by category.

### Directory Structure

```
examples/leptos/
├── README.md                   # This file
├── Cargo.toml                  # Dependencies
├── Trunk.toml                  # Trunk configuration
├── index.html                  # HTML template with CSS styles
├── dist/                       # Built output (generated)
└── src/
    ├── main.rs                 # App shell, router, navigation
    ├── shared/
    │   └── mod.rs              # Shared components (DraggableNode, SourceCodeViewer, etc.)
    └── examples/
        ├── mod.rs              # Category exports
        ├── basic/              # Basic examples
        ├── nodes/              # Node customization examples
        ├── edges/              # Edge customization examples
        ├── connections/        # Connection handling examples
        ├── interactions/       # User interaction examples
        ├── viewport/           # Viewport control examples
        ├── minimap/            # MiniMap examples
        ├── styling/            # Styling and theming examples
        ├── state/              # State management examples
        ├── advanced/           # Advanced patterns
        └── hooks/              # Hooks/API examples
```

### Key Components

- **`main.rs`**: Contains the app shell with `<Router>`, `<Routes>`, `NavSidebar`, `Layout`, and `HomePage` components
- **`shared/mod.rs`**: Shared utilities including:
  - `ExampleMeta`: Metadata struct for example navigation
  - `get_all_examples()`: Returns categorized list of all examples
  - `DraggableNode`: Reusable draggable node wrapper
  - `CustomNode`: Default node component with handles
  - `SourceCodeViewer`: Collapsible source code panel with syntax highlighting
  - `PlaceholderExample`: Template for unimplemented examples
- **Category `mod.rs` files**: Export all examples in each category

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

Navigate to the examples app and start the dev server:

```bash
cd examples/leptos
trunk serve
```

Open your browser to `http://localhost:8080`

### Production Build

```bash
cd examples/leptos
trunk build --release
```

The built files will be in the `dist/` directory.

## Adding a New Example

### Step 1: Create the Example File

Create a new `.rs` file in the appropriate category directory:

```rust
// src/examples/{category}/{example_name}.rs

use leptos::prelude::*;
use xyflow_leptos::*;

use crate::shared::{DraggableNode, get_drag_signal, SourceCodeViewer};

#[component]
pub fn MyNewExample() -> impl IntoView {
    // Create a FlowStore for state management
    let store = FlowStore::new();

    // Define initial nodes
    let initial_nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0))
            .with_data("label", "Node 1")
            .with_data("type", "input"),
        Node::new("2".to_string(), Position::new(300.0, 200.0))
            .with_data("label", "Node 2")
            .with_data("type", "default"),
    ];

    // Define initial edges
    let initial_edges = vec![
        Edge::new("e1-2".to_string(), "1".to_string(), "2".to_string()),
    ];

    // Set initial state
    store.set_nodes(initial_nodes);
    store.set_edges(initial_edges);

    // Get drag signal for node dragging
    let drag_signal = get_drag_signal();

    view! {
        <div class="example-container">
            // Flow viewport
            <div class="flow-wrapper">
                <FlowViewport store=store.clone()>
                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! { <DraggableNode node=node.clone() store=store.clone() /> }
                        }).collect_view()
                    }}
                </FlowViewport>
                <Background variant=BackgroundVariant::Dots />
                <Controls />
                <MiniMap />
            </div>

            // Info panel (optional)
            <div class="panel info-panel">
                <h3>"My New Example"</h3>
                <p>"Description of what this example demonstrates."</p>
            </div>

            // Source code viewer (optional)
            <SourceCodeViewer
                source=include_str!("my_new_example.rs")
                title="my_new_example.rs"
            />
        </div>
    }
}
```

### Step 2: Export from Category Module

Add the export to the category's `mod.rs`:

```rust
// src/examples/{category}/mod.rs

mod my_new_example;
pub use my_new_example::MyNewExample;
```

### Step 3: Register the Route

Add the route in `main.rs`:

```rust
// In the imports section
use examples::{category}::MyNewExample;

// In the Routes component
<Route path=path!("examples/my-new-example") view=MyNewExample />
```

### Step 4: Add to Navigation

Add metadata in `shared/mod.rs` in the `get_all_examples()` function:

```rust
("{Category}", vec![
    // ... existing examples ...
    ExampleMeta {
        id: "my-new-example",
        name: "My New Example",
        description: "Brief description of what this example shows",
        category: "{Category}",
    },
]),
```

### Step 5: Verify

1. Run `cargo check` to verify compilation
2. Start the dev server and navigate to your example
3. Verify the example works as expected

## Category Organization

Examples are organized into 11 categories based on the feature they demonstrate:

| Category | Description | Count |
|----------|-------------|-------|
| **Basic** | Getting started examples (empty, default nodes, basic setup) | 3 |
| **Nodes** | Node customization, resizing, handles, toolbars | 13 |
| **Edges** | Edge types, custom edges, floating edges, routing | 9 |
| **Connections** | Connection handling, validation, reconnection | 5 |
| **Interactions** | Selection, drag handling, touch support, click detection | 6 |
| **Viewport** | Viewport control, layouts, intersection detection | 4 |
| **MiniMap** | MiniMap customization and interaction | 3 |
| **Styling** | Backgrounds, themes, visibility | 3 |
| **State** | State management, serialization, batching, middleware | 5 |
| **Advanced** | Complex patterns (Figma-like, subflows, multi-flows, a11y) | 7 |
| **Hooks** | FlowStore API, keyboard shortcuts, drag-and-drop, dev tools | 5 |

## Available Examples

### Basic (3 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Basic | `/examples/basic` | Draggable nodes, pan/zoom, background, minimap, controls |
| Empty | `/examples/empty` | Minimal starting point with empty flow canvas |
| Default Nodes | `/examples/default-nodes` | Input, default, and output node types |

### Nodes (13 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Custom Nodes | `/examples/custom-node` | User-defined node components with colors |
| Default Node Overwrite | `/examples/default-node-overwrite` | Customize the default node component |
| Node Resizer | `/examples/node-resizer` | Resizable nodes with handles |
| Drag Handle | `/examples/drag-handle` | Limit drag area to specific region |
| Moving Handles | `/examples/moving-handles` | Handles that change position dynamically |
| Detached Handle | `/examples/detached-handle` | Handles positioned outside node body |
| Node Type Change | `/examples/node-type-change` | Dynamically change node type at runtime |
| Node Types Object Change | `/examples/node-types-object-change` | Dynamically change node type definitions |
| Update Node | `/examples/update-node` | Update node properties programmatically |
| Use Update Node Internals | `/examples/use-update-node-internals` | Force re-measurement of node internals |
| Broken Nodes | `/examples/broken-nodes` | Graceful handling of invalid node configurations |
| Node Toolbar | `/examples/node-toolbar` | Context toolbar on nodes |
| Use Nodes Init | `/examples/use-nodes-init` | Lifecycle hook for when nodes are initialized |

### Edges (9 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Edge Types | `/examples/edge-types` | Bezier, step, and straight edge styles |
| Default Edge Overwrite | `/examples/default-edge-overwrite` | Customize the default edge component |
| Custom Edges | `/examples/custom-edges` | Fully custom edge components |
| Custom Connection Line | `/examples/custom-connection-line` | Customize connection preview line |
| Floating Edges | `/examples/floating-edges` | Edges that connect to nodes dynamically |
| Easy Connect | `/examples/easy-connect` | Click-based connection creation |
| Edge Renderer | `/examples/edge-renderer` | Custom edge layer rendering with z-index |
| Edge Toolbar | `/examples/edge-toolbar` | Toolbars on edges |
| Edge Routing | `/examples/edge-routing` | Advanced edge routing with obstacle avoidance |

### Connections (5 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Validation | `/examples/validation` | Connection validation rules |
| Use Connection | `/examples/use-connection` | Connection hook for custom behavior |
| Cancel Connection | `/examples/cancel-connection` | Cancel connections in progress |
| Reconnect Edge | `/examples/reconnect-edge` | Reconnect existing edges to different handles |
| Add Node on Edge Drop | `/examples/add-node-on-edge-drop` | Create new node when dropping connection |

### Interactions (6 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Interactions | `/examples/interactions` | Selection, multi-select, and deletion |
| Selection Change | `/examples/use-on-selection-change` | React to selection changes |
| Node Connections | `/examples/use-node-connections` | Get all connections for a specific node |
| Click Distance | `/examples/click-distance` | Distinguish between clicks and drags |
| Touch Device | `/examples/touch-device` | Touch-optimized interactions |
| Multi-Set Nodes | `/examples/multi-set-nodes` | Multiple disconnected node groups |

### Viewport (4 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Controlled Viewport | `/examples/controlled-viewport` | Programmatically control viewport |
| Controlled/Uncontrolled | `/examples/controlled-uncontrolled` | Compare controlled vs uncontrolled modes |
| Intersection | `/examples/intersection` | Detect nodes in viewport |
| Layouting | `/examples/layouting` | Automatic graph layout algorithms |

### MiniMap (3 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Custom MiniMap Node | `/examples/custom-minimap-node` | Customize minimap node appearance |
| Interactive MiniMap | `/examples/interactive-minimap` | Click and drag to pan viewport |
| Overview | `/examples/overview` | Toggle minimap visibility with animation |

### Styling (3 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Backgrounds | `/examples/backgrounds` | Dots, lines, and cross patterns |
| Color Mode | `/examples/color-mode` | Light and dark mode switching |
| Hidden | `/examples/hidden` | Hide and show flow elements |

### State (5 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Save/Restore | `/examples/save-restore` | Serialize and deserialize flow state |
| Use Nodes Data | `/examples/use-nodes-data` | Reactively access node data |
| Set Nodes Batching | `/examples/set-nodes-batching` | Batch multiple node updates efficiently |
| Reactive Stores | `/examples/reactive-stores` | Integration with Leptos `reactive_stores` |
| Middlewares | `/examples/middlewares` | Custom middleware/hooks for state operations |

### Advanced (7 examples)

| Example | Route | Description |
|---------|-------|-------------|
| Figma | `/examples/figma` | Figma-like selection and interaction |
| Undirectional | `/examples/undirectional` | Restrict connection direction (left to right only) |
| Subflow | `/examples/subflow` | Nested flow graphs |
| Multi Flows | `/examples/multi-flows` | Multiple independent flow instances |
| Provider | `/examples/provider` | FlowStore context pattern for sibling components |
| Accessibility | `/examples/a11y` | Keyboard navigation and screen reader support |
| Stress Test | `/examples/stress` | Performance with 625 nodes |

### Hooks (5 examples)

| Example | Route | Description |
|---------|-------|-------------|
| useSvelteFlow | `/examples/use-svelte-flow` | Main FlowStore API for programmatic control |
| useKeyPress | `/examples/use-key-press` | Keyboard shortcut handling |
| Drag & Drop | `/examples/drag-n-drop` | Drag nodes from sidebar onto canvas |
| Dev Tools | `/examples/dev-tools` | Debug panel showing flow internals |
| Z-Index Mode | `/examples/z-index-mode` | Node stacking order controls |

## Common Patterns

### Using FlowStore

The `FlowStore` is the central state container:

```rust
let store = FlowStore::new();

// Set nodes and edges
store.set_nodes(vec![...]);
store.set_edges(vec![...]);

// Get current state
let nodes = store.get_nodes();
let edges = store.get_edges();

// Update a specific node
store.update_node("node-id", |node| {
    node.position.x += 10.0;
});

// Viewport control
store.set_viewport(Viewport { x: 0.0, y: 0.0, zoom: 1.0 });
```

### Draggable Nodes Pattern

For interactive draggable nodes, use the shared `DraggableNode` component or follow this pattern:

```rust
use crate::shared::{DraggableNode, get_drag_signal};

// Get the global drag signal
let drag_signal = get_drag_signal();

// Set up document-level mouse handlers for dragging
Effect::new(move |_| {
    // mousemove and mouseup handlers for drag tracking
    // See basic.rs for complete implementation
});

// Render nodes with DraggableNode wrapper
{move || {
    store.get_nodes().into_iter().map(|node| {
        view! { <DraggableNode node=node.clone() store=store.clone() /> }
    }).collect_view()
}}
```

### Reactive Updates with Effects

Use Leptos `Effect` for side effects and reactive updates:

```rust
let store_for_effect = store.clone();
Effect::new(move |_| {
    let nodes = store_for_effect.get_nodes();
    // React to node changes
    log!("Nodes changed: {:?}", nodes.len());
});
```

### Keyboard Event Handling

For global keyboard shortcuts:

```rust
use leptos::web_sys;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

Effect::new(move |_| {
    let handler = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Delete" => { /* handle delete */ }
            "Escape" => { /* handle escape */ }
            _ => {}
        }
    }) as Box<dyn Fn(web_sys::KeyboardEvent)>);

    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let _ = document.add_event_listener_with_callback(
                "keydown",
                handler.as_ref().unchecked_ref(),
            );
        }
    }
    handler.forget();
});
```

## CSS Styling

The examples use CSS classes that follow the XYFlow naming convention:

- `.svelte-flow` - Main flow container
- `.xyflow__node` - Node wrapper
- `.xyflow__node-default` - Default node style
- `.xyflow__node-input` - Input node style
- `.xyflow__node-output` - Output node style
- `.xyflow__handle` - Connection handles

Custom styles are defined in `index.html` and can be overridden per-example.

## Troubleshooting

### WASM compilation errors

Ensure the WASM target is installed:
```bash
rustup target add wasm32-unknown-unknown
```

### Trunk not found

Install Trunk:
```bash
cargo install trunk
```

### Examples not loading

Make sure you're running `trunk serve` from the `examples/leptos/` directory.

### Style issues

CSS variables should be set on the `.svelte-flow` class:
```css
.svelte-flow {
    --xy-background-color: #f5f5f5;
    --xy-node-background: white;
}
```

## Learn More

- [XYFlow Main Repository](https://github.com/xyflow/xyflow)
- [Leptos Framework](https://leptos.dev/)
- [XYFlow Documentation](https://xyflow.dev/)
- [xyflow-leptos Library README](../../packages/leptos/README.md)
