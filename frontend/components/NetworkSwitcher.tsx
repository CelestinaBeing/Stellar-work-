"use client";

import { useState, useRef, useEffect } from "react";
import { useNetwork, NETWORK_LIST } from "@/lib/network-context";
import { persistNetwork } from "@/lib/network-config";
import type { StellarNetwork } from "@/lib/network-config";

const NETWORK_ICONS: Record<StellarNetwork, string> = {
  testnet: "\u26A1",
  futurenet: "\uD83D\uDD2E",
  mainnet: "\uD83C\uDF10",
};

export default function NetworkSwitcher() {
  const { network, setNetwork } = useNetwork();
  const [open, setOpen] = useState(false);
  const [confirmTarget, setConfirmTarget] = useState<StellarNetwork | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [open]);

  useEffect(() => {
    if (!open) return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        setOpen(false);
        setConfirmTarget(null);
      }
    };
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, [open]);

  const handleSelect = (target: StellarNetwork) => {
    if (target === network) {
      setOpen(false);
      return;
    }
    if (target === "mainnet") {
      setConfirmTarget(target);
    } else {
      persistNetwork(target);
      setNetwork(target);
      setOpen(false);
      window.location.reload();
    }
  };

  const handleConfirmMainnet = () => {
    if (confirmTarget) {
      persistNetwork(confirmTarget);
      setNetwork(confirmTarget);
      setOpen(false);
      setConfirmTarget(null);
      window.location.reload();
    }
  };

  const handleCancelMainnet = () => {
    setConfirmTarget(null);
    setOpen(false);
  };

  return (
    <div ref={containerRef} className="relative">
      <button
        onClick={() => setOpen((prev) => !prev)}
        className="rounded-md px-2 py-1 text-xs font-medium text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800"
        aria-label={`Switch network (current: ${network})`}
        aria-haspopup="listbox"
        aria-expanded={open}
      >
        {NETWORK_ICONS[network]} {network.charAt(0).toUpperCase() + network.slice(1)}
        <svg className="ml-1 inline-block h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {open && !confirmTarget && (
        <div
          className="absolute right-0 z-50 mt-1 w-48 rounded-md border border-slate-200 bg-white py-1 shadow-lg dark:border-slate-700 dark:bg-slate-800"
          role="listbox"
          aria-label="Select network"
        >
          {NETWORK_LIST.map((n) => {
            const isActive = n === network;
            return (
              <button
                key={n}
                role="option"
                aria-selected={isActive}
                onClick={() => handleSelect(n)}
                className={`flex w-full items-center gap-2 px-3 py-2 text-left text-sm ${
                  isActive
                    ? "bg-slate-100 font-semibold text-slate-900 dark:bg-slate-700 dark:text-slate-100"
                    : "text-slate-700 hover:bg-slate-50 dark:text-slate-300 dark:hover:bg-slate-700"
                }`}
              >
                <span>{NETWORK_ICONS[n]}</span>
                <span>{n.charAt(0).toUpperCase() + n.slice(1)}</span>
                {isActive && (
                  <svg className="ml-auto h-4 w-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                )}
              </button>
            );
          })}
        </div>
      )}

      {confirmTarget === "mainnet" && (
        <div className="absolute right-0 z-50 mt-1 w-72 rounded-md border border-red-200 bg-white p-4 shadow-lg dark:border-red-800 dark:bg-slate-800">
          <div className="mb-2 flex items-center gap-2 text-sm font-semibold text-red-700 dark:text-red-400">
            <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4.5c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
            Switch to Mainnet?
          </div>
          <p className="mb-3 text-xs text-slate-600 dark:text-slate-400">
            You are about to switch to the Stellar mainnet. Transactions will use real XLM and
            real funds. Are you sure?
          </p>
          <div className="flex gap-2">
            <button
              onClick={handleConfirmMainnet}
              className="flex-1 rounded-md bg-red-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-red-700"
            >
              Yes, Switch
            </button>
            <button
              onClick={handleCancelMainnet}
              className="flex-1 rounded-md border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-50 dark:border-slate-600 dark:text-slate-300 dark:hover:bg-slate-700"
            >
              Cancel
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
