import React from "react";
import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import AdminPage from "@/app/admin/page";

// ── Mocks ────────────────────────────────────────────────────────────────────

const mockUseWallet = vi.fn();
const mockConnectWallet = vi.fn();

vi.mock("@/lib/wallet-context", () => ({
  useWallet: () => mockUseWallet(),
}));

vi.mock("@/lib/contract", () => ({
  getNativeToken: vi.fn(),
  getFees: vi.fn(),
  getJobCount: vi.fn(),
  getJob: vi.fn(),
  withdrawFees: vi.fn(),
  freelancerCancelJob: vi.fn(),
  getDescriptionCid: vi.fn(),
  storeDescriptionCid: vi.fn(),
}));

// ── Tests ────────────────────────────────────────────────────────────────────

describe("Admin page — no wallet connected", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseWallet.mockReturnValue({
      wallet: null,
      connectWallet: mockConnectWallet,
    });
  });

  it("mounts without crashing when no wallet is connected", () => {
    expect(() => render(<AdminPage />)).not.toThrow();
  });

  it("shows the Admin Panel heading", () => {
    render(<AdminPage />);
    expect(
      screen.getByRole("heading", { name: "Admin Panel" }),
    ).toBeInTheDocument();
  });

  it("shows a wallet-connect prompt instead of admin controls", () => {
    render(<AdminPage />);
    expect(
      screen.getByText("Connect your wallet to access admin controls."),
    ).toBeInTheDocument();
  });

  it("renders a Connect Wallet button", () => {
    render(<AdminPage />);
    expect(
      screen.getByRole("button", { name: "Connect Wallet" }),
    ).toBeInTheDocument();
  });

  it("does not render the Withdraw Fees button when unauthenticated", () => {
    render(<AdminPage />);
    expect(
      screen.queryByRole("button", { name: /withdraw fees/i }),
    ).not.toBeInTheDocument();
  });

  it("does not render the job table when unauthenticated", () => {
    render(<AdminPage />);
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
  });
});
