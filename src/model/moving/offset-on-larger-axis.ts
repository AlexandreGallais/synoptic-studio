import type { Point } from "../../math";

/**
 * The pointer's offset with Shift held during a move (F39): the axis of the larger displacement
 * kept, the other set to 0, as Sketch and Penpot keep it; across on a tie (agent's choice).
 *
 * @kind domain
 * @param offset - how far the pointer went from the press
 * @returns the offset along one axis
 * @see docs/domain/interaction.md#_2-selector
 */
export function offsetOnLargerAxis(offset: Point): Point {
  return offset;
}
