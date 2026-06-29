export default function MessagesLoading() {
  return (
    <section className="mx-auto max-w-2xl space-y-4">
      <div className="h-8 w-32 animate-pulse rounded-md bg-slate-200" />
      {Array.from({ length: 4 }).map((_, i) => (
        <div key={i} className="flex items-center gap-3 rounded-xl border border-slate-200 bg-white px-4 py-3.5">
          <div className="h-10 w-10 shrink-0 animate-pulse rounded-full bg-slate-200" />
          <div className="flex-1 space-y-1.5">
            <div className="h-3 w-32 animate-pulse rounded bg-slate-200" />
            <div className="h-2.5 w-48 animate-pulse rounded bg-slate-100" />
          </div>
          <div className="h-2.5 w-8 animate-pulse rounded bg-slate-100" />
        </div>
      ))}
    </section>
  );
}
