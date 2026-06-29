/**
 * Helpers for making Soroban JSON-RPC calls from k6 scripts.
 *
 * The Soroban RPC follows JSON-RPC 2.0 and supports methods such as:
 *   - simulateTransaction
 *   - sendTransaction
 *   - getTransaction
 *   - getLedgerEntries
 *   - getLatestLedger
 *   - getNetwork
 *
 * Ref: https://developers.stellar.org/network/soroban-rpc/api-reference
 */

import http from "k6/http";
import { check, sleep } from "k6";
import { Trend, Counter } from "k6/metrics";

// ── Custom metrics ────────────────────────────────────────────────────────────

/** Raw RPC round-trip time (ms), broken down by method. */
export const rpcDuration = new Trend("rpc_duration_ms", true);
/** Count of RPC calls that returned a JSON-RPC error. */
export const rpcErrors   = new Counter("rpc_errors_total");

// ── Core helper ───────────────────────────────────────────────────────────────

/**
 * Send a JSON-RPC 2.0 request to the Soroban RPC endpoint.
 *
 * @param {string} rpcUrl  - Full URL of the Soroban RPC endpoint.
 * @param {string} method  - JSON-RPC method name.
 * @param {object} params  - Method parameters.
 * @param {object} [tags]  - Extra k6 tags to attach to the request.
 * @returns {object|null}  - The `result` field of the response, or null on error.
 */
export function rpcCall(rpcUrl, method, params = {}, tags = {}) {
  const payload = JSON.stringify({
    jsonrpc: "2.0",
    id:      Math.floor(Math.random() * 1e9),
    method,
    params,
  });

  const res = http.post(rpcUrl, payload, {
    headers: { "Content-Type": "application/json" },
    tags: { call_type: "rpc_read", rpc_method: method, ...tags },
  });

  rpcDuration.add(res.timings.duration, { rpc_method: method });

  const ok = check(res, {
    [`${method}: status 200`]:        (r) => r.status === 200,
    [`${method}: has JSON body`]:     (r) => r.body && r.body.length > 0,
    [`${method}: no JSON-RPC error`]: (r) => {
      try {
        const body = JSON.parse(r.body);
        if (body.error) {
          rpcErrors.add(1, { rpc_method: method });
          return false;
        }
        return true;
      } catch {
        return false;
      }
    },
  });

  if (!ok) return null;

  try {
    return JSON.parse(res.body).result ?? null;
  } catch {
    return null;
  }
}

// ── Convenience wrappers ──────────────────────────────────────────────────────

/** getLatestLedger — a lightweight health-check call. */
export function getLatestLedger(rpcUrl) {
  return rpcCall(rpcUrl, "getLatestLedger");
}

/** getNetwork — returns passphrase & friendbot URL. */
export function getNetwork(rpcUrl) {
  return rpcCall(rpcUrl, "getNetwork");
}

/**
 * getLedgerEntries — fetch one or more contract storage entries by XDR key.
 *
 * @param {string}   rpcUrl - RPC endpoint.
 * @param {string[]} keys   - Array of base64-encoded LedgerKey XDR strings.
 */
export function getLedgerEntries(rpcUrl, keys) {
  return rpcCall(rpcUrl, "getLedgerEntries", { keys }, { call_type: "rpc_read" });
}

/**
 * simulateTransaction — dry-run a transaction envelope.
 *
 * @param {string} rpcUrl          - RPC endpoint.
 * @param {string} transactionXdr  - Base64-encoded TransactionEnvelope XDR.
 */
export function simulateTransaction(rpcUrl, transactionXdr) {
  return rpcCall(
    rpcUrl,
    "simulateTransaction",
    { transaction: transactionXdr },
    { call_type: "rpc_simulate" },
  );
}

/**
 * sendTransaction — submit a signed transaction envelope.
 *
 * @param {string} rpcUrl          - RPC endpoint.
 * @param {string} transactionXdr  - Base64-encoded signed TransactionEnvelope XDR.
 */
export function sendTransaction(rpcUrl, transactionXdr) {
  return rpcCall(
    rpcUrl,
    "sendTransaction",
    { transaction: transactionXdr },
    { call_type: "rpc_submit" },
  );
}

/**
 * getTransaction — poll for a previously submitted transaction's status.
 *
 * @param {string} rpcUrl - RPC endpoint.
 * @param {string} hash   - Transaction hash (hex string).
 */
export function getTransaction(rpcUrl, hash) {
  return rpcCall(rpcUrl, "getTransaction", { hash }, { call_type: "rpc_read" });
}

/**
 * waitForTransaction — poll until the transaction is no longer PENDING,
 * or until `maxAttempts` is reached.
 *
 * Returns the final result or null if the timeout was reached.
 */
export function waitForTransaction(rpcUrl, hash, maxAttempts = 20, intervalSecs = 2) {
  for (let i = 0; i < maxAttempts; i++) {
    const result = getTransaction(rpcUrl, hash);
    if (result && result.status !== "NOT_FOUND" && result.status !== "PENDING") {
      return result;
    }
    sleep(intervalSecs);
  }
  return null;
}

// ── Frontend page helper ──────────────────────────────────────────────────────

/**
 * Fetch a frontend page and validate it returns HTML with the expected title fragment.
 *
 * @param {string} baseUrl       - Frontend base URL.
 * @param {string} path          - URL path (e.g. "/", "/job/1").
 * @param {string} [titleHint]   - Substring expected in the HTML body.
 * @param {object} [tags]        - Extra k6 tags.
 */
export function fetchPage(baseUrl, path, titleHint = "StellarWork", tags = {}) {
  const url = `${baseUrl}${path}`;
  const res = http.get(url, {
    tags: { call_type: "page_load", ...tags },
  });

  check(res, {
    [`GET ${path}: status 200`]:       (r) => r.status === 200,
    [`GET ${path}: has HTML body`]:    (r) => r.body && r.body.includes("<!DOCTYPE") || r.body.includes("<html"),
    [`GET ${path}: contains hint`]:    (r) => titleHint === "" || r.body.includes(titleHint),
  });

  return res;
}
