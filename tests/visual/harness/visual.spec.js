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

test("viewer exposes type tokens and persists theme preference", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("button", { name: "Toggle light and dark theme" })).toBeVisible();
  await page.waitForFunction(() => document.documentElement.dataset.theme === "dark");

  const tokens = await page.evaluate(() => {
    const styles = getComputedStyle(document.documentElement);
    return {
      display: styles.getPropertyValue("--font-display").trim(),
      body: styles.getPropertyValue("--font-body").trim(),
      mono: styles.getPropertyValue("--font-mono").trim(),
      ui: styles.getPropertyValue("--font-ui").trim(),
      accent: styles.getPropertyValue("--sl-accent").trim(),
    };
  });

  expect(tokens.display).toContain("Georgia");
  expect(tokens.body).toContain("system-ui");
  expect(tokens.mono).toContain("monospace");
  expect(tokens.ui).toContain("system-ui");
  expect(tokens.accent.toLowerCase()).toBe("#93c5fd");

  await page.getByRole("button", { name: "Toggle light and dark theme" }).click();
  await expect
    .poll(() => page.evaluate(() => document.documentElement.dataset.theme))
    .toBe("light");
  await expect
    .poll(() => page.evaluate(() => window.localStorage.getItem("sl-viewer-theme")))
    .toBe("light");
  await expect
    .poll(() =>
      page.evaluate(() =>
        getComputedStyle(document.documentElement).getPropertyValue("--sl-accent").trim().toLowerCase()
      )
    )
    .toBe("#2563eb");

  await page.reload();
  await expect
    .poll(() => page.evaluate(() => document.documentElement.dataset.theme))
    .toBe("light");
});

test("prefers-reduced-motion flattens transition durations", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();

  const reduced = await page.evaluate(() =>
    window.matchMedia("(prefers-reduced-motion: reduce)").matches
  );
  expect(reduced).toBe(true);

  const duration = await page.evaluate(() => {
    const tab = document.querySelector(".tab");
    if (!tab) return null;
    return getComputedStyle(tab).transitionDuration;
  });

  expect(duration).not.toBeNull();
  // Global guard in app.rs sets 0.01ms under prefers-reduced-motion: reduce.
  // Chromium may report that as "0.01ms" or scientific "1e-05s".
  expect(duration).toMatch(/^0(\.0\d*)?ms$|^0s$|^1e-0[45]s$/);
});

test("viewer exposes spacing and motion tokens", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();

  const tokens = await page.evaluate(() => {
    const styles = getComputedStyle(document.documentElement);
    return {
      spaceMd: styles.getPropertyValue("--sl-space-md").trim(),
      radiusSm: styles.getPropertyValue("--sl-radius-sm").trim(),
      motionFast: styles.getPropertyValue("--sl-motion-fast").trim(),
      easeOut: styles.getPropertyValue("--sl-ease-out").trim(),
    };
  });

  expect(tokens.spaceMd).toBe("12px");
  expect(tokens.radiusSm).toBe("4px");
  expect(tokens.motionFast).toBe("150ms");
  expect(tokens.easeOut).toBe("ease-out");
});
