import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { pieceSamples } from "./piece-samples";

import type { Point } from "../../math";
import type { Arc, Cubic } from "../contour";

/**
 * Asserts that points are close to the expected ones, one coordinate at a time.
 *
 * @param points - what `pieceSamples` gave
 * @param expected - the values computed by hand
 */
function expectPointsClose(points: readonly Point[], expected: readonly Point[]): void {
  expect(points).toHaveLength(expected.length);

  for (const [index, point] of expected.entries()) {
    expect(points[index]?.x).toBeCloseTo(point.x, 9);
    expect(points[index]?.y).toBeCloseTo(point.y, 9);
  }
}

/** The quarter of the circle of radius 10 about the origin, from (10, 0) down to (0, 10). */
const QUARTER: Arc = {
  center: { x: 0, y: 0 },
  end: { x: 0, y: 10 },
  kind: "arc",
  positive: true,
  radius: 10,
  start: { x: 10, y: 0 },
};

describe("pieceSamples", () => {
  it("gives a segment's two ends, whatever the step", () => {
    expect(
      pieceSamples({ end: { x: 10, y: 0 }, kind: "segment", start: { x: 0, y: 0 } }, 2),
    ).toEqual([
      { x: 0, y: 0 },
      { x: 10, y: 0 },
    ]);
  });

  it("cuts a quarter arc of radius 10 into 4 steps of π/8 for a step of 4", () => {
    // Its length 10 · π/2 = 15.708; 15.708 / 4 = 3.93, 4 steps of π/8 = 22.5°:
    // (10 cos 22.5°, 10 sin 22.5°) = (9.2388, 3.8268), (7.0711, 7.0711), (3.8268, 9.2388).
    const points = pieceSamples(QUARTER, 4);

    expectPointsClose(points, [
      { x: 10, y: 0 },
      { x: 9.238795325, y: 3.826834324 },
      { x: 7.071067812, y: 7.071067812 },
      { x: 3.826834324, y: 9.238795325 },
      { x: 0, y: 10 },
    ]);

    // Its ends exactly.
    expect(points[0]).toEqual({ x: 10, y: 0 });
    expect(points.at(-1)).toEqual({ x: 0, y: 10 });
  });

  it("turns a negative arc the other way: from (0, 10) up to (10, 0) by −π/4 twice", () => {
    // Its length 15.708, a step of 10: 2 steps of −45°, the middle (7.0711, 7.0711).
    const arc: Arc = { ...QUARTER, end: QUARTER.start, positive: false, start: QUARTER.end };

    expectPointsClose(pieceSamples(arc, 10), [
      { x: 0, y: 10 },
      { x: 7.071067812, y: 7.071067812 },
      { x: 10, y: 0 },
    ]);
  });

  it("cuts a cubic by its control polygon: 24 long, a step of 12, its point at t = ½", () => {
    // |P0P1| + |P1P2| + |P2P3| = 8 + 8 + 8 = 24, 2 steps; B(½) = (P0 + 3P1 + 3P2 + P3) / 8 =
    // ((0 + 0 + 24 + 8) / 8, (0 + 24 + 24 + 0) / 8) = (4, 6).
    const cubic: Cubic = {
      end: { x: 8, y: 0 },
      first: { x: 0, y: 8 },
      kind: "cubic",
      second: { x: 8, y: 8 },
      start: { x: 0, y: 0 },
    };

    expect(pieceSamples(cubic, 12)).toEqual([
      { x: 0, y: 0 },
      { x: 4, y: 6 },
      { x: 8, y: 0 },
    ]);
  });

  it("gives a cubic of length 0 its two ends, one step", () => {
    const point = { x: 3, y: 3 };
    const cubic: Cubic = { end: point, first: point, kind: "cubic", second: point, start: point };

    expect(pieceSamples(cubic, 2)).toEqual([point, point]);
  });
});

describe("pieceSamples (properties)", () => {
  test.prop({
    angle: fc.double({ max: 3, min: 0.01, noNaN: true }),
    isPositive: fc.boolean(),
    radius: fc.integer({ max: 500, min: 1 }),
    step: fc.integer({ max: 20, min: 1 }),
  })(
    "keeps an arc's points on its circle, its ends exact, its steps along it at most `step`",
    ({ angle, isPositive, radius, step }) => {
      const turn = isPositive ? angle : -angle;
      const arc: Arc = {
        center: { x: 0, y: 0 },
        end: { x: radius * Math.cos(turn), y: radius * Math.sin(turn) },
        kind: "arc",
        positive: isPositive,
        radius,
        start: { x: radius, y: 0 },
      };
      const points = pieceSamples(arc, step);

      expect(points[0]).toEqual(arc.start);
      expect(points.at(-1)).toEqual(arc.end);

      for (const sample of points) {
        expect(Math.hypot(sample.x, sample.y)).toBeCloseTo(radius, 6);
      }

      // Each step is a chord of an angle (n steps of θ / n): its arc, r · θ / n, is at most `step`.
      expect((radius * angle) / (points.length - 1)).toBeLessThanOrEqual(step + 1e-9);
    },
  );

  /** A point on integers of a drawing. */
  const point = fc.record({
    x: fc.integer({ max: 100, min: -100 }),
    y: fc.integer({ max: 100, min: -100 }),
  });

  test.prop({
    points: fc.tuple(point, point, point, point),
    step: fc.integer({ max: 20, min: 1 }),
  })("starts a cubic at its start, ends it at its end, at least one step", ({ points, step }) => {
    const [start, first, second, end] = points;
    const samples = pieceSamples({ end, first, kind: "cubic", second, start }, step);

    expect(samples[0]).toEqual(start);
    expect(samples.at(-1)).toEqual(end);
    expect(samples.length).toBeGreaterThanOrEqual(2);
  });
});
