import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints, integerPoint } from "../../../tests";
import { NO_HANDLES } from "../../geometry";
import { turnedCorners } from "../placement";
import { rectangleAsFreeShape } from "../rectangle";

import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeWithHandles } from "./free-shape-with-handles";

import type { FreeShape } from "./free-shape";

/**
 * Corners of a free shape as drawn on the page.
 *
 * @param shape - free shape
 * @returns its corners turned
 */
function drawn(shape: FreeShape): ReturnType<typeof freeShapeCorners> {
  return turnedCorners(freeShapeCorners(shape), shape);
}

describe("freeShapeWithHandles", () => {
  it("[F12.AC1] sets a point's handles, its points and every other point drawn where they were", () => {
    // The square 100 × 100: point 1's handle after (0, −40) lifts the box to y = −40 · 4/9 ≈ −17.78,
    // yet the four points stay at their corners.
    const square = rectangleAsFreeShape({
      angle: 0,
      height: 100,
      radii: [0, 0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    });
    const curved = freeShapeWithHandles(square, 0, {
      ...NO_HANDLES,
      after: { x: 0, y: -40 },
      isOn: true,
    });

    expect(drawn(curved).map((corner) => corner.point)).toEqual(
      drawn(square).map((corner) => corner.point),
    );

    expect(drawn(curved)[0]?.handles).toEqual({
      ...NO_HANDLES,
      after: { x: 0, y: -40 },
      isOn: true,
    });

    expect(curved.y).toBeCloseTo(-160 / 9, 9);

    expect(
      drawn(curved)
        .slice(1)
        .map((corner) => corner.handles),
    ).toEqual([NO_HANDLES, NO_HANDLES, NO_HANDLES]);
  });

  test.prop({ after: integerPoint(50), angle: fc.constantFrom(0, 45, 90, 200) })(
    "keeps the points of a turned shape in place, and gives back the handles typed",
    ({ after, angle }) => {
      const shape = rectangleAsFreeShape({
        angle,
        height: 60,
        radii: [0, 0, 0, 0],
        width: 120,
        x: 10,
        y: 20,
      });
      const handles = { ...NO_HANDLES, after, before: { x: -after.x, y: -after.y }, isOn: true };
      const curved = freeShapeWithHandles(shape, 2, handles);
      const corner = drawn(curved)[2];

      expectPoints(
        drawn(curved).map((each) => each.point),
        drawn(shape).map((each) => each.point),
        6,
      );

      expect(
        drawn(curved)
          .filter((each, index) => index !== 2)
          .map((each) => each.handles.isOn),
      ).toEqual([false, false, false]);

      expect(corner?.handles.after.x).toBeCloseTo(after.x, 6);
      expect(corner?.handles.after.y).toBeCloseTo(after.y, 6);
    },
  );
});
