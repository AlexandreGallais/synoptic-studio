import { join } from "node:path";

/** Absolute path of the repository root. */
export const PROJECT_ROOT = join(import.meta.dirname, "..", "..");

/** Markdown pages (documentation, backlog, CLAUDE.md, README). */
export const MARKDOWN_FILES = ["**/*.md"];

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
  "CHANGELOG.md",
  // Worktrees of agents and parallel sessions: copies of the repository, checked in their own.
  ".claude/worktrees/**",
  // Private records, their own Git repository, ignored by this one (ADR-0044).
  "records/**",
  // Only TypeScript is authored; the rare JavaScript file is a tool's required format (.ncurc.cjs).
  "**/*.{js,cjs,mjs}",
];

/** Vitest test files, co-located with the code they test. */
export const TEST_FILES = ["**/*.test.ts"];

/** Library source code (published). */
export const LIBRARY_FILES = ["src/**/*.ts"];

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

/** Hooks and scripts run by Node.js as they are (type stripping): imports name the `.ts` file. */
export const NODE_RUN_FILES = [".claude/hooks/*.ts", "scripts/*.ts"];

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

/** Modules subject to one export per file, named like it (ADR-0019). */
export const SINGLE_EXPORT_FILES = ["src/**/*.ts", "playground/**/*.ts"];

/** Files exempt from one export per file: barrels, entry points and tests. */
export const SINGLE_EXPORT_EXEMPT_FILES = ["**/index.ts", "**/*.test.ts"];
