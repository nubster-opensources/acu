# Release policy

acu uses a semi-automated release pipeline driven by
[cargo-release](https://github.com/crate-ci/cargo-release) for version
bumping and a GitHub Actions workflow for publishing.

## Release process

1. **Prepare the release branch.**
   From `main`, create a `release/vX.Y.Z` branch and run:

   ```
   cargo release --execute minor   # or patch / major
   ```

   cargo-release bumps all workspace crate versions to the same number,
   graduates the `[Unreleased]` section in CHANGELOG.md to `[X.Y.Z] - DATE`,
   and opens a commit on the release branch. It does NOT publish, tag, or push
   (see `release.toml`).

2. **Review and merge the bump PR.**
   Open a pull request from `release/vX.Y.Z` to `main`. The CI must be green.
   Once merged, `main` carries the new version.

3. **Push the tag.**
   From a local clone of `main` (after pulling):

   ```
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```

   The `release.yml` workflow triggers on `v*` tag pushes and publishes to
   crates.io, then creates the GitHub Release with notes extracted from
   CHANGELOG.md.

4. **Verify the release.**
   Confirm the GitHub Release was created and all crates appear on crates.io
   at the expected version.

## Crate publish order

The workspace crates must be published in dependency order:

1. `acu-core`
2. `acu-substrate-llm` (depends on `acu-core`)
3. `acu-http` (depends on `acu-core`)

The `scripts/cargo-publish-idempotent.sh` helper skips a crate that is
already live at the target version, making the job safe to retry.

## Dry run

To test the publish step without uploading, trigger the `Release` workflow
manually from the Actions tab and set `dry_run` to `true`.

## Versioning

All workspace crates share the same version number at all times. See
[SEMVER_POLICY.md](SEMVER_POLICY.md) for the versioning rules and
[MSRV_POLICY.md](MSRV_POLICY.md) for MSRV bump rules.
