"use client";

import Link from "next/link";
import { useEffect } from "react";

export default function MessagesError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    console.error("Messages error:", error);
  }, [error]);

  return (
    <section className="mx-auto max-w-2xl space-y-4">
      <h1 className="text-2xl font-semibold">Messages</h1>
      <div className="rounded-lg border border-red-200 bg-red-50 p-5">
        <p className="text-sm font-medium text-red-800">Something went wrong loading messages.</p>
        <p className="mt-1 text-xs text-red-600">{error.message}</p>
        <div className="mt-4 flex gap-2">
          <button
            type="button"
            onClick={reset}
            className="rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-700"
          >
            Try again
          </button>
          <Link href="/" className="rounded-md border border-slate-300 px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50">
            Go home
          </Link>
        </div>
      </div>
    </section>
  );
}
