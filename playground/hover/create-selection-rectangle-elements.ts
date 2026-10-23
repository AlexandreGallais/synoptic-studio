import { SELECTION_DRAGS } from "./selection-drags";

/**
 * Creates the selection rectangle in the layer above the drawing (F37), while a drag with the
 * Selector from an empty place has left the press's integer point (`SELECTION_DRAGS`): its area,
 * a light tint of the outline's color, of class `selection-area`, then its outline, a band 1
 * screen pixel wide centred on its sides, of class `selection-rectangle`, a fill with a hole,
 * never a native stroke; a rectangle thinner than the band gets no hole.
 *
 * @kind procedure
 * @param document - page document
 * @returns its area and its outline; nothing without such a drag
 * @see docs/domain/interaction.md#_2-selector
 */
export function createSelectionRectangleElements(document: Document): readonly SVGPathElement[] {
  // Placeholder until US-069.T2.
  return SELECTION_DRAGS.has(document) ? [] : [];
}
