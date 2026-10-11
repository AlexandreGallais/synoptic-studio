import { describe, expect, it } from "vitest";

import { positionKeepingPoint } from "./position-keeping-point";

describe("positionKeepingPoint (DERIV-turned-shape step 5)", () => {
  it("[F04.AC4] gives back the position of a 100 × 50 box at 90° from its drawn top-left", () => {
    // Its top-left (0, 0) is drawn at (75, −25) (DERIV-turned-shape checks): the box is at (0, 0).
    expect(
      positionKeepingPoint(
        { angle: 90, height: 50, width: 100 },
        { x: 0, y: 0 },
        { x: 75, y: -25 },
      ),
    ).toEqual({ x: 0, y: 0 });
  });

  it("[F04.AC4] places a box typed 200 wide so that its top-left stays at (75, −25)", () => {
    // k' = (100, 25), R(−k') + k' = (25, −100) + (100, 25) = (125, −75): b' = (75, −25) − (125, −75).
    expect(
      positionKeepingPoint(
        { angle: 90, height: 50, width: 200 },
        { x: 0, y: 0 },
        { x: 75, y: -25 },
      ),
    ).toEqual({ x: -50, y: 50 });
  });
});
