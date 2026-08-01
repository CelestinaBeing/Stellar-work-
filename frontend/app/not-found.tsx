import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = {
  title: "404 — Page Not Found",
  description: "The page you were looking for does not exist.",
  robots: { index: false, follow: false },
};

export default function NotFound() {
  return (
    <section
      className="mx-auto flex min-h-[60vh] max-w-2xl flex-col items-center justify-center gap-8 px-6 py-16 text-center"
      aria-labelledby="not-found-heading"
    >
      <div
        aria-hidden="true"
        className="flex h-24 w-24 items-center justify-center rounded-full bg-slate-100 dark:bg-slate-800"
      >
        <span className="text-5xl select-none">🔭</span>
      </div>

      <div className="space-y-3">
        <p className="text-sm font-semibold uppercase tracking-widest text-slate-400 dark:text-slate-500">
          404
        </p>
        <h1
          id="not-found-heading"
          className="text-3xl font-bold text-slate-900 dark:text-slate-100"
        >
          Page not found
        </h1>
        <p className="max-w-md text-base text-slate-500 dark:text-slate-400">
          We couldn&apos;t find what you were looking for. The page may have
          moved, been removed, or never existed.
        </p>
      </div>

      <nav
        aria-label="Recovery options"
        className="flex flex-wrap justify-center gap-3"
      >
        <Link
          href="/"
          className="rounded-lg bg-slate-900 px-5 py-2.5 text-sm font-semibold text-white shadow-sm transition-colors hover:bg-slate-700 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-300"
        >
          Go home
        </Link>
        <Link
          href="/?status=Open"
          className="rounded-lg border border-slate-300 bg-white px-5 py-2.5 text-sm font-semibold text-slate-700 shadow-sm transition-colors hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-200 dark:hover:bg-slate-700"
        >
          Browse jobs
        </Link>
      </nav>
    </section>
  );
}
