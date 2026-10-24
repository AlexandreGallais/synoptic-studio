/** An element a stand-in `ResizeObserver` watches (`createResizeObserver`), and whom to tell. */
export type WatchedElement = {
  /** The element whose size is watched. */
  readonly target: Element;
  /** The observer watching it, given back to its callback. */
  readonly observer: ResizeObserver;
  /** What the observer calls when the element's size changes. */
  readonly callback: ResizeObserverCallback;
};

/**
 * The elements the stand-in observers watch, as the browser keeps them until an observer is
 * disconnected (`REF-RESIZE-OBSERVER-1`); told of a size change by `runResizeObservers`.
 */
export const WATCHED_ELEMENTS = new Set<WatchedElement>();
