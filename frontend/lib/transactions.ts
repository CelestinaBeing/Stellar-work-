"use client";

import type { Job } from "@/lib/types";
import { toXlm } from "@/lib/format";

// ------------------------------------------------------------------
// Transaction types
// ------------------------------------------------------------------

export type TransactionType =
  | "job_posted"        // client locked funds into escrow
  | "payment_received"  // freelancer received payment after approval
  | "fee_deducted"      // platform fee taken from the job amount
  | "refund_received"   // client or freelancer received a refund on cancel
  | "dispute_resolved"; // dispute was resolved and funds split

export type TransactionDirection = "incoming" | "outgoing";

export interface Transaction {
  /** Deterministic ID derived from job ID + type so deduplication is safe. */
  id: string;
  type: TransactionType;
  direction: TransactionDirection;
  /** Amount in XLM (human-readable string). */
  amountXlm: string;
  /** Raw stroops for sorting / math. */
  amountStroops: bigint;
  jobId: number;
  /** The wallet on the other side of the transaction. */
  counterparty: string | null;
  /**
   * Best-effort timestamp in milliseconds.
   * For completed/cancelled jobs we use `created_at` (unix seconds) stored on
   * the contract. For other states we use the same value as a proxy because the
   * contract does not store separate event timestamps.
   */
  timestamp: number;
  /**
   * Transaction status. We use "confirmed" for terminal job states and
   * "pending" for non-terminal ones.
   */
  status: "confirmed" | "pending";
  token: string;
}

// ------------------------------------------------------------------
// Platform fee rate (2.5% = 250 / 10_000 basis points)
// ------------------------------------------------------------------
const FEE_BPS = 250n;
const BPS_BASE = 10_000n;

function calcFee(amount: bigint): bigint {
  return (amount * FEE_BPS) / BPS_BASE;
}

function calcNet(amount: bigint): bigint {
  return amount - calcFee(amount);
}

// ------------------------------------------------------------------
// Derivation logic
// ------------------------------------------------------------------

/**
 * Derive the transactions visible to `wallet` from a single job entry.
 * Returns zero, one, or more transactions depending on the job state.
 */
export function deriveTransactions(
  jobId: number,
  job: Job,
  wallet: string,
): Transaction[] {
  const isClient = job.client === wallet;
  const isFreelancer = job.freelancer === wallet;

  if (!isClient && !isFreelancer) return [];

  const raw = BigInt(job.amount);
  const fee = calcFee(raw);
  const net = calcNet(raw);
  const ts = Number(job.created_at) * 1000 || Date.now();
  const token = job.token;

  const txns: Transaction[] = [];

  // ── 1. Job Posted (client perspective) ──────────────────────────
  if (isClient) {
    txns.push({
      id: `${jobId}-job_posted`,
      type: "job_posted",
      direction: "outgoing",
      amountXlm: toXlm(raw),
      amountStroops: raw,
      jobId,
      counterparty: null, // funds go into escrow, not to a person
      timestamp: ts,
      status: "confirmed",
      token,
    });
  }

  // ── 2. Payment Received (freelancer perspective) ─────────────────
  if (isFreelancer && job.status === "Completed") {
    txns.push({
      id: `${jobId}-payment_received`,
      type: "payment_received",
      direction: "incoming",
      amountXlm: toXlm(net),
      amountStroops: net,
      jobId,
      counterparty: job.client,
      timestamp: ts,
      status: "confirmed",
      token,
    });
  }

  // ── 3. Fee Deducted (freelancer perspective on completed jobs) ───
  if (isFreelancer && job.status === "Completed") {
    txns.push({
      id: `${jobId}-fee_deducted`,
      type: "fee_deducted",
      direction: "outgoing",
      amountXlm: toXlm(fee),
      amountStroops: fee,
      jobId,
      counterparty: null, // goes to platform
      timestamp: ts,
      status: "confirmed",
      token,
    });
  }

  // ── 4. Refund Received (client gets money back on cancellation) ──
  if (isClient && job.status === "Cancelled") {
    txns.push({
      id: `${jobId}-refund_received`,
      type: "refund_received",
      direction: "incoming",
      amountXlm: toXlm(raw),
      amountStroops: raw,
      jobId,
      counterparty: job.freelancer, // may be null if cancelled before accept
      timestamp: ts,
      status: "confirmed",
      token,
    });
  }

  // ── 5. Dispute Resolved ─────────────────────────────────────────
  // We don't know the exact split from job state alone, so we record
  // the gross amount as "pending" and direct users to the job page.
  if (job.status === "Disputed") {
    if (isClient || isFreelancer) {
      txns.push({
        id: `${jobId}-dispute_resolved`,
        type: "dispute_resolved",
        direction: "incoming", // direction is ambiguous; use incoming as neutral
        amountXlm: toXlm(raw),
        amountStroops: raw,
        jobId,
        counterparty: isClient ? job.freelancer : job.client,
        timestamp: ts,
        status: "pending",
        token,
      });
    }
  }

  return txns;
}

// ------------------------------------------------------------------
// Aggregate helper
// ------------------------------------------------------------------

/**
 * Build the full transaction list for `wallet` from a list of jobs.
 */
export function buildTransactionList(
  jobs: Array<{ id: number; job: Job }>,
  wallet: string,
): Transaction[] {
  const all: Transaction[] = [];
  for (const { id, job } of jobs) {
    all.push(...deriveTransactions(id, job, wallet));
  }
  return all;
}

// ------------------------------------------------------------------
// Filter / sort helpers
// ------------------------------------------------------------------

export type SortOrder = "newest" | "oldest" | "amount_desc" | "amount_asc";

export interface TransactionFilters {
  types: TransactionType[];
  /** "YYYY-MM-DD" strings, inclusive. Empty string = unset. */
  dateFrom: string;
  dateTo: string;
}

export function applyFilters(
  txns: Transaction[],
  filters: TransactionFilters,
): Transaction[] {
  return txns.filter((tx) => {
    if (filters.types.length > 0 && !filters.types.includes(tx.type)) {
      return false;
    }
    if (filters.dateFrom) {
      const from = new Date(filters.dateFrom).setHours(0, 0, 0, 0);
      if (tx.timestamp < from) return false;
    }
    if (filters.dateTo) {
      const to = new Date(filters.dateTo).setHours(23, 59, 59, 999);
      if (tx.timestamp > to) return false;
    }
    return true;
  });
}

export function applySortOrder(
  txns: Transaction[],
  order: SortOrder,
): Transaction[] {
  return [...txns].sort((a, b) => {
    switch (order) {
      case "newest":
        return b.timestamp - a.timestamp;
      case "oldest":
        return a.timestamp - b.timestamp;
      case "amount_desc":
        return b.amountStroops > a.amountStroops ? 1 : b.amountStroops < a.amountStroops ? -1 : 0;
      case "amount_asc":
        return a.amountStroops > b.amountStroops ? 1 : a.amountStroops < b.amountStroops ? -1 : 0;
    }
  });
}

// ------------------------------------------------------------------
// Display helpers
// ------------------------------------------------------------------

export const TX_TYPE_LABELS: Record<TransactionType, string> = {
  job_posted: "Job Posted",
  payment_received: "Payment Received",
  fee_deducted: "Fee Deducted",
  refund_received: "Refund Received",
  dispute_resolved: "Dispute",
};

export const TX_TYPE_COLORS: Record<
  TransactionType,
  { pill: string; amount: string; icon: string }
> = {
  job_posted: {
    pill: "bg-slate-100 text-slate-700 ring-slate-200",
    amount: "text-slate-700",
    icon: "⬆",
  },
  payment_received: {
    pill: "bg-emerald-100 text-emerald-700 ring-emerald-200",
    amount: "text-emerald-700",
    icon: "⬇",
  },
  fee_deducted: {
    pill: "bg-orange-100 text-orange-700 ring-orange-200",
    amount: "text-orange-700",
    icon: "−",
  },
  refund_received: {
    pill: "bg-blue-100 text-blue-700 ring-blue-200",
    amount: "text-blue-700",
    icon: "⬇",
  },
  dispute_resolved: {
    pill: "bg-purple-100 text-purple-700 ring-purple-200",
    amount: "text-purple-700",
    icon: "⚖",
  },
};

export const ALL_TX_TYPES: TransactionType[] = [
  "job_posted",
  "payment_received",
  "fee_deducted",
  "refund_received",
  "dispute_resolved",
];
