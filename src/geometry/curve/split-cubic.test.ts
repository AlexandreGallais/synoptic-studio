import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";
import { cubicPoint } from "../placement";

import { splitCubic } from "./split-cubic";

import type { Cubic } from "../contour";

/** A cubic with integer points. */
const CUBIC = fc
  .tuple(integerPoint(1e3), integerPoint(1e3), integerPoint(1e3), integerPoint(1e3))
  .map(([start, first, second, end]): Cubic => ({ end, first, kind: "cubic", second, start }));

describe("splitCubic (REF-WIKI-DE-CASTELJAU, DERIV-cubic-sides step 4)", () => {
  it("[F12.AC3] splits the arch at (50, −30)", () => {
    const arch: Cubic = {
      end: { x: 100, y: 0 },
      first: { x: 0, y: -40 },
      kind: "cubic",
      second: { x: 100, y: -40 },
      start: { x: 0, y: 0 },
    };

    expect(splitCubic(arch, 0.5)).toEqual([
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
      const [first, second] = splitCubic(cubic, 0.5);
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

  test.prop({
    cubic: CUBIC,
    s: fc.double({ max: 1, min: 0, noNaN: true }),
    t: fc.double({ max: 0.99, min: 0.01, noNaN: true }),
  })("lies on the curve at any t: the first piece at s is the curve at s t", ({ cubic, s, t }) => {
    const [first, second] = splitCubic(cubic, t);

    expect(cubicPoint(first, s).x).toBeCloseTo(cubicPoint(cubic, s * t).x, 6);
    expect(cubicPoint(first, s).y).toBeCloseTo(cubicPoint(cubic, s * t).y, 6);
    expect(cubicPoint(second, s).x).toBeCloseTo(cubicPoint(cubic, t + s * (1 - t)).x, 6);
    expect(cubicPoint(second, s).y).toBeCloseTo(cubicPoint(cubic, t + s * (1 - t)).y, 6);
  });

  it("cuts the straight cubic at thirds at a fifth of its length for t = 0.2", () => {
    // B(t) = (100, 50t): at 0.2, (100, 10); the piece after keeps its end (100, 50).
    const straight: Cubic = {
      end: { x: 100, y: 50 },
      first: { x: 100, y: 50 / 3 },
      kind: "cubic",
      second: { x: 100, y: 100 / 3 },
      start: { x: 100, y: 0 },
    };
    const [, after] = splitCubic(straight, 0.2);

    expect(after.start.y).toBeCloseTo(10, 12);
    expect(after.end).toEqual({ x: 100, y: 50 });
  });
});
