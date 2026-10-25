import { join } from "node:path";

import { defineConfig } from "vitest/config";

import type { BuildEnvironmentOptions } from "vite";
import type { ViteUserConfig } from "vitest/config";

/** Absolute path of the repository root (this file's directory). */
const ROOT = import.meta.dirname;

/**
 * Reporters of the CI's test lane, `VITEST_JUNIT` set on every lane and read by Vitest alone, which asks a JUnit report by `VITEST_JUNIT`: each file as it
 * ends, each failure annotated at its line on the pull request, and the report attached to each
 * release (ADR-0027). A configured list replaces Vitest's own, so the GitHub Actions reporter is
 * named (`REF-VITEST-REPORTERS`).
 */
const CI_REPORTERS = ["default", "github-actions", "junit"];

/**
 * The tests whose limit under Vitest's 5 s measures a speed (`tests/fitness.test.ts`): left out of
 * `npm run test`, which runs beside the other checks, and run alone after them by `test:timings`
 * (REV-008), the load of the other lanes having failed one twice (`REF-VITEST-TAGS`).
 */
const TEST_TAGS = [
  { description: "A test whose limit measures its speed, run alone.", name: "timing" },
];

/** Library build: one ESM file from `src/index.ts`, declarations written by `tools/declarations` (ADR-0071). */
const LIBRARY_BUILD: BuildEnvironmentOptions = {
  emptyOutDir: true,
  lib: {
    entry: join(ROOT, "src", "index.ts"),
    fileName: "editor",
    formats: ["es"],
  },
  minify: false,
  outDir: join(ROOT, "dist"),
};

/**
 * The root's tests: their files share a worker's loaded modules, the library imported once per
 * worker instead of once per file, no test keeping state between files (EN-090, ADR-0070). The
 * playground's tests, whose page keeps its state in its modules, run isolated by its own `ng test`
 * (EN-160, EN-175).
 */
const TEST_FILES = [
  "src/**/*.test.ts",
  "tests/*.test.ts",
  "docs/.vitepress/**/*.test.ts",
  "scripts/*.test.ts",
];

/**
 * Vite configuration: the library build of `src/` and Vitest, the CI's reporters when it asks a
 * JUnit report, read at each call. The playground, an Angular app of its own, is served and built
 * by its own `ng` (ADR-0104, EN-175).
 *
 * @returns the configuration
 */
export default defineConfig((): ViteUserConfig => {
  const junitFile = process.env["VITEST_JUNIT"];

  return {
    build: LIBRARY_BUILD,
    test: {
      coverage: {
        exclude: ["**/index.ts", "**/*.test.ts"],
        include: ["src/**/*.ts"],
        provider: "v8",
        reporter: ["text-summary"],
        // The library at 95 % while it is developed, raised again at a later refinement (Product
        // Owner, 2026-10-25); the playground's floor is its own `ng test`'s (`angular.json`).
        thresholds: {
          "src/**/*.ts": { branches: 95, functions: 95, lines: 95, statements: 95 },
        },
      },
      // Each test font file read once per run, its bytes handed to every test file (EN-089).
      globalSetup: ["tests/font-files-setup.ts"],
      include: TEST_FILES,
      isolate: false,
      // Half the machine's cores: the other checks of `check:all` run beside the tests (EN-045).
      maxWorkers: "50%",
      ...(junitFile !== undefined && {
        outputFile: { junit: junitFile },
        reporters: [...CI_REPORTERS],
      }),
      root: ROOT,
      tags: TEST_TAGS,
    },
  };
});
