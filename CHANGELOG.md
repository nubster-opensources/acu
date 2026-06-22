# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1] - 2026-06-10

### Added

- `acu-core`: workspace skeleton, `MindState` event-sourced replay via `step` cycle,
  `Substrate` and `EventStore` ports, `ReflexSubstrate` and `InMemoryEventStore` adapters.
- `acu-substrate-llm`: language-model substrate with a `LanguageModel` port; `MistralModel`
  adapter calls the Mistral chat completions API over HTTPS with a configurable timeout.
- `acu-http`: Axum router that drives one cognitive cycle per `POST /chat` request and
  streams each stage as Server-Sent Events; the host wires the substrate and event store.
- Governance: `.editorconfig`, `.gitattributes`, `deny.toml`, `lefthook.yml`, `cog.toml`,
  CI/CD workflows (fmt, clippy, test, supply-chain, msrv-check, bump, release, docs),
  issue templates and PR template.

[Unreleased]: https://github.com/nubster-opensources/acu/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/nubster-opensources/acu/releases/tag/v0.0.1
