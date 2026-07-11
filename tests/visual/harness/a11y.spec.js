import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";
import { fileURLToPath, pathToFileURL } from "node:url";

const fixtureURL = pathToFileURL(
  fileURLToPath(new URL("./fixtures/a11y.html", import.meta.url)),
).href;

const viewports = [
  { width: 375, height: 812 },
  { width: 768, height: 900 },
  { width: 1280, height: 720 },
];

for (const viewport of viewports) {
  test.describe(`${viewport.width}px viewport`, () => {
    test.use({ viewport });

    test("has no WCAG AA or color-contrast violations", async ({ page }) => {
      await page.goto(fixtureURL);
      const results = await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
        .analyze();

      expect(results.violations).toEqual([]);
      await expect(page.getByRole("main")).toBeVisible();
      await expect(page.getByRole("navigation", { name: "Primary viewer navigation" })).toBeVisible();
    });
  });
}

test("ARIA tabs expose state and support the standard keyboard pattern", async ({ page }) => {
  await page.goto(fixtureURL);
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
  await page.goto(fixtureURL);
  await page.getByRole("tab", { name: "Search" }).click();

  await expect(page.getByRole("tab", { name: "Search" })).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.getByLabel("Query")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.getByRole("button", { name: "Search", exact: true })).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.getByRole("button", { name: "Clear" })).toBeFocused();
});

test("Escape clears the search without moving focus", async ({ page }) => {
  await page.goto(fixtureURL);
  await page.getByRole("tab", { name: "Search" }).click();
  const query = page.getByLabel("Query");
  await query.fill("failed session");
  await query.press("Escape");
  await expect(query).toHaveValue("");
  await expect(query).toBeFocused();
});
