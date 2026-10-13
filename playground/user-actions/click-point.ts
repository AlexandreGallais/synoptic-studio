import { clickWithKeys } from "./click-with-keys";

/**
 * Clicks the row of a point in the list of points.
 *
 * @kind procedure
 * @param vertex - number of the point, from 1
 * @param isCtrlHeld - whether Ctrl is held, adding the point to the selection
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function clickPoint(vertex: number, isCtrlHeld: boolean): void {
  clickWithKeys(`#points [data-vertex="${CSS.escape(String(vertex))}"]`, { ctrlKey: isCtrlHeld });
}
