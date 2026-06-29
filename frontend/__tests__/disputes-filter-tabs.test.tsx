import React from "react";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { ToastProvider } from "@/components/ToastProvider";
import DisputesPage from "@/app/disputes/page";

const mockLoadDisputesPageData = vi.fn();

vi.mock("@/lib/disputes-loader", () => ({
  loadDisputesPageData: (...args: unknown[]) => mockLoadDisputesPageData(...args),
}));

vi.mock("@/lib/wallet-context", () => ({
  useWallet: () => ({
    wallet: "GTESTWALLET",
    connectWallet: vi.fn(),
  }),
}));

describe("Disputes page filter tab toggling", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [
        {
          id: "D-001",
          jobId: "J-001",
          jobTitle: "Audit Contract",
          client: "GCLIENT",
          freelancer: "GFREELANCER",
          amount: 100,
          raisedBy: "client",
          raisedAt: "2025-04-18T09:22:00Z",
          status: "Active",
          reason: "Payment dispute",
        },
        {
          id: "D-002",
          jobId: "J-002",
          jobTitle: "Design Review",
          client: "GCLIENT2",
          freelancer: "GFREELANCER2",
          amount: 250,
          raisedBy: "freelancer",
          raisedAt: "2025-04-19T10:30:00Z",
          status: "Resolved",
          reason: "Scope change",
          resolution: {
            resolvedAt: "2025-04-20T15:00:00Z",
            clientShare: 70,
            freelancerShare: 30,
            note: "Partial refund approved",
          },
        },
      ],
      eligibleJobs: [],
    });
  });

  it("renders all filter tabs", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "all" })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "active" })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "resolved" })).toBeInTheDocument();
    });
  });

  it("shows 'all' tab as selected by default", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      const allTab = screen.getByRole("button", { name: "all" });
      expect(allTab).toHaveClass("bg-slate-900", "text-white");
    });
  });

  it("toggles filter tab selection on click", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "all" })).toBeInTheDocument();
    });

    const activeTab = screen.getByRole("button", { name: "active" });
    fireEvent.click(activeTab);

    await waitFor(() => {
      expect(activeTab).toHaveClass("bg-slate-900", "text-white");
      const allTab = screen.getByRole("button", { name: "all" });
      expect(allTab).not.toHaveClass("bg-slate-900", "text-white");
    });
  });

  it("enforces single-select rule - only one filter tab active at a time", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "all" })).toBeInTheDocument();
    });

    // Click active tab
    const activeTab = screen.getByRole("button", { name: "active" });
    fireEvent.click(activeTab);

    await waitFor(() => {
      expect(activeTab).toHaveClass("bg-slate-900", "text-white");
    });

    // Click resolved tab - should deselect active
    const resolvedTab = screen.getByRole("button", { name: "resolved" });
    fireEvent.click(resolvedTab);

    await waitFor(() => {
      expect(resolvedTab).toHaveClass("bg-slate-900", "text-white");
      expect(activeTab).not.toHaveClass("bg-slate-900", "text-white");
    });
  });

  it("filters disputes based on selected tab", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });

    // Click active tab
    const activeTab = screen.getByRole("button", { name: "active" });
    fireEvent.click(activeTab);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
    });

    // Design Review (Resolved) should not be visible
    expect(screen.queryByText("Design Review")).not.toBeInTheDocument();
  });

  it("shows only resolved disputes when resolved tab is selected", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });

    // Click resolved tab
    const resolvedTab = screen.getByRole("button", { name: "resolved" });
    fireEvent.click(resolvedTab);

    await waitFor(() => {
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });

    // Audit Contract (Active) should not be visible
    expect(screen.queryByText("Audit Contract")).not.toBeInTheDocument();
  });

  it("shows all disputes when all tab is selected", async () => {
    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });

    // Click active tab
    const activeTab = screen.getByRole("button", { name: "active" });
    fireEvent.click(activeTab);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.queryByText("Design Review")).not.toBeInTheDocument();
    });

    // Click all tab to reset
    const allTab = screen.getByRole("button", { name: "all" });
    fireEvent.click(allTab);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });
  });

  it("maintains filter state across re-renders", async () => {
    const { rerender } = render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "all" })).toBeInTheDocument();
    });

    // Select resolved filter
    const resolvedTab = screen.getByRole("button", { name: "resolved" });
    fireEvent.click(resolvedTab);

    await waitFor(() => {
      expect(resolvedTab).toHaveClass("bg-slate-900", "text-white");
    });

    // Rerender to test state persistence
    rerender(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(resolvedTab).toHaveClass("bg-slate-900", "text-white");
    });
  });

  it("shows NoResultsState when filter has no matches", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [
        {
          id: "D-001",
          jobId: "J-001",
          jobTitle: "Audit Contract",
          client: "GCLIENT",
          freelancer: "GFREELANCER",
          amount: 100,
          raisedBy: "client",
          raisedAt: "2025-04-18T09:22:00Z",
          status: "Active",
          reason: "Payment dispute",
        },
      ],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
    });

    // Click resolved tab (no resolved disputes)
    const resolvedTab = screen.getByRole("button", { name: "resolved" });
    fireEvent.click(resolvedTab);

    await waitFor(() => {
      expect(screen.getByText("No disputes match this filter")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: "Show all disputes" })).toBeInTheDocument();
    });
  });

  it("clears filter when clicking Show all disputes action", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [
        {
          id: "D-001",
          jobId: "J-001",
          jobTitle: "Audit Contract",
          client: "GCLIENT",
          freelancer: "GFREELANCER",
          amount: 100,
          raisedBy: "client",
          raisedAt: "2025-04-18T09:22:00Z",
          status: "Active",
          reason: "Payment dispute",
        },
      ],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
    });

    // Click resolved tab
    const resolvedTab = screen.getByRole("button", { name: "resolved" });
    fireEvent.click(resolvedTab);

    await waitFor(() => {
      expect(screen.getByText("No disputes match this filter")).toBeInTheDocument();
    });

    // Click Show all disputes
    const showAllButton = screen.getByRole("button", { name: "Show all disputes" });
    fireEvent.click(showAllButton);

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "all" })).toHaveClass("bg-slate-900", "text-white");
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
    });
  });

  it("filters by status including UnderReview and PendingEvidence in active tab", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [
        {
          id: "D-001",
          jobId: "J-001",
          jobTitle: "Audit Contract",
          client: "GCLIENT",
          freelancer: "GFREELANCER",
          amount: 100,
          raisedBy: "client",
          raisedAt: "2025-04-18T09:22:00Z",
          status: "UnderReview",
          reason: "Payment dispute",
        },
        {
          id: "D-002",
          jobId: "J-002",
          jobTitle: "Design Review",
          client: "GCLIENT2",
          freelancer: "GFREELANCER2",
          amount: 250,
          raisedBy: "freelancer",
          raisedAt: "2025-04-19T10:30:00Z",
          status: "PendingEvidence",
          reason: "Scope change",
        },
        {
          id: "D-003",
          jobId: "J-003",
          jobTitle: "Code Review",
          client: "GCLIENT3",
          freelancer: "GFREELANCER3",
          amount: 150,
          raisedBy: "client",
          raisedAt: "2025-04-20T11:00:00Z",
          status: "Resolved",
          reason: "Quality issue",
          resolution: {
            resolvedAt: "2025-04-21T16:00:00Z",
            clientShare: 50,
            freelancerShare: 50,
            note: "Split payment",
          },
        },
      ],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
      expect(screen.getByText("Code Review")).toBeInTheDocument();
    });

    // Click active tab
    const activeTab = screen.getByRole("button", { name: "active" });
    fireEvent.click(activeTab);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });

    // Code Review (Resolved) should not be visible
    expect(screen.queryByText("Code Review")).not.toBeInTheDocument();
  });

  it("includes Closed status in resolved tab", async () => {
    mockLoadDisputesPageData.mockResolvedValue({
      disputes: [
        {
          id: "D-001",
          jobId: "J-001",
          jobTitle: "Audit Contract",
          client: "GCLIENT",
          freelancer: "GFREELANCER",
          amount: 100,
          raisedBy: "client",
          raisedAt: "2025-04-18T09:22:00Z",
          status: "Closed",
          reason: "Payment dispute",
          resolution: {
            resolvedAt: "2025-04-20T15:00:00Z",
            clientShare: 100,
            freelancerShare: 0,
            note: "Closed without resolution",
          },
        },
        {
          id: "D-002",
          jobId: "J-002",
          jobTitle: "Design Review",
          client: "GCLIENT2",
          freelancer: "GFREELANCER2",
          amount: 250,
          raisedBy: "freelancer",
          raisedAt: "2025-04-19T10:30:00Z",
          status: "Active",
          reason: "Scope change",
        },
      ],
      eligibleJobs: [],
    });

    render(<ToastProvider><DisputesPage /></ToastProvider>);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
      expect(screen.getByText("Design Review")).toBeInTheDocument();
    });

    // Click resolved tab
    const resolvedTab = screen.getByRole("button", { name: "resolved" });
    fireEvent.click(resolvedTab);

    await waitFor(() => {
      expect(screen.getByText("Audit Contract")).toBeInTheDocument();
    });

    // Design Review (Active) should not be visible
    expect(screen.queryByText("Design Review")).not.toBeInTheDocument();
  });
});
