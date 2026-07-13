import { expect, test } from "@playwright/test";

const tabs = ["Bundles", "History", "Unfinished", "Memory", "Live Feed", "Search", "Timeline", "Replay"];

const viewports = [
  { width: 360, height: 800 },
  { width: 768, height: 900 },
  { width: 1280, height: 720 },
];

async function expectNoHorizontalOverflow(page, label) {
  const metrics = await page.evaluate(() => {
    const documentElement = document.documentElement;
    const body = document.body;

    return {
      documentElement: {
        clientWidth: documentElement.clientWidth,
        scrollWidth: documentElement.scrollWidth,
      },
      body: {
        clientWidth: body.clientWidth,
        scrollWidth: body.scrollWidth,
      },
    };
  });

  const overflowPx = Math.max(
    metrics.documentElement.scrollWidth - metrics.documentElement.clientWidth,
    metrics.body.scrollWidth - metrics.body.clientWidth,
  );

  expect(
    overflowPx,
    `${label} has horizontal overflow: ${JSON.stringify(metrics)}`,
  ).toBeLessThanOrEqual(0);
}

for (const viewport of viewports) {
  test.describe(`${viewport.width}px viewport`, () => {
    test.use({ viewport });

    test("viewer tabs do not create document-level horizontal overflow", async ({ page }) => {
      await page.goto("/");
      await expect(page.getByRole("tablist", { name: "SessionLedger views" })).toBeVisible();

      for (const tabName of tabs) {
        const tab = page.getByRole("tab", { name: tabName, exact: true });
        await tab.click();
        await expect(tab).toHaveAttribute("aria-selected", "true");
        await expect(page.getByRole("tabpanel", { name: tabName })).toBeVisible();

        await expectNoHorizontalOverflow(page, `${tabName} at ${viewport.width}px`);
      }
    });
  });
}
