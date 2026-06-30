# StellarWork Escrow Contract

Soroban smart contract for a decentralized freelance escrow flow.

## Implemented

- `initialize(admin, native_token)`
- `post_job(client, amount, desc_hash, deadline)`
- `accept_job(freelancer, job_id)`
- `submit_work(freelancer, job_id)`
- `approve_work(client, job_id)`
- `cancel_job(client, job_id)`
- `get_job(job_id)`
- `get_job_count()`

## Stubbed

- `raise_dispute(job_id)`
- `resolve_dispute(job_id, winner)`

## Events

The contract emits the following symbolic-topic events on key state transitions:

| Event Topic | Trigger | Event Data |
|-------------|---------|------------|
| `init` | `initialize` | `(admin: Address, native_token: Address)` |
| `JobPosted` | `post_job`, `create_job_with_milestones` | `(job_id: u64, client: Address, desc_hash: Bytes, amount: i128)` |
| `JobAccepted` | `accept_job` | `(job_id: u64, client: Address, freelancer: Address, amount: i128)` |
| `WorkSub` | `submit_work` | `(job_id: u64, client: Address, freelancer: Address, amount: i128)` |
| `WorkAppr` | `approve_work` | `(job_id: u64, client: Address, freelancer: Address, amount: i128)` |
| `JobCanc` | `cancel_job`, `freelancer_cancel_job`, `enforce_deadline`, `relay_cancel_job` | `(job_id: u64, client: Address, freelancer: Address, amount: i128)` |
| `Dispute` | `raise_dispute` | `(job_id: u64, client: Address, freelancer: Address, amount: i128)` |
| `DispRes` | `resolve_dispute`, `resolve_dispute_split` | `(job_id: u64, client: Address, freelancer: Address, amount: i128, client_bps: u32, freelancer_bps: u32)` |
| `mstone` | `approve_milestone` | `(job_id: u64, milestone_id: u32, client: Address)` |
| `ttl_ext` | `extend_job_ttl` | `(job_id: u64)` |
| `tok_add` | `add_allowed_token` | `(token: Address)` |
| `tok_rem` | `remove_allowed_token` | `(token: Address)` |
| `fees_wdr` | `withdraw_fees` | `(admin: Address, token: Address, accumulated: i128)` |
| `wl_mode` | `set_whitelist_mode` | `(enabled: bool)` |
| `wl_add` | `add_to_whitelist` | `(address: Address)` |
| `wl_rem` | `remove_from_whitelist` | `(address: Address)` |
| `bl_add` | `add_to_blacklist` | `(address: Address)` |
| `bl_rem` | `remove_from_blacklist` | `(address: Address)` |
| `fwd_set` | `set_trusted_forwarder` | `(forwarder: Address, is_trusted: bool)` |

All events use Soroban's `contract.emit()` with symbolic (short) topics. Off-chain indexers can subscribe to these topic symbols to react to state changes without polling.

## Test

```bash
cargo test
```
