import { writeDeletedPoints } from "./write-deleted-points";

/** Keys deleting points: Delete, and Backspace, sent by the key labelled "delete" on a Mac. */
const DELETE_KEYS: ReadonlySet<string> = new Set(["Backspace", "Delete"]);

/**
 * Listens to the Delete and Backspace keys in Points mode, outside a field being typed in, to
 * delete the points selected, then calls back (F08.AC4).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F08-US-016-delete-points.md
 */
export function listenToDeleteKey(document: Document, update: () => void): void {
  document.addEventListener("keydown", (event) => {
    // In Shape mode no point is selected; in a field, the key edits the field.
    if (event.target instanceof HTMLInputElement || !DELETE_KEYS.has(event.key)) {
      return;
    }

    writeDeletedPoints(document);
    update();
  });
}
