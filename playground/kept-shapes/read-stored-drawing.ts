import { selectElement } from "../page";

import type { Drawing } from "../../src";

/**
 * Reads shapes the page keeps on its canvas as text (F35): the shapes below the one the panel
 * edits, or that one as last drawn (`DRAWING_KEYS`).
 *
 * @kind procedure
 * @param document - page document
 * @param key - `DRAWING_KEYS.below` or `DRAWING_KEYS.edited`
 * @returns the shapes in the order drawn; none until some are kept
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function readStoredDrawing(document: Document, key: string): Drawing {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by writeStoredDrawing only, from shapes drawn valid
  return JSON.parse(selectElement(document, "canvas").dataset[key] ?? "[]") as Drawing;
}
