import { isInIntegerRange, modulo } from "../../math";

/**
 * Whether a number is the one of a point of the shape: a whole number from 0 to `count − 1`.
 *
 * @kind domain
 * @param point - number of a point
 * @param count - number of points of the shape
 * @returns `true` when the shape has that point
 * @see RULE-interaction-3-node-editing
 */
function isPointOf(point: number, count: number): boolean {
  return isInIntegerRange(point, 0, count - 1);
}

/**
 * Edge between two neighbouring points selected: its number is the one of its first point in
 * contour order, the edge closing the contour (from the last point to the first) being the last.
 *
 * @kind domain
 * @param selected - numbers of the points selected, from 0
 * @param count - number of points of the shape
 * @returns the number of the edge, `undefined` unless exactly two distinct neighbouring points of
 * the shape (whole numbers from 0 to `count − 1`) are selected
 * @see RULE-interaction-3-node-editing
 */
export function edgeOfSelection(selected: readonly number[], count: number): number | undefined {
  const isPair =
    selected.length === 2 &&
    selected[0] !== selected[1] &&
    selected.every((point) => isPointOf(point, count));

  // The edge starts at the point whose next one, in contour order, is the other point selected.
  return isPair ? selected.find((point) => selected.includes(modulo(point + 1, count))) : undefined;
}
