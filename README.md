# ACU

> Autonomous Cognitive Unit: an event-sourced cognitive core in pure Rust, hexagonal, with a swappable reasoning substrate.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-red.svg)](#status)

## Status

**Pre-alpha.** Heavy development. Not ready for production use.

## Mission

ACU is a cognitive engine whose mind is rebuilt by replaying what it has lived. State is never written in place: it is a projection over an append-only event log. The reasoning itself sits behind a single port, the `Substrate`, so the same core can run on a language model today and on a stateful learning substrate tomorrow, without rewriting the centre.

## What makes it different

- **Event-sourced cognition.** Every percept and decision is an immutable fact. The agent's state is a deterministic replay, which gives it a lived, auditable memory and a traceable personality.
- **Swappable substrate.** The `Substrate` port decouples reasoning from the core. Reflex rules, a language model, or a stateful neuron model are all just adapters.
- **Hexagonal and sovereign.** The cognitive domain depends on nothing technical. Storage, semantic memory, authorization, and audit all sit behind ports, ready to bind to the Nubster data plane as it matures.

## Architecture

Hexagonal. The pure cognitive domain lives in `crates/acu-core` and defines the ports it needs. Adapters and runtime wiring are added as concrete needs appear.

## License

Licensed under either of MIT or Apache 2.0 at your option.
