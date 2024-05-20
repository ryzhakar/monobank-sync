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
