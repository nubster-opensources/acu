# Roadmap

| Version | Theme | Status |
| ------- | ----- | ------ |
| 0.0.1 | Cognitive core bootstrap | Shipped |
| 0.1.0 | Durable memory and first integration | In progress |
| 0.2.0 | Multi-session and substrate registry | Planned |

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
