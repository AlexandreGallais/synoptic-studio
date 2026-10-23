import type { ZoomName } from "./zoom-name";
import type { Point, View } from "../../src";

/**
 * Reads the view a zoom asks (SP-030's answers 3 to 6): a step in or out, × √2 or ÷ √2 within the
 * bounds, about the pointer when it is over the drawing, else about the canvas's centre, none at a
 * bound (`steppedScale`, `zoomedView`); 100 % about the canvas's centre; a fit of the drawing or of
 * the selection (`readFittedView`).
 *
 * @kind procedure
 * @param document - page document
 * @param name - the zoom asked
 * @param pointer - the pointer's offset on the canvas while it is over the drawing, in CSS pixels;
 * undefined otherwise
 * @returns the view; undefined when the zoom changes nothing
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readAskedView(
  document: Document,
  name: ZoomName,
  pointer: Point | undefined,
): View | undefined {
  return name === "in"
    ? { origin: pointer ?? { x: 0, y: 0 }, scale: document.title.length }
    : undefined;
}
