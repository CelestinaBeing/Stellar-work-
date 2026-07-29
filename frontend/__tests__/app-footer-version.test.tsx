import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vitest";

vi.mock("next/link", () => ({
  default: ({ href, children, className }: { href: string; children: React.ReactNode; className?: string }) => (
    <a href={href} className={className}>{children}</a>
  ),
}));

import AppFooter from "@/components/AppFooter";

describe("AppFooter version display", () => {
  beforeEach(() => {
    vi.stubEnv("NEXT_PUBLIC_APP_VERSION", "1.2.3");
    vi.stubEnv("NEXT_PUBLIC_DEPLOY_ENV", "production");
    vi.stubEnv("NEXT_PUBLIC_COMMIT_SHA", "abc1234def5678");
    vi.stubEnv("NEXT_PUBLIC_BUILD_TIME", "2026-06-29T12:00:00Z");
  });

  it("renders version string in footer", () => {
    render(<AppFooter />);
    expect(screen.getByText(/StellarWork/)).toBeInTheDocument();
  });

  it("renders footer navigation links", () => {
    render(<AppFooter />);
    expect(screen.getByRole("link", { name: "GitHub" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Documentation" })).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "License" })).toBeInTheDocument();
  });

  it("version badge button has accessible label", () => {
    render(<AppFooter />);
    const btn = screen.getByRole("button", { name: /App version/i });
    expect(btn).toBeInTheDocument();
  });

  it("shows build tooltip on focus", () => {
    render(<AppFooter />);
    const btn = screen.getByRole("button", { name: /App version/i });
    fireEvent.focus(btn);
    expect(screen.getByRole("tooltip")).toBeInTheDocument();
  });

  it("hides tooltip after blur", () => {
    render(<AppFooter />);
    const btn = screen.getByRole("button", { name: /App version/i });
    fireEvent.focus(btn);
    fireEvent.blur(btn);
    expect(screen.queryByRole("tooltip")).not.toBeInTheDocument();
  });

  it("renders copyright notice", () => {
    render(<AppFooter />);
    expect(screen.getByText(/StellarWork\. All rights reserved\./)).toBeInTheDocument();
  });

  it("footer element has correct landmark role", () => {
    render(<AppFooter />);
    expect(screen.getByRole("contentinfo")).toBeInTheDocument();
  });
});
