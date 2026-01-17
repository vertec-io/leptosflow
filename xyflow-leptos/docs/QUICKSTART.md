# XYFlow Leptos - Quick Start Guide

## Installation

Add XYFlow Leptos to your `Cargo.toml`:

```toml
[dependencies]
xyflow-leptos = "0.1.0"
leptos = "0.8"
```

## Basic Usage

### 1. Create a Simple Flow

```rust
use leptos::prelude::*;
use xyflow_leptos::*;

#[component]
fn App() -> impl IntoView {
    // Create initial nodes
    let nodes = vec![
        Node::new("1".to_string(), Position::new(100.0, 100.0)),
        Node::new("2".to_string(), Position::new(400.0, 100.0)),
    ];
    
    // Create initial edges
    let edges = vec![
        Edge::new("e1".to_string(), "1".to_string(), "2".to_string()),
    ];
    
    // Create the flow store
    let store = FlowStore::new(nodes, edges);
    
    view! {
        <div style="width: 100vw; height: 100vh;">
            // Your flow visualization here
        </div>
    }
}
```

### 2. Create Custom Nodes with Handles

```rust
use leptos::prelude::*;
use leptos::serde_json::json;
use xyflow_leptos::*;

#[component]
fn CustomNode(
    id: String,
    label: String,
) -> impl IntoView {
    view! {
        <div class="custom-node">
            // Target handle (left side - where edges end)
            <Handle
                node_id=id.clone()
                r#type=HandleType::Target
                position=HandlePosition::Left
                connection_mode=ConnectionMode::Strict
            />
            
            <div class="node-content">
                {label}
            </div>
            
            // Source handle (right side - where edges start)
            <Handle
                node_id=id
                r#type=HandleType::Source
                position=HandlePosition::Right
                connection_mode=ConnectionMode::Strict
            />
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let nodes = vec![
        Node::new("node-1".to_string(), Position::new(100.0, 100.0))
            .with_data(json!("Input")),
        Node::new("node-2".to_string(), Position::new(400.0, 100.0))
            .with_data(json!("Output")),
    ];
    
    let store = FlowStore::new(nodes, vec![]);
    
    view! {
        <div style="width: 100vw; height: 100vh;">
            {move || {
                store.get_nodes().into_iter().map(|node| {
                    let label = node.data.as_str().unwrap_or("Node").to_string();
                    let pos = node.position;
                    
                    view! {
                        <div style=format!(
                            "position: absolute; left: {}px; top: {}px;",
                            pos.x, pos.y
                        )>
                            <CustomNode id=node.id label=label />
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}
```

### 3. Connection Creation

Users can now click and drag from source handles to target handles to create connections!

The connection system includes:
- ✅ Real-time validation during drag
- ✅ Visual feedback for valid/invalid connections
- ✅ Automatic edge creation on successful connection
- ✅ Support for custom validation logic

```rust
// Custom validation example
let custom_validator = |connection: &Connection| -> bool {
    // Only allow connections between specific nodes
    connection.source != connection.target
};

view! {
    <Handle
        node_id=id
        r#type=HandleType::Source
        position=HandlePosition::Right
        connection_mode=ConnectionMode::Strict
        is_valid_connection=Some(custom_validator)
    />
}
```

## Connection Modes

### Strict Mode (Default)
Only allows source→target connections:
```rust
connection_mode=ConnectionMode::Strict
```

### Loose Mode
Allows source→source and target→target connections:
```rust
connection_mode=ConnectionMode::Loose
```

## Styling

Add CSS for handles and nodes:

```css
.xyflow__handle {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: 2px solid #1a192b;
    background: white;
    position: absolute;
    cursor: pointer;
}

.xyflow__handle.source {
    background: #555;
}

.xyflow__handle.target {
    background: #1a192b;
}

.xyflow__handle-left {
    left: -6px;
    top: 50%;
    transform: translateY(-50%);
}

.xyflow__handle-right {
    right: -6px;
    top: 50%;
    transform: translateY(-50%);
}
```

## Next Steps

- Check out the [Connection System Documentation](CONNECTION_SYSTEM.md)
- Explore the [examples](../examples/)
- Read the [API Documentation](https://docs.rs/xyflow-leptos)

