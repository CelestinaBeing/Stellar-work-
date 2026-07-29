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
import type { Notification, NotificationEvent, NotificationPreferences, JobStatus } from "@/lib/types";

const STORAGE_KEY = "stellarwork:notifications";
const PREFS_KEY = "stellarwork:notif-prefs";
const POLL_INTERVAL = 30000;

type NotificationContextValue = {
  notifications: Notification[];
  unreadCount: number;
  addNotification: (event: NotificationEvent, jobId: number, message: string) => void;
  markAsSeen: (id: string) => void;
  markAllAsSeen: () => void;
  preferences: NotificationPreferences;
  setPreference: (event: NotificationEvent, enabled: boolean) => void;
  clearNotifications: () => void;
};

const DEFAULT_PREFS: NotificationPreferences = {
  job_accepted: true,
  work_submitted: true,
  work_approved: true,
  job_cancelled: true,
  dispute_raised: true,
  dispute_resolved: true,
};

const NotificationContext = createContext<NotificationContextValue | null>(null);

function loadNotifications(): Notification[] {
  if (typeof window === "undefined") return [];
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    return JSON.parse(raw) as Notification[];
  } catch {
    return [];
  }
}

function loadPreferences(): NotificationPreferences {
  if (typeof window === "undefined") return DEFAULT_PREFS;
  try {
    const raw = localStorage.getItem(PREFS_KEY);
    if (!raw) return DEFAULT_PREFS;
    return { ...DEFAULT_PREFS, ...JSON.parse(raw) };
  } catch {
    return DEFAULT_PREFS;
  }
}

function saveNotifications(notifications: Notification[]) {
  if (typeof window === "undefined") return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(notifications));
  } catch {
    // Storage full - ignore
  }
}

function savePreferences(prefs: NotificationPreferences) {
  if (typeof window === "undefined") return;
  try {
    localStorage.setItem(PREFS_KEY, JSON.stringify(prefs));
  } catch {
    // Storage full - ignore
  }
}

const EVENT_LABELS: Record<NotificationEvent, string> = {
  job_accepted: "Job Accepted",
  work_submitted: "Work Submitted",
  work_approved: "Work Approved",
  job_cancelled: "Job Cancelled",
  dispute_raised: "Dispute Raised",
  dispute_resolved: "Dispute Resolved",
};

export function getEventLabel(event: NotificationEvent): string {
  return EVENT_LABELS[event];
}

export const STATUS_TO_EVENT: Record<JobStatus, NotificationEvent | null> = {
  Open: null,
  InProgress: "job_accepted",
  SubmittedForReview: "work_submitted",
  Completed: "work_approved",
  Cancelled: "job_cancelled",
  Disputed: "dispute_raised",
};

export function NotificationProvider({ children }: { children: ReactNode }) {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [preferences, setPreferences] = useState<NotificationPreferences>(DEFAULT_PREFS);
  const [trackedJobs, setTrackedJobs] = useState<Map<number, JobStatus>>(new Map());
  const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const prevTrackedRef = useRef<string>("");

  useEffect(() => {
    setNotifications(loadNotifications());
    setPreferences(loadPreferences());
  }, []);

  useEffect(() => {
    saveNotifications(notifications);
  }, [notifications]);

  useEffect(() => {
    savePreferences(preferences);
  }, [preferences]);

  const addNotification = useCallback(
    (event: NotificationEvent, jobId: number, message: string) => {
      if (!preferences[event]) return;
      const notification: Notification = {
        id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
        event,
        jobId,
        message,
        timestamp: Date.now(),
        seen: false,
      };
      setNotifications((prev) => [notification, ...prev].slice(0, 100));
    },
    [preferences],
  );

  useEffect(() => {
    const trackedStr = JSON.stringify(Array.from(trackedJobs.entries()));
    if (trackedStr === prevTrackedRef.current) return;
    prevTrackedRef.current = trackedStr;

    if (pollRef.current) {
      clearInterval(pollRef.current);
      pollRef.current = null;
    }

    if (trackedJobs.size === 0) return;

    pollRef.current = setInterval(async () => {
      try {
        const { getJob } = await import("@/lib/contract");
        for (const [jobId, prevStatus] of trackedJobs.entries()) {
          const job = await getJob(String(jobId));
          if (job && job.status !== prevStatus) {
            const event = STATUS_TO_EVENT[job.status];
            if (event && preferences[event]) {
              addNotification(event, jobId, job.status);
            }
            setTrackedJobs((prev) => {
              const next = new Map(prev);
              next.set(jobId, job.status);
              return next;
            });
          }
        }
      } catch {
        // Silently handle poll errors
      }
    }, POLL_INTERVAL);

    return () => {
      if (pollRef.current) {
        clearInterval(pollRef.current);
        pollRef.current = null;
      }
    };
  }, [trackedJobs, preferences, addNotification]);

  const unreadCount = useMemo(
    () => notifications.filter((n) => !n.seen).length,
    [notifications],
  );

  const markAsSeen = useCallback((id: string) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, seen: true } : n)),
    );
  }, []);

  const markAllAsSeen = useCallback(() => {
    setNotifications((prev) => prev.map((n) => ({ ...n, seen: true })));
  }, []);

  const setPreference = useCallback(
    (event: NotificationEvent, enabled: boolean) => {
      setPreferences((prev) => ({ ...prev, [event]: enabled }));
    },
    [],
  );

  const clearNotifications = useCallback(() => {
    setNotifications([]);
  }, []);

  const trackJob = useCallback((jobId: number, status: JobStatus) => {
    setTrackedJobs((prev) => {
      if (prev.get(jobId) === status) return prev;
      const next = new Map(prev);
      next.set(jobId, status);
      return next;
    });
  }, []);

  const untrackJob = useCallback((jobId: number) => {
    setTrackedJobs((prev) => {
      if (!prev.has(jobId)) return prev;
      const next = new Map(prev);
      next.delete(jobId);
      return next;
    });
  }, []);

  const value = useMemo(
    () => ({
      notifications,
      unreadCount,
      addNotification,
      markAsSeen,
      markAllAsSeen,
      preferences,
      setPreference,
      clearNotifications,
      trackJob,
      untrackJob,
    }),
    [
      notifications,
      unreadCount,
      addNotification,
      markAsSeen,
      markAllAsSeen,
      preferences,
      setPreference,
      clearNotifications,
      trackJob,
      untrackJob,
    ],
  );

  return (
    <NotificationContext.Provider value={value}>
      {children}
    </NotificationContext.Provider>
  );
}

export function useNotifications() {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error("useNotifications must be used within a NotificationProvider");
  }
  return context;
}
