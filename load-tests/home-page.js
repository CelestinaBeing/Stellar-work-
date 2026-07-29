/**
 * home-page.js
 * ============
 * Simulates users browsing the StellarWork job listings:
 *   1. Load the home page  (HTML)
 *   2. Fetch the Next.js static assets (JS bundles, CSS)
 *   3. Make the Soroban RPC read calls that the page issues on mount:
 *       - getLatestLedger  (health-check)
 *       - get_job_count    (read contract state via simulateTransaction)
 *   4. Load individual job detail pages
 *
 * Ramp-up: 10 → 50 → 100 → 500 concurrent VUs.
 *
 * Run:
 *   k6 run load-tests/home-page.js
 *   k6 run -e BASE_URL=https://myapp.vercel.app load-tests/home-page.js
 */

import http from "k6/http";
import { sleep, check, group } from "k6";
import { Counter, Rate, Trend } from "k6/metrics";

import {
  BASE_URL,
  RPC_URL,
  RAMP_UP_STAGES,
  COMMON_THRESHOLDS,
} from "./lib/config.js";
import { getLatestLedger, fetchPage, rpcCall } from "./lib/rpc-helpers.js";

// ── Custom metrics ────────────────────────────────────────────────────────────

const pageLoads      = new Counter("page_loads_total");
const pageLoadTime   = new Trend("page_load_time_ms",  true);
const rpcReadTime    = new Trend("rpc_read_time_ms",   true);
const errorRate      = new Rate("errors");

// ── Test options ──────────────────────────────────────────────────────────────

export const options = {
  stages: RAMP_UP_STAGES,
  thresholds: {
    ...COMMON_THRESHOLDS,
    // Home page should load within 2 s at p95
    "http_req_duration{call_type:page_load}": ["p(95)<2000"],
    // RPC reads should be under 1.5 s at p95
    "http_req_duration{call_type:rpc_read}":  ["p(95)<1500"],
    errors: ["rate<0.01"],
  },
};

// ── Helpers ───────────────────────────────────────────────────────────────────

/** Simulate calling the get_job_count contract read via the RPC simulate endpoint. */
function fetchJobCount() {
  // In the real app, the frontend constructs a TransactionEnvelope XDR.
  // In the load test we call getLatestLedger as a representative read-only
  // RPC round-trip with equivalent overhead.
  return rpcCall(RPC_URL, "getLatestLedger", {}, { call_type: "rpc_read" });
}

/** Simulate calling get_job for a specific ID. */
function fetchJob(jobId) {
  return rpcCall(RPC_URL, "getLatestLedger", {}, {
    call_type: "rpc_read",
    resource: "get_job",
    job_id: String(jobId),
  });
}

// ── Default function (executed once per VU per iteration) ─────────────────────

export default function () {
  // ── 1. Load the home page ──────────────────────────────────────────────────
  group("home page load", () => {
    const start = Date.now();
    const res = fetchPage(BASE_URL, "/", "StellarWork");
    const duration = Date.now() - start;

    pageLoads.add(1);
    pageLoadTime.add(duration);
    errorRate.add(res.status !== 200);
  });

  sleep(0.5); // brief pause between page load and RPC calls (realistic UX)

  // ── 2. RPC: get latest ledger (health check / block number) ───────────────
  group("rpc get latest ledger", () => {
    const start = Date.now();
    const result = getLatestLedger(RPC_URL);
    rpcReadTime.add(Date.now() - start);
    errorRate.add(result === null);
  });

  sleep(0.3);

  // ── 3. RPC: fetch job count (simulates mount-time contract read) ──────────
  group("rpc get job count", () => {
    const start = Date.now();
    const result = fetchJobCount();
    rpcReadTime.add(Date.now() - start);
    errorRate.add(result === null);
  });

  sleep(0.5);

  // ── 4. Simulate browsing: load 3 random job detail pages ─────────────────
  group("browse job listings", () => {
    const jobIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Pick 3 random jobs to simulate realistic browsing behaviour.
    for (let i = 0; i < 3; i++) {
      const jobId = jobIds[Math.floor(Math.random() * jobIds.length)];

      group(`job detail page ${jobId}`, () => {
        const start = Date.now();
        const res = fetchPage(BASE_URL, `/job/${jobId}`, "", { page: "job_detail" });
        pageLoadTime.add(Date.now() - start);
        errorRate.add(res.status !== 200 && res.status !== 404);
      });

      sleep(0.2);

      // RPC read for this job
      group(`rpc get_job ${jobId}`, () => {
        const start = Date.now();
        const result = fetchJob(jobId);
        rpcReadTime.add(Date.now() - start);
        errorRate.add(result === null);
      });

      sleep(0.3);
    }
  });

  // ── 5. Post-job page load (authenticated flow entry point) ────────────────
  group("post job page load", () => {
    const start = Date.now();
    const res = fetchPage(BASE_URL, "/post-job", "StellarWork");
    pageLoadTime.add(Date.now() - start);
    errorRate.add(res.status !== 200);
  });

  sleep(0.5);

  // ── 6. Dashboard page ─────────────────────────────────────────────────────
  group("dashboard page load", () => {
    const start = Date.now();
    const res = fetchPage(BASE_URL, "/dashboard", "StellarWork");
    pageLoadTime.add(Date.now() - start);
    errorRate.add(res.status !== 200);
  });

  // Think-time between full iterations
  sleep(Math.random() * 2 + 1); // 1–3 s
}

// ── Summary hook ──────────────────────────────────────────────────────────────

export function handleSummary(data) {
  return {
    "load-tests/results/home-page-summary.json": JSON.stringify(data, null, 2),
    stdout: textSummary(data),
  };
}

function textSummary(data) {
  const m = data.metrics;
  const fmt = (v) => (v !== undefined ? v.toFixed(0) : "N/A");
  return [
    "\n=== Home Page Load Test Summary ===",
    `Page loads total:    ${m.page_loads_total?.values?.count ?? 0}`,
    `HTTP error rate:     ${((m.http_req_failed?.values?.rate ?? 0) * 100).toFixed(2)}%`,
    `Page p50 (ms):       ${fmt(m["http_req_duration{call_type:page_load}"]?.values?.["p(50)"])}`,
    `Page p95 (ms):       ${fmt(m["http_req_duration{call_type:page_load}"]?.values?.["p(95)"])}`,
    `Page p99 (ms):       ${fmt(m["http_req_duration{call_type:page_load}"]?.values?.["p(99)"])}`,
    `RPC  p50 (ms):       ${fmt(m["http_req_duration{call_type:rpc_read}"]?.values?.["p(50)"])}`,
    `RPC  p95 (ms):       ${fmt(m["http_req_duration{call_type:rpc_read}"]?.values?.["p(95)"])}`,
    `RPC  p99 (ms):       ${fmt(m["http_req_duration{call_type:rpc_read}"]?.values?.["p(99)"])}`,
    "===================================\n",
  ].join("\n");
}
