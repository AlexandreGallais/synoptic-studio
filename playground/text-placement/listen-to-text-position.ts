import { readShapeKind, selectElement } from "../page";

/**
 * Makes X and Y redraw a text whatever is typed (US-038), so that a fraction typed is refused
 * at once, outlined; the shapes' own moves act on integers only (`listenToPosition`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function listenToTextPosition(document: Document, update: () => void): void {
  /** Redraws the text, in Text mode only. */
  function redrawText(): void {
    if (readShapeKind(document) === "text") {
      update();
    }
  }

  for (const id of ["shape-x", "shape-y"]) {
    selectElement(document, id).addEventListener("input", redrawText);
  }
}
