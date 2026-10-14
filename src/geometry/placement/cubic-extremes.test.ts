import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints, integerPoint } from "../../../tests";

import { cubicExtremes } from "./cubic-extremes";
import { cubicPoint } from "./cubic-point";

import type { Cubic } from "../contour";

/**
 * A cubic from its four points.
 *
 * @param points - start, first control point, second, end, each as [x, y]
 * @returns the cubic piece
 */
function cubicOf(points: readonly (readonly [number, number])[]): Cubic {
  const [start, first, second, end] = points.map(([x, y]) => ({ x, y }));

  return {
    end: end ?? { x: 0, y: 0 },
    first: first ?? { x: 0, y: 0 },
    kind: "cubic",
    second: second ?? { x: 0, y: 0 },
    start: start ?? { x: 0, y: 0 },
  };
}

describe("cubicExtremes (DERIV-cubic-sides step 3)", () => {
  it("[F12.AC3] finds the top of the story's side at t = ⅓, y = −20/3", () => {
    // y: −135 t² + 180 t − 45 = 0 at t = ⅓ (and 1, an end); x has no root inside. At t = ⅓,
    // x = (8 · 100 + 12 · 130 + 6 · 200 + 200) / 27 = 3760 / 27 ≈ 139.25926, y = −20/3.
    expectPoints(
      cubicExtremes(
        cubicOf([
          [100, 0],
          [130, -15],
          [200, 0],
          [200, 0],
        ]),
      ),
      [{ x: 139.25926, y: -6.66667 }],
      5,
    );
  });

  it("finds the arch's top with a = 0, and the S's two extremes", () => {
    expectPoints(
      cubicExtremes(
        cubicOf([
          [0, 0],
          [0, -40],
          [100, -40],
          [100, 0],
        ]),
      ),
      [{ x: 50, y: -30 }],
      9,
    );

    // Roots (3 ∓ √3)/6: y = ∓5√3.
    const extremes = cubicExtremes(
      cubicOf([
        [0, 0],
        [30, -30],
        [70, 30],
        [100, 0],
      ]),
    );

    expect(extremes.map((point) => point.y).toSorted((a, b) => a - b)).toEqual([
      expect.closeTo(-5 * Math.sqrt(3), 9),
      expect.closeTo(5 * Math.sqrt(3), 9),
    ]);
  });

  it("finds an overshoot past the end, and nothing on a point", () => {
    // x: t = 6/7, x = 5400/49.
    expect(
      cubicExtremes(
        cubicOf([
          [0, 0],
          [0, 0],
          [150, 0],
          [100, 0],
        ]),
      )[0]?.x,
    ).toBeCloseTo(5400 / 49, 9);

    expect(
      cubicExtremes(
        cubicOf([
          [5, 5],
          [5, 5],
          [5, 5],
          [5, 5],
        ]),
      ),
    ).toEqual([]);
  });

  test.prop({
    points: fc.tuple(integerPoint(500), integerPoint(500), integerPoint(500), integerPoint(500)),
  })("gives a box around every point of the curve", ({ points }) => {
    const cubic = cubicOf(points.map(({ x, y }) => [x, y] as const));
    const boxPoints = [cubic.start, cubic.end, ...cubicExtremes(cubic)];
    const samples = Array.from({ length: 41 }, (unused, index) => cubicPoint(cubic, index / 40));

    for (const sample of samples) {
      expect(sample.x).toBeGreaterThanOrEqual(
        Math.min(...boxPoints.map((point) => point.x)) - 1e-6,
      );

      expect(sample.x).toBeLessThanOrEqual(Math.max(...boxPoints.map((point) => point.x)) + 1e-6);

      expect(sample.y).toBeGreaterThanOrEqual(
        Math.min(...boxPoints.map((point) => point.y)) - 1e-6,
      );

      expect(sample.y).toBeLessThanOrEqual(Math.max(...boxPoints.map((point) => point.y)) + 1e-6);
    }
  });
});
