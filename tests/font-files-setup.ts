// Vitest's global setup (EN-089): reads each test font file once per test run, in the main
// process, and hands its bytes to every test file through Vitest's provided context, so that no
// test reads it again. Tests only: never part of the library.

import { readFileSync } from "node:fs";
import { join } from "node:path";

import type { TestProject } from "vitest/node";

/**
 * Reads the test font files and provides their bytes to the test files, Inter Regular under the
 * key `interRegularFile` (`interRegularFile` in `font-fixtures.ts`).
 *
 * @param project - the Vitest project being run
 */
export function setup(project: TestProject): void {
  const file = new Uint8Array(readFileSync(join(import.meta.dirname, "inter-regular.ttf")));

  // Called through `Reflect.apply`: TypeScript 7 cannot augment Vitest's `ProvidedContext`
  // (TS2300), whose keys stay `never` for `provide`.
  Reflect.apply(project.provide, project, ["interRegularFile", file]);
}
