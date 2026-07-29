# Event Replayer

A tool for fetching historical contract events from Stellar RPC and replaying them against a local Soroban test environment for integration testing.

## Features

- Fetch historical contract events from Stellar RPC `getEvents`
- Parse and order events chronologically
- Replay events against a local Soroban test environment
- Verify final contract state matches expected values
- Configuration for event filters (by contract ID, event type, date range)
- Output summary report: events processed, state transitions, final balances
- Generate sample replay datasets from testnet activity

## Installation

```bash
cd tools/event-replayer
cargo build --release
```

## Usage

### Generate Sample Dataset

Generate a sample replay dataset from testnet activity:

```bash
cargo run --release -- --sample --output sample-dataset.json
```

### Fetch Events from RPC

Fetch events from a specific contract:

```bash
cargo run --release -- \
  --rpc-url https://soroban-testnet.stellar.org \
  --contract-id CC... \
  --output events.json
```

### Filter by Date Range

```bash
cargo run --release -- \
  --contract-id CC... \
  --start-time "2024-01-01T00:00:00Z" \
  --end-time "2024-01-31T23:59:59Z" \
  --output events.json
```

### Filter by Event Type

```bash
cargo run --release -- \
  --contract-id CC... \
  --event-type job_approved \
  --output events.json
```

### Combined Filters

```bash
cargo run --release -- \
  --rpc-url https://soroban-testnet.stellar.org \
  --contract-id CC... \
  --start-time "2024-01-01T00:00:00Z" \
  --end-time "2024-01-31T23:59:59Z" \
  --event-type job_posted \
  --output filtered-events.json
```

## Output

The tool generates two files:

1. **Dataset file** (e.g., `events.json`): Contains the fetched events in JSON format
2. **Report file** (e.g., `events.report.json`): Contains the replay summary with state transitions and final balances

### Dataset Format

```json
{
  "contract_id": "CC...",
  "network": "testnet",
  "start_time": "2024-01-01T00:00:00Z",
  "end_time": "2024-01-31T23:59:59Z",
  "events": [
    {
      "timestamp": "2024-01-01T12:00:00Z",
      "event_type": "job_posted",
      "contract_id": "CC...",
      "topic": ["job_posted"],
      "data": {"job_id": 1, "amount": 1000000}
    }
  ]
}
```

### Report Format

```json
{
  "dataset": {...},
  "events_processed": 4,
  "state_transitions": [
    "2024-01-01 12:00:00: job_posted",
    "2024-01-02 10:00:00: job_accepted"
  ],
  "final_balances": {
    "contract_balance": "25000",
    "platform_fees": "25000"
  },
  "summary": "Processed 4 events from 2024-01-01 to 2024-01-31"
}
```

## CI Integration

Add replay tests to CI pipeline (weekly or on-demand):

```yaml
# .github/workflows/event-replay.yml
name: Event Replay Test
on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  workflow_dispatch:

jobs:
  replay:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build event replayer
        run: cd tools/event-replayer && cargo build --release
      - name: Generate sample dataset
        run: cargo run --release -- --sample --output sample.json
      - name: Run replay
        run: cargo run --release -- --output sample.json
```

## Documentation

For more information on generating and using replay data for testing, see the main project documentation.

## License

MIT
<arg_value><arg_key>EmptyFile</arg_key><arg_value>false
