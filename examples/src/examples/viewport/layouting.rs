//! Layouting Example
//!
//! Demonstrates automatic graph layout algorithms:
//! - Hierarchical layout (TB, BT, LR, RL directions)
//! - Force-directed layout simulation
//! - Animated transitions to new positions
//! - Custom layout algorithms implemented in Rust

use leptos::prelude::*;
use leptos::serde_json::json;
use std::collections::HashMap;
use std::sync::OnceLock;
use xyflow_leptos::*;

// ============================================================================
// Drag State (global for this example)
// ============================================================================

static LAYOUTING_DRAG_STATE: OnceLock<RwSignal<Option<LayoutingDragState>>> = OnceLock::new();

#[derive(Clone, Debug)]
struct LayoutingDragState {
    node_id: String,
    start_mouse: (f64, f64),
    start_pos: (f64, f64),
}

fn get_drag_signal() -> RwSignal<Option<LayoutingDragState>> {
    *LAYOUTING_DRAG_STATE.get_or_init(|| RwSignal::new(None))
}

// ============================================================================
// Layout Direction Enum
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutDirection {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
}

impl LayoutDirection {
    fn label(&self) -> &'static str {
        match self {
            Self::TopToBottom => "Top to Bottom (TB)",
            Self::BottomToTop => "Bottom to Top (BT)",
            Self::LeftToRight => "Left to Right (LR)",
            Self::RightToLeft => "Right to Left (RL)",
        }
    }

    fn short_label(&self) -> &'static str {
        match self {
            Self::TopToBottom => "TB",
            Self::BottomToTop => "BT",
            Self::LeftToRight => "LR",
            Self::RightToLeft => "RL",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::TopToBottom => "↓",
            Self::BottomToTop => "↑",
            Self::LeftToRight => "→",
            Self::RightToLeft => "←",
        }
    }
}

// ============================================================================
// Layout Algorithm Type
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutAlgorithm {
    Hierarchical,
    ForceDirected,
    Grid,
}

impl LayoutAlgorithm {
    fn label(&self) -> &'static str {
        match self {
            Self::Hierarchical => "Hierarchical",
            Self::ForceDirected => "Force-Directed",
            Self::Grid => "Grid",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Hierarchical => "Layer nodes by graph depth with directed flow",
            Self::ForceDirected => "Physics simulation with attractive/repulsive forces",
            Self::Grid => "Arrange nodes in a regular grid pattern",
        }
    }
}

// ============================================================================
// Layout Algorithms
// ============================================================================

/// Calculate hierarchical layout positions
fn calculate_hierarchical_layout(
    nodes: &[Node],
    edges: &[Edge],
    direction: LayoutDirection,
    spacing_x: f64,
    spacing_y: f64,
) -> HashMap<String, Position> {
    let mut positions = HashMap::new();

    // Build adjacency list for graph traversal
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();

    for edge in edges {
        children
            .entry(edge.source.clone())
            .or_default()
            .push(edge.target.clone());
        parents
            .entry(edge.target.clone())
            .or_default()
            .push(edge.source.clone());
    }

    // Find root nodes (nodes with no incoming edges)
    let mut root_nodes: Vec<String> = nodes
        .iter()
        .filter(|n| !parents.contains_key(&n.id) || parents.get(&n.id).map(|p| p.is_empty()).unwrap_or(true))
        .map(|n| n.id.clone())
        .collect();

    // If no roots found, pick the first node
    if root_nodes.is_empty() && !nodes.is_empty() {
        root_nodes.push(nodes[0].id.clone());
    }

    // Calculate levels using BFS
    let mut node_levels: HashMap<String, usize> = HashMap::new();
    let mut queue: std::collections::VecDeque<(String, usize)> = std::collections::VecDeque::new();

    for root in &root_nodes {
        queue.push_back((root.clone(), 0));
        node_levels.insert(root.clone(), 0);
    }

    while let Some((node_id, level)) = queue.pop_front() {
        if let Some(node_children) = children.get(&node_id) {
            for child in node_children {
                if !node_levels.contains_key(child) || node_levels[child] < level + 1 {
                    node_levels.insert(child.clone(), level + 1);
                    queue.push_back((child.clone(), level + 1));
                }
            }
        }
    }

    // For nodes not reachable, assign level 0
    for node in nodes {
        if !node_levels.contains_key(&node.id) {
            node_levels.insert(node.id.clone(), 0);
        }
    }

    // Group nodes by level
    let max_level = node_levels.values().max().copied().unwrap_or(0);
    let mut levels: Vec<Vec<String>> = vec![Vec::new(); max_level + 1];

    for (node_id, level) in &node_levels {
        levels[*level].push(node_id.clone());
    }

    // Calculate positions based on direction
    let node_width = 150.0;
    let node_height = 60.0;

    for (level_idx, level_nodes) in levels.iter().enumerate() {
        let level_idx = level_idx as f64;
        let num_nodes = level_nodes.len() as f64;

        for (node_idx, node_id) in level_nodes.iter().enumerate() {
            let node_idx = node_idx as f64;

            // Center nodes within their level
            let offset = -(num_nodes - 1.0) / 2.0;

            let (x, y) = match direction {
                LayoutDirection::TopToBottom => {
                    let x = (offset + node_idx) * (node_width + spacing_x) + 400.0;
                    let y = level_idx * (node_height + spacing_y) + 50.0;
                    (x, y)
                }
                LayoutDirection::BottomToTop => {
                    let x = (offset + node_idx) * (node_width + spacing_x) + 400.0;
                    let y = (max_level as f64 - level_idx) * (node_height + spacing_y) + 50.0;
                    (x, y)
                }
                LayoutDirection::LeftToRight => {
                    let x = level_idx * (node_width + spacing_x) + 50.0;
                    let y = (offset + node_idx) * (node_height + spacing_y) + 300.0;
                    (x, y)
                }
                LayoutDirection::RightToLeft => {
                    let x = (max_level as f64 - level_idx) * (node_width + spacing_x) + 50.0;
                    let y = (offset + node_idx) * (node_height + spacing_y) + 300.0;
                    (x, y)
                }
            };

            positions.insert(node_id.clone(), Position::new(x, y));
        }
    }

    positions
}

/// Calculate force-directed layout using a simple simulation
fn calculate_force_layout(
    nodes: &[Node],
    edges: &[Edge],
    iterations: usize,
) -> HashMap<String, Position> {
    let mut positions: HashMap<String, (f64, f64)> = HashMap::new();

    // Initialize with current positions
    for node in nodes {
        positions.insert(node.id.clone(), (node.position.x, node.position.y));
    }

    // Parameters
    let repulsion = 10000.0;
    let attraction = 0.01;
    let damping = 0.95;
    let min_dist = 50.0;

    let mut velocities: HashMap<String, (f64, f64)> = nodes
        .iter()
        .map(|n| (n.id.clone(), (0.0, 0.0)))
        .collect();

    // Build edge set for quick lookup
    let edge_pairs: std::collections::HashSet<(String, String)> = edges
        .iter()
        .map(|e| (e.source.clone(), e.target.clone()))
        .collect();

    for _ in 0..iterations {
        let mut forces: HashMap<String, (f64, f64)> = nodes
            .iter()
            .map(|n| (n.id.clone(), (0.0, 0.0)))
            .collect();

        // Calculate repulsive forces between all pairs of nodes
        let node_ids: Vec<String> = nodes.iter().map(|n| n.id.clone()).collect();
        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                let id_i = &node_ids[i];
                let id_j = &node_ids[j];

                let pos_i = positions[id_i];
                let pos_j = positions[id_j];

                let dx = pos_j.0 - pos_i.0;
                let dy = pos_j.1 - pos_i.1;
                let dist = (dx * dx + dy * dy).sqrt().max(min_dist);

                // Repulsive force (Coulomb's law style)
                let force = repulsion / (dist * dist);
                let fx = force * dx / dist;
                let fy = force * dy / dist;

                forces.get_mut(id_i).map(|f| {
                    f.0 -= fx;
                    f.1 -= fy;
                });
                forces.get_mut(id_j).map(|f| {
                    f.0 += fx;
                    f.1 += fy;
                });
            }
        }

        // Calculate attractive forces along edges (spring force)
        for (source, target) in &edge_pairs {
            if let (Some(&pos_s), Some(&pos_t)) = (positions.get(source), positions.get(target)) {
                let dx = pos_t.0 - pos_s.0;
                let dy = pos_t.1 - pos_s.1;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);

                // Attractive force (Hooke's law)
                let force = attraction * dist;
                let fx = force * dx / dist;
                let fy = force * dy / dist;

                forces.get_mut(source).map(|f| {
                    f.0 += fx;
                    f.1 += fy;
                });
                forces.get_mut(target).map(|f| {
                    f.0 -= fx;
                    f.1 -= fy;
                });
            }
        }

        // Update velocities and positions
        for node in nodes {
            if let (Some(force), Some(vel)) = (forces.get(&node.id), velocities.get_mut(&node.id)) {
                vel.0 = (vel.0 + force.0) * damping;
                vel.1 = (vel.1 + force.1) * damping;

                // Limit velocity
                let max_vel = 50.0;
                let vel_mag = (vel.0 * vel.0 + vel.1 * vel.1).sqrt();
                if vel_mag > max_vel {
                    vel.0 = vel.0 * max_vel / vel_mag;
                    vel.1 = vel.1 * max_vel / vel_mag;
                }
            }

            if let (Some(pos), Some(vel)) = (positions.get_mut(&node.id), velocities.get(&node.id)) {
                pos.0 += vel.0;
                pos.1 += vel.1;

                // Keep nodes in bounds
                pos.0 = pos.0.max(50.0).min(700.0);
                pos.1 = pos.1.max(50.0).min(500.0);
            }
        }
    }

    // Convert to Position
    positions
        .into_iter()
        .map(|(id, (x, y))| (id, Position::new(x, y)))
        .collect()
}

/// Calculate grid layout
fn calculate_grid_layout(nodes: &[Node], columns: usize, spacing_x: f64, spacing_y: f64) -> HashMap<String, Position> {
    let mut positions = HashMap::new();
    let node_width = 150.0;
    let node_height = 60.0;

    for (idx, node) in nodes.iter().enumerate() {
        let row = idx / columns;
        let col = idx % columns;

        let x = col as f64 * (node_width + spacing_x) + 100.0;
        let y = row as f64 * (node_height + spacing_y) + 100.0;

        positions.insert(node.id.clone(), Position::new(x, y));
    }

    positions
}

// ============================================================================
// Animation Target State
// ============================================================================

#[derive(Clone, Debug)]
struct AnimationTarget {
    target_positions: HashMap<String, Position>,
    start_time: f64,
    duration: f64,
}

// ============================================================================
// Main Example Component
// ============================================================================

/// Layouting Example - Automatic graph layout algorithms
#[component]
pub fn LayoutingExample() -> impl IntoView {
    // Layout state
    let layout_direction = RwSignal::new(LayoutDirection::TopToBottom);
    let layout_algorithm = RwSignal::new(LayoutAlgorithm::Hierarchical);
    let is_animating = RwSignal::new(false);
    let layout_count = RwSignal::new(0_i32);

    // Action log
    let action_log = RwSignal::new(Vec::<(f64, String)>::new());
    let add_log = move |message: String| {
        action_log.update(|log| {
            log.push((js_sys::Date::now(), message));
            if log.len() > 15 {
                log.remove(0);
            }
        });
    };

    // Create initial nodes in a random layout
    let initial_nodes = vec![
        Node::new("a".to_string(), Position::new(100.0, 100.0))
            .with_data(json!({"label": "Start", "type": "input", "color": "#6ede87"})),
        Node::new("b".to_string(), Position::new(250.0, 50.0))
            .with_data(json!({"label": "Process A", "type": "default", "color": "#6865A5"})),
        Node::new("c".to_string(), Position::new(250.0, 200.0))
            .with_data(json!({"label": "Process B", "type": "default", "color": "#6865A5"})),
        Node::new("d".to_string(), Position::new(400.0, 100.0))
            .with_data(json!({"label": "Validate", "type": "default", "color": "#f0ad4e"})),
        Node::new("e".to_string(), Position::new(400.0, 250.0))
            .with_data(json!({"label": "Transform", "type": "default", "color": "#f0ad4e"})),
        Node::new("f".to_string(), Position::new(550.0, 150.0))
            .with_data(json!({"label": "Merge", "type": "default", "color": "#5bc0de"})),
        Node::new("g".to_string(), Position::new(700.0, 150.0))
            .with_data(json!({"label": "End", "type": "output", "color": "#ff6b6b"})),
    ];

    // Create edges
    let initial_edges = vec![
        Edge::new("e-ab".to_string(), "a".to_string(), "b".to_string()),
        Edge::new("e-ac".to_string(), "a".to_string(), "c".to_string()),
        Edge::new("e-bd".to_string(), "b".to_string(), "d".to_string()),
        Edge::new("e-ce".to_string(), "c".to_string(), "e".to_string()),
        Edge::new("e-df".to_string(), "d".to_string(), "f".to_string()),
        Edge::new("e-ef".to_string(), "e".to_string(), "f".to_string()),
        Edge::new("e-fg".to_string(), "f".to_string(), "g".to_string()),
    ];

    // Create the flow store
    let store = FlowStore::new(initial_nodes, initial_edges);
    provide_context(store);

    // Animation target positions
    let animation_target = RwSignal::new(None::<AnimationTarget>);

    // Apply layout function
    let add_log_for_layout = add_log.clone();
    let apply_layout = move || {
        let nodes = store.get_nodes();
        let edges = store.get_edges();
        let algorithm = layout_algorithm.get();
        let direction = layout_direction.get();

        let new_positions = match algorithm {
            LayoutAlgorithm::Hierarchical => {
                calculate_hierarchical_layout(&nodes, &edges, direction, 50.0, 80.0)
            }
            LayoutAlgorithm::ForceDirected => {
                calculate_force_layout(&nodes, &edges, 100)
            }
            LayoutAlgorithm::Grid => {
                calculate_grid_layout(&nodes, 3, 50.0, 80.0)
            }
        };

        // Start animation
        animation_target.set(Some(AnimationTarget {
            target_positions: new_positions,
            start_time: js_sys::Date::now(),
            duration: 500.0, // 500ms animation
        }));
        is_animating.set(true);
        layout_count.update(|c| *c += 1);

        add_log_for_layout(format!("{} layout applied ({})", algorithm.label(), direction.short_label()));
    };

    // Animation loop effect
    Effect::new(move |_| {
        if let Some(target) = animation_target.get() {
            let now = js_sys::Date::now();
            let elapsed = now - target.start_time;
            let progress = (elapsed / target.duration).min(1.0);

            // Ease-out cubic
            let eased = 1.0 - (1.0 - progress).powi(3);

            // Update node positions
            let nodes = store.get_nodes();
            for node in &nodes {
                if let Some(target_pos) = target.target_positions.get(&node.id) {
                    let new_x = node.position.x + (target_pos.x - node.position.x) * eased;
                    let new_y = node.position.y + (target_pos.y - node.position.y) * eased;

                    store.update_node(&node.id, |n| {
                        n.position = Position::new(new_x, new_y);
                    });
                }
            }

            if progress >= 1.0 {
                animation_target.set(None);
                is_animating.set(false);
            }
        }
    });

    // Drag signal
    let drag_signal = get_drag_signal();

    // Mouse move handler
    let on_canvas_mousemove = move |ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            let dx = ev.client_x() as f64 - drag_state.start_mouse.0;
            let dy = ev.client_y() as f64 - drag_state.start_mouse.1;

            store.update_node(&drag_state.node_id, |n| {
                n.position = Position::new(drag_state.start_pos.0 + dx, drag_state.start_pos.1 + dy);
            });
        }
    };

    // Mouse up handler
    let on_canvas_mouseup = move |_ev: leptos::ev::MouseEvent| {
        if let Some(drag_state) = drag_signal.get() {
            store.update_node(&drag_state.node_id, |n| {
                n.dragging = false;
            });
            drag_signal.set(None);
        }
    };

    // Randomize positions
    let add_log_for_random = add_log.clone();
    let randomize_positions = move || {
        let nodes = store.get_nodes();
        for node in &nodes {
            let x = js_sys::Math::random() * 600.0 + 50.0;
            let y = js_sys::Math::random() * 400.0 + 50.0;
            store.update_node(&node.id, |n| {
                n.position = Position::new(x, y);
            });
        }
        add_log_for_random("Positions randomized".to_string());
    };

    view! {
        <div class="example-container">
            <div class="xyflow leptos-flow layouting-example"
                 style="width: 100%; height: 100%; position: relative; background: #fafafa;"
                 on:mousemove=on_canvas_mousemove
                 on:mouseup=on_canvas_mouseup
                 on:mouseleave=move |_| {
                     if let Some(ds) = drag_signal.get() {
                         store.update_node(&ds.node_id, |n| n.dragging = false);
                         drag_signal.set(None);
                     }
                 }
            >
                // Background
                <Background variant=BackgroundVariant::Dots />

                // Flow viewport
                <FlowViewport store=store>
                    // Render edges
                    <LayoutingEdgeRenderer store=store direction=layout_direction />

                    // Render nodes
                    {move || {
                        store.get_nodes().into_iter().map(|node| {
                            view! {
                                <LayoutingNode node=node.clone() store=store />
                            }
                        }).collect_view()
                    }}
                </FlowViewport>

                // Controls
                <Controls position=PanelPosition::BottomLeft />

                // MiniMap
                <MiniMap position=PanelPosition::BottomRight />

                // Info Panel
                <Panel position=PanelPosition::TopRight>
                    <div style="background: white; padding: 16px; border-radius: 8px; max-width: 300px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                        <h3 style="margin: 0 0 12px 0; font-size: 16px; color: #333; display: flex; align-items: center; gap: 8px;">
                            <span style="display: inline-block; width: 8px; height: 8px; background: #667eea; border-radius: 50%;"></span>
                            "Graph Layouting"
                        </h3>

                        // Layout Algorithm Selection
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Layout Algorithm"</div>
                            <div style="display: flex; flex-direction: column; gap: 4px;">
                                {[LayoutAlgorithm::Hierarchical, LayoutAlgorithm::ForceDirected, LayoutAlgorithm::Grid]
                                    .into_iter()
                                    .map(|algo| {
                                        view! {
                                            <label style="display: flex; align-items: center; gap: 8px; padding: 6px 8px; background: #f5f5f5; border-radius: 4px; cursor: pointer; font-size: 12px;">
                                                <input
                                                    type="radio"
                                                    name="algorithm"
                                                    checked=move || layout_algorithm.get() == algo
                                                    on:change=move |_| layout_algorithm.set(algo)
                                                    style="margin: 0;"
                                                />
                                                <div>
                                                    <div style="font-weight: 500; color: #333;">{algo.label()}</div>
                                                    <div style="font-size: 10px; color: #888; margin-top: 2px;">{algo.description()}</div>
                                                </div>
                                            </label>
                                        }
                                    })
                                    .collect_view()
                                }
                            </div>
                        </div>

                        // Direction Selection (only for hierarchical)
                        {move || {
                            if layout_algorithm.get() == LayoutAlgorithm::Hierarchical {
                                view! {
                                    <div style="margin-bottom: 16px;">
                                        <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Direction"</div>
                                        <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 4px;">
                                            {[LayoutDirection::TopToBottom, LayoutDirection::BottomToTop, LayoutDirection::LeftToRight, LayoutDirection::RightToLeft]
                                                .into_iter()
                                                .map(|dir| {
                                                    view! {
                                                        <button
                                                            style=move || format!(
                                                                "padding: 6px 10px; border-radius: 4px; border: 1px solid {}; background: {}; color: {}; cursor: pointer; font-size: 11px; display: flex; align-items: center; gap: 4px; justify-content: center;",
                                                                if layout_direction.get() == dir { "#667eea" } else { "#ddd" },
                                                                if layout_direction.get() == dir { "#667eea" } else { "white" },
                                                                if layout_direction.get() == dir { "white" } else { "#333" }
                                                            )
                                                            on:click=move |_| layout_direction.set(dir)
                                                        >
                                                            <span style="font-size: 14px;">{dir.icon()}</span>
                                                            <span>{dir.short_label()}</span>
                                                        </button>
                                                    }
                                                })
                                                .collect_view()
                                            }
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}

                        // Apply Layout Button
                        <div style="margin-bottom: 16px;">
                            <button
                                style="width: 100%; padding: 10px 16px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 6px; font-weight: 600; cursor: pointer; font-size: 13px; display: flex; align-items: center; justify-content: center; gap: 8px;"
                                on:click=move |_| apply_layout()
                                disabled=move || is_animating.get()
                            >
                                {move || if is_animating.get() {
                                    view! {
                                        <span style="display: inline-block; width: 14px; height: 14px; border: 2px solid white; border-top-color: transparent; border-radius: 50%; animation: spin 1s linear infinite;"></span>
                                        "Animating..."
                                    }.into_any()
                                } else {
                                    view! {
                                        <span>"⚡"</span>
                                        "Apply Layout"
                                    }.into_any()
                                }}
                            </button>
                        </div>

                        // Quick Actions
                        <div style="margin-bottom: 16px;">
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Quick Actions"</div>
                            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 4px;">
                                <button
                                    style="padding: 6px 10px; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 11px;"
                                    on:click=move |_| randomize_positions()
                                >
                                    "🎲 Randomize"
                                </button>
                                <button
                                    style="padding: 6px 10px; border: 1px solid #ddd; border-radius: 4px; background: white; cursor: pointer; font-size: 11px;"
                                    on:click=move |_| {
                                        layout_algorithm.set(LayoutAlgorithm::Hierarchical);
                                        layout_direction.set(LayoutDirection::TopToBottom);
                                        add_log("Reset to defaults".to_string());
                                    }
                                >
                                    "↺ Reset"
                                </button>
                            </div>
                        </div>

                        // Stats
                        <div style="margin-bottom: 12px; padding: 10px; background: #f9f9f9; border-radius: 6px;">
                            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px; font-size: 12px;">
                                <div>
                                    <div style="color: #888; font-size: 10px;">"Layouts Applied"</div>
                                    <div style="font-weight: 600; color: #667eea;">{move || layout_count.get()}</div>
                                </div>
                                <div>
                                    <div style="color: #888; font-size: 10px;">"Nodes"</div>
                                    <div style="font-weight: 600; color: #333;">{move || store.get_nodes().len()}</div>
                                </div>
                            </div>
                        </div>

                        // Action Log
                        <div>
                            <div style="font-size: 12px; font-weight: 600; color: #555; margin-bottom: 8px;">"Action Log"</div>
                            <div style="max-height: 100px; overflow-y: auto; font-size: 11px; font-family: monospace; background: #fafafa; border-radius: 4px; padding: 8px;">
                                {move || {
                                    let log = action_log.get();
                                    if log.is_empty() {
                                        view! {
                                            <div style="color: #999; text-align: center; padding: 8px 0;">
                                                "Apply a layout to see actions..."
                                            </div>
                                        }.into_any()
                                    } else {
                                        log.iter().rev().map(|(time, msg)| {
                                            let time_str = format!("{:.0}", time % 100000.0);
                                            let msg = msg.clone();
                                            view! {
                                                <div style="margin-bottom: 4px; padding: 4px 6px; background: white; border-radius: 3px; display: flex; gap: 6px; align-items: center;">
                                                    <span style="color: #999; font-size: 9px;">{time_str}</span>
                                                    <span style="color: #333;">{msg}</span>
                                                </div>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </Panel>

                // Instructions badge
                <Panel position=PanelPosition::TopLeft>
                    <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 10px 16px; border-radius: 8px; box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);">
                        <div style="color: white; font-size: 11px; line-height: 1.5;">
                            <div style="font-weight: 600; margin-bottom: 4px;">"Automatic Layout"</div>
                            <div style="opacity: 0.9;">"• Select an algorithm"</div>
                            <div style="opacity: 0.9;">"• Choose direction (hierarchical)"</div>
                            <div style="opacity: 0.9;">"• Click Apply Layout"</div>
                            <div style="opacity: 0.9;">"• Watch animated transition"</div>
                        </div>
                    </div>
                </Panel>
            </div>

            // CSS for spinner animation
            <style>
                "@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }"
            </style>
        </div>
    }
}

// ============================================================================
// Layouting Node Component
// ============================================================================

#[component]
fn LayoutingNode(node: Node, store: FlowStore) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_render = node.id.clone();

    // Extract node data
    let label = node.data.get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("Node")
        .to_string();
    let color = node.data.get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6865A5")
        .to_string();
    let node_type = node.data.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();

    let drag_signal = get_drag_signal();

    // Mouse down handler
    let on_mousedown = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let nodes = store.get_nodes();
        if let Some(n) = nodes.iter().find(|n| n.id == node_id) {
            drag_signal.set(Some(LayoutingDragState {
                node_id: node_id.clone(),
                start_mouse: (ev.client_x() as f64, ev.client_y() as f64),
                start_pos: (n.position.x, n.position.y),
            }));

            store.update_node(&node_id, |n| {
                n.dragging = true;
            });
        }
    };

    // Get reactive position
    let pos = move || {
        store.get_nodes()
            .iter()
            .find(|n| n.id == node_id_for_render)
            .map(|n| n.position)
            .unwrap_or(Position::new(0.0, 0.0))
    };

    let has_source = node_type != "output";
    let has_target = node_type != "input";
    let color_for_border = color.clone();

    view! {
        <div
            class="xyflow__node"
            style=move || format!(
                "position: absolute; transform: translate({}px, {}px); cursor: grab; transition: transform 0.05s ease-out;",
                pos().x, pos().y
            )
            on:mousedown=on_mousedown
        >
            <div
                class="xyflow__node-default light"
                style=format!(
                    "background: {}; border: 2px solid {}; border-radius: 8px; padding: 12px 18px; min-width: 100px; text-align: center; box-shadow: 0 2px 8px rgba(0,0,0,0.1);",
                    color, color_for_border
                )
            >
                // Target handle
                {has_target.then(|| {
                    let node_id = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id
                            r#type=HandleType::Target
                            position=HandlePosition::Top
                            connection_mode=ConnectionMode::Strict
                        />
                    }
                })}

                <span style="font-weight: 600; color: white; text-shadow: 0 1px 2px rgba(0,0,0,0.2); font-size: 13px;">
                    {label}
                </span>

                // Source handle
                {has_source.then(|| {
                    let node_id = node.id.clone();
                    view! {
                        <Handle
                            node_id=node_id
                            r#type=HandleType::Source
                            position=HandlePosition::Bottom
                            connection_mode=ConnectionMode::Strict
                        />
                    }
                })}
            </div>
        </div>
    }
}

// ============================================================================
// Edge Renderer Component
// ============================================================================

#[component]
fn LayoutingEdgeRenderer(store: FlowStore, direction: RwSignal<LayoutDirection>) -> impl IntoView {
    view! {
        <svg
            class="edges-layer"
            style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; overflow: visible;"
        >
            <defs>
                <linearGradient id="layouting-edge-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
                    <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
                </linearGradient>
                <marker
                    id="layouting-arrow"
                    markerWidth="12"
                    markerHeight="12"
                    refX="10"
                    refY="6"
                    orient="auto"
                    markerUnits="userSpaceOnUse"
                >
                    <path d="M2,2 L10,6 L2,10 L4,6 Z" fill="#764ba2" />
                </marker>
            </defs>

            {move || {
                let edges = store.get_edges();
                let nodes = store.get_nodes();
                let dir = direction.get();

                edges.iter().map(|edge| {
                    let source_node = nodes.iter().find(|n| n.id == edge.source);
                    let target_node = nodes.iter().find(|n| n.id == edge.target);

                    if let (Some(source), Some(target)) = (source_node, target_node) {
                        let source_width = source.width.unwrap_or(120.0);
                        let source_height = source.height.unwrap_or(60.0);
                        let target_width = target.width.unwrap_or(120.0);
                        let target_height = target.height.unwrap_or(60.0);

                        // Calculate connection points based on layout direction
                        let (source_x, source_y, target_x, target_y) = match dir {
                            LayoutDirection::TopToBottom => {
                                let sx = source.position.x + source_width / 2.0;
                                let sy = source.position.y + source_height;
                                let tx = target.position.x + target_width / 2.0;
                                let ty = target.position.y;
                                (sx, sy, tx, ty)
                            }
                            LayoutDirection::BottomToTop => {
                                let sx = source.position.x + source_width / 2.0;
                                let sy = source.position.y;
                                let tx = target.position.x + target_width / 2.0;
                                let ty = target.position.y + target_height;
                                (sx, sy, tx, ty)
                            }
                            LayoutDirection::LeftToRight => {
                                let sx = source.position.x + source_width;
                                let sy = source.position.y + source_height / 2.0;
                                let tx = target.position.x;
                                let ty = target.position.y + target_height / 2.0;
                                (sx, sy, tx, ty)
                            }
                            LayoutDirection::RightToLeft => {
                                let sx = source.position.x;
                                let sy = source.position.y + source_height / 2.0;
                                let tx = target.position.x + target_width;
                                let ty = target.position.y + target_height / 2.0;
                                (sx, sy, tx, ty)
                            }
                        };

                        // Generate bezier path
                        let ctrl_offset = match dir {
                            LayoutDirection::TopToBottom | LayoutDirection::BottomToTop => {
                                (target_y - source_y).abs() * 0.4
                            }
                            LayoutDirection::LeftToRight | LayoutDirection::RightToLeft => {
                                (target_x - source_x).abs() * 0.4
                            }
                        };

                        let path = match dir {
                            LayoutDirection::TopToBottom => format!(
                                "M {} {} C {} {}, {} {}, {} {}",
                                source_x, source_y,
                                source_x, source_y + ctrl_offset,
                                target_x, target_y - ctrl_offset,
                                target_x, target_y
                            ),
                            LayoutDirection::BottomToTop => format!(
                                "M {} {} C {} {}, {} {}, {} {}",
                                source_x, source_y,
                                source_x, source_y - ctrl_offset,
                                target_x, target_y + ctrl_offset,
                                target_x, target_y
                            ),
                            LayoutDirection::LeftToRight => format!(
                                "M {} {} C {} {}, {} {}, {} {}",
                                source_x, source_y,
                                source_x + ctrl_offset, source_y,
                                target_x - ctrl_offset, target_y,
                                target_x, target_y
                            ),
                            LayoutDirection::RightToLeft => format!(
                                "M {} {} C {} {}, {} {}, {} {}",
                                source_x, source_y,
                                source_x - ctrl_offset, source_y,
                                target_x + ctrl_offset, target_y,
                                target_x, target_y
                            ),
                        };

                        view! {
                            <path
                                d=path
                                fill="none"
                                stroke="url(#layouting-edge-gradient)"
                                stroke-width="2"
                                marker-end="url(#layouting-arrow)"
                            />
                        }.into_any()
                    } else {
                        view! { <g></g> }.into_any()
                    }
                }).collect_view()
            }}
        </svg>
    }
}
