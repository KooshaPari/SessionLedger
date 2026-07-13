import { expect, test } from "@playwright/test";

test("E1 bundle detail empty state matches its golden", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();
  // Cross-platform font AA routinely differs ~1-2% of pixels between Windows
  // baseline authors and Linux CI Chromium; keep structural contract tight but
  // tolerate that AA noise.
  await expect(page).toHaveScreenshot("e1-bundle-empty.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});
