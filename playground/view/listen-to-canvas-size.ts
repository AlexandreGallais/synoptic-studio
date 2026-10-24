import { readListenerSignal } from "../listeners";
import { selectElement } from "../page";

/**
 * Listens to the canvas's size (US-084): a `ResizeObserver` on `#canvas` asks the redraw at each
 * change of its size, whatever its cause, the window resized, the pipeline column hidden or shown,
 * the checklist shown after the first draw, so that both SVGs get the canvas's new `viewBox`
 * before the next paint (`REF-RESIZE-OBSERVER-1`, `REF-MDN-RESIZE-OBSERVER`); a `viewBox` of the
 * old size would be fitted to the new box by `xMidYMid meet`, shrunk or moved (`REF-SVG2-COORDS`,
 * §8.2). The observer is disconnected when the next mount removes the page's listeners
 * (`readListenerSignal`, EN-150); a document without a window has no observer.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToCanvasSize(document: Document, update: () => void): void {
  const view = document.defaultView;

  if (view === null) {
    return;
  }

  const observer = new view.ResizeObserver(update);

  observer.observe(selectElement(document, "canvas"));

  readListenerSignal(document).addEventListener("abort", () => {
    observer.disconnect();
  });
}
