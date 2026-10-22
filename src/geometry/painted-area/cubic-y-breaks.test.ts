import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cubicYBreaks } from "./cubic-y-breaks";

import type { Cubic } from "../contour";

/** An arch from (0, 0) to (100, 0), 30 high: y = −120 t (1 − t) (`DERIV-cubic-sides`, checks). */
const ARCH: Cubic = {
  end: { x: 100, y: 0 },
  first: { x: 0, y: -40 },
  kind: "cubic",
  second: { x: 100, y: -40 },
  start: { x: 0, y: 0 },
};

/** A coordinate, an integer. */
const COORDINATE = fc.integer({ max: 1000, min: -1000 });

/** A point on integers. */
const POINT = fc.record({ x: COORDINATE, y: COORDINATE });

describe("cubicYBreaks (DERIV-painted-area step 4)", () => {
  it("cuts the arch at its top, t = ½", () => {
    // y′ = 240 t − 120 (a = 0, b = 240, c = −120): its root ½.
    expect(cubicYBreaks(ARCH)).toEqual([0, 0.5, 1]);
  });

  it("cuts an S at both its turns, (3 ∓ √3) / 6", () => {
    // y: 0, −30, 30, 0: a = −540, b = 540, c = −90, so 6t² − 6t + 1 = 0.
    const breaks = cubicYBreaks({
      end: { x: 100, y: 0 },
      first: { x: 30, y: -30 },
      kind: "cubic",
      second: { x: 70, y: 30 },
      start: { x: 0, y: 0 },
    });

    expect(breaks).toHaveLength(4);
    expect(breaks[1]).toBeCloseTo((3 - Math.sqrt(3)) / 6, 12);
    expect(breaks[2]).toBeCloseTo((3 + Math.sqrt(3)) / 6, 12);
  });

  it("keeps only its ends for a side whose y never turns", () => {
    // y: 0, 10, 20, 30: y′ = 30, no root.
    const straight: Cubic = {
      end: { x: 30, y: 30 },
      first: { x: 10, y: 10 },
      kind: "cubic",
      second: { x: 20, y: 20 },
      start: { x: 0, y: 0 },
    };

    expect(cubicYBreaks(straight)).toEqual([0, 1]);
  });

  it("keeps only its ends for four equal points", () => {
    // y′ = 0 everywhere: q = 0, the roots NaN, left out.
    const point = { x: 5, y: 5 };

    expect(
      cubicYBreaks({ end: point, first: point, kind: "cubic", second: point, start: point }),
    ).toEqual([0, 1]);
  });
});

describe("cubicYBreaks (properties)", () => {
  test.prop({ end: POINT, first: POINT, second: POINT, start: POINT })(
    "runs from 0 to 1 in increasing order, at most two breaks between",
    (points) => {
      const breaks = cubicYBreaks({ ...points, kind: "cubic" });

      expect(breaks.at(0)).toBe(0);
      expect(breaks.at(-1)).toBe(1);
      expect(breaks.length).toBeLessThanOrEqual(4);
      expect(breaks.toSorted((a, b) => a - b)).toEqual(breaks);
    },
  );
});
