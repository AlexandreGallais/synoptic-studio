import { hasArea, isRoundShape } from "../../src";
import { readSelection } from "../page";

import type { CurveCorner } from "../../src";
import type { ShapeReading } from "../shape";

/** The one point selected, as drawn, and its number. */
export type SelectedCorner = {
  /** Its corner as drawn: point, radius and handles turned with the shape. */
  readonly corner: CurveCorner;
  /** Its number, from 0. */
  readonly index: number;
};

/**
 * Number of the point selected when exactly one is.
 *
 * @kind procedure
 * @param document - page document
 * @returns its number, or `undefined`
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function readOnlySelected(document: Document): number | undefined {
  const { selected } = readSelection(document);

  return selected.length === 1 ? selected[0] : undefined;
}

/**
 * The corner of a point, with its number: a polygon's point too, its handles switch converting
 * it first (US-028), unless the polygon has no width or height and cannot be converted.
 *
 * @kind procedure
 * @param reading - the shape read from the inputs
 * @param index - number of the point
 * @returns that corner and its number, or `undefined`
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
function readFreeCorner(reading: ShapeReading, index: number): SelectedCorner | undefined {
  const corner = reading.corners[index];

  // A shape without width or height, a polygon that cannot be converted, gets no switch.
  return corner === undefined || !hasArea(reading.model) ? undefined : { corner, index };
}

/**
 * The corner of the one point selected, as drawn; none when no point or several are selected.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - the shape read from the inputs
 * @returns that corner and its number, or `undefined`
 * @see docs/backlog/stories/E14-F12-US-022-handles-in-points-mode.md
 */
export function readSelectedCorner(
  document: Document,
  reading: ShapeReading,
): SelectedCorner | undefined {
  const index = readOnlySelected(document);

  // A round shape's points already have their handles: it is converted first (US-030).
  return index === undefined || isRoundShape(reading.model)
    ? undefined
    : readFreeCorner(reading, index);
}
