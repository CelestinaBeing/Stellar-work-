"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { countTotalUnread } from "@/lib/messaging";
import { useWallet } from "@/lib/wallet-context";

// ─── Types ────────────────────────────────────────────────────────────────────

interface MessagingContextValue {
  /** Total unread message count across all conversations. */
  unreadCount: number;
  /** Call to manually force a refresh of the unread count. */
  refreshUnread: () => void;
}

// ─── Context ──────────────────────────────────────────────────────────────────

const MessagingContext = createContext<MessagingContextValue | null>(null);

const POLL_INTERVAL = 5_000; // poll every 5 s to detect messages from other tabs

// ─── Provider ─────────────────────────────────────────────────────────────────

export function MessagingProvider({ children }: { children: ReactNode }) {
  const { wallet } = useWallet();
  const [unreadCount, setUnreadCount] = useState(0);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const refreshUnread = useCallback(() => {
    if (!wallet) {
      setUnreadCount(0);
      return;
    }
    setUnreadCount(countTotalUnread(wallet));
  }, [wallet]);

  // Poll for new messages (cross-tab delivery via localStorage).
  useEffect(() => {
    refreshUnread();

    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    if (!wallet) return;

    intervalRef.current = setInterval(refreshUnread, POLL_INTERVAL);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, [wallet, refreshUnread]);

  // Also react to storage events from other tabs.
  useEffect(() => {
    const handler = (e: StorageEvent) => {
      if (e.key?.startsWith("sw:msg:")) {
        refreshUnread();
      }
    };
    window.addEventListener("storage", handler);
    return () => window.removeEventListener("storage", handler);
  }, [refreshUnread]);

  const value = useMemo(
    () => ({ unreadCount, refreshUnread }),
    [unreadCount, refreshUnread],
  );

  return (
    <MessagingContext.Provider value={value}>
      {children}
    </MessagingContext.Provider>
  );
}

// ─── Hook ─────────────────────────────────────────────────────────────────────

export function useMessaging(): MessagingContextValue {
  const ctx = useContext(MessagingContext);
  if (!ctx) throw new Error("useMessaging must be used within MessagingProvider");
  return ctx;
}
