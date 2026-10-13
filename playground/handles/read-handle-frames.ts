import { cyclicItem, subtract } from "../../src";

import type { HandleFrame } from "../../src";
import type { ShapeReading } from "../shape";
import type { SelectedCorner } from "./read-selected-corner";

/** The frames of the two handles of a point. */
export type HandleFrames = {
  /** The handle towards the next point. */
  readonly after: HandleFrame;
  /** The handle towards the previous point. */
  readonly before: HandleFrame;
};

/**
 * What each handle of the point selected is read against: the point, and its side — towards the
 * next point after, towards the previous point before (`DERIV-handle-angle`).
 *
 * @kind procedure
 * @param reading - the shape read from the inputs, its corners as drawn
 * @param selected - the point selected and its number
 * @returns the frames of its handles after and before
 * @see docs/backlog/stories/E14-F12-US-024-handles-as-angle-and-length.md
 */
export function readHandleFrames(reading: ShapeReading, selected: SelectedCorner): HandleFrames {
  const { point } = selected.corner;
  const next = cyclicItem(reading.corners, selected.index + 1, selected.corner).point;
  const previous = cyclicItem(reading.corners, selected.index - 1, selected.corner).point;

  return {
    after: { isBefore: false, point, side: subtract(next, point) },
    before: { isBefore: true, point, side: subtract(previous, point) },
  };
}
