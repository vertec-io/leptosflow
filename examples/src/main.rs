//! XYFlow Leptos Examples
//!
//! A comprehensive example demonstrating all features of xyflow-leptos,
//! with proper routing for each example organized by category.

mod examples;
mod shared;

use leptos::prelude::*;
use leptos::web_sys;
use leptos_router::components::{Router, Routes, Route, ParentRoute, Outlet};
use leptos_router::hooks::use_location;
use leptos_router::path;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use shared::{get_all_examples, PlaceholderExample};
use examples::basic::{BasicExample, EmptyExample, DefaultNodesExample, SwitchExample};
use examples::nodes::{CustomNodesExample, DefaultNodeOverwriteExample, NodeResizerExample, DragHandleExample, MovingHandlesExample, DetachedHandleExample, NodeTypeChangeExample, NodeTypesObjectChangeExample, UpdateNodeExample, UseUpdateNodeInternalsExample, BrokenNodesExample, NodeToolbarExample, UseNodesInitExample};
use examples::edges::{EdgeTypesExample, DefaultEdgeOverwriteExample, CustomEdgesExample, CustomConnectionLineExample, FloatingEdgesExample, EasyConnectExample, EdgeRendererExample, EdgeToolbarExample, EdgeRoutingExample};
use examples::connections::{ValidationExample, UseConnectionExample, CancelConnectionExample, ReconnectEdgeExample, AddNodeOnEdgeDropExample};
use examples::interactions::{InteractionsExample, UseOnSelectionChangeExample, UseNodeConnectionsExample, ClickDistanceExample, TouchDeviceExample, MultiSetNodesExample};
use examples::viewport::{ControlledViewportExample, ControlledUncontrolledExample, IntersectionExample, LayoutingExample};
use examples::minimap::{CustomMiniMapNodeExample, InteractiveMinimapExample, OverviewExample};
use examples::styling::{BackgroundsExample, ColorModeExample, HiddenExample};
use examples::state::{MiddlewaresExample, ReactiveStoresExample, SaveRestoreExample, SetNodesBatchingExample, UseNodesDataExample};
use examples::advanced::{A11yExample, FigmaExample, MultiFlowsExample, ProviderExample, StressExample, SubflowExample, UndirectionalExample};
use examples::hooks::{DevToolsExample, DragNDropExample, UseKeyPressExample, UseSvelteFlowExample, ZIndexModeExample};

// ============================================================================
// Navigation Sidebar
// ============================================================================

#[component]
fn NavSidebar() -> impl IntoView {
    let examples = get_all_examples();
    let collapsed_categories = RwSignal::new(std::collections::HashSet::<String>::new());
    let location = use_location();
    let search_query = RwSignal::new(String::new());
    let search_input_ref = NodeRef::<leptos::html::Input>::new();

    let toggle_category = move |category: String| {
        collapsed_categories.update(|set| {
            if set.contains(&category) {
                set.remove(&category);
            } else {
                set.insert(category);
            }
        });
    };

    // Calculate total examples count
    let total_count: usize = examples.iter().map(|(_, items)| items.len()).sum();

    // Clone examples for filtering
    let examples_for_filter = examples.clone();

    // Filter examples based on search query
    let filtered_examples = move || {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            examples_for_filter.clone()
        } else {
            examples_for_filter.iter()
                .filter_map(|(category, items)| {
                    let filtered_items: Vec<_> = items.iter()
                        .filter(|example| {
                            example.name.to_lowercase().contains(&query) ||
                            example.description.to_lowercase().contains(&query) ||
                            example.id.to_lowercase().contains(&query)
                        })
                        .cloned()
                        .collect();

                    if filtered_items.is_empty() {
                        None
                    } else {
                        Some((*category, filtered_items))
                    }
                })
                .collect()
        }
    };

    // Clone for filtered_count closure
    let examples_for_count = examples.clone();

    // Calculate filtered count
    let filtered_count = move || {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            examples_for_count.iter().map(|(_, items)| items.len()).sum::<usize>()
        } else {
            examples_for_count.iter()
                .filter_map(|(_, items)| {
                    let count = items.iter()
                        .filter(|example| {
                            example.name.to_lowercase().contains(&query) ||
                            example.description.to_lowercase().contains(&query) ||
                            example.id.to_lowercase().contains(&query)
                        })
                        .count();
                    if count > 0 { Some(count) } else { None }
                })
                .sum::<usize>()
        }
    };

    // Set up Ctrl+K keyboard shortcut
    let search_ref_for_effect = search_input_ref;
    Effect::new(move |_| {
        let input_ref = search_ref_for_effect;

        let handler = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            // Ctrl+K or Cmd+K to focus search
            if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "k" {
                ev.prevent_default();
                if let Some(input) = input_ref.get() {
                    let _ = input.focus();
                    let _ = input.select();
                }
            }
            // Escape to clear search and blur
            if ev.key() == "Escape" {
                if let Some(input) = input_ref.get() {
                    let _ = input.blur();
                }
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

    view! {
        <nav class="nav-sidebar">
            <div class="nav-header">
                <a href="/" class="nav-logo">"XYFlow Leptos"</a>
            </div>

            // Search input
            <div class="nav-search">
                <div class="nav-search-container">
                    <svg class="nav-search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="11" cy="11" r="8" />
                        <line x1="21" y1="21" x2="16.65" y2="16.65" />
                    </svg>
                    <input
                        type="text"
                        class="nav-search-input"
                        placeholder="Search examples..."
                        node_ref=search_input_ref
                        on:input=move |ev| {
                            search_query.set(event_target_value(&ev));
                        }
                        prop:value=move || search_query.get()
                    />
                    <span class="nav-search-shortcut">"⌘K"</span>
                </div>
                <div class="nav-search-stats">
                    {move || {
                        let count = filtered_count();
                        let query = search_query.get();
                        if query.is_empty() {
                            format!("{} examples", total_count)
                        } else {
                            format!("{} of {} examples", count, total_count)
                        }
                    }}
                </div>
            </div>

            <div class="nav-categories">
                {move || filtered_examples().into_iter().map(|(category, items)| {
                    let category_for_toggle = category.to_string();
                    let category_for_icon = category.to_string();
                    let category_for_style = category.to_string();
                    let items_len = items.len();

                    view! {
                        <div class="nav-category">
                            <button
                                class="nav-category-header"
                                on:click=move |_| toggle_category(category_for_toggle.clone())
                            >
                                <span class="nav-category-icon">
                                    {move || if collapsed_categories.get().contains(&category_for_icon) { "+" } else { "-" }}
                                </span>
                                <span class="nav-category-name">{category}</span>
                                <span class="nav-category-count">{items_len}</span>
                            </button>
                            <div class="nav-items" style=move || if collapsed_categories.get().contains(&category_for_style) { "display: none;" } else { "" }>
                                {items.iter().map(|example| {
                                    let href = format!("/examples/{}", example.id);
                                    let href_for_class = href.clone();
                                    let name = example.name.to_string();
                                    view! {
                                        <a
                                            href=href
                                            class=move || {
                                                let current_path = location.pathname.get();
                                                if current_path == href_for_class {
                                                    "nav-item active"
                                                } else {
                                                    "nav-item"
                                                }
                                            }
                                        >
                                            <span class="nav-item-name">{name}</span>
                                        </a>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </nav>
    }
}

// ============================================================================
// Layout Component
// ============================================================================

#[component]
fn Layout() -> impl IntoView {
    view! {
        <div class="app-layout">
            <NavSidebar />
            <main class="main-content">
                <Outlet />
            </main>
        </div>
    }
}

// ============================================================================
// Home Page
// ============================================================================

#[component]
fn HomePage() -> impl IntoView {
    let examples = get_all_examples();
    let total_count: usize = examples.iter().map(|(_, items)| items.len()).sum();

    view! {
        <div class="home-page">
            <header class="home-header">
                <h1>"XYFlow Leptos Examples"</h1>
                <p class="home-subtitle">
                    "A comprehensive collection of " {total_count} " examples demonstrating xyflow-leptos features"
                </p>
            </header>

            <div class="home-categories">
                {examples.into_iter().map(|(category, items)| {
                    view! {
                        <div class="home-category">
                            <h2 class="home-category-title">{category}</h2>
                            <div class="home-examples-grid">
                                {items.iter().map(|example| {
                                    let href = format!("/examples/{}", example.id);
                                    view! {
                                        <a href=href class="home-example-card">
                                            <h3 class="home-example-name">{example.name}</h3>
                                            <p class="home-example-desc">{example.description}</p>
                                        </a>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

// ============================================================================
// 404 Page
// ============================================================================

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found-page">
            <h1>"404"</h1>
            <p>"Example not found"</p>
            <a href="/" class="not-found-link">"Return to home"</a>
        </div>
    }
}

// ============================================================================
// Main App with Router
// ============================================================================

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <ParentRoute path=path!("/") view=Layout>
                    <Route path=path!("") view=HomePage />

                    // Basic examples
                    <Route path=path!("examples/basic") view=BasicExample />
                    <Route path=path!("examples/empty") view=EmptyExample />
                    <Route path=path!("examples/default-nodes") view=DefaultNodesExample />
                    <Route path=path!("examples/switch") view=SwitchExample />

                    // Node examples
                    <Route path=path!("examples/custom-node") view=CustomNodesExample />
                    <Route path=path!("examples/default-node-overwrite") view=DefaultNodeOverwriteExample />
                    <Route path=path!("examples/node-resizer") view=NodeResizerExample />
                    <Route path=path!("examples/drag-handle") view=DragHandleExample />
                    <Route path=path!("examples/moving-handles") view=MovingHandlesExample />
                    <Route path=path!("examples/detached-handle") view=DetachedHandleExample />
                    <Route path=path!("examples/node-type-change") view=NodeTypeChangeExample />
                    <Route path=path!("examples/node-types-object-change") view=NodeTypesObjectChangeExample />
                    <Route path=path!("examples/update-node") view=UpdateNodeExample />
                    <Route path=path!("examples/use-update-node-internals") view=UseUpdateNodeInternalsExample />
                    <Route path=path!("examples/broken-nodes") view=BrokenNodesExample />
                    <Route path=path!("examples/node-toolbar") view=NodeToolbarExample />
                    <Route path=path!("examples/use-nodes-init") view=UseNodesInitExample />

                    // Edge examples
                    <Route path=path!("examples/edge-types") view=EdgeTypesExample />
                    <Route path=path!("examples/default-edge-overwrite") view=DefaultEdgeOverwriteExample />
                    <Route path=path!("examples/custom-edges") view=CustomEdgesExample />
                    <Route path=path!("examples/custom-connection-line") view=CustomConnectionLineExample />
                    <Route path=path!("examples/floating-edges") view=FloatingEdgesExample />
                    <Route path=path!("examples/easy-connect") view=EasyConnectExample />
                    <Route path=path!("examples/edge-renderer") view=EdgeRendererExample />
                    <Route path=path!("examples/edge-toolbar") view=EdgeToolbarExample />
                    <Route path=path!("examples/edge-routing") view=EdgeRoutingExample />

                    // Connection examples
                    <Route path=path!("examples/validation") view=ValidationExample />
                    <Route path=path!("examples/use-connection") view=UseConnectionExample />
                    <Route path=path!("examples/cancel-connection") view=CancelConnectionExample />
                    <Route path=path!("examples/reconnect-edge") view=ReconnectEdgeExample />
                    <Route path=path!("examples/add-node-on-edge-drop") view=AddNodeOnEdgeDropExample />

                    // Interaction examples
                    <Route path=path!("examples/interactions") view=InteractionsExample />
                    <Route path=path!("examples/use-on-selection-change") view=UseOnSelectionChangeExample />
                    <Route path=path!("examples/use-node-connections") view=UseNodeConnectionsExample />
                    <Route path=path!("examples/click-distance") view=ClickDistanceExample />
                    <Route path=path!("examples/touch-device") view=TouchDeviceExample />
                    <Route path=path!("examples/multi-set-nodes") view=MultiSetNodesExample />

                    // Viewport examples
                    <Route path=path!("examples/controlled-viewport") view=ControlledViewportExample />
                    <Route path=path!("examples/controlled-uncontrolled") view=ControlledUncontrolledExample />
                    <Route path=path!("examples/intersection") view=IntersectionExample />
                    <Route path=path!("examples/layouting") view=LayoutingExample />

                    // Minimap examples
                    <Route path=path!("examples/custom-minimap-node") view=CustomMiniMapNodeExample />
                    <Route path=path!("examples/interactive-minimap") view=InteractiveMinimapExample />
                    <Route path=path!("examples/overview") view=OverviewExample />

                    // Styling examples
                    <Route path=path!("examples/backgrounds") view=BackgroundsExample />
                    <Route path=path!("examples/color-mode") view=ColorModeExample />
                    <Route path=path!("examples/hidden") view=HiddenExample />

                    // State examples
                    <Route path=path!("examples/save-restore") view=SaveRestoreExample />
                    <Route path=path!("examples/use-nodes-data") view=UseNodesDataExample />
                    <Route path=path!("examples/set-nodes-batching") view=SetNodesBatchingExample />
                    <Route path=path!("examples/reactive-stores") view=ReactiveStoresExample />
                    <Route path=path!("examples/middlewares") view=MiddlewaresExample />

                    // Advanced examples
                    <Route path=path!("examples/figma") view=FigmaExample />
                    <Route path=path!("examples/undirectional") view=UndirectionalExample />
                    <Route path=path!("examples/subflow") view=SubflowExample />
                    <Route path=path!("examples/multi-flows") view=MultiFlowsExample />
                    <Route path=path!("examples/provider") view=ProviderExample />
                    <Route path=path!("examples/a11y") view=A11yExample />
                    <Route path=path!("examples/stress") view=StressExample />

                    // Hooks examples
                    <Route path=path!("examples/use-svelte-flow") view=UseSvelteFlowExample />
                    <Route path=path!("examples/use-key-press") view=UseKeyPressExample />
                    <Route path=path!("examples/drag-n-drop") view=DragNDropExample />
                    <Route path=path!("examples/dev-tools") view=DevToolsExample />
                    <Route path=path!("examples/z-index-mode") view=ZIndexModeExample />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
