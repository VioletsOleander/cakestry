# Release Workflow

Assume we want to release `<version>`, for example `0.1.0`. The release workflow is:

1. Run `git switch -c` to create a new branch on top of current commit.
2. Run `cargo set-version <version>` or `cargo set-version --bump` to update the version field in
    `Cargo.toml`.
3. Run `git-cliff --tag <version> | preserve-summaries` to generate new `CHANGELOG.md`.
4. Manually add new `# Summary` section to the newly generated `CHANGELOG.md`.
5. Stage and commit `Cargo.toml` and `CHANGELOG.md`, the commit message should be "chore: release
    v<version>". For example: "chore: release v0.1.1".
6. Open a new pull request on current branch, and merge that pull request.
7. Run `git switch main` and `git pull` to pull the merged commit into the main branch.
8. Run `git tag v<version>` to add a new tag, and push that tag. After the tag is pushed to
    GitHub, CI will take over anything left for a GitHub release.
9. Run `cargo publish` to publish the crate to `crates.io`.
