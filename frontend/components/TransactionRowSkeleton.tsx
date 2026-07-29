export default function TransactionRowSkeleton() {
  return (
    <tr className="animate-pulse border-b border-slate-100" aria-hidden="true">
      {/* Date */}
      <td className="py-3 pr-4">
        <div className="h-4 w-28 rounded bg-slate-200" />
        <div className="mt-1 h-3 w-16 rounded bg-slate-200" />
      </td>
      {/* Type */}
      <td className="py-3 pr-4">
        <div className="h-5 w-24 rounded-full bg-slate-200" />
      </td>
      {/* Amount */}
      <td className="py-3 pr-4 text-right">
        <div className="ml-auto h-4 w-20 rounded bg-slate-200" />
      </td>
      {/* Job */}
      <td className="py-3 pr-4">
        <div className="h-4 w-12 rounded bg-slate-200" />
      </td>
      {/* Counterparty */}
      <td className="py-3 pr-4">
        <div className="h-4 w-24 rounded bg-slate-200" />
      </td>
      {/* Status */}
      <td className="py-3">
        <div className="h-5 w-16 rounded-full bg-slate-200" />
      </td>
    </tr>
  );
}
