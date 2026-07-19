//! Hook for observing the in-flight connection drag

use leptos::prelude::*;
use crate::hooks::use_flow_store;
use crate::store::ConnectionState;

/// Reactive access to the connection currently being dragged, if any.
///
/// `Some(ConnectionState)` from pointerdown on a connectable handle until the
/// drop/cancel, updating as the cursor moves. The state carries the fixed end
/// (`from_node` / `from_handle` / `from_handle_type`), the current snap
/// [`candidate`](crate::store::ConnectionCandidate), and whether dropping on
/// that candidate would be `is_valid`.
///
/// This is the building block for styling handles during a drag — e.g.
/// dimming every handle that could not accept the connection, on both sides:
///
/// ```ignore
/// let connection = use_connection();
/// // In a node/handle view, derive per-handle compatibility from the drag's
/// // fixed end and the host's own domain rules:
/// let dimmed = Memo::new(move |_| {
///     connection.get().is_some_and(|conn| {
///         // this handle cannot take part in the in-flight connection
///         !host_compatible(&conn.from_node, conn.from_handle.as_deref(),
///                          &my_node_id, my_handle_id.as_deref())
///     })
/// });
/// view! { <Handle class:dimmed=move || dimmed.get() /* ... */ /> }
/// ```
///
/// The handle that is currently snapped also receives the `connectingto` +
/// `valid`/`invalid` CSS classes automatically (and the fixed end
/// `connectingfrom`), so simple styling needs no host code at all.
pub fn use_connection() -> Signal<Option<ConnectionState>> {
    let store = use_flow_store();
    let connection = store.state.connection_in_progress;
    Signal::derive(move || connection.get())
}
