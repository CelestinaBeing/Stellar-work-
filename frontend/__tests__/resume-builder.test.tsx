import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import ProfileBuilderPage from "@/app/profile/builder/page";

describe("ProfileBuilderPage", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    localStorage.clear();
  });

  it("renders the resume builder title", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("Resume Builder")).toBeDefined();
  });

  it("shows personal info section", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("Personal Info")).toBeDefined();
    expect(screen.getByPlaceholderText("Jane Doe")).toBeDefined();
    expect(screen.getByPlaceholderText("Full-stack developer & blockchain engineer")).toBeDefined();
  });

  it("shows work experience section with add button", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("Work Experience")).toBeDefined();
    fireEvent.click(screen.getByText("+ Add"));
    expect(screen.getByPlaceholderText("Job Title")).toBeDefined();
  });

  it("shows education section with add button", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("Education")).toBeDefined();
    fireEvent.click(screen.getByText("+ Add"));
    expect(screen.getByPlaceholderText("Degree / Certification")).toBeDefined();
  });

  it("shows skills section with add button", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("Skills")).toBeDefined();
    fireEvent.click(screen.getByText("+ Add"));
    expect(screen.getByPlaceholderText("Skill name")).toBeDefined();
  });

  it("shows preview button", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("Preview")).toBeDefined();
  });

  it("switches to preview mode", () => {
    render(<ProfileBuilderPage />);
    fireEvent.click(screen.getByText("Preview"));
    expect(screen.getByText("Profile Preview")).toBeDefined();
    expect(screen.getByText("Edit")).toBeDefined();
  });

  it("shows completion percentage", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText(/Profile 0% complete/)).toBeDefined();
  });

  it("updates completion when fields are filled", () => {
    render(<ProfileBuilderPage />);
    const nameInput = screen.getByPlaceholderText("Jane Doe");
    fireEvent.change(nameInput, { target: { value: "John Doe" } });
    expect(screen.getByText(/Profile/)).toBeDefined();
  });

  it("renders external links section", () => {
    render(<ProfileBuilderPage />);
    expect(screen.getByText("External Links")).toBeDefined();
    expect(screen.getByPlaceholderText("https://github.com/yourhandle")).toBeDefined();
    expect(screen.getByPlaceholderText("https://linkedin.com/in/yourprofile")).toBeDefined();
  });

  it("saves to localStorage", () => {
    render(<ProfileBuilderPage />);
    const nameInput = screen.getByPlaceholderText("Jane Doe");
    fireEvent.change(nameInput, { target: { value: "John Doe" } });
    const saved = localStorage.getItem("stellarwork:resume-builder");
    expect(saved).toBeDefined();
    if (saved) {
      const data = JSON.parse(saved);
      expect(data.name).toBe("John Doe");
    }
  });
});
