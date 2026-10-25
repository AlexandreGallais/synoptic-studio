import type { DrawingSplit } from "./drawing-split";

/**
 * The drawing split around a shape picked (F36): the shapes before it below, the shape edited, the
 * shapes after it above, so that it keeps its place in the order drawn; no shape picked, an index
 * outside the drawing, leaves every shape below and none edited.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param drawing - the shapes in the order drawn, the last on top
 * @param index - the index of the shape picked (`shapeAtPoint`), −1 for none
 * @returns the three parts, which joined give the drawing back
 * @see RULE-interaction-2-selector
 */
export function splitDrawing<T>(drawing: readonly T[], index: number): DrawingSplit<T> {
  // A negative index reads no item, as one past the last.
  const picked = drawing[index];

  if (picked === undefined) {
    return { above: [], below: drawing, edited: [] };
  }

  return { above: drawing.slice(index + 1), below: drawing.slice(0, index), edited: [picked] };
}
