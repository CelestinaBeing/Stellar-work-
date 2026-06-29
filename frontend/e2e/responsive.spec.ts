import { test, expect } from "@playwright/test";

test.describe("Responsive Layout", () => {
  test("should show hamburger menu on mobile viewport", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");

    const menuButton = page.getByRole("button", {
      name: "Toggle navigation menu",
    });
    await expect(menuButton).toBeVisible();
    await expect(menuButton).toHaveAttribute("aria-expanded", "false");
  });

  test("should toggle mobile menu open and closed", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");

    const menuButton = page.getByRole("button", {
      name: "Toggle navigation menu",
    });
    await menuButton.click();
    await expect(menuButton).toHaveAttribute("aria-expanded", "true");

    const mobileNav = page.locator("#mobile-nav-menu");
    await expect(mobileNav).toBeVisible();

    await menuButton.click();
    await expect(mobileNav).not.toBeVisible();
  });

  test("should show desktop navigation on wide viewport", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto("/");

    const desktopNav = page.getByRole("navigation", { name: "Main navigation" });
    await expect(desktopNav).toBeVisible();
  });

  test("should show footer on all breakpoints", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");

    const footer = page.locator("footer");
    await expect(footer).toBeVisible();
    await expect(footer.getByText("StellarWork")).toBeVisible();
  });

  test("should show main content area on tablet", async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto("/");
    const mainContent = page.locator("#main-content");
    await expect(mainContent).toBeVisible();
  });

  test("should close mobile menu on Escape key", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");

    const menuButton = page.getByRole("button", {
      name: "Toggle navigation menu",
    });
    await menuButton.click();

    await page.keyboard.press("Escape");
    await expect(menuButton).toHaveAttribute("aria-expanded", "false");
  });

  test("should close mobile menu on route change", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");

    const menuButton = page.getByRole("button", {
      name: "Toggle navigation menu",
    });
    await menuButton.click();

    await page.getByRole("link", { name: "Dashboard" }).click();
    await expect(page).toHaveURL(/\/dashboard/);
    await expect(page.locator("#mobile-nav-menu")).not.toBeVisible();
  });
});
