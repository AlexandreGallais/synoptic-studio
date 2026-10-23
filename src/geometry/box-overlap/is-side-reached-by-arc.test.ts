import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isSideReachedByArc } from "./is-side-reached-by-arc";

import type { Arc, Segment } from "../contour";

/** The quarter from the left (0, 50) to the top (50, 0) of the circle about (50, 50), positive. */
const QUARTER: Arc = {
  center: { x: 50, y: 50 },
  end: { x: 50, y: 0 },
  kind: "arc",
  positive: true,
  radius: 50,
  start: { x: 0, y: 50 },
};

/** How far a side reaches on each side of a point of the arc. */
const REACH = 0.01;

/**
 * A horizontal side.
 *
 * @param level - its ordinate
 * @param from - its left abscissa
 * @param to - its right abscissa
 * @returns its start and its end
 */
function side(level: number, from: number, to: number): Pick<Segment, "end" | "start"> {
  return { end: { x: to, y: level }, start: { x: from, y: level } };
}

describe("isSideReachedByArc (DERIV-box-overlap step 7)", () => {
  it("reaches a side through its point on the line", () => {
    // On y = 25: x ≈ 6.69873, on the arc, in [0, 10].
    expect(isSideReachedByArc(QUARTER, side(25, 0, 10))).toBe(true);
  });

  it("misses a side through the circle's other point, off the arc", () => {
    // x ≈ 93.30127 is on the circle, beyond the chord on the centre's side.
    expect(isSideReachedByArc(QUARTER, side(25, 90, 100))).toBe(false);
  });

  it("misses a side below the arc's start", () => {
    // On y = 60, h = 10, x = 50 ∓ √2400 ≈ 1.01021, 98.98979: (1.01021, 60) gives
    // 50 · 10 + 50 · 1.01021 > 0, off the arc; 98.98979 is outside [0, 10].
    expect(isSideReachedByArc(QUARTER, side(60, 0, 10))).toBe(false);
  });

  it("misses a side beyond the circle", () => {
    // |−10 − 50| = 60 > 50.
    expect(isSideReachedByArc(QUARTER, side(-10, 0, 100))).toBe(false);
  });

  it("reaches a side at the arc's end, the circle's top", () => {
    // y = 0 touches the circle at (50, 0), the arc's end.
    expect(isSideReachedByArc(QUARTER, side(0, 40, 60))).toBe(true);
  });
});

describe("isSideReachedByArc (properties)", () => {
  test.prop({
    angle: fc.double({ max: (3 * Math.PI) / 2 - 1e-6, min: Math.PI + 1e-6, noNaN: true }),
  })(
    "reaches a short side about any point of the arc between its ends, which isPointInBox takes",
    ({ angle }) => {
      const point = { x: 50 + 50 * Math.cos(angle), y: 50 + 50 * Math.sin(angle) };

      expect(isSideReachedByArc(QUARTER, side(point.y, point.x - REACH, point.x + REACH))).toBe(
        true,
      );
    },
  );
});
