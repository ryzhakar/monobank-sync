# monobank sync tool

## Overview
This tool synchronizes data from Monobanks Personal API into a local SQLite database.
Meant as a companion app for [monobank-report](https://github.com/ryzhakar/monobank-report) tool, which ... should make useful reports based on this data.

Sync is pretty slow due to rate-limiting, but acceptable for a cron task. Expect spending `months * cards * tokens + tokens` minutes on each run.

## Configuration and Operation
Configure the tool by setting the necessary environment variables in the `.env` file at the project's root:

- `MULTIPLE_MONOBANK_TOKENS`: Monobank API tokens, comma-separated.
- `DATABASE_URL`: Connection string for your database.
- `ALLOWED_CARD_TYPES`: Filter transactions by card types, comma-separated.
- `SYNC_START_TIMESTAMP`: Initial sync date; defaults to the start of the current month if unspecified.

## Development

This project uses [just](https://github.com/casey/just) for cumbersome repetitive tasks. For standard Rust development, use `cargo` commands directly (`cargo build`, `cargo test`, `cargo fmt`, `cargo clippy`, etc.).

### Quick Start
```bash
# Install just (if not already installed)
cargo install just

# Set up development environment (installs deps, pre-commit hooks, generates SQLx data)
just setup

# Standard development workflow
cargo fmt
cargo clippy
cargo test
cargo run
```

### Just Commands (for cumbersome tasks)
- `just setup` - Set up development environment
- `just prepare-offline` - Generate SQLx offline query data (used by pre-commit hook)
- `just run-sqlite` - Run with SQLite feature flags
- `just migrate-new NAME` - Create new migration
- `just migrate-run DATABASE_URL` - Run migrations
- `just migrate-revert DATABASE_URL` - Revert last migration
- `just migrate-info DATABASE_URL` - Show migration status

See `just --list` for all available commands.

## Quirks and Rate Limiting
- **Single request per minute**: monobanks personal api is rate-limited.
- **Which is not even an exact minute**: loading the whole dataset one batch per minute is discouraged by monobank. We use jitter to avoid some arbitrary blocking.
- **Waiting is very naive**: time for data processing and storage is negligable, so we don't subtract it.
- **Using synchronous requests**: can't remember the reason, but I swear I had one.
- **No webhook integration**: not using it has no practical effect in this case.
- **No jars**: don't need them yet. You're welcome to implement them if you want.

## TODO
- Update account data on each run
- Don't request client info after initial request
- Manage tokens based on hashes instead of storing them.
- Handle the 'older-then-account-creation' API errors.
