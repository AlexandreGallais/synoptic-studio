import { isDrawnText } from "../../src";

import { loadPickedGeometry } from "./load-picked-geometry";
import { loadPickedText } from "./load-picked-text";

import type { KeptShape } from "../kept-shapes";

/**
 * Loads a shape picked into the panel, without firing any event: its kind chosen, and its
 * values written into its fields — a rectangle as a Rectangle, its box and its radius, though
 * stored as a free shape (ADR-0032); a polygon its corners, stretched option and anchor; a free
 * shape kept as one (`loadPickedGeometry`); a text its whole style, the font session taking its
 * family (`loadPickedText`).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the shape clicked, and the kind the panel showed it as
 * @see RULE-interaction-2-selector
 */
export function loadPickedShape(document: Document, kept: KeptShape): void {
  const { kind, shape } = kept;

  if (isDrawnText(shape)) {
    loadPickedText(document, shape);
  } else {
    loadPickedGeometry(document, kind, shape);
  }
}
