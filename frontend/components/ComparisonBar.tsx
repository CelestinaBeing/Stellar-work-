"use client";

import Link from "next/link";

interface ComparisonBarProps {
  selectedIds: number[];
  onRemove: (id: number) => void;
  onClear: () => void;
}

export default function ComparisonBar({ selectedIds, onRemove, onClear }: ComparisonBarProps) {
  if (selectedIds.length === 0) return null;

  const compareHref = `/compare?ids=${selectedIds.join(",")}`;

  return (
    <div
      className="fixed bottom-0 left-0 right-0 z-40 border-t border-slate-200 bg-white shadow-lg"
      role="region"
      aria-label="Job comparison bar"
    >
      <div className="mx-auto flex max-w-5xl flex-wrap items-center gap-3 px-4 py-3">
        <span className="text-sm font-medium text-slate-700">
          {selectedIds.length} job{selectedIds.length === 1 ? "" : "s"} selected for comparison
        </span>

        <div className="flex flex-wrap gap-2">
          {selectedIds.map((id) => (
            <span
              key={id}
              className="inline-flex items-center gap-1 rounded-full bg-blue-100 px-3 py-1 text-sm font-medium text-blue-800"
            >
              Job #{id}
              <button
                type="button"
                onClick={() => onRemove(id)}
                className="ml-1 rounded-full p-0.5 text-blue-600 hover:bg-blue-200 hover:text-blue-900"
                aria-label={`Remove Job #${id} from comparison`}
              >
                <svg
                  aria-hidden="true"
                  viewBox="0 0 16 16"
                  fill="currentColor"
                  className="h-3 w-3"
                >
                  <path d="M4.293 4.293a1 1 0 011.414 0L8 6.586l2.293-2.293a1 1 0 111.414 1.414L9.414 8l2.293 2.293a1 1 0 01-1.414 1.414L8 9.414l-2.293 2.293a1 1 0 01-1.414-1.414L6.586 8 4.293 5.707a1 1 0 010-1.414z" />
                </svg>
              </button>
            </span>
          ))}
        </div>

        <div className="ml-auto flex items-center gap-2">
          {selectedIds.length < 2 && (
            <p className="text-xs text-slate-500">Select at least 2 jobs to compare</p>
          )}
          <button
            type="button"
            onClick={onClear}
            className="rounded-md border border-slate-300 bg-white px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-50"
          >
            Clear
          </button>
          <Link
            href={compareHref}
            className={`rounded-md px-4 py-1.5 text-sm font-medium text-white transition-colors ${
              selectedIds.length >= 2
                ? "bg-blue-600 hover:bg-blue-700"
                : "pointer-events-none bg-slate-300"
            }`}
            aria-disabled={selectedIds.length < 2}
            tabIndex={selectedIds.length < 2 ? -1 : 0}
          >
            Compare
          </Link>
        </div>
      </div>
    </div>
  );
}
