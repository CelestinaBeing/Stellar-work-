import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ToastProvider } from "@/components/ToastProvider";
import DisputesPage from "@/app/disputes/page";

const mockLoadDisputesPageData = vi.fn();

vi.mock("@/lib/disputes-loader", () => ({
  loadDisputesPageData: (...args: unknown[]) => mockLoadDisputesPageData(...args),
}));

vi.mock("@/lib/wallet-context", () => ({
  useWallet: () => ({
    wallet: "GCLIENT",
    connectWallet: vi.fn(),
  }),
}));

describe("Disputes page empty state", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [],
      eligibleJobs: [
        {
          id: "J-1",
          title: "Escrow UI",
          counterparty: "GFREELANCER",
          amount: 250,
        },
      ],
    });
  });

  it("renders helpful empty-state copy and available CTA with zero disputes", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() =>
      expect(screen.queryByLabelText("Loading disputes")).not.toBeInTheDocument(),
    );
    expect(screen.getByRole("heading", { name: "Disputes" })).toBeInTheDocument();
    expect(screen.getByText("No disputes yet.")).toBeInTheDocument();
    expect(screen.getByText("No disputes found")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "+ Raise Dispute" })).toBeEnabled();
  });
});
