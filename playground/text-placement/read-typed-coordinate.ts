import { nearestInteger } from "../../src";
import { readNumber } from "../page";

/**
 * Reads a coordinate the designer typed in X or Y: an integer (ADR-0003, ADR-0057 decision 5), a
 * fraction rounded to the nearest one, a tie up (`nearestInteger`, Q45, AUD-046), so that the
 * model takes integers alone.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `shape-x` or `shape-y`
 * @returns the integer typed, `NaN` for a field showing no number
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function readTypedCoordinate(document: Document, id: string): number {
  return nearestInteger(readNumber(document, id));
}
