import { readListenerSignal } from "../listeners";
import { selectElement } from "../page";
import { drawPendingRedraw } from "../redraw";

/**
 * Listens to the canvas's size (US-084): a `ResizeObserver` on `#canvas` redraws the page at each
 * change of its size, whatever its cause, the window resized, the pipeline column hidden or shown,
 * the checklist shown after the first draw, within its own callback, which the browser runs after
 * the layout and before the paint (`REF-RESIZE-OBSERVER-1`, §3.6.1, `REF-MDN-RESIZE-OBSERVER`):
 * the redraw asked (`update`) is drawn at once (`drawPendingRedraw`, US-090), the inputs asked
 * before it with it, so that both SVGs are painted with the canvas's new `viewBox` in the frame
 * the canvas changes; drawn at the next frame, the old `viewBox` was painted for a frame, fitted
 * to the new box by `xMidYMid meet`, shrunk or moved (`REF-SVG2-COORDS`, §8.2). In a browser the
 * observer also calls back once as observing starts, the canvas being rendered and not 0 × 0
 * (`REF-RESIZE-OBSERVER-1`, §1): the page drawn again at the size the mount drew it at, one draw
 * more per mount, accepted (AUD-046) rather than told apart from a change; the tests' stand-in
 * observer leaves it out (`createResizeObserver`), so that their counts of redraws are a mount's
 * own. The observer is disconnected when the next mount removes the page's listeners
 * (`readListenerSignal`, EN-150); a document without a window has no observer.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see RULE-interaction-7-navigation-and-history
 */
export function listenToCanvasSize(document: Document, update: () => void): void {
  const view = document.defaultView;

  if (view === null) {
    return;
  }

  const observer = new view.ResizeObserver(() => {
    update();
    drawPendingRedraw(document);
  });

  observer.observe(selectElement(document, "canvas"));

  readListenerSignal(document).addEventListener("abort", () => {
    observer.disconnect();
  });
}
