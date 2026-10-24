import { join } from "node:path";

import { defineConfig } from "vitest/config";

import type { BuildEnvironmentOptions } from "vite";
import type { TestProjectInlineConfiguration } from "vitest/config";

/** Absolute path of the repository root (this file's directory). */
const ROOT = import.meta.dirname;

/** Vite mode building the playground app instead of the library. */
const PLAYGROUND_MODE = "playground";

/**
 * Reporters of the CI's test lane, `VITEST_JUNIT` set on every lane and read by Vitest alone, which asks a JUnit report by `VITEST_JUNIT`: each file as it
 * ends, each failure annotated at its line on the pull request, and the report attached to each
 * release (ADR-0027). A configured list replaces Vitest's own, so the GitHub Actions reporter is
 * named (`REF-VITEST-REPORTERS`).
 */
const CI_REPORTERS = ["default", "github-actions", "junit"];

/** Coverage floor of the playground (RET-001), measured then rounded down; raised, never lowered. */
const PLAYGROUND_COVERAGE = { branches: 89, functions: 100, lines: 98, statements: 98 };

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

/** Playground build: the static app, published next to the docs site. */
const PLAYGROUND_BUILD: BuildEnvironmentOptions = {
  emptyOutDir: true,
  outDir: join(ROOT, "docs", ".vitepress", "dist", "playground"),
};

/**
 * The two groups of tests: `shared`, whose files share a worker's loaded modules, the library
 * imported once per worker instead of once per file, where no test keeps state between files;
 * `isolated`, the playground's, whose page keeps its state in its
 * modules and whose tests mock the library (EN-090, ADR-0070).
 */
const TEST_PROJECTS: readonly TestProjectInlineConfiguration[] = [
  {
    extends: true,
    test: {
      include: [
        "src/**/*.test.ts",
        "tests/*.test.ts",
        "docs/.vitepress/**/*.test.ts",
        "scripts/*.test.ts",
      ],
      isolate: false,
      name: "shared",
    },
  },
  {
    extends: true,
    test: { include: ["playground/**/*.test.ts"], isolate: true, name: "isolated" },
  },
];

/**
 * Vite configuration: dev server on `playground/`, library build of `src/` (default mode) or
 * playground build (`--mode playground`), Vitest; the CI's reporters when it asks a JUnit report,
 * read at each call.
 *
 * @param env - what Vite runs
 * @param env.mode - `playground` for the playground build, the library build otherwise
 * @returns the configuration of that mode
 * @see docs/tooling/commands.md
 */
export default defineConfig(({ mode }) => {
  const junitFile = process.env["VITEST_JUNIT"];

  return {
    base: mode === PLAYGROUND_MODE ? (process.env["PLAYGROUND_BASE"] ?? "/") : "/",
    build: mode === PLAYGROUND_MODE ? PLAYGROUND_BUILD : LIBRARY_BUILD,
    root: join(ROOT, "playground"),
    test: {
      coverage: {
        exclude: ["**/index.ts", "**/*.test.ts", "playground/user-actions/**"],
        include: ["src/**/*.ts", "playground/**/*.ts"],
        provider: "v8",
        reporter: ["text-summary"],
        // The library is covered entirely; the playground, a demo shell, has a floor that only rises.
        thresholds: {
          "playground/**/*.ts": PLAYGROUND_COVERAGE,
          "src/**/*.ts": { 100: true },
        },
      },
      // Each test font file read once per run, its bytes handed to every test file (EN-089).
      globalSetup: ["tests/font-files-setup.ts"],
      // Half the machine's cores: the other checks of `check:all` run beside the tests (EN-045).
      maxWorkers: "50%",
      projects: [...TEST_PROJECTS],
      ...(junitFile !== undefined && {
        outputFile: { junit: junitFile },
        reporters: [...CI_REPORTERS],
      }),
      root: ROOT,
      tags: TEST_TAGS,
    },
  };
});
