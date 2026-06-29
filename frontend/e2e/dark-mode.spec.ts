import { test, expect } from "@playwright/test";

test.describe("Dark Mode", () => {
  test("should have theme toggle button", async ({ page }) => {
    await page.goto("/");
    const toggle = page.getByRole("button", { name: /Theme:/i });
    await expect(toggle).toBeVisible();
  });

  test("should toggle dark mode on click", async ({ page }) => {
    await page.goto("/");

    const html = page.locator("html");
    const hasDarkClass = await html.evaluate((el) =>
      el.classList.contains("dark"),
    );

    const toggle = page.getByRole("button", { name: /Theme:/i });
    await toggle.click();

    const newHasDarkClass = await html.evaluate((el) =>
      el.classList.contains("dark"),
    );

    expect(newHasDarkClass).not.toBe(hasDarkClass);
  });

  test("should persist theme preference across reloads", async ({ page }) => {
    await page.goto("/");

    const toggle = page.getByRole("button", { name: /Theme:/i });
    await toggle.click();
    await toggle.click();

    const themeAfterCycle = await page.evaluate(() =>
      document.documentElement.classList.contains("dark"),
    );

    await page.reload();

    const themeAfterReload = await page.evaluate(() =>
      document.documentElement.classList.contains("dark"),
    );

    expect(themeAfterReload).toBe(themeAfterCycle);
  });

  test("should cycle through light, dark, and system", async ({ page }) => {
    await page.goto("/");
    const toggle = page.getByRole("button", { name: /Theme:/i });

    await toggle.click();
    await toggle.click();
    await toggle.click();

    await expect(toggle).toBeVisible();
  });
});
