// Vitest's global setup of the playground's tests (EN-162): reads the page's global style once per
// test run, in the main process, and hands its text to every test file through Vitest's provided
// context. Angular's unit-test builder links the app's `styles` entry from an HTML page only, which
// a test on happy-dom has none of; `loadPage` puts the text in the head instead. Tests only.

import { readFileSync } from "node:fs";
import { join } from "node:path";

import type { TestProject } from "vitest/node";

/**
 * Reads the page's global style and provides its text to the test files under the key `pageStyle`
 * (`loadPage`).
 *
 * @param project - the Vitest project being run
 */
export function setup(project: TestProject): void {
  const style = readFileSync(
    join(import.meta.dirname, "..", "src", "app", "shared", "styles", "page.scss"),
    "utf8",
  );

  // Called through `Reflect.apply`, as `font-files.setup.ts` does: TypeScript cannot augment
  // Vitest's `ProvidedContext`, whose keys stay `never` for `provide`.
  Reflect.apply(project.provide, project, ["pageStyle", style]);
}
