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

test("Help control opens keyboard help and Escape closes it", async ({ page }) => {
  await page.goto("/");
  const helpDialog = page.locator('[data-testid="keyboard-help-dialog"]');
  const helpButton = page.locator("#viewer-help-button");
  await expect(helpDialog).toHaveCount(0);
  await expect(helpButton).toBeVisible();

  await helpButton.click();
  await expect(helpDialog).toHaveCount(1);
  await expect(page.getByRole("heading", { name: "Keyboard shortcuts" })).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(helpDialog).toHaveCount(0);
  await expect(helpButton).toBeFocused();
});

test.describe("status regions and cognitive fixtures", () => {
  test("content skeleton exposes busy status region", async ({ page }) => {
    await page.goto("/?fixture=skeleton");
    const skeleton = page.getByTestId("content-skeleton");
    await expect(skeleton).toBeVisible();
    await expect(skeleton).toHaveAttribute("role", "status");
    await expect(skeleton).toHaveAttribute("aria-live", "polite");
    await expect(skeleton).toHaveAttribute("aria-busy", "true");
    await expect(skeleton).toHaveAttribute("aria-label", /loading/i);
  });

  test("long loading fixture exposes patience hint on status region", async ({ page }) => {
    await page.goto("/?fixture=loading-long");
    const loading = page.getByTestId("loading-state");
    await expect(loading).toBeVisible();
    await expect(loading).toHaveAttribute("role", "status");
    await expect(loading).toHaveAttribute("aria-live", "polite");
    await expect(loading).toHaveAttribute("aria-busy", "true");
    await expect(page.getByTestId("loading-patience-hint")).toContainText(/minute/i);
  });

  test("search error fixture exposes assertive alert with retry", async ({ page }) => {
    await page.goto("/?fixture=search-error");
    const error = page.getByTestId("error-state");
    await expect(error).toBeVisible();
    await expect(error).toHaveAttribute("role", "alert");
    await expect(error).toHaveAttribute("aria-live", "assertive");
    await expect(error).toContainText(/something went wrong/i);
    await expect(page.getByTestId("error-state-retry")).toBeVisible();
  });

  test("stream skeleton fixture exposes labelled feed status and stream skeleton", async ({ page }) => {
    await page.goto("/?fixture=stream-skeleton");
    const status = page.getByTestId("live-feed-status");
    await expect(status).toBeVisible();
    await expect(status).toHaveAttribute("role", "status");
    await expect(status).toHaveAttribute("aria-live", "polite");
    await expect(status).toHaveAttribute("aria-label", /connecting/i);
    await expect(page.getByTestId("content-skeleton")).toBeVisible();
  });
});

test.describe("landmarks and reduced motion", () => {
  test.use({ reducedMotion: "reduce" });

  test("landmarks stay visible under reduced motion on default tab", async ({ page }) => {
    await page.goto("/");
    await expect(
      page.getByRole("navigation", { name: "Primary viewer navigation" }),
    ).toBeVisible();
    await expect(page.getByRole("main")).toBeVisible();
    await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();
  });

  test("loading spinner animation is flattened under reduced motion", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto("/?fixture=loading-long");
    await expect(page.getByTestId("loading-state")).toBeVisible();
    const reduced = await page.evaluate(() =>
      window.matchMedia("(prefers-reduced-motion: reduce)").matches,
    );
    expect(reduced).toBe(true);
    const info = await page.evaluate(() => {
      const spinner = document.querySelector(".sl-loading-spinner");
      if (!spinner) return null;
      const style = getComputedStyle(spinner);
      return {
        duration: style.animationDuration,
        name: style.animationName,
        playState: style.animationPlayState,
      };
    });
    expect(info).not.toBeNull();
    const flattened =
      info.name === "none" ||
      info.duration === "0s" ||
      info.duration === "0.01ms" ||
      /^0(\.0\d*)?ms$/.test(info.duration) ||
      /^1e-0[45]s$/.test(info.duration);
    expect(flattened).toBeTruthy();
  });
});

test("help overlay lists every shortcut row for keyboard efficiency", async ({ page }) => {
  await page.goto("/");
  await page.locator("#viewer-help-button").click();
  const rows = page.locator(".help-overlay-table tbody tr");
  await expect(rows).toHaveCount(9);
  await expect(page.getByRole("dialog")).toHaveAttribute("aria-labelledby", "help-overlay-title");
  await expect(page.getByRole("columnheader", { name: "Shortcut" })).toBeVisible();
  await expect(page.getByRole("columnheader", { name: "Action" })).toBeVisible();
});
