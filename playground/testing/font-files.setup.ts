// Vitest's global setup of the playground's tests (EN-089): reads each test font file once per
// test run, in the main process, and hands its bytes to every test file through Vitest's provided
// context, so that no test reads it again. The playground's copy of the root's
// `tests/font-files-setup.ts`, reading its own copy of the font, so that its tests run on the
// playground's install alone (EN-176). Tests only: never part of the library.

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

  // Called through `Reflect.apply`, as the root's copy is: TypeScript 7 there cannot augment
  // Vitest's `ProvidedContext` (TS2300), whose keys stay `never` for `provide`.
  Reflect.apply(project.provide, project, ["interRegularFile", file]);
}
