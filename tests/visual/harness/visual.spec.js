import { expect, test } from "@playwright/test";

test("E1 bundle detail empty state matches its golden", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();
  await expect(page).toHaveScreenshot("e1-bundle-empty.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.001,
  });
});
