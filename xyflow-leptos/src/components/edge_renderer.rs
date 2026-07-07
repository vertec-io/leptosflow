//! Edge renderer component

use leptos::prelude::*;
use crate::hooks::use_flow_store;
use crate::types::{HandleBoundPosition, Node, Position};
use crate::utils::edge_path::{
    calculate_label_position_oriented, generate_edge_path_oriented, EdgePathType,
};
use crate::components::markers::MarkerDefinitions;

/// Get the anchor position and orientation for an edge endpoint
///
/// If handle bounds are available (measured from the DOM by the `Handle`
/// component), uses the exact handle center and its orientation.
/// Otherwise, falls back to node edge positions based on handle type.
fn get_handle_position(
    node: &Node,
    handle_id: &Option<String>,
    is_source: bool,
) -> (Position, HandleBoundPosition) {
    let node_pos = &node.position;
    let node_width = node.width.unwrap_or(150.0);
    let node_height = node.height.unwrap_or(40.0);

    // Try to find handle bounds
    if let Some(ref bounds) = node.internals.handle_bounds {
        let handles = if is_source { &bounds.source } else { &bounds.target };

        // Find matching handle by ID, or use first handle
        let handle = if let Some(id) = handle_id {
            handles.iter().find(|h| h.id.as_ref() == Some(id))
        } else {
            handles.first()
        };

        if let Some(handle) = handle {
            // Handle bounds are relative to node, convert to absolute
            return (handle.center_absolute(node_pos), handle.position);
        }
    }

    // Fallback: use node edge positions based on handle type
    // Source handles are typically at the bottom, target handles at the top
    if is_source {
        (
            Position::new(node_pos.x + node_width / 2.0, node_pos.y + node_height),
            HandleBoundPosition::Bottom,
        )
    } else {
        (
            Position::new(node_pos.x + node_width / 2.0, node_pos.y),
            HandleBoundPosition::Top,
        )
    }
}
/// Renders a single edge reactively
///
/// This component renders an edge as an SVG path and updates when nodes move.
#[component]
fn EdgeComponent(
    /// The edge ID (used to look up edge data)
    edge_id: String,
    /// Source node ID
    source_id: String,
    /// Target node ID
    target_id: String,
) -> impl IntoView {
    let store = use_flow_store();
    let store_click = store.clone();

    // Create a reactive memo that recalculates when nodes change
    let path_data = Memo::new({
        let store = store.clone();
        let edge_id = edge_id.clone();
        let source_id = source_id.clone();
        let target_id = target_id.clone();
        move |_| {
            // This will track the nodes signal
            let nodes = store.get_nodes();
            let edges = store.get_edges();

            // Find source and target nodes
            let source = nodes.iter().find(|n| n.id == source_id);
            let target = nodes.iter().find(|n| n.id == target_id);
            let edge = edges.iter().find(|e| e.id == edge_id);

            if let (Some(source), Some(target), Some(edge)) = (source, target, edge) {
                let (source_pos, source_orient) =
                    get_handle_position(source, &edge.source_handle, true);
                let (target_pos, target_orient) =
                    get_handle_position(target, &edge.target_handle, false);

                let path_type = match edge.edge_type.as_deref() {
                    Some("straight") => EdgePathType::Straight,
                    Some("step") => EdgePathType::Step,
                    Some("smoothstep") => EdgePathType::SmoothStep,
                    Some("simplebezier") => EdgePathType::SimpleBezier,
                    _ => EdgePathType::Bezier,
                };

                let path = generate_edge_path_oriented(
                    source_pos, source_orient, target_pos, target_orient, path_type,
                );
                let label_pos = calculate_label_position_oriented(
                    source_pos, source_orient, target_pos, target_orient, path_type,
                );

                (path, label_pos.x, label_pos.y)
            } else {
                (String::new(), 0.0, 0.0)
            }
        }
    });

    // Edge class (reactive for selection state)
    let edge_class = Memo::new({
        let store = store.clone();
        let edge_id = edge_id.clone();
        move |_| {
            let edges = store.get_edges();
            let edge = edges.iter().find(|e| e.id == edge_id);

            let mut classes = vec!["xyflow__edge", "leptos-flow__edge"];
            if let Some(edge) = edge {
                if edge.selected {
                    classes.push("selected");
                }
                if edge.animated {
                    classes.push("animated");
                }
                if edge.hidden {
                    classes.push("hidden");
                }
            }
            classes.join(" ")
        }
    });

    // Get edge label
    let edge_label = Memo::new({
        let store = store.clone();
        let edge_id = edge_id.clone();
        move |_| {
            let edges = store.get_edges();
            edges.iter().find(|e| e.id == edge_id).and_then(|e| e.label.clone())
        }
    });

    // Click handler for selection
    let on_click = {
        let edge_id = edge_id.clone();
        move |ev: leptos::ev::MouseEvent| {
            ev.stop_propagation();
            let multi_select = ev.ctrl_key() || ev.meta_key();
            store_click.select_edge(&edge_id, multi_select);
        }
    };

    view! {
        <g class=move || edge_class.get() data-id=edge_id.clone() on:click=on_click>
            <path
                class="xyflow__edge-path"
                d=move || path_data.get().0
                attr:marker-end="url(#xyflow__arrowclosed)"
            />
            {move || {
                let (_, label_x, label_y) = path_data.get();
                edge_label.get().map(|label| {
                    view! {
                        <text
                            class="xyflow__edge-label"
                            attr:x=format!("{}", label_x)
                            attr:y=format!("{}", label_y)
                            attr:text-anchor="middle"
                            attr:dominant-baseline="middle"
                            attr:font-size="12"
                            attr:fill="#555"
                        >
                            {label}
                        </text>
                    }
                })
            }}
        </g>
    }
}

/// Renders all edges in the flow
///
/// This component iterates over all edges and renders them as SVG paths.
#[component]
pub fn EdgeRenderer() -> impl IntoView {
    let store = use_flow_store();

    // Get edges reactively
    let store_for_edges = store.clone();
    let edges = move || store_for_edges.get_edges();

    view! {
        <svg class="xyflow__edges leptos-flow__edges" style="position: absolute; width: 100%; height: 100%; pointer-events: none;">
            <MarkerDefinitions />
            <For
                each=edges
                key=|edge| edge.id.clone()
                children=move |edge| {
                    // Pass IDs so EdgeComponent can look up nodes reactively
                    view! {
                        <EdgeComponent
                            edge_id=edge.id.clone()
                            source_id=edge.source.clone()
                            target_id=edge.target.clone()
                        />
                    }
                }
            />
        </svg>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_edge_renderer_exists() {
        // Placeholder test - real tests need browser environment
        assert!(true);
    }
}

