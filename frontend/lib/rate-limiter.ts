"use client";

const RATE_LIMIT_STORAGE_KEY = "stellarwork:post-job-rate-limit";
const MAX_POSTS_PER_HOUR = 5;
const RATE_LIMIT_WINDOW_MS = 60 * 60 * 1000;

interface RateLimitEntry {
  timestamp: number;
}

export interface RateLimitStatus {
  remaining: number;
  cooldownEndsAt: number | null;
  isLimited: boolean;
}

function loadEntries(): RateLimitEntry[] {
  if (typeof window === "undefined") return [];
  try {
    const raw = localStorage.getItem(RATE_LIMIT_STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw) as unknown;
    if (!Array.isArray(parsed)) return [];
    return parsed.filter(
      (entry): entry is RateLimitEntry =>
        typeof entry === "object" &&
        entry !== null &&
        typeof (entry as RateLimitEntry).timestamp === "number",
    );
  } catch {
    return [];
  }
}

function saveEntries(entries: RateLimitEntry[]): void {
  if (typeof window === "undefined") return;
  try {
    localStorage.setItem(RATE_LIMIT_STORAGE_KEY, JSON.stringify(entries));
  } catch {
    // localStorage may be full or unavailable
  }
}

function purgeExpiredEntries(entries: RateLimitEntry[]): RateLimitEntry[] {
  const cutoff = Date.now() - RATE_LIMIT_WINDOW_MS;
  return entries.filter((e) => e.timestamp > cutoff);
}

export function getRateLimitStatus(): RateLimitStatus {
  const entries = purgeExpiredEntries(loadEntries());
  saveEntries(entries);

  const remaining = Math.max(0, MAX_POSTS_PER_HOUR - entries.length);
  const cooldownEndsAt =
    entries.length > 0
      ? entries[0].timestamp + RATE_LIMIT_WINDOW_MS
      : null;

  if (cooldownEndsAt !== null && Date.now() < cooldownEndsAt) {
    return { remaining, cooldownEndsAt, isLimited: remaining === 0 };
  }

  return {
    remaining: MAX_POSTS_PER_HOUR,
    cooldownEndsAt: null,
    isLimited: false,
  };
}

export function recordPostJob(): void {
  const entries = purgeExpiredEntries(loadEntries());
  entries.push({ timestamp: Date.now() });
  saveEntries(entries);
}

export function formatCooldown(cooldownEndsAt: number): string {
  const remaining = Math.max(0, cooldownEndsAt - Date.now());
  const minutes = Math.floor(remaining / 60000);
  const seconds = Math.floor((remaining % 60000) / 1000);
  return `${minutes}m ${seconds.toString().padStart(2, "0")}s`;
}
