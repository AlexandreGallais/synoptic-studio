import { selectElement } from "../page";

import type { Drawing } from "../../src";

/**
 * Keeps shapes on the page's canvas as text (F35), read back by `readStoredDrawing`.
 *
 * @kind procedure
 * @param document - page document
 * @param key - `DRAWING_KEYS.below` or `DRAWING_KEYS.edited`
 * @param drawing - the shapes in the order drawn
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function writeStoredDrawing(document: Document, key: string, drawing: Drawing): void {
  selectElement(document, "canvas").dataset[key] = JSON.stringify(drawing);
}
