import { readGuidedTestSteps } from "../guided-test";
import page from "../index.html?raw";
import { mountPlayground } from "../mount-playground";

/**
 * Opens the playground at an address exactly as `playground/index.ts` mounts it: the steps of the
 * guided test read from the page and its address (`?guide=epic`), not given by the test.
 *
 * @kind procedure
 * @param search - query of the address, `?guide=epic`, or empty
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function openServedPage(search: string): void {
  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  globalThis.history.replaceState(undefined, "", `/${search}`);
  parsed.querySelector("script")?.remove();
  document.head.replaceChildren(...parsed.head.childNodes);
  document.body.replaceChildren(...parsed.body.childNodes);
  mountPlayground(document, readGuidedTestSteps(document));
}
