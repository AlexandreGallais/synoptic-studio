import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { LINE_AND_THIRDS, sideOf } from "../../../tests";
import { straightCubic } from "../curve";
import { orientedSides } from "../fillet-side";

import { fittedFilletRadius } from "./fitted-fillet-radius";

describe("fittedFilletRadius (ADR-0047)", () => {
  it("[F12.AC7] keeps a radius that fits, and reduces one too large to the curve's whole length", () => {
    // The curve is straight and 50 long: a right corner holds a radius up to 50.
    const sides = orientedSides(...LINE_AND_THIRDS);

    expect(fittedFilletRadius(sides, 10)).toBe(10);
    expect(fittedFilletRadius(sides, 80)).toBeCloseTo(50, 6);
  });

  it("draws no arc for radius 0, or sides leaving along one line", () => {
    const corner = { x: 0, y: 0 };
    const flat = orientedSides(
      sideOf(straightCubic(corner, { x: -10, y: 0 }), true),
      sideOf(straightCubic(corner, { x: 10, y: 0 }), false),
    );

    expect([
      fittedFilletRadius(orientedSides(...LINE_AND_THIRDS), 0),
      fittedFilletRadius(flat, 5),
    ]).toEqual([0, 0]);
  });

  test.prop({ radius: fc.integer({ max: 1000, min: 0 }) })(
    "never draws more than asked",
    ({ radius }) => {
      expect(fittedFilletRadius(orientedSides(...LINE_AND_THIRDS), radius)).toBeLessThanOrEqual(
        radius,
      );
    },
  );
});
