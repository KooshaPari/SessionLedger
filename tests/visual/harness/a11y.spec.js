import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

const tabs = ["Bundles", "History", "Unfinished", "Memory", "Live Feed", "Search", "Timeline", "Replay"];

const viewports = [
  { width: 375, height: 812 },
  { width: 768, height: 900 },
  { width: 1280, height: 720 },
];

for (const viewport of viewports) {
  test.describe(`${viewport.width}px viewport`, () => {
    test.use({ viewport });

    for (const tabName of tabs) {
      test(`${tabName} has no WCAG AA violations`, async ({ page }) => {
        await page.goto("/");
        const tab = page.getByRole("tab", { name: tabName, exact: true });
        await tab.click();
        await expect(tab).toHaveAttribute("aria-selected", "true");

        const results = await new AxeBuilder({ page })
          .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
          .analyze();

        expect(results.violations).toEqual([]);
        await expect(page.getByRole("main")).toBeVisible();
        await expect(
          page.getByRole("navigation", { name: "Primary viewer navigation" }),
        ).toBeVisible();
      });
    }
  });
}

test("ARIA tabs expose state and support the standard keyboard pattern", async ({ page }) => {
  await page.goto("/");
  const tablist = page.getByRole("tablist", { name: "SessionLedger views" });
  const bundles = tablist.getByRole("tab", { name: "Bundles" });
  const history = tablist.getByRole("tab", { name: "History" });
  const replay = tablist.getByRole("tab", { name: "Replay" });

  await bundles.focus();
  await page.keyboard.press("ArrowRight");
  await expect(history).toBeFocused();
  await expect(history).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("tabpanel", { name: "History" })).toBeVisible();

  await page.keyboard.press("End");
  await expect(replay).toBeFocused();
  await page.keyboard.press("Home");
  await expect(bundles).toBeFocused();
});

test("Tab order reaches active tab then active-panel controls", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("tab", { name: "Search" }).click();

  await expect(page.getByRole("tab", { name: "Search" })).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.locator(".search-input").first()).toBeFocused();
});

test("Escape clears the search without moving focus", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("tab", { name: "Search" }).click();
  const since = page.locator(".search-input").first();
  await since.fill("2026-01-01");
  await since.press("Escape");
  await expect(since).toHaveValue("");
  await expect(since).toBeFocused();
});

test("question mark opens keyboard help and Escape closes it", async ({ page }) => {
  await page.goto("/");
  const helpDialog = page.locator('[data-testid="keyboard-help-dialog"]');
  await expect(helpDialog).toHaveCount(0);

  // Focus the document shell, then send Shift+/ (produces "?" reliably in Chromium).
  await page.locator("body").click({ position: { x: 8, y: 8 } });
  await page.keyboard.press("Shift+/");
  await expect(helpDialog).toHaveCount(1);
  await expect(page.getByRole("heading", { name: "Keyboard shortcuts" })).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(helpDialog).toHaveCount(0);
  await expect(page.locator("#viewer-help-button")).toBeFocused();
});
