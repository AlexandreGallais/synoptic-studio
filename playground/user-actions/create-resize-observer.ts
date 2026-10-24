import { WATCHED_ELEMENTS } from "./watched-elements";

import type { WatchedElement } from "./watched-elements";

/**
 * Stops watching the elements a test of the watched ones picks, as an observer unobserving them.
 *
 * @kind procedure
 * @param isForgotten - whether a watched element is no longer watched
 * @see REF-RESIZE-OBSERVER-1
 */
function removeWatched(isForgotten: (watched: WatchedElement) => boolean): void {
  const forgotten = [...WATCHED_ELEMENTS].filter((each) => isForgotten(each));

  for (const watched of forgotten) {
    WATCHED_ELEMENTS.delete(watched);
  }
}

/**
 * Creates a stand-in `ResizeObserver`, called with `new` as the browser's: happy-dom's observes
 * nothing (note 0033, §3), so this one keeps each element it observes with its callback
 * (`WATCHED_ELEMENTS`), which the test calls by `runResizeObservers` where the browser would,
 * after a layout that changed the element's size (`REF-RESIZE-OBSERVER-1`, §3.6.1). Unlike the
 * browser's, it does not call back when observing starts, so that a mount draws the page once.
 *
 * @kind procedure
 * @param callback - what the observer calls on a size change
 * @returns the observer: `observe`, `unobserve` and `disconnect`, as the browser's
 * @see REF-RESIZE-OBSERVER-1
 */
export function createResizeObserver(callback: ResizeObserverCallback): ResizeObserver {
  const observer: ResizeObserver = {
    disconnect: () => {
      removeWatched((watched) => watched.observer === observer);
    },
    observe: (target) => {
      WATCHED_ELEMENTS.add({ callback, observer, target });
    },
    unobserve: (target) => {
      removeWatched((watched) => watched.observer === observer && watched.target === target);
    },
  };

  return observer;
}
