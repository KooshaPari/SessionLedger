import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: ".",
  testMatch: ["a11y.spec.js", "visual.spec.js"],
  forbidOnly: Boolean(process.env.CI),
  retries: process.env.CI ? 2 : 0,
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL: process.env.VISUAL_BASE_URL,
    viewport: { width: 1280, height: 720 },
    colorScheme: "dark",
    reducedMotion: "reduce",
  },
  snapshotPathTemplate: "../golden/{arg}{ext}",
});
