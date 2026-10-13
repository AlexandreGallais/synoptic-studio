import { readGuidedTestSteps } from "../guided-test";
import { mountPlayground } from "../mount";

import { loadPage } from "./load-page";

/**
 * Opens the playground at an address exactly as `playground/index.ts` mounts it: the steps of the
 * guided test read from the page and its address (`?guide=epic`), not given by the test.
 *
 * @kind procedure
 * @param search - query of the address, `?guide=epic`, or empty
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function openServedPage(search: string): void {
  loadPage(search);
  mountPlayground(document, readGuidedTestSteps(document));
}
