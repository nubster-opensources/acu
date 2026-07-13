# acu

> Event-sourced cognitive agent framework in Rust: hexagonal core, swappable reasoning substrate, append-only memory.

[![CI](https://github.com/nubster-opensources/acu/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/nubster-opensources/acu/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-blue.svg)](./docs/MSRV_POLICY.md)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Status](https://img.shields.io/badge/status-pre--alpha-red)](#status)
[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)

acu is a framework for building cognitive agents whose state is a deterministic replay of an
append-only event log. The reasoning substrate is a port: swap in reflex rules, a language
model, or anything else without touching the cognitive core.

acu is sponsored by [Nubster](https://nubster.com).

## Status

**Pre-alpha.** The core domain model is stable; the HTTP and LLM crates are experimental.
Breaking changes may occur on any `0.x` minor bump. See [SEMVER policy](./docs/SEMVER_POLICY.md).

No external contributions are accepted yet. Contribution guidelines will apply once the
project reaches `v0.1.0`.

## Quick start

Add the core crate to your `Cargo.toml`:

```toml
acu-core = "0.0.1"
```

Wire a reflex substrate and an in-memory store, then run one cognitive cycle:

```rust
use acu_core::{InMemoryEventStore, Percept, ReflexSubstrate, step};

let substrate = ReflexSubstrate;
let mut store = InMemoryEventStore::default();
let percept = Percept { utterance: "hello".to_string() };

// drive one cycle synchronously in a tokio runtime
let decision = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(step(&mut store, &substrate, percept))
    .unwrap();
println!("{}", decision.response); // "ack: hello"
```

## Why acu

Most agent libraries couple reasoning to a specific model API and mix state into mutable
variables scattered across handler functions. acu separates those concerns cleanly:

- **Append-only memory.** The agent state is a projection over an immutable event log. Any
  past state is reproducible by replaying the log from the beginning.
- **Substrate port.** Reasoning is a dependency, not a detail baked into the core. Reflex
  rules, language models, and stateful neuron models are all interchangeable adapters.
- **Hexagonal layout.** The cognitive domain depends on nothing technical. Storage and
  transport sit behind ports, so the core compiles with zero runtime dependencies.

## What acu is not

- A complete agent runtime. acu provides the cognitive core; scheduling, orchestration, and
  tool calling are not included.
- A model SDK. The `acu-substrate-llm` crate adapts one provider (Mistral) as an example.
  acu has no opinion on which model you use.
- Production-ready. The project is in pre-alpha; the API surface will change.

## Documentation

- [MSRV policy](./docs/MSRV_POLICY.md)
- [SEMVER policy](./docs/SEMVER_POLICY.md)
- [Roadmap](./docs/explanation/roadmap.md)
- Rustdoc (not yet published on docs.rs)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache License 2.0](./LICENSE-APACHE), at
your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in acu shall be dual-licensed as above, without any additional terms or conditions.
