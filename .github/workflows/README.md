# CI/CD Documentation

## What This Does

Every push to main triggers automated testing, version management, crates.io publishing, and multi-platform binary distribution to GitHub Releases.

## The Big Picture

This project uses two complementary tools:

**release-plz** manages versions and publishes source code to crates.io. It reads your git commits, determines the next version number, updates Cargo.toml, and handles the publishing.

**cargo-dist** builds binaries for multiple platforms and creates GitHub Releases. It triggers automatically when release-plz creates a version tag.

These are separate because they solve different problems: release-plz handles version management and source distribution, cargo-dist handles binary distribution. Both are standard tools in the Rust ecosystem.

## How It Works

### The Linear Flow

**Step 1: You push code to main**

The `validate` job runs - formatting checks, clippy, build, and tests. This ensures code quality before anything else happens.

The `create-release-pr` job analyzes your commits using conventional commit format (fix:, feat:, etc.) and determines what version bump is needed. It creates or updates a pull request with the version change and generated changelog.

**Step 2: You merge the release PR**

The `publish-and-tag` job publishes your crate to crates.io and creates a git tag (like v0.2.0). This tag is the trigger point for binary builds.

**Step 3: Cargo-dist builds binaries automatically**

The tag triggers a separate workflow that builds binaries for five platforms in parallel: macOS (ARM + Intel), Linux (ARM + x64), and Windows (x64). When builds complete, it creates a GitHub Release and attaches all binaries.

### Why This Architecture

**Why two separate workflows?**

They trigger on different events. release-plz responds to code pushes and PR merges. cargo-dist responds to tag creation. Trying to combine them into one workflow would require complex conditional logic and lose the clean separation of concerns.

**Why does validate skip on PR merge?**

The release PR was already validated before merge. Running tests again wastes CI time and delays the release. The code hasn't changed between the last test run and the merge.

**Why does create-release-pr run after every push?**

release-plz is proactive. It creates a PR for the next version as soon as there are unreleased commits. This PR updates itself as you push more changes. When you're ready to release, just merge it.

**Why not just publish on every push?**

You might want to batch multiple changes into one release. The PR approach gives you explicit control over when versions are published while still automating the busy work of version bumping and changelog generation.

## Jobs Reference

### validate (release-plz.yml)

**When:** Push to main, PR updates (but NOT PR merge)
**Purpose:** Code quality gates
**What it does:** cargo fmt check, clippy with warnings as errors, build, test
**Why it exists:** Catch issues before they reach main or get released

### create-release-pr (release-plz.yml)

**When:** After validate passes on push to main
**Purpose:** Prepare the next release
**What it does:** Analyzes commits since last release, determines version bump, updates Cargo.toml and CHANGELOG.md, creates/updates PR
**Concurrency:** Only one PR update at a time (prevents race conditions)
**Why it exists:** Automates version management while giving you merge control

### publish-and-tag (release-plz.yml)

**When:** Release PR is merged
**Purpose:** Publish to crates.io and trigger binary builds
**What it does:** Publishes crate, creates git tag
**Why it exists:** Automated publishing with the tag serving as the handoff point to cargo-dist

### cargo-dist jobs (release.yml)

**When:** Version tag is created
**Purpose:** Multi-platform binary distribution
**What it does:** Builds binaries for five platforms, generates checksums, creates GitHub Release, uploads artifacts
**Why it exists:** Users can install pre-built binaries instead of compiling from source

## Configuration

### Secrets Required

**RELEASE_PLZ_TOKEN** - GitHub Personal Access Token with permissions for contents (write) and pull requests (write). Used for creating PRs and tags.

**CARGO_REGISTRY_TOKEN** - crates.io API token. Used for publishing packages.

Both are configured in repository settings under Secrets and variables → Actions.

### Version Bumping Rules

Uses conventional commits to determine version changes:

- fix: → patch (0.2.0 → 0.2.1)
- feat: → minor (0.2.0 → 0.3.0)
- BREAKING CHANGE: or ! suffix → major (0.2.0 → 1.0.0)
- Other types (chore:, docs:, etc.) → no version bump but included in changelog

### Platform Targets

Configured in dist-workspace.toml. Currently builds for five targets. Add or remove targets there, run `dist generate`, and commit both files. cargo-dist auto-generates the workflow YAML.

### Important: release.yml is auto-generated

Never edit .github/workflows/release.yml manually. It's generated by cargo-dist and excluded from YAML formatting hooks. To modify it, change dist-workspace.toml and regenerate.

## Common Scenarios

### Scenario: I pushed code and now there's a PR I didn't create

Normal behavior. release-plz created a release PR with your version bump. Review it, push more changes if needed (the PR updates automatically), or merge when ready.

### Scenario: I merged the release PR but no release appeared

Check the publish-and-tag job logs. Common issues: secrets not set correctly, crates.io token expired, version already published (can't republish same version).

### Scenario: Publishing succeeded but binary builds failed

The crates.io release is permanent (can't unpublish). Check which platform failed in the cargo-dist logs. You can delete the GitHub Release and tag, fix the issue, re-tag the same version, and cargo-dist will rebuild.

### Scenario: I want to release a specific version number

release-plz determines versions from conventional commits. To force a specific version, edit Cargo.toml manually in a PR, merge it, and release-plz will detect and publish that version.

### Scenario: There's a PR for v0.2.1 but v0.2.0 just released

You pushed commits after v0.2.0 was tagged. release-plz immediately creates a PR for the next version. This is expected. Leave it open and it will accumulate your changes, or close it if you don't plan to release soon.

## Maintenance

### Updating cargo-dist version

Change cargo-dist-version in dist-workspace.toml, run `dist generate`, commit both changed files.

### Changing what gets validated

Edit the validate job steps in release-plz.yml. The job runs standard Rust quality checks - modify as needed for your project.

### Disabling binary builds temporarily

Remove targets from dist-workspace.toml and regenerate, or add `if: false` to the release.yml workflow (but this gets overwritten on regeneration).

## Troubleshooting

### Tests pass locally but fail in CI

Check if you're using SQLX_OFFLINE mode correctly. The workflow sets SQLX_OFFLINE=true, which requires sqlx-data.json to be up-to-date. Run `just prepare-offline` before committing.

### release-plz says "no changes to release"

Your commits don't include any conventional commit types that trigger version bumps. Add at least one fix: or feat: commit.

### cargo-dist builds fail on specific platforms

Check the failed job logs. Common issues: dependency doesn't support that platform, Rust version mismatch. You can remove problematic targets from dist-workspace.toml.

### PR updates but version number seems wrong

release-plz analyzes commit messages. Check that your commits follow conventional format. A feat: commit followed by only chore: commits will show a minor bump, not a patch.

## Philosophy

This setup prioritizes:

**Automation over manual work** - No manual version bumping, changelog writing, or binary building.

**Explicit control over implicit behavior** - You explicitly merge the release PR rather than auto-publishing on every push.

**Separation of concerns** - Source distribution and binary distribution are separate pipelines.

**Standard tooling** - Uses established Rust ecosystem tools rather than custom scripts.

**Fast feedback** - Tests run on every push. Release PR appears immediately so you know what version you're working toward.

The tradeoff is complexity - two workflows instead of one, and you need to understand how they interact. The benefit is that each piece does one thing well and the whole system is maintainable using standard tools.
