import { describe, expect, it } from "vitest";

import { effectiveRadii, roundedContour } from "../geometry";

import { freeShapeCorners } from "./free-shape-corners";
import { isValidFreeShape } from "./is-valid-free-shape";
import { roundShapeAsFreeShape } from "./round-shape-as-free-shape";
import { roundShapeCorners } from "./round-shape-corners";

import type { RoundShape } from "./round-shape";

/** A 120 × 80 round shape, a capsule, at (5, 7), turned by 30°. */
const CAPSULE: RoundShape = { angle: 30, height: 80, kind: "round", width: 120, x: 5, y: 7 };

describe("roundShapeAsFreeShape (DERIV-round-shape step 2)", () => {
  it("[F10.AC3] gives 4 points numbered as the rectangle's, radii 40, the same drawing", () => {
    const shape = roundShapeAsFreeShape(CAPSULE);

    expect(shape.angle).toBe(30);

    expect(freeShapeCorners(shape).map((corner) => corner.point)).toEqual([
      { x: 5, y: 7 },
      { x: 125, y: 7 },
      { x: 125, y: 87 },
      { x: 5, y: 87 },
    ]);

    expect(freeShapeCorners(shape).map((corner) => corner.radius)).toEqual([40, 40, 40, 40]);

    expect(roundedContour(freeShapeCorners(shape))).toEqual(
      roundedContour(roundShapeCorners(CAPSULE)),
    );
  });

  it("[F10.AC3] rounds 81 × 81 by 41, clamped to 40.5 when drawn: the same circle", () => {
    const shape = roundShapeAsFreeShape({ ...CAPSULE, height: 81, width: 81 });

    expect(freeShapeCorners(shape).map((corner) => corner.radius)).toEqual([41, 41, 41, 41]);

    // The same circle: every piece ends where the round shape's does, within rounding.
    const drawn = roundedContour(freeShapeCorners(shape)).flatMap((piece) => [
      piece.end.x,
      piece.end.y,
    ]);
    const expected = roundedContour(
      roundShapeCorners({ ...CAPSULE, height: 81, width: 81 }),
    ).flatMap((piece) => [piece.end.x, piece.end.y]);

    expect(drawn).toHaveLength(expected.length);

    for (const [index, value] of drawn.entries()) {
      expect(value).toBeCloseTo(expected[index] ?? NaN, 9);
    }

    // 41 · 81 / 82 = 40.5, within rounding (40.50000000000001).
    const radii = effectiveRadii(freeShapeCorners(shape));

    for (const radius of radii) {
      expect(radius).toBeCloseTo(40.5, 9);
    }
  });

  it("[F10.AC3] converts a round shape of 0 × 80 to a valid free shape, the same line", () => {
    const shape = roundShapeAsFreeShape({ ...CAPSULE, width: 0 });

    expect(isValidFreeShape(shape)).toBe(true);

    // The rectangle tool's unit square, stretched to 0 × 80, sharp.
    expect(shape.corners.map((corner) => corner.point)).toEqual([
      { x: 0, y: 0 },
      { x: 1, y: 0 },
      { x: 1, y: 1 },
      { x: 0, y: 1 },
    ]);

    expect(shape.corners.map((corner) => corner.radius)).toEqual([0, 0, 0, 0]);
    expect([shape.width, shape.height]).toEqual([0, 80]);
  });
});
