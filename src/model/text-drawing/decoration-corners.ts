import { rectangleCorners } from "../rectangle";

import type { LineDecoration } from "./line-decoration";
import type { Corner } from "../../geometry";

/** A sharp corner at each of the four vertices. */
const SHARP = [0, 0, 0, 0];

/**
 * The corners of an underline's, a strike-through's or a background's rectangle, sharp, clockwise
 * from its top-left (Q11): the rectangle tool's corners (`rectangleCorners`), so that every rectangle of
 * the library is drawn by one route.
 *
 * @kind domain
 * @param decoration - the rectangle, its width and height positive
 * @returns its four corners
 * @see docs/domain/shapes.md#text
 */
export function decorationCorners(decoration: LineDecoration): readonly Corner[] {
  return rectangleCorners({ ...decoration, angle: 0, radii: SHARP });
}
