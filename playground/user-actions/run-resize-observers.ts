import { selectPageElement } from "./select-page-element";
import { WATCHED_ELEMENTS } from "./watched-elements";

/**
 * Runs the stand-in observers watching an element of the page, telling them its size changed, as
 * the browser does in the rendering step after the layout that changed it
 * (`REF-RESIZE-OBSERVER-1`, §3.6.1): each observer's callback called with the observer; the
 * element's new box is set before by `setElementBox`, happy-dom laying nothing out.
 *
 * @kind procedure
 * @param selector - CSS selector of the element, `#canvas`
 */
export function runResizeObservers(selector: string): void {
  const target = selectPageElement(selector);

  const told = [...WATCHED_ELEMENTS].filter((each) => each.target === target);

  for (const watched of told) {
    watched.callback([], watched.observer);
  }
}
