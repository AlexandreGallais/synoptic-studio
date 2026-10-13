import { GUIDED_STEPS } from "../guided-steps";
import page from "../index.html?raw";
import { mountPlayground } from "../mount-playground";

/**
 * Opens the playground at an address, as the browser serves it: the page without its script tag,
 * then mounted with the guided test's steps.
 *
 * @kind procedure
 * @param search - query of the address, `?step=10`, or empty
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function openPage(search: string): void {
  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  globalThis.history.replaceState(undefined, "", `/${search}`);
  parsed.querySelector("script")?.remove();
  // The head carries the page's style: visibility is checked as the browser computes it.
  document.head.replaceChildren(...parsed.head.childNodes);
  document.body.replaceChildren(...parsed.body.childNodes);
  mountPlayground(document, GUIDED_STEPS);
}
