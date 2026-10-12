import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { filletArc } from "./fillet-arc";

import type { Point } from "../math";

/** Integer point. */
const POINT = fc.record({
  x: fc.integer({ max: 500, min: -500 }),
  y: fc.integer({ max: 500, min: -500 }),
});

/** Decimals compared: tangents and square roots of doubles. */
const DECIMALS = 9;

/**
 * Checks a computed point against hand-computed coordinates.
 *
 * @param actual - computed point
 * @param expected - coordinates from the DERIV-fillet-arc check table
 */
function expectPoint(actual: Point, expected: Point): void {
  expect(actual.x).toBeCloseTo(expected.x, DECIMALS);
  expect(actual.y).toBeCloseTo(expected.y, DECIMALS);
}

describe("filletArc (DERIV-fillet-arc)", () => {
  it.each([
    // Rectangle 100 × 50, radius 10, s = 10 at every corner.
    {
      center: { x: 10, y: 10 },
      corner: { next: { x: 100, y: 0 }, previous: { x: 0, y: 50 }, vertex: { x: 0, y: 0 } },
      end: { x: 10, y: 0 },
      start: { x: 0, y: 10 },
    },
    {
      center: { x: 90, y: 10 },
      corner: { next: { x: 100, y: 50 }, previous: { x: 0, y: 0 }, vertex: { x: 100, y: 0 } },
      end: { x: 100, y: 10 },
      start: { x: 90, y: 0 },
    },
    {
      center: { x: 90, y: 40 },
      corner: { next: { x: 0, y: 50 }, previous: { x: 100, y: 0 }, vertex: { x: 100, y: 50 } },
      end: { x: 90, y: 50 },
      start: { x: 100, y: 40 },
    },
    {
      center: { x: 10, y: 40 },
      corner: { next: { x: 0, y: 0 }, previous: { x: 100, y: 50 }, vertex: { x: 0, y: 50 } },
      end: { x: 0, y: 40 },
      start: { x: 10, y: 50 },
    },
  ])(
    "rounds the rectangle corner at $corner.vertex.x, $corner.vertex.y in the positive direction",
    (row) => {
      const arc = filletArc(row.corner, 10);

      expectPoint(arc.start, row.start);
      expectPoint(arc.end, row.end);
      expectPoint(arc.center, row.center);
      expect(arc.radius).toBe(10);
      expect(arc.positive).toBe(true);
    },
  );

  it("rounds a concave corner in the negative direction, center on the other side", () => {
    // u_in = (1, 0), u_out = (0, −1): τ = −π/2, s = 5, C = (5, 0) − 5 · (0, 1) = (5, −5).
    const arc = filletArc(
      { next: { x: 10, y: -10 }, previous: { x: 0, y: 0 }, vertex: { x: 10, y: 0 } },
      5,
    );

    expectPoint(arc.start, { x: 5, y: 0 });
    expectPoint(arc.end, { x: 10, y: -5 });
    expectPoint(arc.center, { x: 5, y: -5 });
    expect(arc.positive).toBe(false);
  });

  it("degenerates to the vertex at an aligned vertex: no arc", () => {
    // τ = 0 → s = 0: T_in = T_out = V.
    const arc = filletArc(
      { next: { x: 100, y: 0 }, previous: { x: 0, y: 0 }, vertex: { x: 50, y: 0 } },
      30,
    );

    expect(arc.start).toEqual({ x: 50, y: 0 });
    expect(arc.end).toEqual({ x: 50, y: 0 });
    // Positive only when τ > 0; here τ = 0.
    expect(arc.positive).toBe(false);
  });

  it("degenerates to the vertex on a zero-length edge, without NaN (Q15)", () => {
    // Rectangle 0 × 50: the outgoing edge has no direction, τ = 0, s = 0.
    const arc = filletArc(
      { next: { x: 0, y: 0 }, previous: { x: 0, y: 50 }, vertex: { x: 0, y: 0 } },
      10,
    );

    expectPoint(arc.start, { x: 0, y: 0 });
    expectPoint(arc.end, { x: 0, y: 0 });
    expectPoint(arc.center, { x: 0, y: 0 });
  });

  it("keeps every center at the radius from both tangent points", () => {
    // Turn of π/3: s = 10 · tan(π/6) = 10 / √3; both distances to the center equal 10.
    const arc = filletArc(
      {
        next: { x: 150, y: 50 * Math.sqrt(3) },
        previous: { x: 0, y: 0 },
        vertex: { x: 100, y: 0 },
      },
      10,
    );

    expect(Math.hypot(arc.center.x - arc.start.x, arc.center.y - arc.start.y)).toBeCloseTo(
      10,
      DECIMALS,
    );

    expect(Math.hypot(arc.center.x - arc.end.x, arc.center.y - arc.end.y)).toBeCloseTo(
      10,
      DECIMALS,
    );

    expectPoint(arc.start, { x: 100 - 10 / Math.sqrt(3), y: 0 });
  });
});

describe("filletArc (properties)", () => {
  test.prop({
    next: POINT,
    previous: POINT,
    radius: fc.integer({ max: 100, min: 1 }),
    vertex: POINT,
  })(
    "puts the center at the radius from both tangent points",
    ({ next, previous, radius, vertex }) => {
      const turn = Math.atan2(
        (vertex.x - previous.x) * (next.y - vertex.y) -
          (vertex.y - previous.y) * (next.x - vertex.x),
        (vertex.x - previous.x) * (next.x - vertex.x) +
          (vertex.y - previous.y) * (next.y - vertex.y),
      );

      // Corners with a real turn, away from the spike of Q16.
      fc.pre(Math.abs(turn) > 0.01 && Math.abs(turn) < Math.PI - 0.1);

      const arc = filletArc({ next, previous, vertex }, radius);

      expect(Math.hypot(arc.center.x - arc.start.x, arc.center.y - arc.start.y)).toBeCloseTo(
        radius,
        6,
      );

      expect(Math.hypot(arc.center.x - arc.end.x, arc.center.y - arc.end.y)).toBeCloseTo(radius, 6);
    },
  );
});
