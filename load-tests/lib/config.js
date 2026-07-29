/**
 * Shared configuration for all k6 load-test scripts.
 *
 * Override any value by setting the corresponding environment variable
 * when invoking k6:
 *
 *   k6 run -e BASE_URL=https://staging.example.com home-page.js
 */

/** Base URL of the deployed frontend (Next.js). */
export const BASE_URL =
  __ENV.BASE_URL || "http://localhost:3000";

/** Soroban RPC endpoint being exercised. */
export const RPC_URL =
  __ENV.RPC_URL || "https://soroban-testnet.stellar.org";

/** Deployed escrow contract ID (Soroban C… address). */
export const CONTRACT_ID =
  __ENV.CONTRACT_ID || __ENV.NEXT_PUBLIC_CONTRACT_ID || "";

/**
 * Standard ramp-up scenario stages used across test scripts.
 * Four traffic levels: 10 → 50 → 100 → 500 VUs.
 *
 * The shape is deliberately conservative so the test can run
 * against a real testnet RPC without hammering it.
 */
export const RAMP_UP_STAGES = [
  { duration: "30s", target: 10 },   // warm-up to 10 VUs
  { duration: "1m",  target: 50 },   // ramp to 50 VUs
  { duration: "2m",  target: 100 },  // sustain at 100 VUs
  { duration: "1m",  target: 500 },  // spike to 500 VUs
  { duration: "1m",  target: 100 },  // step-down
  { duration: "30s", target: 0 },    // cool-down
];

/**
 * Lighter stages used in the read-operations script where we
 * run many concurrent RPC reads; the spike is kept lower to stay
 * within public-RPC rate limits during CI.
 */
export const READ_RAMP_UP_STAGES = [
  { duration: "20s", target: 10 },
  { duration: "1m",  target: 50 },
  { duration: "2m",  target: 100 },
  { duration: "30s", target: 0 },
];

/**
 * Performance thresholds applied to every test.
 * Failing any threshold causes k6 to exit with a non-zero code,
 * which fails the CI job.
 */
export const COMMON_THRESHOLDS = {
  // Overall HTTP error rate must stay below 1 %
  http_req_failed: ["rate<0.01"],
  // 95th-percentile response time < 3 s for all HTTP requests
  http_req_duration: ["p(95)<3000"],
};

/**
 * Stricter thresholds for read-only RPC calls.
 * Reads should be fast; we fail if p99 > 2 s.
 */
export const RPC_READ_THRESHOLDS = {
  http_req_failed:   ["rate<0.01"],
  http_req_duration: ["p(50)<500", "p(95)<1500", "p(99)<2000"],
  // Named group for RPC calls specifically
  "http_req_duration{call_type:rpc_read}": ["p(95)<1500"],
};

/**
 * Thresholds for the full job-lifecycle test (includes write txns,
 * which involve Soroban simulation + submission – inherently slower).
 */
export const LIFECYCLE_THRESHOLDS = {
  http_req_failed:   ["rate<0.05"],  // allow up to 5 % failure (wallet sims)
  http_req_duration: ["p(95)<5000"], // 5 s p95 is acceptable for signed txns
  "http_req_duration{call_type:rpc_simulate}": ["p(95)<3000"],
  "http_req_duration{call_type:rpc_submit}":   ["p(95)<5000"],
};
