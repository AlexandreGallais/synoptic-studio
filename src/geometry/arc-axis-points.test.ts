import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { arcAxisPoints } from "./arc-axis-points";

import type { Point } from "../math";

/**
 * Point of the unit circle at an angle.
 *
 * @param degrees - angle in degrees, clockwise on screen
 * @returns `(cos, sin)` of the angle
 */
function onUnitCircle(degrees: number): Point {
  return { x: Math.cos((degrees * Math.PI) / 180), y: Math.sin((degrees * Math.PI) / 180) };
}

describe("arcAxisPoints (DERIV-arc-bounding-box step 3)", () => {
  it("gives the two ends of a straight top-left fillet, which lie on the axes", () => {
    // Center (10, 10), from (0, 10) to (10, 0), clockwise on screen: s = (−10, 0), e = (0, −10).
    // Left: s × d = 0, d × e = 10 — on the arc; up: s × d = 10, d × e = 0 — on the arc;
    // right: d × e = −10; down: s × d = −10.
    expect(
      arcAxisPoints({
        center: { x: 10, y: 10 },
        end: { x: 10, y: 0 },
        kind: "arc",
        positive: true,
        radius: 10,
        start: { x: 0, y: 10 },
      }),
    ).toEqual([
      { x: 0, y: 10 },
      { x: 10, y: 0 },
    ]);
  });

  it("gives the same points for the same arc run the other way", () => {
    expect(
      arcAxisPoints({
        center: { x: 10, y: 10 },
        end: { x: 0, y: 10 },
        kind: "arc",
        positive: false,
        radius: 10,
        start: { x: 10, y: 0 },
      }),
    ).toEqual([
      { x: 0, y: 10 },
      { x: 10, y: 0 },
    ]);
  });

  it("[F04.AC3] gives the top of an arc from 225° to 315°, which crosses the upward axis", () => {
    const half = Math.SQRT1_2;

    expect(
      arcAxisPoints({
        center: { x: 0, y: 0 },
        end: { x: half, y: -half },
        kind: "arc",
        positive: true,
        radius: 1,
        start: { x: -half, y: -half },
      }),
    ).toEqual([{ x: 0, y: -1 }]);
  });

  it("gives nothing for an arc between two axes", () => {
    // From 200° to 250°: neither 180° nor 270°.
    expect(
      arcAxisPoints({
        center: { x: 0, y: 0 },
        end: onUnitCircle(250),
        kind: "arc",
        positive: true,
        radius: 1,
        start: onUnitCircle(200),
      }),
    ).toEqual([]);
  });

  test.prop({
    from: fc.integer({ max: 359, min: 0 }),
    radius: fc.integer({ max: 100, min: 1 }),
    sweep: fc.integer({ max: 179, min: 1 }),
  })("gives points on the circle, between the ends, at the axes", ({ from, radius, sweep }) => {
    const start = onUnitCircle(from);
    const end = onUnitCircle(from + sweep);
    const points = arcAxisPoints({
      center: { x: 0, y: 0 },
      end: { x: radius * end.x, y: radius * end.y },
      kind: "arc",
      positive: true,
      radius,
      start: { x: radius * start.x, y: radius * start.y },
    });
    // The axis angles 0, 90, 180, 270 swept between from and from + sweep; an axis exactly at an
    // end may fall on either side by rounding, and the end counts anyway.
    const offsets = [0, 90, 180, 270].map((axis) => (axis - from + 360) % 360);
    const inside = offsets.filter((offset) => offset > 0 && offset < sweep).length;
    const reached = offsets.filter((offset) => offset <= sweep).length;

    expect(points.length).toBeGreaterThanOrEqual(inside);
    expect(points.length).toBeLessThanOrEqual(reached);

    for (const point of points) {
      expect(Math.hypot(point.x, point.y)).toBeCloseTo(radius, 9);
    }
  });
});
