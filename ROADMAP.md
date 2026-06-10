# ACU Roadmap

**Status: pre-alpha**

ACU (Autonomous Cognitive Unit) is a pure-Rust, event-sourced, hexagonal cognitive engine. The `Substrate` port is the single swappable reasoning boundary; every persistent change is an event replayed via `MindState::replay`.

---

## P0 - Cognitive kernel (DONE)

The foundational cognitive loop.

- [x] `MindState` - event-sourced state with full replay.
- [x] `step` - deterministic cognitive cycle: observe, reason, act.
- [x] `Substrate` port - interchangeable reasoning boundary.
- [x] `EventStore` port - pluggable event persistence boundary.
- [x] `ReflexSubstrate` adapter - rule-based reflex substrate (no LLM).
- [x] `InMemoryEventStore` adapter - in-process event store for tests and prototyping.
- [x] Workspace bootstrap: `crates/acu-core`, MIT OR Apache-2.0, Rust 1.88, edition 2024.

---

## P1 - LLM substrate adapter

Connect ACU to a large language model as its reasoning substrate.

- [ ] `LlmSubstrate` adapter backed by an HTTP client (model-agnostic, configurable endpoint).
- [ ] Prompt templating and structured output parsing.
- [ ] Token budget guard and retry policy.
- [ ] Feature flag: `substrate-llm` (optional dependency, off by default).

---

## P2 - Memory adapters

Persistent memory backends for the `EventStore` port and semantic retrieval.

- [ ] `MnemoEventStore` - durable event store backed by MnemoDB.
- [ ] `EidosMemory` - semantic memory layer backed by EidosDB (vector search, embedding calls out-of-engine).
- [ ] Snapshot and compaction strategy for long-lived cognitive units.

---

## P3 - Perception, effector ports, runtime and CLI

Full input-output surface and a standalone runtime.

- [ ] `Perception` port - typed sensor input boundary.
- [ ] `Effector` port - typed action output boundary.
- [ ] Async runtime: `tokio`-backed cognitive loop with graceful shutdown.
- [ ] `acu` CLI - launch, inspect and replay cognitive units from the command line.
- [ ] `acu-core` promoted to `0.1.0` once the async loop is stable.

---

## P4 - Authorization and audit

Production-grade access control and immutable audit trail.

- [ ] ThemisDB integration: policy enforcement on cognitive actions.
- [ ] StyxDB integration: append-only audit ledger for every cognitive event.
- [ ] `AuditEventStore` decorator wrapping any `EventStore` adapter.
