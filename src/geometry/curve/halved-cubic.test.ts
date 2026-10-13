import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { cubicPoint } from "../placement";

import { halvedCubic } from "./halved-cubic";

import type { Cubic } from "../contour";

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(1e3), integerPoint(1e3), integerPoint(1e3), integerPoint(1e3))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

describe("halvedCubic (REF-POMAX-BEZIER, DERIV-cubic-sides step 4)", () => {
  it("[F12.AC3] splits the arch at (50, −30)", () => {
    const arch: Cubic = {
      end: { x: 100, y: 0 },
      first: { x: 0, y: -40 },
      kind: "cubic",
      second: { x: 100, y: -40 },
      start: { x: 0, y: 0 },
    };

    expect(halvedCubic(arch)).toEqual([
      {
        end: { x: 50, y: -30 },
        first: { x: 0, y: -20 },
        kind: "cubic",
        second: { x: 25, y: -30 },
        start: { x: 0, y: 0 },
      },
      {
        end: { x: 100, y: 0 },
        first: { x: 75, y: -30 },
        kind: "cubic",
        second: { x: 100, y: -20 },
        start: { x: 50, y: -30 },
      },
    ]);
  });

  test.prop({ cubic: CUBIC, s: fc.double({ max: 1, min: 0, noNaN: true }) })(
    "lies on the curve: the first half at s is the curve at s / 2, the second at (1 + s) / 2",
    ({ cubic, s }) => {
      const [first, second] = halvedCubic(cubic);
      const pairs = [
        [cubicPoint(first, s), cubicPoint(cubic, s / 2)],
        [cubicPoint(second, s), cubicPoint(cubic, (1 + s) / 2)],
      ] as const;

      for (const [half, whole] of pairs) {
        expect(half.x).toBeCloseTo(whole.x, 6);
        expect(half.y).toBeCloseTo(whole.y, 6);
      }
    },
  );
});
