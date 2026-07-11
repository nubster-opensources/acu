# Roadmap

| Version | Theme | Status |
| ------- | ----- | ------ |
| 0.0.1 | Cognitive core bootstrap | Shipped |
| 0.1.0 | Durable memory and first integration | In progress |
| 0.2.0 | Multi-session and substrate registry | Planned |
| 0.3.0 | Memory adapters (MnemoDB, EidosDB) | Planned |
| 0.4.0 | Perception, effector ports, runtime and CLI | Planned |
| 0.5.0 | Authorization and audit (ThemisDB, StyxDB) | Planned |

## 0.0.1 - Cognitive core bootstrap (Shipped)

Established the workspace skeleton and the three founding crates:

- `acu-core`: event-sourced `MindState`, `step` cycle, `Substrate` and
  `EventStore` ports, `ReflexSubstrate` and `InMemoryEventStore` adapters.
- `acu-substrate-llm`: `LanguageModel` port with a `MistralModel` adapter
  calling the Mistral chat completions API over HTTPS.
- `acu-http`: Axum router exposing `POST /chat` and streaming cognitive stages
  as Server-Sent Events.

Governance baseline: CI/CD, supply-chain checks, issue and PR templates.

## 0.1.0 - Durable memory and first integration (In progress)

Goals:

- Persistent `EventStore` backed by an append-only file or embedded database,
  so the agent survives process restarts.
- Public rustdoc coverage: all public items documented, doctests for the
  happy path.
- First end-to-end example: a CLI host that wires `acu-http` with a durable
  store and a `MistralModel`.

## 0.2.0 - Multi-session and substrate registry (Planned)

Goals:

- Named sessions: multiple independent cognitive threads, each with its own
  event log.
- Substrate registry: select the substrate by name at runtime, without
  recompiling.
- OpenTelemetry traces on the `step` cycle.

## 0.3.0 - Memory adapters (Planned)

Goals:

- `MnemoEventStore`: durable event store backed by MnemoDB.
- `EidosMemory`: semantic memory layer backed by EidosDB (vector search,
  embedding calls out-of-engine).
- Snapshot and compaction strategy for long-lived cognitive units.

## 0.4.0 - Perception, effector ports, runtime and CLI (Planned)

Goals:

- `Perception` port: typed sensor input boundary.
- `Effector` port: typed action output boundary.
- Async runtime: `tokio`-backed cognitive loop with graceful shutdown.
- `acu` CLI: launch, inspect and replay cognitive units from the command line.

## 0.5.0 - Authorization and audit (Planned)

Goals:

- ThemisDB integration: policy enforcement on cognitive actions.
- StyxDB integration: append-only audit ledger for every cognitive event.
- `AuditEventStore` decorator wrapping any `EventStore` adapter.
