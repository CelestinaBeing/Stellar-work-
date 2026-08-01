# StellarWork Contract API Reference

The [`openapi.yaml`](./openapi.yaml) file in this directory is an **OpenAPI 3.0**
specification that documents every public function of the StellarWork escrow
smart contract as a REST-like endpoint.

## How to use this spec

### Browse interactively

Paste the raw URL of `openapi.yaml` (or a local file path) into
[Swagger Editor](https://editor.swagger.io/) or
[Redoc](https://redocly.github.io/redoc/) to render interactive documentation.

### Generate a TypeScript client

```bash
npx @openapitools/openapi-generator-cli generate \
  -i docs/api/openapi.yaml \
  -g typescript-fetch \
  -o generated/stellar-work-client
```

### Validate the spec

```bash
npx @redocly/cli lint docs/api/openapi.yaml
```

## Example calls (curl)

> All Soroban contract invocations are submitted as XDR-encoded transactions via
> the Soroban RPC `sendTransaction` method.  The examples below use the
> [soroban-cli](https://github.com/stellar/soroban-tools) for convenience.

### Get a job by ID

```bash
soroban contract invoke \
  --network testnet \
  --id <CONTRACT_ID> \
  -- get_job \
  --job_id 1
```

### Post a job

```bash
soroban contract invoke \
  --network testnet \
  --source <CLIENT_SECRET> \
  --id <CONTRACT_ID> \
  -- post_job \
  --client <CLIENT_ADDRESS> \
  --amount 10000000 \
  --desc_hash <32_BYTE_HEX> \
  --description_payload_len 512 \
  --deadline 0 \
  --token <TOKEN_ADDRESS>
```

### Get dashboard stats (admin)

```bash
soroban contract invoke \
  --network testnet \
  --source <ADMIN_SECRET> \
  --id <CONTRACT_ID> \
  -- get_dashboard_stats \
  --admin <ADMIN_ADDRESS>
```

## Authentication

StellarWork uses **wallet signatures** for authentication — there are no API
keys or bearer tokens.  Every mutating call must be submitted as a signed
Stellar transaction.

1. Build a Soroban `InvokeHostFunction` operation containing the contract call.
2. Sign the transaction envelope with the caller's private key.
3. Submit via `sendTransaction` to the Soroban RPC.

The contract validates `require_auth()` for each privileged caller inside the
WASM execution — the RPC node enforces the cryptographic signature.

## Servers

| Environment | RPC URL |
|-------------|---------|
| Testnet | `https://soroban-testnet.stellar.org` |
| Futurenet | `https://rpc-futurenet.stellar.org` |
| Mainnet | `https://mainnet.stellar.gateway.fm` |

## Contract addresses

Deployed contract addresses are tracked in
[`docs/environments.md`](../environments.md).
