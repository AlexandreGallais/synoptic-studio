import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { turnDirection } from "../math";

import { turnAbout } from "./turn-about";

/** Point with integer coordinates. */
const POINT = fc.record({
  x: fc.integer({ max: 1e4, min: -1e4 }),
  y: fc.integer({ max: 1e4, min: -1e4 }),
});

describe("turnAbout (DERIV-turned-shape steps 2 and 4, REF-SVG11-TRANSFORM)", () => {
  it.each([
    // Rectangle 100 × 50, center (50, 25), 90°: (x, y) − c ↦ (−(y − c_y), x − cₓ), then + c.
    // (0, 0): (−50, −25) ↦ (25, −50) ↦ (75, −25).
    { point: { x: 0, y: 0 }, turned: { x: 75, y: -25 } },
    // (100, 0): (50, −25) ↦ (25, 50) ↦ (75, 75).
    { point: { x: 100, y: 0 }, turned: { x: 75, y: 75 } },
    // (100, 50): (50, 25) ↦ (−25, 50) ↦ (25, 75).
    { point: { x: 100, y: 50 }, turned: { x: 25, y: 75 } },
    // (0, 50): (−50, 25) ↦ (−25, −50) ↦ (25, −25).
    { point: { x: 0, y: 50 }, turned: { x: 25, y: -25 } },
  ])("[F04.AC1] turns $point clockwise by 90° about (50, 25), exactly", ({ point, turned }) => {
    expect(turnAbout(point, { x: 50, y: 25 }, turnDirection(90))).toEqual(turned);
  });

  it("[F04.AC1] turns the top-left of a 100 square by 45° about its center to (50, −20.71068)", () => {
    // (−50, −50): x' = −50·√2/2 + 50·√2/2 = 0, y' = −50·√2/2 − 50·√2/2 = −50√2.
    const turned = turnAbout({ x: 0, y: 0 }, { x: 50, y: 50 }, turnDirection(45));

    expect(turned.x).toBeCloseTo(50, 12);
    expect(turned.y).toBeCloseTo(50 - 50 * Math.SQRT2, 12);
  });

  it("[F04.AC3] leaves the center where it is", () => {
    expect(turnAbout({ x: 50, y: 25 }, { x: 50, y: 25 }, turnDirection(37))).toEqual({
      x: 50,
      y: 25,
    });
  });

  test.prop({ center: POINT, point: POINT })(
    "gives the point back exactly after four quarter turns",
    ({ center, point }) => {
      const quarter = turnDirection(90);
      const turned = [1, 2, 3, 4].reduce((current) => turnAbout(current, center, quarter), point);

      expect(turned.x).toBe(point.x);
      expect(turned.y).toBe(point.y);
    },
  );

  test.prop({ angle: fc.integer({ max: 359, min: 0 }), center: POINT, point: POINT })(
    "keeps the distance to the center",
    ({ angle, center, point }) => {
      const turned = turnAbout(point, center, turnDirection(angle));

      expect(Math.hypot(turned.x - center.x, turned.y - center.y)).toBeCloseTo(
        Math.hypot(point.x - center.x, point.y - center.y),
        6,
      );
    },
  );
});
