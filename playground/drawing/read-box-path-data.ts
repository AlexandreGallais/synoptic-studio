import { contourPiecesToPathData, insetBox, roundedContour, turnedBoxCorners } from "../../src";

import type { TurnedBox } from "../../src";

/**
 * Path data of a box inset on each side (`insetBox`), sharp, turned with its angle about its center
 * (ADR-0035): the rectangle tool's corners turned (`turnedBoxCorners`), so that every frame is
 * drawn by one route.
 *
 * @kind procedure
 * @param box - the box: its position, size and angle
 * @param inset - distance from the box's sides, in user units
 * @returns the `d` of the closed contour
 */
export function readBoxPathData(box: TurnedBox, inset: number): string {
  const corners = turnedBoxCorners(insetBox(box, inset), box);

  return contourPiecesToPathData(roundedContour(corners));
}
