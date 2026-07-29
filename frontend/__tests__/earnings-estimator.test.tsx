import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import EarningsEstimatorPage from "@/app/earnings-estimator/page";

vi.mock("@/lib/wallet-context", () => ({
  useWallet: () => ({ wallet: null, connectWallet: vi.fn() }),
}));

vi.mock("@/lib/notifications-context", () => ({
  useNotifications: () => ({
    notifications: [],
    addNotification: vi.fn(),
  }),
}));

describe("EarningsEstimatorPage", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it("renders the calculator with default values", () => {
    render(<EarningsEstimatorPage />);
    expect(screen.getByText("Earnings Estimator")).toBeInTheDocument();
    expect(screen.getByText("Calculator")).toBeInTheDocument();
    expect(screen.getByText("Compare Scenarios")).toBeInTheDocument();
  });

  it("shows current scenario earnings card", () => {
    render(<EarningsEstimatorPage />);
    expect(screen.getByText("Monthly Gross")).toBeInTheDocument();
    const feeLabels = screen.getAllByText(/Platform Fees/i);
    expect(feeLabels.length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("Net Earnings")).toBeInTheDocument();
    expect(screen.getByText("Hourly Rate")).toBeInTheDocument();
    expect(screen.getByText("Annual Projection")).toBeInTheDocument();
  });

  it("allows changing input values", () => {
    render(<EarningsEstimatorPage />);
    const inputs = screen.getAllByRole("spinbutton") as HTMLInputElement[];
    const jobAmountInput = inputs[0];
    fireEvent.change(jobAmountInput, { target: { value: "500" } });
    expect(jobAmountInput.value).toBe("500");
  });

  it("allows changing currency", () => {
    render(<EarningsEstimatorPage />);
    fireEvent.click(screen.getByRole("button", { name: "$ USD" }));
  });

  it("allows saving and viewing scenarios", () => {
    render(<EarningsEstimatorPage />);
    const input = screen.getByPlaceholderText("Scenario label...") as HTMLInputElement;
    fireEvent.change(input, { target: { value: "My Scenario" } });
    fireEvent.click(screen.getByText("Save Scenario"));
    expect(screen.getByText("My Scenario")).toBeInTheDocument();
  });

  it("allows removing saved scenarios", () => {
    render(<EarningsEstimatorPage />);
    const input = screen.getByPlaceholderText("Scenario label...") as HTMLInputElement;
    fireEvent.change(input, { target: { value: "Remove Me" } });
    fireEvent.click(screen.getByText("Save Scenario"));
    expect(screen.getByText("Remove Me")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Delete Remove Me/i }));
    expect(screen.queryByText("Remove Me")).not.toBeInTheDocument();
  });
});
