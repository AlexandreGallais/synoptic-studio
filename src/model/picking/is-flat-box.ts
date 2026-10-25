import type { TurnedBox } from "../placement";

/**
 * Whether a box has a width or a height of 0, so that the shape filling it paints nothing (Q15).
 *
 * @kind domain
 * @param box - its width and its height
 * @returns true when either is 0
 * @see RULE-interaction-2-selector
 */
export function isFlatBox(box: Pick<TurnedBox, "height" | "width">): boolean {
  return box.width === 0 || box.height === 0;
}
