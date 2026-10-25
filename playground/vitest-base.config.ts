import { fileURLToPath } from "node:url";

import { defineConfig } from "vitest/config";

/**
 * What Angular's unit-test builder leaves to Vitest (EN-160): the library, which the test build
 * leaves out of its bundle so that `vi.mock("editor")` replaces it, resolved to its source; the
 * test font files read once per run and handed to every test file, as the root's tests have them
 * (EN-089); and the tag of the tests whose limit measures their speed, as the root names it.
 */
export default defineConfig({
  resolve: {
    alias: { editor: fileURLToPath(new URL("../src/index.ts", import.meta.url)) },
  },
  test: {
    globalSetup: ["../tests/font-files-setup.ts"],
    tags: [{ description: "A test whose limit measures its speed, run alone.", name: "timing" }],
  },
});
