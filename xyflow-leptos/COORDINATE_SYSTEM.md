# Coordinate System in XYFlow Leptos

## Overview

The Leptos port of XYFlow uses the same coordinate system as React Flow, with handle positions stored **relative to the node's top-left corner**, not in absolute flow coordinates.

## Key Concepts

### 1. Handle Bounds Storage

Handle bounds are measured and stored with coordinates **relative to the node element**:

```rust
pub struct HandleBound {
    /// X coordinate relative to the node's top-left corner
    pub x: f64,
    /// Y coordinate relative to the node's top-left corner  
    pub y: f64,
    pub width: f64,
    pub height: f64,
    // ...
}
```

This matches React Flow's implementation in `packages/system/src/utils/dom.ts`:
```typescript
x: (handleBounds.left - nodeBounds.left) / zoom,
y: (handleBounds.top - nodeBounds.top) / zoom,
```

### 2. Converting to Absolute Coordinates

When finding the closest handle during connection creation, we convert relative coordinates to absolute flow coordinates by adding the node's position:

```rust
// In HandleBound
pub fn center_absolute(&self, node_position: &Position) -> Position {
    Position::new(
        node_position.x + self.x + self.width / 2.0,
        node_position.y + self.y + self.height / 2.0,
    )
}
```

This matches React Flow's `getHandlePosition` in `packages/system/src/utils/edges/positions.ts`:
```typescript
const x = (handle?.x ?? 0) + node.internals.positionAbsolute.x;
const y = (handle?.y ?? 0) + node.internals.positionAbsolute.y;
```

### 3. Mouse Position Conversion

Mouse coordinates from browser events are in **screen space** (relative to the viewport). We convert them to **flow space** by:

1. Subtracting the flow container's position on screen
2. Applying viewport transform (pan/zoom)

```rust
pub fn screen_to_flow_position(screen_x: f64, screen_y: f64, viewport: &Viewport) -> Position {
    // Find flow container position
    let (container_x, container_y) = get_container_position();
    
    // Convert to flow coordinates
    let (x, y) = CoordinateSystem::screen_to_flow(
        screen_x, screen_y, *viewport, container_x, container_y
    );
    Position::new(x, y)
}
```

This matches React Flow's approach in `packages/system/src/utils/dom.ts`:
```typescript
const pointerPos = pointToRendererPoint(
    { x: x - (containerBounds?.left ?? 0), y: y - (containerBounds?.top ?? 0) },
    transform
);
```

## Container Reference Using NodeRef

The Leptos implementation uses Leptos's `NodeRef` API to store a reference to the flow container element in the store. This provides efficient access to container bounds without DOM queries:

```rust
// In FlowState
pub container_ref: NodeRef<html::Div>,

// In the view
<div node_ref=store.state.container_ref ...>
```

When converting coordinates, we use the NodeRef directly:

```rust
pub fn screen_to_flow_position_with_ref(
    screen_x: f64, screen_y: f64, viewport: &Viewport, container_ref: NodeRef<html::Div>
) -> Position {
    let (container_x, container_y) = if let Some(element) = container_ref.get() {
        let rect = element.get_bounding_client_rect();
        (rect.left(), rect.top())
    } else {
        (0.0, 0.0)
    };
    // ... convert coordinates
}
```

This matches React Flow's approach of passing container bounds to event handlers, but uses Leptos's reactive NodeRef instead of prop drilling.

## Deviations from React Flow

### 1. Event Handler Registration

**React Flow**: Registers global event handlers at the flow container level.

**Leptos**: Each Handle component registers its own window-level event handlers, but they check if they should run:
```rust
if conn.from_node != node_id || conn.from_handle != handle_id || conn.from_handle_type != handle_type {
    return; // Not our connection
}
```

**Reason**: Leptos's component model and reactive system make per-component event handlers more natural.

**Trade-off**: Multiple event handlers are registered, but only one runs per connection. This is acceptable and doesn't impact performance.

### 2. Connection State

**React Flow**: Stores connection state in a global store without handle type.

**Leptos**: Stores `from_handle_type` in `ConnectionState`:
```rust
pub struct ConnectionState {
    pub from_node: String,
    pub from_handle: Option<String>,
    pub from_handle_type: HandleType, // Added in Leptos
    // ...
}
```

**Reason**: Needed to identify which handle started the connection when multiple handles register event handlers.

**Trade-off**: Slightly larger state object, but necessary for the per-component event handler approach.

## Testing

The coordinate system can be tested by:
1. Creating nodes at different positions
2. Dragging connections between handles
3. Verifying edges are created correctly
4. Testing with viewport pan/zoom

See `packages/leptos/examples/connections` for a working example.

## Future Improvements

1. **Global Event Handlers**: Move to a single set of global event handlers like React Flow
2. **Position Absolute**: Add `positionAbsolute` to node internals for nested flows (parent/child nodes)

