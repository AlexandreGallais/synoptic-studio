import { join } from "node:path";

/** Absolute path of the repository root. */
export const PROJECT_ROOT = join(import.meta.dirname, "..", "..");

/** Every TypeScript file linted by the project. */
export const TS_FILES = ["**/*.ts"];

/** Generated or vendored paths never linted. */
export const IGNORED_FILES = [
  "dist/**",
  "coverage/**",
  "node_modules/**",
  "docs/api/**",
  "docs/.vitepress/cache/**",
  "docs/.vitepress/dist/**",
  "docs/.vitepress/.temp/**",
  // Worktrees of agents and parallel sessions: copies of the repository, checked in their own.
  ".claude/worktrees/**",
  // Private records, their own Git repository, ignored by this one (ADR-0044).
  "records/**",
  // Build output of the Rust workspace (EN-068).
  "target/**",
  // Inputs of a Rust tool's tests: modules of `src/` copied unchanged (EN-072), checked in `src/`.
  "tools/*/tests/fixtures/**",
  // Only TypeScript is authored; the rare JavaScript file is a tool's required format (.ncurc.cjs).
  "**/*.{js,cjs,mjs}",
];

/** Vitest test files, co-located with the code they test. */
export const TEST_FILES = ["**/*.test.ts"];

/** Library layers without DOM access: the functional core. */
export const PURE_LAYER_FILES = [
  "src/math/**/*.ts",
  "src/geometry/**/*.ts",
  "src/model/**/*.ts",
  "src/routing/**/*.ts",
  "src/io/**/*.ts",
];

/** Library layers allowed to touch the DOM: the imperative shell. */
export const DOM_LAYER_FILES = [
  "src/render/**/*.ts",
  "src/interaction/**/*.ts",
  "playground/**/*.ts",
];

/** Lint configuration, custom ESLint plugin, agent hooks and scripts (Node.js tooling). */
export const TOOLING_FILES = [
  "eslint/**/*.ts",
  "*.config.ts",
  "*.setup.ts",
  "*.test.ts",
  "tests/*.ts",
  "docs/.vitepress/**/*.ts",
  ".claude/hooks/**/*.ts",
  "scripts/**/*.ts",
];

/** Root configuration files whose tools require a default export. */
export const ROOT_CONFIG_FILES = [
  "*.config.ts",
  "docs/.vitepress/config.ts",
  "docs/.vitepress/theme/index.ts",
];

/** Entry module of the playground app (top-level side effects allowed). */
export const PLAYGROUND_ENTRY_FILES = ["playground/index.ts"];

/** Folders whose `index.ts` files are barrels (re-exports only); `playground/index.ts` is the entry. */
export const BARREL_FILES = ["src/**/index.ts", "eslint/**/index.ts", "playground/*/**/index.ts"];

/** Lists of rule themes, one import per theme file. */
export const AGGREGATOR_FILES = ["eslint/rules/*/all.ts"];
