# XYFlow Leptos Demo

This is a standalone demo application showcasing the connection creation system.

## Quick Start

### Prerequisites

```bash
# Install Rust and cargo-leptos
cargo install cargo-leptos

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Running the Demo

```bash
# From this directory (packages/leptos/demo)
cargo leptos serve
```

Then open your browser to `http://localhost:3000`

## What This Demo Shows

- ✅ **Connection Creation**: Click and drag from source handles (right) to target handles (left)
- ✅ **Real-time Validation**: See visual feedback during connection creation
- ✅ **Automatic Edge Creation**: Edges are created automatically on successful connection
- ✅ **Strict Mode**: Only source→target connections are allowed
- ✅ **Live Edge List**: See the list of edges update in real-time

## Features Demonstrated

1. **Custom Nodes with Handles**
   - Input node (only source handle)
   - Process node (both source and target handles)
   - Output node (only target handle)

2. **Connection Validation**
   - Type checking (source→target only in strict mode)
   - Self-loop prevention
   - Same-handle prevention

3. **Reactive State Management**
   - Leptos signals for reactive updates
   - Real-time edge list display
   - Connection state tracking

## Code Structure

```
demo/
├── Cargo.toml          # Demo app dependencies
├── src/
│   └── main.rs         # Demo application code
└── README.md           # This file
```

## Customization

You can modify `src/main.rs` to:
- Add more nodes
- Change connection modes (Strict/Loose)
- Add custom validation logic
- Style the nodes and handles
- Add more features

## Troubleshooting

### Port already in use

If port 3000 is already in use, you can specify a different port:

```bash
cargo leptos serve --port 3001
```

### WASM compilation errors

Make sure you have the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

### cargo-leptos not found

Install it:
```bash
cargo install cargo-leptos
```

## Next Steps

After exploring the demo:
1. Check out the [Quick Start Guide](../docs/QUICKSTART.md)
2. Read the [Connection System Documentation](../docs/CONNECTION_SYSTEM.md)
3. Explore the [examples](../examples/)

