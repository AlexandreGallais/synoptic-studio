import { defineConfig } from "vitest/config";

import base from "./vitest-base.config";

/**
 * The runner configuration of `ng test --configuration timings` (EN-175): the base's, its timing
 * tests no longer skipped, run alone once the other checks ended, as the root's `test:timings`
 * runs its own (REV-008).
 */
export default defineConfig({
  ...base,
  test: {
    ...base.test,
    tags: [{ description: "A test whose limit measures its speed, run alone.", name: "timing" }],
  },
});
