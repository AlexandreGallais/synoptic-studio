import page from "../index.html?raw";

import { createResizeObserver } from "./create-resize-observer";

/**
 * Loads the playground page at an address, as the browser serves it, without mounting it: its
 * script tag removed, its head and body put in the document; its animation frames run at once,
 * and its `ResizeObserver` a stand-in the test tells of a size change (`runResizeObservers`, US-084).
 *
 * @kind procedure
 * @param search - query of the address, `?step=10`, or empty
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function loadPage(search: string): void {
  const parser = new DOMParser();
  const parsed = parser.parseFromString(page, "text/html");

  globalThis.history.replaceState(undefined, "", `/${search}`);
  parsed.querySelector("script")?.remove();
  // The head carries the page's style: visibility is checked as the browser computes it.
  document.head.replaceChildren(...parsed.head.childNodes);
  document.body.replaceChildren(...parsed.body.childNodes);

  // A test reads the page right after acting on it: its animation frames run at once, so that
  // a redraw asked for the next frame (US-048) is drawn before the test reads it.
  Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
    configurable: true,
    value: (callback: FrameRequestCallback): number => {
      callback(0);

      return 0;
    },
  });

  // happy-dom's `ResizeObserver` observes nothing: a stand-in the test calls back (US-084).
  Object.defineProperty(document.defaultView ?? {}, "ResizeObserver", {
    configurable: true,
    value: createResizeObserver,
  });
}
