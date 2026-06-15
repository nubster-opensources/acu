//! The cognitive cycle must yield a `Send` future from the port contract alone.
//!
//! Joel serves the agent on a multi-threaded async runtime, where driving a cycle from a request
//! handler means handing its future to the executor, and that requires `Future: Send`. A generic
//! call site only knows `S: Substrate`, so the guarantee must live in the port, not leak from a
//! concrete type. This test walks that generic path: it compiles only if the `Substrate` future
//! is `Send`.

use acu_core::{EventStore, InMemoryEventStore, Percept, ReflexSubstrate, Substrate, step};

fn assert_send<T: Send>(_value: T) {}

/// Mirrors Joel's generic spawn site: bounded only by the ports, it must still observe a `Send`
/// cycle future. The auxiliary `Sync`/`Send` bounds cover the borrowed handles, leaving the
/// substrate's future as the single value whose `Send`-ness can only come from the port contract.
fn the_cycle_future_is_send_for_any_substrate<S, E>(store: &mut E, substrate: &S, percept: Percept)
where
    S: Substrate + Sync,
    E: EventStore + Send,
{
    assert_send(step(store, substrate, percept));
}

#[test]
fn the_cognitive_cycle_yields_a_send_future() {
    let mut store = InMemoryEventStore::new();
    let substrate = ReflexSubstrate;

    the_cycle_future_is_send_for_any_substrate(
        &mut store,
        &substrate,
        Percept {
            utterance: "ping".to_string(),
        },
    );
}
