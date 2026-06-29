# Escrow Contract — Gas & Fee Reference

## Contract Size

Run `soroban contract build && du -h target/wasm32-unknown-unknown/release/escrow.wasm` to get current size.

## Function Gas Costs (approximate, in stroops)

Values are estimates from local `soroban contract invoke` dry-runs against a standalone network. Production costs vary.

| Function | Storage Reads | Storage Writes | Notes |
|---|---|---|---|
| `initialize` | 0 | 8 (instance) + 1 (persistent) | One-time setup |
| `post_job` | 5-8 | 3 (persistent) + 1 (instance) | Includes token transfer |
| `accept_job` | 3 | 1 (persistent) | Updates job status |
| `submit_work` | 3 | 1 (persistent) | Updates job status |
| `approve_work` | 5 | 3 (persistent) | Includes token payout |
| `reject_work` | 3 | 1 (persistent) | Updates job + revision count |
| `cancel_job` | 3 | 1 (persistent) | Includes token refund |
| `freelancer_cancel_job` | 3 | 1 (persistent) | Includes token refund |
| `enforce_deadline` | 3 | 1 (persistent) | Time-sensitive check |
| `raise_dispute` | 4 | 2 (persistent) | Collects dispute fee |
| `resolve_dispute` | 5 | 4 (persistent) | Complex split logic |
| `get_job` | 1 | 0 | Read-only |
| `get_jobs_batch` | n | 0 | n = batch size |
| `get_fee_bps` | 1 | 0 | Read-only |
| `update_fee` | 1 | 1 (instance) | Admin only |
| `withdraw_fees` | 2 | 1 (persistent) | Admin only |

## Storage Layout

### Instance Storage (shared, small)

| Key | Type | Purpose |
|---|---|---|
| `Admin` | Address | Contract administrator |
| `NativeToken` | Address | Native token for fees |
| `JobsCount` | u64 | Total jobs created |
| `FeeBps` | i128 | Default platform fee |
| `FeeTierCount` | u32 | Number of fee tiers |
| `FeeTier(i)` | FeeTier | Per-tier configuration |
| `DescriptionPayloadMaxBytes` | u32 | Max description size |

### Persistent Storage (per-entry TTL)

| Key | Type | Purpose |
|---|---|---|
| `Job(id)` | Job | Per-job state |
| `TokenFees(token)` | i128 | Accumulated fees per token |
| `AllowedToken(token)` | bool | Token whitelist |
| `AllJobIds` | Vec<u64> | List of all job IDs |
| `DescriptionCidMapping(hash)` | String | IPFS CID lookup |
| `Blacklisted(addr)` | bool | Access control |
| `Whitelisted(addr)` | bool | Access control |
| `ReferralCode(code)` | Address | Referral mapping |
| `ReferralEarnings(addr)` | i128 | Referral balance |
| `ClientReferrer(addr)` | Address | Client → referrer |
| `ReferralBonusPaid(addr)` | bool | One-time bonus flag |

## TTL Bump Strategy

| Category | Threshold | Bump Amount |
|---|---|---|
| Instance | 17,280 ledgers (~24h) | 518,400 ledgers (~30d) |
| Active Jobs | 17,280 ledgers | 518,400 ledgers |
| Completed/Cancelled Jobs | N/A | 120,960 ledgers (~7d) |

## Optimization Notes

1. **Storage reads are the primary cost driver.** Each persistent storage read costs ~6,250 CPU instructions. Minimize calls to `e.storage().persistent().get()` in hot paths.

2. **`AllJobIds` indexing.** The contract maintains `AllJobIds` for admin queries alongside sequential IDs (1..n). Consider using only `AllJobIds` for iteration to reduce storage assumption coupling.

3. **Token transfers** (via the token contract's `transfer` call) are the most expensive operation. Batched payouts should use a single transfer where possible.

4. **Fee tier lookups** iterate through all tiers on every fee calculation. When no tiers are configured, the function returns immediately using the default fee — this is already optimized.

5. **Access control checks** (`require_active_access`) perform 2-3 storage reads per invocation (blacklist + whitelist mode + whitelist entry). These are called on every authenticated contract invocation and could be cached per-transaction.

## Benchmarking

Run the benchmark script:

```bash
./contracts/benchmark.sh
```

This builds the contract in release mode and simulates each function, reporting storage bytes read/written and estimated CPU cost.
