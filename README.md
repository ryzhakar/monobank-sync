# Monobank Sync Tool README

## Overview
This tool synchronizes data from Monobank APIs into a local database. It handles multiple accounts and efficiently manages transaction statements retrieval within user-defined time intervals.

## Configuration and Operation
Configure the tool by setting the necessary environment variables in the `.env` file at the project's root:

- `MULTIPLE_MONOBANK_TOKENS`: Monobank API tokens, comma-separated.
- `DATABASE_URL`: Connection string for your database.
- `ALLOWED_CARD_TYPES`: Filter transactions by card types, comma-separated.
- `SYNC_START_TIMESTAMP`: Initial sync date; defaults to the start of the current month if unspecified.

### Quirks and Rate Limiting
The synchronization process involves iterating over accounts and fetching transaction statements using a custom iterator that respects Monobank's API rate limits. The iterator employs a sleep strategy with jitter (`WAIT_TIME_SEC` and `WAIT_JITTER_SEC`) to avoid hitting these limits, adjusting the delay dynamically based on the server's response. If a batch fetch returns exactly the maximum number of allowed records, the iterator reduces the time window for subsequent requests to ensure no data is missed due to pagination limits.

## Issues
Ensure that the `.env` variables are correctly configured and that the specified database is accessible. Check network settings and API token validity if connection issues occur.
