import { describe, expect, it } from "vitest";

import { rectangleAsFreeShape } from "../rectangle";

import { freeShapeKeptInPlace } from "./free-shape-kept-in-place";

describe("freeShapeKeptInPlace (DERIV-turned-shape step 5)", () => {
  it("[F04.AC4] leaves a straight shape where its box is", () => {
    const straight = rectangleAsFreeShape({
      angle: 0,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
      x: 0,
      y: 0,
    });

    expect(freeShapeKeptInPlace(straight, [], { after: 0, before: 0 })).toBe(straight);
  });

  it("[F04.AC4] places a turned shape so that the point kept is drawn at its former place", () => {
    // 100 × 50 at 90°: point 1 drawn at (75, −25) when the box is at (0, 0).
    const turned = rectangleAsFreeShape({
      angle: 90,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
      x: 7,
      y: 9,
    });
    const placed = freeShapeKeptInPlace(turned, [{ point: { x: 75, y: -25 }, radius: 0 }], {
      after: 0,
      before: 0,
    });

    expect(placed.x).toBeCloseTo(0, 12);
    expect(placed.y).toBeCloseTo(0, 12);
  });
});
