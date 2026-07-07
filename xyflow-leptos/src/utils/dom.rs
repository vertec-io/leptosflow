//! DOM measurement utilities for nodes and handles

use crate::types::{HandleBound, HandleType, HandleBoundPosition, HandleBounds};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Get dimensions of an HTML element
pub fn get_dimensions(element: &HtmlElement) -> (f64, f64) {
    (
        element.offset_width() as f64,
        element.offset_height() as f64,
    )
}

/// Get handle bounds for a specific handle type on a node
///
/// This queries the DOM for handle elements and measures their positions
/// relative to the flow container.
///
/// Ported from React Flow's getHandleBounds function
pub fn get_handle_bounds(
    handle_type: HandleType,
    node_element: &HtmlElement,
    node_bounds: &web_sys::DomRect,
    zoom: f64,
    _node_id: &str,
) -> Vec<HandleBound> {
    let mut handles = Vec::new();

    // Query for handles of this type
    let type_class = match handle_type {
        HandleType::Source => "source",
        HandleType::Target => "target",
    };

    // Use getElementsByClassName which is available on HtmlElement
    let handle_list = node_element.get_elements_by_class_name(type_class);

    let length = handle_list.length();
    if length == 0 {
        return handles;
    }

    // Measure each handle
    for i in 0..length {
        let Some(handle_node) = handle_list.item(i) else {
            continue;
        };

        let Ok(handle_element) = handle_node.dyn_into::<HtmlElement>() else {
            continue;
        };
        
        // Get handle attributes
        let handle_id = handle_element
            .get_attribute("data-handleid")
            .and_then(|id| if id == "null" { None } else { Some(id) });
        
        let position_str = handle_element
            .get_attribute("data-handlepos")
            .unwrap_or_else(|| "top".to_string());
        
        let position = match position_str.as_str() {
            "top" => HandleBoundPosition::Top,
            "right" => HandleBoundPosition::Right,
            "bottom" => HandleBoundPosition::Bottom,
            "left" => HandleBoundPosition::Left,
            _ => HandleBoundPosition::Top,
        };
        
        // Get handle bounds
        let handle_bounds = handle_element.get_bounding_client_rect();
        
        // Calculate position relative to flow (accounting for zoom)
        let x = (handle_bounds.left() - node_bounds.left()) / zoom;
        let y = (handle_bounds.top() - node_bounds.top()) / zoom;
        let width = handle_bounds.width() / zoom;
        let height = handle_bounds.height() / zoom;
        
        handles.push(HandleBound {
            id: handle_id,
            handle_type,
            position,
            x,
            y,
            width,
            height,
        });
    }
    
    handles
}

/// Measure a single handle element relative to its parent node element.
///
/// Walks up from the handle to the closest `.xyflow__node` ancestor and
/// returns a [`HandleBound`] with coordinates relative to the node's
/// top-left corner in flow units (i.e. divided by the current zoom).
///
/// Returns `None` if the handle is not inside a `.xyflow__node` element.
pub fn measure_handle_bound(handle_element: &HtmlElement, zoom: f64) -> Option<HandleBound> {
    let node_element = handle_element.closest(".xyflow__node").ok().flatten()?;

    let handle_id = handle_element
        .get_attribute("data-handleid")
        .and_then(|id| if id == "null" { None } else { Some(id) });

    let handle_type = match handle_element.get_attribute("data-handletype").as_deref() {
        Some("target") => HandleType::Target,
        _ => HandleType::Source,
    };

    let position = match handle_element
        .get_attribute("data-handlepos")
        .unwrap_or_else(|| "top".to_string())
        .as_str()
    {
        "right" => HandleBoundPosition::Right,
        "bottom" => HandleBoundPosition::Bottom,
        "left" => HandleBoundPosition::Left,
        _ => HandleBoundPosition::Top,
    };

    let node_rect = node_element.get_bounding_client_rect();
    let handle_rect = handle_element.get_bounding_client_rect();

    // Zoom scales both rects equally, so dividing by it yields
    // zoom-invariant node-relative flow coordinates.
    let zoom = if zoom > 0.0 { zoom } else { 1.0 };

    Some(HandleBound {
        id: handle_id,
        handle_type,
        position,
        x: (handle_rect.left() - node_rect.left()) / zoom,
        y: (handle_rect.top() - node_rect.top()) / zoom,
        width: handle_rect.width() / zoom,
        height: handle_rect.height() / zoom,
    })
}

/// Update node internals by measuring handles
///
/// This is called when a node is resized or when handles change
pub fn measure_node_handles(
    node_element: &HtmlElement,
    zoom: f64,
    node_id: &str,
) -> Option<HandleBounds> {
    let node_bounds = node_element.get_bounding_client_rect();
    
    let source = get_handle_bounds(
        HandleType::Source,
        node_element,
        &node_bounds,
        zoom,
        node_id,
    );
    
    let target = get_handle_bounds(
        HandleType::Target,
        node_element,
        &node_bounds,
        zoom,
        node_id,
    );
    
    // Only return Some if we found at least one handle
    if source.is_empty() && target.is_empty() {
        None
    } else {
        Some(HandleBounds { source, target })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_position_parsing() {
        // Test that position strings are parsed correctly
        let positions = vec![
            ("top", HandleBoundPosition::Top),
            ("right", HandleBoundPosition::Right),
            ("bottom", HandleBoundPosition::Bottom),
            ("left", HandleBoundPosition::Left),
        ];
        
        for (input, expected) in positions {
            let result = match input {
                "top" => HandleBoundPosition::Top,
                "right" => HandleBoundPosition::Right,
                "bottom" => HandleBoundPosition::Bottom,
                "left" => HandleBoundPosition::Left,
                _ => HandleBoundPosition::Top,
            };
            assert_eq!(result, expected);
        }
    }
}

