import { PENDING_REDRAWS } from "./pending-redraws";

/**
 * Draws the redraw the page asked and has not drawn yet, if any, at once rather than at its
 * animation frame (`scheduleRedraw`), so that what was typed before is drawn, and its step taken,
 * before an action reads the page as last drawn, and so that a canvas resized is painted
 * at its new size in the frame it changes (`listenToCanvasSize`); the frame then finds
 * nothing to draw.
 *
 * @kind procedure
 * @param document - page document
 * @see REF-MDN-RAF
 */
export function drawPendingRedraw(document: Document): void {
  const draw = PENDING_REDRAWS.get(document);

  if (draw === undefined) {
    return;
  }

  PENDING_REDRAWS.delete(document);
  draw();
}
