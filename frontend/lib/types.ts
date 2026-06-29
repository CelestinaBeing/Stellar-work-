export type JobStatus =
  | "Open"
  | "InProgress"
  | "SubmittedForReview"
  | "Completed"
  | "Cancelled"
  | "Disputed";

export interface Job {
  client: string;
  freelancer: string | null;
  amount: string;
  description_hash: string;
  status: JobStatus;
  created_at: string;
  deadline: string;
  token: string;
  revision_count: number;
}

/** A single milestone within a milestone-based job. */
export interface Milestone {
  id: number;
  description_hash: string;
  amount: string; // stroops as string
  is_released: boolean;
}

export type NotificationEvent =
  | "job_accepted"
  | "work_submitted"
  | "work_approved"
  | "job_cancelled"
  | "dispute_raised"
  | "dispute_resolved";

export interface Notification {
  id: string;
  event: NotificationEvent;
  jobId: number;
  message: string;
  timestamp: number;
  seen: boolean;
}

export type NotificationPreferences = Record<NotificationEvent, boolean>;
