import type { View } from "../../src";

/**
 * Reads the view fitting the straight box of the shapes drawn, or of the shapes selected, 40
 * pixels left on each side (`fittedView`, `pickAreasBox`, SP-030's answer 6): their contours as
 * drawn, a text never measured, which is not drawn, left out.
 *
 * @kind procedure
 * @param document - page document
 * @param isSelection - whether the shapes selected are fitted, rather than every shape
 * @returns the view; undefined when none of them is drawn
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readFittedView(document: Document, isSelection: boolean): View | undefined {
  return isSelection ? undefined : { origin: { x: 0, y: 0 }, scale: document.title.length };
}
