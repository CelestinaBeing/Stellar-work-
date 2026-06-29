import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ToastProvider } from "@/components/ToastProvider";
import DisputesPage from "@/app/disputes/page";

const mockLoadDisputesPageData = vi.fn();

vi.mock("@/lib/disputes-loader", () => ({
  loadDisputesPageData: (...args: unknown[]) =>
    mockLoadDisputesPageData(...args),
}));

vi.mock("@/lib/wallet-context", () => ({
  useWallet: () => ({
    wallet: "GTESTWALLET",
    connectWallet: vi.fn(),
  }),
}));

vi.mock("@/lib/modal", () => ({
  useModalFocusTrap: () => ({ current: null }),
}));

const makeDispute = (overrides = {}) => ({
  id: "D-001",
  jobId: "J-001",
  jobTitle: "Build a portfolio site",
  client: "Alice",
  freelancer: "Bob",
  amount: 500,
  raisedBy: "client" as const,
  raisedAt: "2026-01-10T10:00:00Z",
  status: "Active" as const,
  reason: "Work not delivered",
  ...overrides,
});

describe("Disputes page render (#305)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders the main Disputes heading", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() =>
      expect(
        screen.getAllByRole("heading", { name: /disputes/i })[0],
      ).toBeInTheDocument(),
    );
  });

  it("shows the empty state when there are no disputes", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() =>
      expect(screen.getAllByText(/No disputes/i).length).toBeGreaterThan(0),
    );
  });

  it("renders a populated dispute list with job title visible", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [makeDispute()],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() =>
      expect(
        screen.getByText("Build a portfolio site"),
      ).toBeInTheDocument(),
    );
  });

  it("renders multiple disputes in a list", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [
        makeDispute({ id: "D-001", jobTitle: "Logo design" }),
        makeDispute({ id: "D-002", jobTitle: "API integration" }),
      ],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Logo design")).toBeInTheDocument();
      expect(screen.getByText("API integration")).toBeInTheDocument();
    });
  });
});
