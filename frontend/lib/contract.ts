"use client";

import { callContract, nativeToScVal, xdr } from "@/lib/stellar";
import { requireContractId } from "@/lib/config";
export { requireContractId };
import type { Job, Milestone } from "@/lib/types";

export function hexToBytes(hex: string): Uint8Array {
  const normalized = hex.startsWith("0x") ? hex.slice(2) : hex;
  if (normalized.length % 2 !== 0) {
    throw new Error("Invalid hex input.");
  }
  if (!/^[0-9a-fA-F]*$/.test(normalized)) {
    throw new Error("Invalid hex input.");
  }
  const bytes = new Uint8Array(normalized.length / 2);
  for (let i = 0; i < normalized.length; i += 2) {
    bytes[i / 2] = Number.parseInt(normalized.slice(i, i + 2), 16);
  }
  return bytes;
}

export async function postJob(
  client: string,
  amount: string,
  descHashHex: string,
  descriptionPayloadLen: number,
  deadline: string,
  tokenAddress: string,
) {
  return callContract(requireContractId(), "post_job", [
    nativeToScVal(client, { type: "address" }),
    nativeToScVal(amount, { type: "i128" }),
    nativeToScVal(hexToBytes(descHashHex), { type: "bytes" }),
    nativeToScVal(descriptionPayloadLen, { type: "u32" }),
    nativeToScVal(deadline, { type: "u64" }),
    nativeToScVal(tokenAddress, { type: "address" }),
  ]);
}

export async function getCompletedJobsCount(): Promise<number> {
  const response = await callContract(
    requireContractId(),
    "get_completed_jobs_count",
    [],
    { readOnly: true },
  );
  return Number(response.data ?? 0);
}

export async function getDescPayloadMax(): Promise<number> {
  const response = await callContract(
    requireContractId(),
    "get_desc_payload_max",
    [],
    { readOnly: true },
  );
  return Number(response.data ?? 0);
}

export async function acceptJob(freelancer: string, jobId: string) {
  return callContract(requireContractId(), "accept_job", [
    nativeToScVal(freelancer, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function submitWork(freelancer: string, jobId: string) {
  return callContract(requireContractId(), "submit_work", [
    nativeToScVal(freelancer, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function approveWork(client: string, jobId: string) {
  return callContract(requireContractId(), "approve_work", [
    nativeToScVal(client, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function cancelJob(client: string, jobId: string) {
  return callContract(requireContractId(), "cancel_job", [
    nativeToScVal(client, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function enforceDeadline(client: string, jobId: string) {
  return callContract(requireContractId(), "enforce_deadline", [
    nativeToScVal(client, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function extendDeadline(
  client: string,
  jobId: string,
  newDeadline: string,
  freelancerConsent?: string,
) {
  const args = [
    nativeToScVal(client, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
    nativeToScVal(newDeadline, { type: "u64" }),
  ];
  if (freelancerConsent) {
    return callContract(requireContractId(), "extend_deadline", [
      ...args,
      nativeToScVal(xdr.ScVal.scvVec([nativeToScVal(freelancerConsent, { type: "address" })])),
    ]);
  }
  return callContract(requireContractId(), "extend_deadline", [
    ...args,
    xdr.ScVal.scvVec([]),
  ]);
}

export async function extendJobTtl(caller: string, jobId: string) {
  return callContract(requireContractId(), "extend_job_ttl", [
    nativeToScVal(caller, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function raiseDispute(caller: string, jobId: string) {
  return callContract(requireContractId(), "raise_dispute", [
    nativeToScVal(caller, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function resolveDispute(jobId: string, clientBps: number) {
  return callContract(requireContractId(), "resolve_dispute", [
    nativeToScVal(jobId, { type: "u64" }),
    xdr.ScVal.scvVec([nativeToScVal(clientBps, { type: "u32" })]),
  ]);
}

export async function withdrawFees(tokenAddress: string) {
  return callContract(requireContractId(), "withdraw_fees", [
    nativeToScVal(tokenAddress, { type: "address" }),
  ]);
}

export async function getFees(tokenAddress: string): Promise<number> {
  const response = await callContract(
    requireContractId(),
    "get_fees",
    [nativeToScVal(tokenAddress, { type: "address" })],
    { readOnly: true },
  );
  return Number(response.data ?? 0);
}

export async function addAllowedToken(tokenAddress: string) {
  return callContract(requireContractId(), "add_allowed_token", [
    nativeToScVal(tokenAddress, { type: "address" }),
  ]);
}

export async function removeAllowedToken(tokenAddress: string) {
  return callContract(requireContractId(), "remove_allowed_token", [
    nativeToScVal(tokenAddress, { type: "address" }),
  ]);
}

export async function isTokenAllowed(tokenAddress: string): Promise<boolean> {
  const response = await callContract(
    requireContractId(),
    "is_token_allowed",
    [nativeToScVal(tokenAddress, { type: "address" })],
    { readOnly: true },
  );
  return Boolean(response.data ?? false);
}

export async function getNativeToken(): Promise<string> {
  const response = await callContract(
    requireContractId(),
    "get_native_token",
    [],
    { readOnly: true },
  );
  return String(response.data ?? "");
}

export async function getJob(jobId: string): Promise<Job | null> {
  const response = await callContract(
    requireContractId(),
    "get_job",
    [nativeToScVal(jobId, { type: "u64" })],
    { readOnly: true },
  );
  return (response.data as Job) ?? null;
}

export async function getJobCount(): Promise<number> {
  const response = await callContract(
    requireContractId(),
    "get_job_count",
    [],
    {
      readOnly: true,
    },
  );
  return Number(response.data ?? 0);
}

export async function freelancerCancelJob(freelancer: string, jobId: string) {
  return callContract(requireContractId(), "freelancer_cancel_job", [
    nativeToScVal(freelancer, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
  ]);
}

export async function storeDescriptionCid(caller: string, descHashHex: string, cid: string) {
  return callContract(requireContractId(), "store_description_cid", [
    nativeToScVal(caller, { type: "address" }),
    nativeToScVal(hexToBytes(descHashHex), { type: "bytes" }),
    nativeToScVal(cid, { type: "string" }),
  ]);
}

export async function getDescriptionCid(descHashHex: string): Promise<string | null> {
  const response = await callContract(
    requireContractId(),
    "get_description_cid",
    [nativeToScVal(hexToBytes(descHashHex), { type: "bytes" })],
    { readOnly: true },
  );
  const cid = response.data as string;
  return cid || null;
}

// ─── Milestone helpers ────────────────────────────────────────────────────────

/** Input for a single milestone when creating a milestone-based job. */
export interface MilestoneInput {
  /** 32-byte description hash as a hex string (64 hex chars). */
  descriptionHashHex: string;
  /** Amount in stroops as a string. */
  amount: string;
}

/**
 * Create a job whose total escrow is the sum of all milestone amounts.
 * Returns the new job ID.
 */
export async function createJobWithMilestones(
  client: string,
  milestones: MilestoneInput[],
  descHashHex: string,
  descriptionPayloadLen: number,
  deadline: string,
  tokenAddress: string,
) {
  // Encode milestones as a Vec<MilestoneInput> — each element is a struct map.
  const encodedMilestones = xdr.ScVal.scvVec(
    milestones.map((m) =>
      xdr.ScVal.scvMap([
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("description_hash"),
          val: nativeToScVal(hexToBytes(m.descriptionHashHex), { type: "bytes" }),
        }),
        new xdr.ScMapEntry({
          key: xdr.ScVal.scvSymbol("amount"),
          val: nativeToScVal(m.amount, { type: "i128" }),
        }),
      ])
    )
  );

  return callContract(requireContractId(), "create_job_with_milestones", [
    nativeToScVal(client, { type: "address" }),
    encodedMilestones,
    nativeToScVal(hexToBytes(descHashHex), { type: "bytes" }),
    nativeToScVal(descriptionPayloadLen, { type: "u32" }),
    nativeToScVal(deadline, { type: "u64" }),
    nativeToScVal(tokenAddress, { type: "address" }),
  ]);
}

/**
 * Release payment for a single milestone.
 * Only the client may call this; the job must be InProgress.
 */
export async function approveMilestone(
  client: string,
  jobId: string,
  milestoneId: number,
) {
  return callContract(requireContractId(), "approve_milestone", [
    nativeToScVal(client, { type: "address" }),
    nativeToScVal(jobId, { type: "u64" }),
    nativeToScVal(milestoneId, { type: "u32" }),
  ]);
}

/**
 * Fetch all milestones for a job.
 * Returns null if the job has no milestones (regular job).
 */
export async function getMilestones(jobId: string): Promise<Milestone[] | null> {
  try {
    const response = await callContract(
      requireContractId(),
      "get_milestones",
      [nativeToScVal(jobId, { type: "u64" })],
      { readOnly: true },
    );
    if (!response.data) return null;
    return response.data as Milestone[];
  } catch {
    // Contract panics with NoMilestones (#23) for regular jobs — treat as null.
    return null;
  }
}

// --- Admin Job Views ---

export async function adminGetAllJobs(admin: string, startIndex: number, limit: number): Promise<Job[]> {
  const response = await callContract(
    requireContractId(),
    "admin_get_all_jobs",
    [
      nativeToScVal(admin, { type: "address" }),
      nativeToScVal(startIndex, { type: "u32" }),
      nativeToScVal(limit, { type: "u32" }),
    ],
    { readOnly: true },
  );
  return (response.data as Job[]) ?? [];
}

export async function adminGetJobCount(admin: string): Promise<number> {
  const response = await callContract(
    requireContractId(),
    "admin_get_job_count",
    [nativeToScVal(admin, { type: "address" })],
    { readOnly: true },
  );
  return Number(response.data ?? 0);
}

export async function adminGetJobsByStatus(admin: string, status: string, startIndex: number, limit: number): Promise<Job[]> {
  const response = await callContract(
    requireContractId(),
    "admin_get_jobs_by_status",
    [
      nativeToScVal(admin, { type: "address" }),
      // For enums, nativeToScVal converts the string literal cleanly for Soroban
      nativeToScVal(status, { type: "symbol" }),
      nativeToScVal(startIndex, { type: "u32" }),
      nativeToScVal(limit, { type: "u32" }),
    ],
    { readOnly: true },
  );
  return (response.data as Job[]) ?? [];
}

// --- Access Control ---

export async function setWhitelistMode(admin: string, enabled: boolean) {
  return callContract(requireContractId(), "set_whitelist_mode", [
    nativeToScVal(admin, { type: "address" }),
    nativeToScVal(enabled, { type: "bool" }),
  ]);
}

export async function isWhitelistModeEnabled(): Promise<boolean> {
  const response = await callContract(
    requireContractId(),
    "is_whitelist_mode_enabled",
    [],
    { readOnly: true },
  );
  return Boolean(response.data ?? false);
}

export async function addToBlacklist(admin: string, address: string) {
  return callContract(requireContractId(), "add_to_blacklist", [
    nativeToScVal(admin, { type: "address" }),
    nativeToScVal(address, { type: "address" }),
  ]);
}

export async function removeFromBlacklist(admin: string, address: string) {
  return callContract(requireContractId(), "remove_from_blacklist", [
    nativeToScVal(admin, { type: "address" }),
    nativeToScVal(address, { type: "address" }),
  ]);
}

export async function addToWhitelist(admin: string, address: string) {
  return callContract(requireContractId(), "add_to_whitelist", [
    nativeToScVal(admin, { type: "address" }),
    nativeToScVal(address, { type: "address" }),
  ]);
}

export async function removeFromWhitelist(admin: string, address: string) {
  return callContract(requireContractId(), "remove_from_whitelist", [
    nativeToScVal(admin, { type: "address" }),
    nativeToScVal(address, { type: "address" }),
  ]);
}

export async function isBlacklisted(address: string): Promise<boolean> {
  const response = await callContract(
    requireContractId(),
    "is_blacklisted",
    [nativeToScVal(address, { type: "address" })],
    { readOnly: true },
  );
  return Boolean(response.data ?? false);
}

export async function isWhitelisted(address: string): Promise<boolean> {
  const response = await callContract(
    requireContractId(),
    "is_whitelisted",
    [nativeToScVal(address, { type: "address" })],
    { readOnly: true },
  );
  return Boolean(response.data ?? false);
}

