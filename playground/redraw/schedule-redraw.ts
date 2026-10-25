import { drawPendingRedraw } from "./draw-pending-redraw";
import { PENDING_REDRAWS } from "./pending-redraws";

/**
 * Asks the page to be redrawn before its next repaint (`requestAnimationFrame`, `REF-MDN-RAF`),
 * once: the requests made before that frame — an arrow key held sends about 30 `input` events a
 * second — are gathered into one redraw, which reads the values as they are at that frame, the
 * last typed, so that no redraw waits behind another. The redraw is kept until then
 * (`PENDING_REDRAWS`), so that a click drawing it earlier leaves the frame nothing to draw
 * (`drawPendingRedraw`). A window without animation frames redraws at once.
 *
 * @kind procedure
 * @param document - page document
 * @param draw - redraws the page
 * @see REF-MDN-RAF
 */
export function scheduleRedraw(document: Document, draw: () => void): void {
  const view = document.defaultView;

  if (PENDING_REDRAWS.has(document)) {
    return;
  }

  if (view === null) {
    draw();

    return;
  }

  PENDING_REDRAWS.set(document, draw);

  view.requestAnimationFrame(() => {
    drawPendingRedraw(document);
  });
}
