"use client";

import {
  approveWork,
  batchApproveJobs,
  cancelJob,
  freelancerCancelJob,
  getJob,
  getJobCount,
  getCompletedJobsCount,
  submitWork,
  enforceDeadline,
} from "@/lib/contract";
import CancelJobConfirmModal from "@/components/CancelJobConfirmModal";
import EmptyState from "@/components/EmptyState";
import ErrorBanner from "@/components/ErrorBanner";
import InfoTooltip from "@/components/InfoTooltip";
import JobCardSkeleton from "@/components/JobCardSkeleton";
import NoResultsState from "@/components/NoResultsState";
import SectionCard from "@/components/SectionCard";
import StatusPill from "@/components/StatusPill";
import { useToast } from "@/components/ToastProvider";
import { useNotifications, getEventLabel } from "@/lib/notifications-context";
import { formatDeadline, toXlm } from "@/lib/format";
import { useWallet } from "@/lib/wallet-context";
import type { Job, JobStatus, NotificationEvent } from "@/lib/types";
import { useEffect, useState, useCallback, useRef, type KeyboardEvent } from "react";

const STATUS_OPTIONS: JobStatus[] = [
  "Open",
  "InProgress",
  "SubmittedForReview",
  "Completed",
  "Cancelled",
];

const EVENT_DOT: Record<string, string> = {
  job_accepted: "bg-blue-500",
  work_submitted: "bg-amber-500",
  work_approved: "bg-emerald-500",
  job_cancelled: "bg-slate-500",
  dispute_raised: "bg-red-500",
  dispute_resolved: "bg-violet-500",
};

const STATUS_LABELS: Record<JobStatus, string> = {
  Open: "Open",
  InProgress: "In Progress",
  SubmittedForReview: "Submitted for Review",
  Completed: "Completed",
  Cancelled: "Cancelled",
  Disputed: "Disputed",
};

export default function DashboardPage() {
  const { wallet, connectWallet } = useWallet();
  const { showSuccess, showError } = useToast();
  const { notifications, addNotification } = useNotifications();
  const [allJobs, setAllJobs] = useState<Array<{ id: number; job: Job }>>([]);
  const [statusFilter, setStatusFilter] = useState<JobStatus | "All">("All");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [actionLoading, setActionLoading] = useState<number | null>(null);
  const [pendingCancelJobId, setPendingCancelJobId] = useState<number | null>(null);
  const [completedJobsCount, setCompletedJobsCount] = useState<number | null>(null);
  const [selectedJobs, setSelectedJobs] = useState<Set<number>>(new Set());
  const [batchLoading, setBatchLoading] = useState(false);
  const filterOptions: Array<JobStatus | "All"> = ["All", ...STATUS_OPTIONS];
  const filterButtonRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const fetchJobs = useCallback(async () => {
    if (!wallet) return;
    setLoading(true);
    setError(null);
    try {
      const count = await getJobCount();
      const fetched: Array<{ id: number; job: Job }> = [];
      for (let id = 1; id <= count; id += 1) {
        const job = await getJob(String(id));
        if (job && (job.client === wallet || job.freelancer === wallet)) {
          fetched.push({ id, job });
        }
      }
      setAllJobs(fetched);
      try {
        const completed = await getCompletedJobsCount();
        setCompletedJobsCount(completed);
      } catch {
        setCompletedJobsCount(null);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to fetch jobs.");
    } finally {
      setLoading(false);
    }
  }, [wallet]);

  useEffect(() => {
    if (wallet) {
      fetchJobs();
    } else {
      setAllJobs([]);
      setCompletedJobsCount(null);
      setLoading(false);
      setError(null);
    }
  }, [wallet, fetchJobs]);

  const handleAction = async (
    fn: () => Promise<unknown>,
    jobId: number,
    notification?: { event: NotificationEvent; message: string },
  ) => {
    setActionLoading(jobId);
    setError(null);
    try {
      await fn();
      if (notification) {
        addNotification(notification.event, jobId, notification.message);
      }
      await fetchJobs();
      setError(null);
      showSuccess("Action completed successfully.");
    } catch (e) {
      const message = e instanceof Error ? e.message : "Action failed.";
      setError(message);
      showError(message);
    } finally {
      setActionLoading(null);
    }
  };

  const handleBatchApprove = async () => {
    if (!wallet || selectedJobs.size === 0) return;
    setBatchLoading(true);
    setError(null);
    try {
      const jobIds = Array.from(selectedJobs).map(String);
      await batchApproveJobs(wallet, jobIds);
      showSuccess(`${jobIds.length} job(s) approved.`);
      setSelectedJobs(new Set());
      await fetchJobs();
    } catch (e) {
      const message = e instanceof Error ? e.message : "Batch approval failed.";
      setError(message);
      showError(message);
    } finally {
      setBatchLoading(false);
    }
  };

  const handleConfirmCancel = async () => {
    if (!wallet || pendingCancelJobId === null) {
      return;
    }
    const jobId = pendingCancelJobId;
    await handleAction(
      () => cancelJob(wallet, String(jobId)),
      jobId,
      { event: "job_cancelled", message: `Job #${jobId} was cancelled and funds refunded.` },
    );
    setPendingCancelJobId(null);
  };

  const postedJobs = allJobs.filter((j) => j.job.client === wallet);
  const acceptedJobs = allJobs.filter((j) => j.job.freelancer === wallet);

  const filterJobs = (jobs: Array<{ id: number; job: Job }>) => {
    if (statusFilter === "All") return jobs;
    return jobs.filter((j) => j.job.status === statusFilter);
  };

  const filteredPosted = filterJobs(postedJobs);
  const filteredAccepted = filterJobs(acceptedJobs);

  const handleFilterKeyDown = (
    event: KeyboardEvent<HTMLButtonElement>,
    index: number,
  ) => {
    if (event.key !== "ArrowRight" && event.key !== "ArrowLeft") {
      return;
    }
    event.preventDefault();
    const delta = event.key === "ArrowRight" ? 1 : -1;
    const nextIndex =
      (index + delta + filterOptions.length) % filterOptions.length;
    const nextFilter = filterOptions[nextIndex];
    setStatusFilter(nextFilter);
    filterButtonRefs.current[nextIndex]?.focus();
  };

  if (!wallet) {
    return (
      <section className="mx-auto max-w-3xl space-y-6">
        <h1 className="text-2xl font-semibold">Dashboard</h1>
        <SectionCard className="p-8 text-center">
          <p className="text-slate-600">Connect your wallet to view your jobs.</p>
          <button
            className="mt-4 rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-700"
            onClick={async () => {
              try { await connectWallet(); } catch { /* cancelled */ }
            }}
          >
            Connect Wallet
          </button>
        </SectionCard>
      </section>
    );
  }

  return (
    <section className="space-y-6">
      <h1 className="text-2xl font-semibold">Dashboard</h1>

      <div className="grid grid-cols-2 gap-4 sm:max-w-md">
        <div className="interactive-card p-4">
          <p className="text-2xl font-bold tabular-nums">
            {completedJobsCount ?? "—"}
          </p>
          <p className="text-xs text-slate-500">Completed jobs (contract)</p>
        </div>
        <div className="interactive-card p-4">
          <p className="text-2xl font-bold tabular-nums">{allJobs.length}</p>
          <p className="text-xs text-slate-500">Your jobs on record</p>
        </div>
      </div>

      {/* Activity Feed */}
      {notifications.length > 0 && (
        <SectionCard className="overflow-hidden">
          <div className="flex items-center justify-between border-b border-slate-100 px-5 py-3">
            <h2 className="text-sm font-semibold text-slate-900">Recent Activity</h2>
            <span className="text-xs text-slate-400">{notifications.length} event{notifications.length !== 1 ? "s" : ""}</span>
          </div>
          <div className="divide-y divide-slate-100">
            {notifications.slice(0, 10).map((n) => {
              const dot = EVENT_DOT[n.event] ?? "bg-slate-400";
              return (
                <div
                  key={n.id}
                  className={`flex items-start gap-3 px-5 py-3 ${n.seen ? "" : "bg-blue-50/50"}`}
                >
                  <span className={`mt-1.5 h-2 w-2 shrink-0 rounded-full ${dot}`} />
                  <div className="min-w-0 flex-1">
                    <p className="text-sm text-slate-800">{n.message}</p>
                    <p className="mt-0.5 text-xs text-slate-400">
                      {getEventLabel(n.event)} &middot; {new Date(n.timestamp).toLocaleDateString("en-GB", { day: "numeric", month: "short", hour: "2-digit", minute: "2-digit" })}
                    </p>
                  </div>
                </div>
              );
            })}
          </div>
        </SectionCard>
      )}

      <div
        className="flex flex-wrap gap-2"
        role="toolbar"
        aria-label="Filter jobs by status"
      >
        <div className="mr-1 flex items-center gap-2 text-sm text-slate-600">
          <span>Filter:</span>
          <InfoTooltip
            label="Filter jobs by status help"
            content="Use the status chips to narrow your job history. Arrow keys move between filters."
          />
        </div>
        {filterOptions.map((s, index) => (
          <button
            key={s}
            ref={(element) => {
              filterButtonRefs.current[index] = element;
            }}
            className={`rounded-full px-3 py-1 text-sm ${statusFilter === s ? "bg-slate-900 text-white" : "border border-slate-300 text-slate-700"}`}
            onClick={() => setStatusFilter(s)}
            onKeyDown={(event) => handleFilterKeyDown(event, index)}
            aria-pressed={statusFilter === s}
            aria-label={`${s === "All" ? "All statuses" : STATUS_LABELS[s]} filter, ${
              statusFilter === s ? "selected" : "not selected"
            }`}
          >
            {s === "All" ? "All" : STATUS_LABELS[s]}
          </button>
        ))}
      </div>

      {error && (
        <ErrorBanner
          message={error}
          onDismiss={() => setError(null)}
          onRetry={() => void fetchJobs()}
        />
      )}
      {loading && (
        <div className="grid gap-4 md:grid-cols-2" aria-label="Loading jobs">
          {Array.from({ length: 4 }).map((_, index) => (
            <JobCardSkeleton key={index} />
          ))}
        </div>
      )}

      {!loading && (
        <>
          <JobSection
            title="Posted Jobs"
            subtitle="Jobs you created as a client"
            allJobs={postedJobs}
            jobs={filteredPosted}
            filterActive={statusFilter !== "All"}
            wallet={wallet}
            role="client"
            actionLoading={actionLoading}
            onAction={handleAction}
            onRequestCancel={setPendingCancelJobId}
            onClearFilter={() => setStatusFilter("All")}
            selectedJobs={selectedJobs}
            onToggleSelect={(id) => {
              setSelectedJobs((prev) => {
                const next = new Set(prev);
                if (next.has(id)) next.delete(id);
                else next.add(id);
                return next;
              });
            }}
            onBatchApprove={handleBatchApprove}
            batchLoading={batchLoading}
          />
          <JobSection
            title="Accepted Jobs"
            subtitle="Jobs you accepted as a freelancer"
            allJobs={acceptedJobs}
            jobs={filteredAccepted}
            filterActive={statusFilter !== "All"}
            wallet={wallet}
            role="freelancer"
            actionLoading={actionLoading}
            onAction={handleAction}
            onRequestCancel={setPendingCancelJobId}
            onClearFilter={() => setStatusFilter("All")}
          />
        </>
      )}

      {pendingCancelJobId !== null && (
        <CancelJobConfirmModal
          jobId={String(pendingCancelJobId)}
          loading={actionLoading === pendingCancelJobId}
          onClose={() => setPendingCancelJobId(null)}
          onConfirm={() => {
            void handleConfirmCancel();
          }}
        />
      )}
    </section>
  );
}

function JobSection({
  title,
  subtitle,
  allJobs,
  jobs,
  filterActive,
  wallet,
  role,
  actionLoading,
  onAction,
  onRequestCancel,
  onClearFilter,
  selectedJobs,
  onToggleSelect,
  onBatchApprove,
  batchLoading,
}: {
  title: string;
  subtitle: string;
  allJobs: Array<{ id: number; job: Job }>;
  jobs: Array<{ id: number; job: Job }>;
  filterActive: boolean;
  wallet: string;
  role: "client" | "freelancer";
  actionLoading: number | null;
  onAction: (fn: () => Promise<unknown>, jobId: number, notification?: { event: NotificationEvent; message: string }) => Promise<void>;
  onRequestCancel: (jobId: number) => void;
  onClearFilter: () => void;
  selectedJobs?: Set<number>;
  onToggleSelect?: (id: number) => void;
  onBatchApprove?: () => void;
  batchLoading?: boolean;
}) {
  const pendingReviewIds = allJobs
    .filter((j) => j.job.status === "SubmittedForReview")
    .map((j) => j.id);
  const hasPendingReview = pendingReviewIds.length > 0;

  return (
    <div>
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-lg font-semibold">{title}</h2>
          <p className="mb-3 text-sm text-slate-500">{subtitle}</p>
        </div>
        {role === "client" && hasPendingReview && (
          <div className="flex items-center gap-2">
            <span className="text-xs text-slate-400">
              {selectedJobs?.size ?? 0} of {pendingReviewIds.length} selected
            </span>
            <button
              type="button"
              onClick={onBatchApprove}
              disabled={!selectedJobs || selectedJobs.size === 0 || batchLoading}
              className="rounded-md bg-emerald-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-emerald-700 disabled:cursor-not-allowed disabled:opacity-50"
            >
              {batchLoading ? "Approving..." : `Approve Selected (${selectedJobs?.size ?? 0})`}
            </button>
          </div>
        )}
      </div>
      {jobs.length === 0 ? (
        filterActive && allJobs.length > 0 ? (
          <NoResultsState
            title="No jobs match this filter"
            description="Try a different status or clear the filter to show every job in this section."
            actionLabel="Clear filter"
            onAction={onClearFilter}
          />
        ) : (
          <EmptyState
            title="No jobs yet"
            description="No jobs match this filter yet."
          />
        )
      ) : (
          <ul className="grid list-none gap-4 sm:grid-cols-2" aria-label={title}>
            {jobs.map(({ id, job }) => (
              <li key={id}>
                <JobCard
                  id={id}
                  job={job}
                  wallet={wallet}
                  role={role}
                  isLoading={actionLoading === id}
                  onAction={onAction}
                  onRequestCancel={onRequestCancel}
                  isSelected={selectedJobs?.has(id) ?? false}
                  onToggleSelect={role === "client" ? onToggleSelect : undefined}
                />
              </li>
            ))}
          </ul>
      )}
    </div>
  );
}

function JobCard({
  id,
  job,
  wallet,
  role,
  isLoading,
  onAction,
  onRequestCancel,
  isSelected,
  onToggleSelect,
}: {
  id: number;
  job: Job;
  wallet: string;
  role: "client" | "freelancer";
  isLoading: boolean;
  onAction: (fn: () => Promise<unknown>, jobId: number, notification?: { event: NotificationEvent; message: string }) => Promise<void>;
  onRequestCancel: (jobId: number) => void;
  isSelected?: boolean;
  onToggleSelect?: (id: number) => void;
}) {
  const actions = getActions(id, job, wallet, role);

  return (
    <article className={`interactive-card h-full p-4 ${isSelected ? "ring-2 ring-emerald-400" : ""}`}>
      <div className="flex items-start justify-between gap-2">
        <div className="flex items-center gap-2">
          {onToggleSelect && job.status === "SubmittedForReview" && (
            <input
              type="checkbox"
              checked={isSelected ?? false}
              onChange={() => onToggleSelect(id)}
              className="h-4 w-4 rounded border-slate-300 text-emerald-600"
              aria-label={`Select Job #${id} for batch approval`}
            />
          )}
          <h3 className="font-medium">Job #{id}</h3>
        </div>
        <StatusPill status={job.status} />
      </div>
      <div className="mt-2 space-y-1 text-sm text-slate-600">
        <p className="flex min-w-0 items-baseline gap-1">
          <span className="min-w-0 flex-1 overflow-hidden text-ellipsis whitespace-nowrap tabular-nums">
            {toXlm(job.amount)}
          </span>
          <span className="shrink-0">XLM</span>
        </p>
        <p>
          {(() => {
            const deadline = formatDeadline(job.deadline);
            if (!deadline) return "Deadline: No deadline";
            return `Deadline: ${deadline.isPast ? "Past due" : deadline.relative} • ${deadline.exact}`;
          })()}
        </p>
        {role === "client" && job.freelancer && (
          <p className="truncate">Freelancer: {job.freelancer}</p>
        )}
        {role === "freelancer" && (
          <p className="truncate">Client: {job.client}</p>
        )}
      </div>
      {actions.length > 0 && (
        <div className="mt-3 flex flex-wrap gap-2">
          {actions.map((action) => (
            <button
              key={action.label}
              disabled={isLoading}
              className="min-w-0 flex-1 rounded-md border border-slate-300 px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50 disabled:cursor-not-allowed disabled:opacity-60 sm:flex-none sm:max-w-44"
              onClick={() => {
                if (action.label === "Cancel Job") {
                  onRequestCancel(id);
                  return;
                }
                void onAction(() => action.fn(), id, action.notification ?? undefined);
              }}
              title={action.label}
              aria-haspopup={action.label === "Cancel Job" ? "dialog" : undefined}
            >
              <span className="block truncate">{isLoading ? "..." : action.label}</span>
            </button>
          ))}
        </div>
      )}
    </article>
  );
}

type Action = {
  label: string;
  fn: () => Promise<unknown>;
  notification: { event: NotificationEvent; message: string } | null;
};

function getActions(
  id: number,
  job: Job,
  wallet: string,
  role: "client" | "freelancer",
): Action[] {
  const actions: Action[] = [];
  const jobId = String(id);

  if (role === "client") {
    if (job.status === "Open") {
      actions.push({
        label: "Cancel Job",
        fn: () => cancelJob(wallet, jobId),
        notification: { event: "job_cancelled", message: `Job #${id} was cancelled and funds refunded.` },
      });
    }
    if (job.status === "SubmittedForReview") {
      actions.push({
        label: "Approve Work",
        fn: () => approveWork(wallet, jobId),
        notification: { event: "work_approved", message: `Work for Job #${id} was approved and payment released.` },
      });
    }
    if (job.status === "InProgress" && job.deadline !== "0") {
      actions.push({
        label: "Enforce Deadline",
        fn: () => enforceDeadline(wallet, jobId),
        notification: null,
      });
    }
  }

  if (role === "freelancer") {
    if (job.status === "InProgress") {
      actions.push({
        label: "Submit Work",
        fn: () => submitWork(wallet, jobId),
        notification: { event: "work_submitted", message: `Work for Job #${id} was submitted for review.` },
      });
      actions.push({
        label: "Cancel Job",
        fn: () => freelancerCancelJob(wallet, jobId),
        notification: null,
      });
    }
  }

  return actions;
}
