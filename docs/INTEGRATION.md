# Integration Guide

This guide is for third-party developers building on top of the StellarWork escrow contract. It covers contract interaction patterns, TypeScript examples using `@stellar/stellar-sdk`, and common workflow sequences.

## Prerequisites

- A deployed instance of the StellarWork escrow contract
- The contract ID (starts with `C`)
- A Stellar account funded on the target network (testnet or mainnet)
- `@stellar/stellar-sdk` v15+ installed

```bash
npm install @stellar/stellar-sdk
```

## Contract Connection Setup

```typescript
import {
  Contract,
  rpc,
  TransactionBuilder,
  BASE_FEE,
  Networks,
  nativeToScVal,
  scValToNative,
  Account,
  xdr,
} from "@stellar/stellar-sdk";

const CONTRACT_ID = "C...";
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = Networks.TESTNET;

const server = new rpc.Server(RPC_URL);
const contract = new Contract(CONTRACT_ID);
```

## Read-Only Queries

Read-only calls use transaction simulation and do not require signing or fees.

### Get Job Details

```typescript
async function getJob(jobId: number) {
  const account = new Account(
    "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
    "0"
  );

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call("get_job", nativeToScVal(jobId, { type: "u64" })))
    .setTimeout(30)
    .build();

  const simulation = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(simulation)) {
    throw new Error(simulation.error);
  }

  return scValToNative(simulation.result!.retval);
}

const job = await getJob(1);
console.log(job);
// {
//   client: "G...",
//   freelancer: "G..." | null,
//   amount: 10000000n,
//   description_hash: Uint8Array(32),
//   status: "Open",
//   created_at: 1710000000n,
//   deadline: 0n,
//   token: "C...",
//   revision_count: 0
// }
```

### Get Job Count

```typescript
async function getJobCount(): Promise<number> {
  const account = new Account(
    "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
    "0"
  );

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call("get_job_count"))
    .setTimeout(30)
    .build();

  const simulation = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(simulation)) {
    throw new Error(simulation.error);
  }

  return Number(scValToNative(simulation.result!.retval));
}
```

### Check Token Allowlist

```typescript
async function isTokenAllowed(tokenAddress: string): Promise<boolean> {
  const account = new Account(
    "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
    "0"
  );

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(
      contract.call("is_token_allowed", nativeToScVal(tokenAddress, { type: "address" }))
    )
    .setTimeout(30)
    .build();

  const simulation = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(simulation)) {
    throw new Error(simulation.error);
  }

  return Boolean(scValToNative(simulation.result!.retval));
}
```

## Write Transactions

Write transactions require signing by the appropriate account. Use Freighter or a keypair for signing.

### Helper: Build, Sign, and Submit

```typescript
async function invokeContract(
  method: string,
  args: xdr.ScVal[],
  sourcePublicKey: string,
  signTransaction: (xdr: string) => Promise<string>,
) {
  const account = await server.getAccount(sourcePublicKey);

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(60)
    .build();

  const simulation = await server.simulateTransaction(tx);
  if (rpc.Api.isSimulationError(simulation)) {
    throw new Error(simulation.error);
  }

  const assembled = rpc.assembleTransaction(tx, simulation).build();
  const prepared = await server.prepareTransaction(assembled);
  const signedXdr = await signTransaction(prepared.toXDR());
  const signedTx = TransactionBuilder.fromXDR(signedXdr, NETWORK_PASSPHRASE);
  const result = await server.sendTransaction(signedTx);

  if (result.status === rpc.Api.SendTransactionStatus.ERROR) {
    throw new Error(result.errorResult?.toXDR().toString());
  }

  // Poll for confirmation
  const timeout = 30000;
  const start = Date.now();
  while (Date.now() - start < timeout) {
    await new Promise((r) => setTimeout(r, 3000));
    const status = await server.getTransaction(result.hash);
    if (status.status === rpc.Api.GetTransactionStatus.SUCCESS) {
      return { hash: result.hash, status: "SUCCESS" };
    }
    if (status.status === rpc.Api.GetTransactionStatus.FAILED) {
      return { hash: result.hash, status: "FAILED" };
    }
  }

  throw new Error(`Transaction timed out: ${result.hash}`);
}
```

### Post a Job

```typescript
import { createHash } from "crypto";

async function postJob(
  clientAddress: string,
  amount: bigint,
  description: string,
  deadline: bigint,
  tokenAddress: string,
  signTransaction: (xdr: string) => Promise<string>,
) {
  const descHash = createHash("sha256").update(description).digest();
  const descHashBytes = new Uint8Array(descHash);

  return invokeContract(
    "post_job",
    [
      nativeToScVal(clientAddress, { type: "address" }),
      nativeToScVal(amount, { type: "i128" }),
      nativeToScVal(descHashBytes, { type: "bytes" }),
      nativeToScVal(description.length, { type: "u32" }),
      nativeToScVal(deadline, { type: "u64" }),
      nativeToScVal(tokenAddress, { type: "address" }),
    ],
    clientAddress,
    signTransaction,
  );
}
```

### Accept a Job

```typescript
async function acceptJob(
  freelancerAddress: string,
  jobId: number,
  signTransaction: (xdr: string) => Promise<string>,
) {
  return invokeContract(
    "accept_job",
    [
      nativeToScVal(freelancerAddress, { type: "address" }),
      nativeToScVal(jobId, { type: "u64" }),
    ],
    freelancerAddress,
    signTransaction,
  );
}
```

### Submit Work

```typescript
async function submitWork(
  freelancerAddress: string,
  jobId: number,
  signTransaction: (xdr: string) => Promise<string>,
) {
  return invokeContract(
    "submit_work",
    [
      nativeToScVal(freelancerAddress, { type: "address" }),
      nativeToScVal(jobId, { type: "u64" }),
    ],
    freelancerAddress,
    signTransaction,
  );
}
```

### Approve Work

```typescript
async function approveWork(
  clientAddress: string,
  jobId: number,
  signTransaction: (xdr: string) => Promise<string>,
) {
  return invokeContract(
    "approve_work",
    [
      nativeToScVal(clientAddress, { type: "address" }),
      nativeToScVal(jobId, { type: "u64" }),
    ],
    clientAddress,
    signTransaction,
  );
}
```

## Common Workflows

### Post → Accept → Submit → Approve (Happy Path)

```typescript
const TOKEN = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";

// 1. Client posts a job (escrows 10 XLM = 100,000,000 stroops)
await postJob(clientAddress, 100_000_000n, "Build a website", 0n, TOKEN, clientSign);

// 2. Freelancer accepts the job
await acceptJob(freelancerAddress, 1, freelancerSign);

// 3. Freelancer submits work
await submitWork(freelancerAddress, 1, freelancerSign);

// 4. Client approves work (freelancer receives 97.5% after 2.5% platform fee)
await approveWork(clientAddress, 1, clientSign);
```

### Dispute Flow

```typescript
// Either party raises a dispute (job must be InProgress or SubmittedForReview)
await invokeContract(
  "raise_dispute",
  [
    nativeToScVal(callerAddress, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ],
  callerAddress,
  signTransaction,
);

// Admin resolves the dispute with a 60/40 split (60% to client)
await invokeContract(
  "resolve_dispute",
  [
    nativeToScVal(jobId, { type: "u64" }),
    // DisputeResolution struct: { client_bps: u32 }
    xdr.ScVal.scvVec([nativeToScVal(6000, { type: "u32" })]),
  ],
  adminAddress,
  signTransaction,
);
```

### Cancellation Flows

```typescript
// Client cancels an Open job (full refund)
await invokeContract(
  "cancel_job",
  [
    nativeToScVal(clientAddress, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ],
  clientAddress,
  signTransaction,
);

// Freelancer cancels an InProgress job (full refund to client)
await invokeContract(
  "freelancer_cancel_job",
  [
    nativeToScVal(freelancerAddress, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ],
  freelancerAddress,
  signTransaction,
);

// Mutual cancellation with custom split (50/50)
await invokeContract(
  "mutual_cancel",
  [
    nativeToScVal(clientAddress, { type: "address" }),
    nativeToScVal(freelancerAddress, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
    nativeToScVal(5000, { type: "i128" }),
  ],
  clientAddress,
  signTransaction,
);
```

## Token Management

The contract supports multiple whitelisted tokens. Before posting a job, verify the token is allowed:

```typescript
const allowed = await isTokenAllowed(tokenAddress);
if (!allowed) {
  throw new Error("Token is not whitelisted. Contact the contract admin.");
}
```

Common Stellar token addresses on testnet:
- Native XLM: Use the address returned by `register_stellar_asset_contract` or the known native asset contract.

## Event Monitoring

Subscribe to contract events via Soroban RPC to track job lifecycle changes:

```typescript
const events = await server.getEvents({
  startLedger: currentLedger - 1000,
  filters: [
    {
      type: "contract",
      contractIds: [CONTRACT_ID],
      topics: [["job_created"]],
    },
  ],
});

for (const event of events.events) {
  console.log("Event:", event.topic, "Data:", event.value);
}
```

## Error Handling

Contract errors are returned as simulation errors with the error code:

```typescript
try {
  await acceptJob(freelancerAddress, jobId, signTransaction);
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  if (message.includes("#3")) {
    console.error("Invalid status: job is not open");
  } else if (message.includes("#5")) {
    console.error("Job already accepted");
  } else if (message.includes("#8")) {
    console.error("Token not in whitelist");
  } else {
    console.error("Transaction failed:", message);
  }
}
```

Refer to `docs/CONTRACT.md` for the complete error code table.

## Rate Limiting and Best Practices

1. **Batch reads**: Use `get_jobs_batch(start, limit)` to fetch multiple jobs in a single call instead of individual `get_job` calls.
2. **Cache job data**: Job data is immutable once completed/cancelled. Cache these states locally.
3. **Poll intervals**: When polling for transaction confirmation, use 3-second intervals with a 30-second timeout.
4. **Description storage**: Store job descriptions on IPFS and pass the SHA-256 hash to `post_job`. Use `store_description_cid` to make descriptions discoverable.
5. **Token amounts**: All amounts are in the token's smallest unit (stroops for XLM, where 1 XLM = 10^7 stroops).
