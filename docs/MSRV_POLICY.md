# Minimum Supported Rust Version (MSRV) policy

The current MSRV is **Rust 1.88** (stable channel).

The MSRV is pinned in `rust-toolchain.toml` at the repository root and
declared in `Cargo.toml` via `rust-version = "1.88"`.

## How the MSRV evolves

- acu does not commit to supporting Rust versions older than 1.88.
- An MSRV bump is treated as a **minor** version bump per the
  [semver policy](SEMVER_POLICY.md). For example, raising the MSRV from 1.88
  to 1.92 ships in a `0.X.0` release (or `X.0.0` once at 1.0).
- The current MSRV is documented in CHANGELOG.md under the `Changed` section
  of the release that bumps it.

## Why we pick the floor we pick

- **1.88** is required because acu uses Rust edition 2024 features, including
  `async fn` in traits stabilised in that edition.
- Future bumps will be driven by concrete features the crates need, not by
  chasing the latest stable.

## How we verify the MSRV in CI

The CI has two distinct toolchain tracks:

- The `test` job (format, clippy, test suite, doc build) runs on the current
  **stable** toolchain so that the crates always work on the latest stable
  release.
- The dedicated `msrv-check` job pins **1.88** and runs
  `cargo check --workspace --all-features`, which guarantees that no feature
  requiring a newer compiler has crept in.

## Downstream impact

If you depend on any acu crate, the dependency resolver will refuse to compile
your project on a Rust version older than the MSRV. You can pin to an older
version only if that version supported your Rust version, as documented in
CHANGELOG.md.
