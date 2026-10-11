import { writeDeletedPoints } from "./write-deleted-points";

/**
 * Listens to the Delete key in Points mode, outside a field being typed in, to delete the points
 * selected, then calls back (F08.AC4).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F08-US-016-delete-points.md
 */
export function listenToDeleteKey(document: Document, update: () => void): void {
  document.addEventListener("keydown", (event) => {
    // In Shape mode no point is selected; in a field, Delete edits the field.
    if (event.target instanceof HTMLInputElement || event.key !== "Delete") {
      return;
    }

    writeDeletedPoints(document);
    update();
  });
}
