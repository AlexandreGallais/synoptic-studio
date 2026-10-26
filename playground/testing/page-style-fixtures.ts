// The page's global style for the tests (EN-162): the text `page-style.setup.ts` read once per
// run, which `loadPage` puts in the head as the app's `styles` entry is in the browser. Tests only.

import { inject } from "vitest";

/**
 * The page's global style, `src/app/shared/styles/page.scss`, as `page-style.setup.ts` read it.
 *
 * @returns its text
 * @throws {TypeError} when the global setup did not provide it
 */
export function pageStyle(): string {
  // Called through `Reflect.apply`, as `font-fixtures.ts` does: Vitest's `ProvidedContext` keys
  // stay `never` for `inject`.
  const provided: unknown = Reflect.apply(inject, undefined, ["pageStyle"]);

  if (typeof provided !== "string") {
    throw new TypeError("page-style.setup.ts did not provide pageStyle");
  }

  return provided;
}
