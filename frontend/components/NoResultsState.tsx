"use client";

type NoResultsStateProps = {
  title: string;
  description: string;
  actionLabel?: string;
  onAction?: () => void;
  className?: string;
};

export default function NoResultsState({
  title,
  description,
  actionLabel,
  onAction,
  className = "",
}: NoResultsStateProps) {
  return (
    <div
      className={`rounded-lg border border-dashed border-slate-300 bg-slate-50 px-4 py-10 text-center sm:px-6 sm:py-12 ${className}`.trim()}
    >
      <p className="font-medium text-slate-700">{title}</p>
      <p className="mt-1 text-sm text-slate-500">{description}</p>
      {actionLabel && onAction && (
        <button
          type="button"
          onClick={onAction}
          className="mt-4 rounded-md border border-slate-300 bg-white px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100"
          aria-label={actionLabel}
        >
          {actionLabel}
        </button>
      )}
    </div>
  );
}
