import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicPoint } from "../placement";

import { isSideReachedByCubic } from "./is-side-reached-by-cubic";

import type { Cubic, Segment } from "../contour";

/** The arch of `DERIV-box-overlap`: y = −120 t (1 − t), x = 300 t² − 200 t³, its top at t = ½. */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

/** An S from (0, 0) to (100, 100), its handles crossed: x and y both turn. */
const WAVE: Cubic = {
  end: { x: 100, y: 100 },
  first: { x: 150, y: -50 },
  kind: "cubic",
  second: { x: -50, y: 150 },
  start: { x: 0, y: 0 },
};

/** How far a side reaches on each side of a point of the curve. */
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

describe("isSideReachedByCubic (DERIV-box-overlap step 6)", () => {
  it("reaches a side near either crossing, not one between them", () => {
    // On y = −15: x ≈ 5.80583 and 94.19417; [10, 90] holds neither.
    expect(isSideReachedByCubic(ARCH, side(-15, 0, 10))).toBe(true);
    expect(isSideReachedByCubic(ARCH, side(-15, 10, 90))).toBe(false);
    expect(isSideReachedByCubic(ARCH, side(-15, 90, 100))).toBe(true);
  });

  it("reaches a side at the top, where y turns, and misses one above it", () => {
    // The double root t = ½: (50, −30); −35 is above it.
    expect(isSideReachedByCubic(ARCH, side(-30, 40, 60))).toBe(true);
    expect(isSideReachedByCubic(ARCH, side(-35, 0, 100))).toBe(false);
  });
});

describe("isSideReachedByCubic (properties)", () => {
  test.prop({ t: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "reaches a short side about any point of a curve turning on both axes",
    ({ t }) => {
      const point = cubicPoint(WAVE, t);

      expect(isSideReachedByCubic(WAVE, side(point.y, point.x - REACH, point.x + REACH))).toBe(
        true,
      );
    },
  );
});
