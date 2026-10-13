import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints } from "../../tests";

import {
  freeShapeCorners,
  freeShapeWithMidpoint,
  freeShapeWithPoint,
  freeShapeWithoutPoints,
} from "./free-shape";
import { turnedCorners } from "./placement";
import { rectangleAsFreeShape } from "./rectangle";

import type { FreeShape } from "./free-shape";

/**
 * Points of a free shape as drawn, turned.
 *
 * @param shape - free shape
 * @returns its drawn points, in contour order
 */
function drawnPoints(shape: FreeShape): readonly FreeShape["corners"][number]["point"][] {
  return turnedCorners(freeShapeCorners(shape), shape).map((corner) => corner.point);
}

/** The rectangle tool's 100 × 50 shape turned by 90°: drawn (75, −25), (75, 75), (25, 75), (25, −25). */
const TURNED = rectangleAsFreeShape({
  angle: 90,
  height: 50,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 0,
  y: 0,
});

describe("edits of a turned free shape (ADR-0035, DERIV-turned-shape step 5)", () => {
  it("[F04.AC4] moves only the point typed: x 80 for point 1, the others drawn where they were", () => {
    const edited = freeShapeWithPoint(TURNED, 0, { x: 80, y: -25 });

    expectPoints(
      drawnPoints(edited),
      [
        { x: 80, y: -25 },
        { x: 75, y: 75 },
        { x: 25, y: 75 },
        { x: 25, y: -25 },
      ],
      9,
    );

    expect(edited.angle).toBe(90);
  });

  test.prop({
    angle: fc.integer({ max: 359, min: 1 }),
    index: fc.integer({ max: 3, min: 0 }),
    x: fc.integer({ max: 300, min: -300 }),
    y: fc.integer({ max: 300, min: -300 }),
  })("[F04.AC4] lands the point typed on its integers and keeps every other point", (typed) => {
    const shape = { ...TURNED, angle: typed.angle };
    const before = drawnPoints(shape);
    const edited = freeShapeWithPoint(shape, typed.index, { x: typed.x, y: typed.y });
    const expected = before.map((point, index) =>
      index === typed.index ? { x: typed.x, y: typed.y } : point,
    );

    expectPoints(drawnPoints(edited), expected, 9);
  });

  it("[F04.AC4] adds a point in the middle of an edge without moving the others", () => {
    // Between (75, −25) and (75, 75): (75, 25).
    expectPoints(
      drawnPoints(freeShapeWithMidpoint(TURNED, 0)),
      [
        { x: 75, y: -25 },
        { x: 75, y: 25 },
        { x: 75, y: 75 },
        { x: 25, y: 75 },
        { x: 25, y: -25 },
      ],
      9,
    );
  });

  it("[F04.AC4] deletes a point without moving the others", () => {
    // Point 1 removed: the triangle of the three others, drawn where they were.
    expectPoints(
      drawnPoints(freeShapeWithoutPoints(TURNED, [0])),
      [
        { x: 75, y: 75 },
        { x: 25, y: 75 },
        { x: 25, y: -25 },
      ],
      9,
    );
  });

  test.prop({ angle: fc.integer({ max: 359, min: 1 }), edge: fc.integer({ max: 3, min: 0 }) })(
    "[F04.AC4] adds a point at any angle without moving the others",
    ({ angle, edge }) => {
      const shape = { ...TURNED, angle };
      const before = drawnPoints(shape);
      const after = drawnPoints(freeShapeWithMidpoint(shape, edge));

      expectPoints(
        after.filter((point, index) => index !== edge + 1),
        before,
        9,
      );
    },
  );

  test.prop({ angle: fc.integer({ max: 359, min: 1 }), removed: fc.integer({ max: 3, min: 0 }) })(
    "[F04.AC4] deletes a point at any angle without moving the others",
    ({ angle, removed }) => {
      const shape = { ...TURNED, angle };
      const before = drawnPoints(shape).filter((point, index) => index !== removed);

      expectPoints(drawnPoints(freeShapeWithoutPoints(shape, [removed])), before, 9);
    },
  );
});
