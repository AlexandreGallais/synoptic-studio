import { join } from "node:path";

import { defineConfig } from "vitest/config";

import type { BuildEnvironmentOptions } from "vite";

/** Absolute path of the repository root (this file's directory). */
const ROOT = import.meta.dirname;

/** Vite mode building the playground app instead of the library. */
const PLAYGROUND_MODE = "playground";

/** JUnit report file asked by the CI (`VITEST_JUNIT`), attached to each release (ADR-0027). */
const JUNIT_FILE = process.env["VITEST_JUNIT"];

/** Coverage floor of the playground (RET-001), measured then rounded down; raised, never lowered. */
const PLAYGROUND_COVERAGE = { branches: 89, functions: 100, lines: 98, statements: 98 };

/** Library build: one ESM file from `src/index.ts`, declarations emitted by `tsc`. */
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
 * Vite configuration: dev server on `playground/`, library build of `src/` (default mode) or
 * playground build (`--mode playground`), Vitest.
 *
 * @see docs/tooling/commands.md
 */
export default defineConfig(({ mode }) => ({
  base: mode === PLAYGROUND_MODE ? (process.env["PLAYGROUND_BASE"] ?? "/") : "/",
  build: mode === PLAYGROUND_MODE ? PLAYGROUND_BUILD : LIBRARY_BUILD,
  root: join(ROOT, "playground"),
  test: {
    coverage: {
      exclude: ["**/index.ts", "**/*.test.ts"],
      include: ["src/**/*.ts", "playground/**/*.ts"],
      provider: "v8",
      reporter: ["text-summary"],
      // The library is covered entirely; the playground, a demo shell, has a floor that only rises.
      thresholds: {
        "playground/**/*.ts": PLAYGROUND_COVERAGE,
        "src/**/*.ts": { 100: true },
      },
    },
    // Temporal is behind a flag in Node.js 24: the metrics scripts use it (REF-NODE-TS).
    execArgv: ["--harmony-temporal"],
    include: [
      "src/**/*.test.ts",
      "playground/**/*.test.ts",
      "eslint/**/*.test.ts",
      "tests/*.test.ts",
      "docs/.vitepress/**/*.test.ts",
      ".claude/hooks/*.test.ts",
      "scripts/*.test.ts",
    ],
    ...(JUNIT_FILE !== undefined && {
      outputFile: { junit: JUNIT_FILE },
      reporters: ["default", "junit"],
    }),
    root: ROOT,
    setupFiles: ["vitest.setup.ts"],
  },
}));
