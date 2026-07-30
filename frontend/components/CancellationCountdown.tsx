"use client";

import { useEffect, useState } from "react";

interface CancellationCountdownProps {
  createdAt: number;
  gracePeriodLedgers: number;
}

function ledgersToMs(ledgers: number): number {
  return ledgers * 5000;
}

function formatCountdown(ms: number): string {
  if (ms <= 0) return "Expired";
  const totalSec = Math.ceil(ms / 1000);
  const min = Math.floor(totalSec / 60);
  const sec = totalSec % 60;
  if (min > 0) return `${min}m ${sec}s`;
  return `${sec}s`;
}

export default function CancellationCountdown({
  createdAt,
  gracePeriodLedgers,
}: CancellationCountdownProps) {
  const [remaining, setRemaining] = useState<number>(0);
  const [expired, setExpired] = useState(false);

  useEffect(() => {
    const graceMs = ledgersToMs(gracePeriodLedgers);
    const createdMs = createdAt * 5000;
    const deadlineMs = createdMs + graceMs;

    const tick = () => {
      const now = Date.now();
      const left = deadlineMs - now;
      if (left <= 0) {
        setRemaining(0);
        setExpired(true);
      } else {
        setRemaining(left);
        setExpired(false);
      }
    };

    tick();
    const interval = setInterval(tick, 1000);
    return () => clearInterval(interval);
  }, [createdAt, gracePeriodLedgers]);

  if (expired) return null;

  const pct = (() => {
    const graceMs = ledgersToMs(gracePeriodLedgers);
    return ((graceMs - remaining) / graceMs) * 100;
  })();

  return (
    <div className="flex items-center gap-2 rounded-lg border border-emerald-200 bg-emerald-50 px-3 py-2 text-sm dark:border-emerald-800 dark:bg-emerald-950/30" role="status" aria-live="polite">
      <svg className="h-4 w-4 shrink-0 text-emerald-600 dark:text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5} aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <div className="flex-1">
        <span className="font-medium text-emerald-800 dark:text-emerald-300">
          Rebate eligible — {formatCountdown(remaining)}
        </span>
        <div className="mt-1 h-1.5 w-full overflow-hidden rounded-full bg-emerald-200 dark:bg-emerald-800">
          <div
            className="h-full rounded-full bg-emerald-500 transition-all duration-1000"
            style={{ width: `${Math.min(100, pct)}%` }}
          />
        </div>
      </div>
      <span className="shrink-0 text-xs text-emerald-600 dark:text-emerald-400">
        Full refund
      </span>
    </div>
  );
}
