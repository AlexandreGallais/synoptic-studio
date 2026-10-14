import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { turnAbout } from "../../geometry";
import { turnDirection } from "../../math";

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

  test.prop({
    angle: fc.integer({ max: 359, min: 0 }),
    drawn: fc.record({
      x: fc.integer({ max: 500, min: -500 }),
      y: fc.integer({ max: 500, min: -500 }),
    }),
    height: fc.integer({ max: 500, min: 0 }),
    local: fc.record({ x: fc.integer({ max: 500, min: 0 }), y: fc.integer({ max: 500, min: 0 }) }),
    width: fc.integer({ max: 500, min: 0 }),
  })(
    "[F04.AC4] draws the local point at the place asked",
    ({ angle, drawn, height, local, width }) => {
      const position = positionKeepingPoint({ angle, height, width }, local, drawn);
      const center = { x: position.x + width / 2, y: position.y + height / 2 };
      const turned = turnAbout(
        { x: position.x + local.x, y: position.y + local.y },
        center,
        turnDirection(angle),
      );

      expect(turned.x).toBeCloseTo(drawn.x, 9);
      expect(turned.y).toBeCloseTo(drawn.y, 9);
    },
  );
});
