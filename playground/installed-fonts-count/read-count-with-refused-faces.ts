import type { ReadAheadCount } from "./read-ahead-count";
import type { RefusedFace } from "./refused-face";

/**
 * Counts the faces refused in a family read ahead and listed (US-062).
 *
 * @kind procedure
 * @param count - what was counted before them
 * @param faces - the faces refused, in the browser's order
 * @returns the count with them, after those counted before
 * @see docs/domain/shapes.md#text
 */
export function readCountWithRefusedFaces(
  count: ReadAheadCount,
  faces: readonly RefusedFace[],
): ReadAheadCount {
  return { ...count, refused: [...count.refused, ...faces] };
}
