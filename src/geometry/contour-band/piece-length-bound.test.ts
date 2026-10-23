import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { distance } from "../../math";

import { pieceLengthBound } from "./piece-length-bound";
import { pieceSamples } from "./piece-samples";

import type { Arc, Cubic } from "../contour";

/** The quarter of the circle of radius 10 about the origin, from (10, 0) down to (0, 10). */
const QUARTER: Arc = {
  center: { x: 0, y: 0 },
  end: { x: 0, y: 10 },
  kind: "arc",
  positive: true,
  radius: 10,
  start: { x: 10, y: 0 },
};

describe("pieceLengthBound (DERIV-band-within-box step 4)", () => {
  it("gives a segment its length", () => {
    // √(3² + 4²) = 5.
    expect(pieceLengthBound({ end: { x: 3, y: 4 }, kind: "segment", start: { x: 0, y: 0 } })).toBe(
      5,
    );
  });

  it("gives a quarter circle of radius 10 its length, 10 · π / 2", () => {
    expect(pieceLengthBound(QUARTER)).toBeCloseTo(15.707963268, 9);
  });

  it("gives the same quarter turned the other way the same length", () => {
    // From (0, 10) back to (10, 0): the signed angle is −π / 2, its size π / 2.
    const back: Arc = { ...QUARTER, end: QUARTER.start, positive: false, start: QUARTER.end };

    expect(pieceLengthBound(back)).toBeCloseTo(15.707963268, 9);
  });

  it("gives R's corner, a quarter of radius 49 999.5, 78 539.03", () => {
    // 49 999.5 · π / 2 = 78 539.030 94.
    const corner: Arc = {
      center: { x: 49_999.5, y: 49_999.5 },
      end: { x: 49_999.5, y: 0 },
      kind: "arc",
      positive: true,
      radius: 49_999.5,
      start: { x: 0, y: 49_999.5 },
    };

    expect(pieceLengthBound(corner)).toBeCloseTo(78_539.03094, 4);
  });

  it("gives a cubic three times its control polygon's longest side", () => {
    // Sides (0, 0)–(3, 0) 3, (3, 0)–(3, 4) 4, (3, 4)–(10, 4) 7: 3 · 7 = 21.
    const cubic: Cubic = {
      end: { x: 10, y: 4 },
      first: { x: 3, y: 0 },
      kind: "cubic",
      second: { x: 3, y: 4 },
      start: { x: 0, y: 0 },
    };

    expect(pieceLengthBound(cubic)).toBe(21);
  });

  it("gives 0 for a point: a segment, an arc or a cubic of no length", () => {
    const point = { x: 5, y: 5 };

    expect(pieceLengthBound({ end: point, kind: "segment", start: point })).toBe(0);

    expect(
      pieceLengthBound({
        center: point,
        end: point,
        kind: "arc",
        positive: true,
        radius: 0,
        start: point,
      }),
    ).toBe(0);

    expect(
      pieceLengthBound({ end: point, first: point, kind: "cubic", second: point, start: point }),
    ).toBe(0);
  });
});

describe("pieceLengthBound (properties)", () => {
  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 100, min: -100 }),
    y: fc.integer({ max: 100, min: -100 }),
  });

  test.prop({
    points: fc.tuple(point, point, point, point),
    step: fc.integer({ max: 20, min: 1 }),
  })(
    "bounds a cubic's chords: n of them at equal steps of t each at most the bound over n",
    ({ points, step }) => {
      const [start, first, second, end] = points;
      const cubic: Cubic = { end, first, kind: "cubic", second, start };
      const samples = pieceSamples(cubic, step);
      const count = samples.length - 1;
      const bound = pieceLengthBound(cubic);

      for (const [index, sample] of samples.slice(1).entries()) {
        expect(distance(samples[index] ?? sample, sample)).toBeLessThanOrEqual(
          bound / count + 1e-9,
        );
      }
    },
  );

  test.prop({
    angle: fc.double({ max: 3, min: 0.01, noNaN: true }),
    radius: fc.integer({ max: 500, min: 1 }),
  })("gives an arc a length at least its chord, r · θ", ({ angle, radius }) => {
    const arc: Arc = {
      center: { x: 0, y: 0 },
      end: { x: radius * Math.cos(angle), y: radius * Math.sin(angle) },
      kind: "arc",
      positive: true,
      radius,
      start: { x: radius, y: 0 },
    };

    expect(pieceLengthBound(arc)).toBeCloseTo(radius * angle, 6);
    expect(pieceLengthBound(arc)).toBeGreaterThanOrEqual(distance(arc.start, arc.end) - 1e-9);
  });
});
