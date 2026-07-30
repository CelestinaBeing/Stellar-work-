import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import FeeCalculatorPage from "@/app/fee-calculator/page";

describe("FeeCalculatorPage", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("renders the fee calculator title", () => {
    render(<FeeCalculatorPage />);
    expect(screen.getByText("Fee Calculator")).toBeDefined();
  });

  it("shows fee breakdown with default values", () => {
    render(<FeeCalculatorPage />);
    expect(screen.getByText("Fee Breakdown")).toBeDefined();
    expect(screen.getByText(/Platform Fee/)).toBeDefined();
    expect(screen.getByText(/Network Fee/)).toBeDefined();
    expect(screen.getByText(/Freelancer Net Earnings/)).toBeDefined();
    expect(screen.getByText(/Total Cost to Client/)).toBeDefined();
  });

  it("has quick-preset buttons", () => {
    render(<FeeCalculatorPage />);
    expect(screen.getByText("10 XLM")).toBeDefined();
    expect(screen.getByText("50 XLM")).toBeDefined();
    expect(screen.getByText("100 XLM")).toBeDefined();
    expect(screen.getByText("500 XLM")).toBeDefined();
    expect(screen.getByText("1000 XLM")).toBeDefined();
  });

  it("updates amount on quick-preset click", () => {
    render(<FeeCalculatorPage />);
    fireEvent.click(screen.getByText("500 XLM"));
    const input = screen.getByRole("spinbutton") as HTMLInputElement;
    expect(input.value).toBe("500");
  });

  it("toggles fiat display", () => {
    render(<FeeCalculatorPage />);
    const toggleBtn = screen.getByText(/Show in/);
    expect(toggleBtn).toBeDefined();
    fireEvent.click(toggleBtn);
    expect(screen.getByText(/Switch to/)).toBeDefined();
  });

  it("switches fiat currency", () => {
    render(<FeeCalculatorPage />);
    fireEvent.click(screen.getByText(/Show in/));
    fireEvent.click(screen.getByText(/Switch to/));
    expect(screen.getByText(/EUR/)).toBeDefined();
  });
});
