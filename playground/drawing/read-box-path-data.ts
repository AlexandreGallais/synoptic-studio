import { contourPiecesToPathData, roundedContour, turnedBoxCorners } from "../../src";

import type { TurnedBox } from "../../src";

/**
 * Path data of a box inset on each side, sharp, turned with its angle about its center
 * (ADR-0035): the rectangle tool's corners turned (`turnedBoxCorners`), so that every frame is
 * drawn by one route.
 *
 * @kind procedure
 * @param box - the box: its position, size and angle
 * @param inset - distance from the box's sides, in user units
 * @returns the `d` of the closed contour
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function readBoxPathData(box: TurnedBox, inset: number): string {
  const inner = {
    height: Math.max(box.height - 2 * inset, 0),
    width: Math.max(box.width - 2 * inset, 0),
    x: box.x + inset,
    y: box.y + inset,
  };

  return contourPiecesToPathData(roundedContour(turnedBoxCorners(inner, box)));
}
