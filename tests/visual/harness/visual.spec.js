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

test("E2 history detail empty state matches its golden", async ({ page }) => {
  await page.goto("/?fixture=history-empty");
  await expect(page.getByRole("tab", { name: "History", selected: true })).toBeVisible();
  await expect(page.getByText("Select a session from the timeline to inspect")).toBeVisible();
  await expect(page).toHaveScreenshot("e2-history-empty.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("E4 search zero-match empty state matches its golden", async ({ page }) => {
  await page.goto("/?fixture=search-empty");
  await expect(page.getByRole("tab", { name: "Search", selected: true })).toBeVisible();
  await expect(page.getByTestId("search-zero-match")).toBeVisible();
  await expect(page).toHaveScreenshot("e4-search-empty.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("E5 first-run empty state matches its golden", async ({ page }) => {
  await page.goto("/?fixture=first-run");
  await expect(page.getByRole("tab", { name: "Bundles", selected: true })).toBeVisible();
  await expect(page.getByTestId("first-run-empty")).toBeVisible();
  await expect(page.getByTestId("first-run-cta")).toBeVisible();
  await expect(page).toHaveScreenshot("e5-first-run-empty.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("R1 search error state matches its golden", async ({ page }) => {
  await page.goto("/?fixture=search-error");
  await expect(page.getByRole("tab", { name: "Search", selected: true })).toBeVisible();
  await expect(page.getByText("daemon not reachable")).toBeVisible();
  await expect(page).toHaveScreenshot("r1-search-error.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("R2 replay stream error state matches its golden", async ({ page }) => {
  await page.goto("/?fixture=replay-error");
  await expect(page.getByRole("tab", { name: "Replay", selected: true })).toBeVisible();
  await expect(page.getByTestId("replay-status-error")).toBeVisible();
  await expect(page.getByTestId("error-state-retry")).toBeVisible();
  await expect(page).toHaveScreenshot("r2-replay-error.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("R3 error color contract matches its golden", async ({ page }) => {
  await page.goto("/?fixture=error-color");
  await expect(page.getByRole("tab", { name: "Bundles", selected: true })).toBeVisible();
  await expect(page.getByTestId("error-color-panel")).toBeVisible();
  await expect(page.getByTestId("error-color-live-badge")).toBeVisible();
  await expect(page).toHaveScreenshot("r3-error-color.png", {
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

test("viewer exposes caption and measure typography tokens", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();

  const tokens = await page.evaluate(() => {
    const styles = getComputedStyle(document.documentElement);
    return {
      captionFont: styles.getPropertyValue("--sl-font-caption").trim(),
      captionSize: styles.getPropertyValue("--sl-font-size-caption").trim(),
      captionLine: styles.getPropertyValue("--sl-line-height-caption").trim(),
      measureMax: styles.getPropertyValue("--sl-measure-max").trim(),
    };
  });

  expect(tokens.captionFont).toContain("system-ui");
  expect(tokens.captionSize).toBe("0.75rem");
  expect(tokens.captionLine).toBe("1.35");
  expect(tokens.measureMax).toBe("65ch");
});

test("launch splash is present then dismisses", async ({ page }) => {
  // Default harness uses reduced motion, which pins splash hidden; exercise
  // the real dismiss path with motion allowed.
  await page.emulateMedia({ reducedMotion: "no-preference" });
  await page.goto("/");
  const splash = page.locator(".launch-splash");
  await expect(splash).toBeVisible();
  await expect(splash).toHaveCount(0, { timeout: 5000 });
});

test("S1 launch splash matches its golden", async ({ page }) => {
  await page.goto("/?fixture=launch-splash");
  await expect(page.getByTestId("launch-splash")).toBeVisible();
  await expect
    .poll(() => page.evaluate(() => document.documentElement.dataset.theme))
    .toBe("dark");
  await expect(page.getByText("SessionLedger", { exact: true }).first()).toBeVisible();
  await expect(page.getByText("Viewer", { exact: true }).first()).toBeVisible();
  await expect(page).toHaveScreenshot("s1-launch-splash.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("S1 launch splash light theme matches its golden", async ({ page }) => {
  await page.goto("/?fixture=launch-splash-light");
  await expect(page.getByTestId("launch-splash")).toBeVisible();
  await expect
    .poll(() => page.evaluate(() => document.documentElement.dataset.theme))
    .toBe("light");
  await expect(page).toHaveScreenshot("s1-launch-splash-light.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
});

test("content skeleton fixture exposes tokens and stable layout", async ({ page }) => {
  await page.goto("/?fixture=skeleton");
  await expect(page.getByTestId("content-skeleton")).toBeVisible();

  const tokens = await page.evaluate(() => {
    const styles = getComputedStyle(document.documentElement);
    return {
      skeletonBase: styles.getPropertyValue("--sl-skeleton-base").trim(),
      skeletonHighlight: styles.getPropertyValue("--sl-skeleton-highlight").trim(),
    };
  });

  expect(tokens.skeletonBase).toBe("#2b3544");
  expect(tokens.skeletonHighlight).toBe("rgba(147, 197, 253, 0.14)");

  await expect(page).toHaveScreenshot("l1-content-skeleton.png", {
    animations: "disabled",
    maxDiffPixelRatio: 0.03,
  });
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
