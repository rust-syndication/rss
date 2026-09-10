# Releasing

This project follows [Semantic Versioning](https://semver.org/):

- Use a patch release for backward-compatible bug fixes.
- Use a minor release for backward-compatible public API additions. Treat an
  increased minimum supported Rust version as a minor release.
- Use a major release for incompatible public API changes.

## Prepare the release

1. Start a `release/X.Y.Z` branch from the latest `master`.
2. Update the package version with `cargo set-version X.Y.Z`.
3. Add a dated `X.Y.Z` section below `Unreleased` in `CHANGELOG.md`. Include all
   user-visible changes since the previous release and link their pull requests.
4. Commit only the version and release documentation changes with the subject
   `chore: release rss X.Y.Z`.
5. Run the release checks from a clean worktree:

   ```console
   cargo test --all-features --no-fail-fast --all
   cargo publish --dry-run
   ```

6. Open a pull request and wait for its CI checks to pass. Keep it as a draft
   until the intended release contents and version are settled.

## Publish the release

1. Merge the release pull request.
2. Check out the updated `master` and confirm that `Cargo.toml` and
   `CHANGELOG.md` contain the intended version.
3. Run `cargo publish --dry-run` once more from the clean release commit, then
   publish it with `cargo publish`.
4. Create a GitHub release and tag named `X.Y.Z` for the same commit. Use the
   changelog entry as the release notes, or generate equivalent notes on GitHub.
5. Confirm that the version is available on crates.io and that the GitHub tag
   points to the published commit.

Published crate versions cannot be overwritten, so resolve any version,
packaging, or CI problem before running `cargo publish`.
