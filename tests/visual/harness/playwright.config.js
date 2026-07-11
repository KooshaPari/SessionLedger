import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: ".",
  testMatch: "visual.spec.js",
  use: {
    baseURL: process.env.VISUAL_BASE_URL,
    viewport: { width: 1280, height: 720 },
    colorScheme: "dark",
    reducedMotion: "reduce",
  },
  snapshotPathTemplate: "../golden/{arg}{ext}",
});
