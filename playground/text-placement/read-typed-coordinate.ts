import { readNumber } from "../page";

/**
 * Reads a coordinate the designer typed in X or Y: an integer (ADR-0003, ADR-0057 decision 5);
 * a fraction typed is refused, read as `NaN` so that the text cannot enter the model.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `shape-x` or `shape-y`
 * @returns the integer typed, `NaN` for anything else
 * @see docs/backlog/stories/E15-F05-US-038-a-turned-text-grows-in-place.md
 */
export function readTypedCoordinate(document: Document, id: string): number {
  const typed = readNumber(document, id);

  return Number.isSafeInteger(typed) ? typed : NaN;
}
