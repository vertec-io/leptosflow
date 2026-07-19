//! Main SvelteFlow component

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::prelude::Closure;
use crate::types::{Node, Edge};
use crate::store::{FlowStore, WheelMode};
use crate::events::{
    use_context_menu_handler, use_flow_keydown_handler, use_pane_pan_handlers, use_wheel_handler,
};

/// The main flow component
///
/// This component sets up the flow context and renders the flow view.
///
/// # Props
///
/// * `nodes` - The flow nodes (as RwSignal)
/// * `edges` - The flow edges (as RwSignal)
/// * `children` - Child components to render (Controls, MiniMap, etc.)
///
/// # Example
///
/// ```ignore
/// #[component]
/// fn App() -> impl IntoView {
///     let nodes = RwSignal::new(vec![
///         Node::new("1".to_string(), Position::new(0.0, 0.0)),
///     ]);
///     let edges = RwSignal::new(vec![]);
///
///     view! {
///         <SvelteFlow nodes edges>
///             <Controls />
///             <Background />
///         </SvelteFlow>
///     }
/// }
/// ```
#[component]
pub fn SvelteFlow(
    /// Node signal (ignored when `store` is provided)
    #[prop(optional)] nodes: Option<RwSignal<Vec<Node>>>,
    /// Edge signal (ignored when `store` is provided)
    #[prop(optional)] edges: Option<RwSignal<Vec<Edge>>>,
    /// Pre-built store to use instead of creating one from the signals.
    /// Lets the consumer keep a handle for `fit_view`, drag-end callbacks, etc.
    #[prop(optional)] store: Option<FlowStore>,
    /// How a plain (no-modifier) wheel scroll drives the viewport. Reactive:
    /// pass a signal and toggling it flips the store's mode live. When omitted
    /// the store keeps its default ([`WheelMode::ZoomOnScroll`]).
    #[prop(optional, into)] wheel_mode: Option<Signal<WheelMode>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Use the provided store, or create one from the signals
    let store = store.unwrap_or_else(|| {
        FlowStore::from_signals(
            nodes.unwrap_or_else(|| RwSignal::new(Vec::new())),
            edges.unwrap_or_else(|| RwSignal::new(Vec::new())),
        )
    });

    // Provide the store to child components
    provide_context(store);

    // Reactively mirror the host's wheel-mode signal into the store so the
    // wheel handler (which reads it untracked) honors live toggles.
    if let Some(wheel_mode) = wheel_mode {
        Effect::new(move |_| {
            store.set_wheel_mode(wheel_mode.get());
        });
    }

    // Viewport gestures live on this untransformed container, NOT on the
    // transformed `.xyflow__viewport` element (which slides out from under
    // the cursor as soon as the view pans or zooms):
    // * wheel            -> pan; ctrl+wheel / trackpad pinch -> zoom at cursor
    // * primary-button   -> pan the pane after a small movement threshold
    //   drag                (nodes and handles stop propagation on
    //                       pointerdown, so their drags are unaffected;
    //                       clicks without movement pass through untouched)
    let (on_pane_pointer_down, on_pane_pointer_move, on_pane_pointer_up) =
        use_pane_pan_handlers();

    // Right-click: classify edge/node/pane and fire the registered
    // context-menu callback (native menu suppressed only when one exists).
    let on_context_menu = use_context_menu_handler();

    // Keyboard: Delete/Backspace requests selection deletion, Escape
    // cancels an in-flight connection. The container carries tabindex="0"
    // so clicking anywhere in the flow focuses it and keys arrive here.
    let on_keydown = use_flow_keydown_handler();

    // The wheel listener is attached manually with `passive: false`.
    // `on:wheel` cannot be used here: leptos delegates bubbling events to a
    // single window-level listener, and browsers force wheel listeners on
    // window/document/body to be passive — `preventDefault()` would be
    // ignored and the wheel would scroll/zoom the page instead of the flow.
    // A directly-attached element listener defaults to non-passive.
    let on_wheel = use_wheel_handler();
    let wheel_listener: StoredValue<Option<Closure<dyn FnMut(web_sys::WheelEvent)>>, LocalStorage> =
        StoredValue::new_local(None);

    Effect::new(move |_| {
        let Some(container) = store.state.container_ref.get() else {
            return;
        };
        // The container mounts once per SvelteFlow instance; guard anyway.
        if wheel_listener.with_value(|l| l.is_some()) {
            return;
        }
        let closure = Closure::<dyn FnMut(web_sys::WheelEvent)>::new({
            let on_wheel = on_wheel.clone();
            move |ev: web_sys::WheelEvent| on_wheel(ev)
        });
        let options = web_sys::AddEventListenerOptions::new();
        options.set_passive(false);
        let _ = container.add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            closure.as_ref().unchecked_ref(),
            &options,
        );
        wheel_listener.set_value(Some(closure));
    });

    on_cleanup(move || {
        wheel_listener.update_value(|slot| {
            if let Some(closure) = slot.take() {
                if let Some(container) = store.state.container_ref.get_untracked() {
                    let _ = container.remove_event_listener_with_callback(
                        "wheel",
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }
        });
    });

    view! {
        <div
            class="xyflow svelte-flow"
            node_ref=store.state.container_ref
            tabindex="0"
            on:pointerdown=on_pane_pointer_down
            on:pointermove=on_pane_pointer_move
            on:pointerup=on_pane_pointer_up.clone()
            on:pointercancel=on_pane_pointer_up
            on:contextmenu=on_context_menu
            on:keydown=on_keydown
        >
            {children.map(|children| children())}
        </div>
    }
}
