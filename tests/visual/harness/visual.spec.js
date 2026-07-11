import { expect, test } from "@playwright/test";

const baseURL = process.env.VISUAL_BASE_URL;

test.skip(!baseURL, "Set VISUAL_BASE_URL to a running viewer URL.");

test("E1 bundle detail empty state matches its golden", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();
  await expect(page).toHaveScreenshot("e1-bundle-empty.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.001,
  });
});
