import { test, expect } from "@playwright/test";

test.describe("Dashboard Page", () => {
  test("should load dashboard page", async ({ page }) => {
    await page.goto("/dashboard");
    await expect(page.getByRole("heading", { name: /Dashboard/i })).toBeVisible();
  });

  test("should show posted jobs section", async ({ page }) => {
    await page.goto("/dashboard");
    await expect(page.getByText(/Posted Jobs/i)).toBeVisible();
  });

  test("should show accepted jobs section", async ({ page }) => {
    await page.goto("/dashboard");
    await expect(page.getByText(/Accepted Jobs/i)).toBeVisible();
  });

  test("should navigate to post job from dashboard", async ({ page }) => {
    await page.goto("/dashboard");
    const postLink = page.getByRole("link", { name: "Post Job" });
    if (await postLink.isVisible()) {
      await postLink.click();
      await expect(page).toHaveURL(/\/post-job/);
    }
  });
});
