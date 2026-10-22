import { writeText } from "../page";

import type { KeptShape } from "../kept-shapes";

/**
 * Loads a shape picked into the panel, without firing any event (F36): its kind chosen, and its
 * values written into its fields — a rectangle as a Rectangle, its box and its radius, though
 * stored as a free shape (ADR-0032); a polygon its corners, stretched option and anchor; a free
 * shape kept as one; a text its whole style, the font session taking its family — no point
 * selected.
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the shape picked, and the kind the panel showed it as
 * @see docs/domain/interaction.md#_2-selector
 */
export function loadPickedShape(document: Document, kept: KeptShape): void {
  writeText(document, "status", kept.kind);
}
