import { test, expect } from "@playwright/test";

test.describe("Language Switching", () => {
  test("should have language switcher button", async ({ page }) => {
    await page.goto("/");
    const langBtn = page.getByRole("button", { name: /Switch language/i });
    await expect(langBtn).toBeVisible();
  });

  test("should display ES or EN based on locale", async ({ page }) => {
    await page.goto("/");
    const langBtn = page.getByRole("button", { name: /Switch language/i });
    const text = await langBtn.textContent();
    expect(text).toMatch(/^(ES|EN)$/);
  });
});
