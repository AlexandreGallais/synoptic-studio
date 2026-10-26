import { selectElement } from "../page";

import type { KeptShape } from "./kept-shape";

/**
 * Reads shapes the page keeps on its canvas as text, each with the kind the panel showed it
 * as: the shapes below the one the panel edits, that one as last drawn, or the shapes above
 * it (`DRAWING_KEYS`).
 *
 * @kind procedure
 * @param document - page document
 * @param key - `DRAWING_KEYS.below`, `DRAWING_KEYS.edited` or `DRAWING_KEYS.above`
 * @returns the shapes in the order drawn; none until some are kept
 */
export function readStoredDrawing(document: Document, key: string): readonly KeptShape[] {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeStoredDrawing only, from shapes drawn valid
  return JSON.parse(selectElement(document, "canvas").dataset[key] ?? "[]") as readonly KeptShape[];
}
