import { selectElement } from "../page";

import type { KeptShape } from "./kept-shape";

/**
 * Keeps shapes on the page's canvas as text (F35), each with the kind the panel showed it as
 * (F36), read back by `readStoredDrawing`.
 *
 * @kind procedure
 * @param document - page document
 * @param key - `DRAWING_KEYS.below`, `DRAWING_KEYS.edited` or `DRAWING_KEYS.above`
 * @param kept - the shapes in the order drawn
 * @see docs/domain/interaction.md#_2-selector
 */
export function writeStoredDrawing(
  document: Document,
  key: string,
  kept: readonly KeptShape[],
): void {
  selectElement(document, "canvas").dataset[key] = JSON.stringify(kept);
}
