import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicPoint } from "../placement";

import { isSideReachedByMonotonePart } from "./is-side-reached-by-monotone-part";

import type { Cubic, Segment } from "../contour";

/** The arch of `DERIV-box-overlap`: y = −120 t (1 − t), x = 300 t² − 200 t³, its top at t = ½. */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
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

describe("isSideReachedByMonotonePart (DERIV-box-overlap step 6)", () => {
  it("reaches a side on the part that crosses it there, not on the other part", () => {
    // On y = −15, t (1 − t) = ⅛: t ≈ 0.14645 on [0, ½], x ≈ 5.80583, in [0, 10]; t ≈ 0.85355 on
    // [½, 1], x ≈ 94.19417, not in [0, 10].
    expect(isSideReachedByMonotonePart(ARCH, [0, 0.5], side(-15, 0, 10))).toBe(true);
    expect(isSideReachedByMonotonePart(ARCH, [0.5, 1], side(-15, 0, 10))).toBe(false);
  });

  it("reaches a side at the part's end, the arch's top", () => {
    // y(½) = −30, x(½) = 75 − 25 = 50, in [40, 60].
    expect(isSideReachedByMonotonePart(ARCH, [0, 0.5], side(-30, 40, 60))).toBe(true);
  });

  it("misses a side beyond the part's ordinates", () => {
    // −35 is above the top −30.
    expect(isSideReachedByMonotonePart(ARCH, [0, 0.5], side(-35, 0, 100))).toBe(false);
  });

  it("leaves out a part of constant y", () => {
    // Every point at y = 0: its start and the vertical sides tell (step 8).
    const flat: Cubic = {
      end: { x: 100, y: 0 },
      first: { x: 30, y: 0 },
      kind: "cubic",
      second: { x: 70, y: 0 },
      start: { x: 0, y: 0 },
    };

    expect(isSideReachedByMonotonePart(flat, [0, 1], side(0, 0, 100))).toBe(false);
  });
});

describe("isSideReachedByMonotonePart (properties)", () => {
  test.prop({ t: fc.double({ max: 0.49, min: 0.01, noNaN: true }) })(
    "reaches a short side about any point of the rising part",
    ({ t }) => {
      const point = cubicPoint(ARCH, t);

      expect(
        isSideReachedByMonotonePart(
          ARCH,
          [0, 0.5],
          side(point.y, point.x - REACH, point.x + REACH),
        ),
      ).toBe(true);
    },
  );
});
