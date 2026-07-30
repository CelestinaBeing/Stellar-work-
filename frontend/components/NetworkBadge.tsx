"use client";

import { memo } from "react";
import { useNetwork } from "@/lib/network-context";

export default memo(function NetworkBadge() {
  const { network, config } = useNetwork();

  return (
    <span
      className={`inline-flex items-center gap-1 rounded-full border px-2.5 py-1 text-[11px] font-semibold uppercase tracking-[0.18em] ${config.badgeBg} ${config.badgeText} ${config.badgeBorder}`}
      aria-label={`Network: ${network}`}
    >
      <span
        className={`h-1.5 w-1.5 rounded-full ${config.dotColor}`}
        aria-hidden="true"
      />
      {network}
    </span>
  );
});
