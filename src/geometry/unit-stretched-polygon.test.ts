import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints } from "../../tests";

import { edgeLengths } from "./edge-lengths";
import { unitStretchedPolygon } from "./unit-stretched-polygon";

/** Decimals compared: sums of cosines and sines of doubles. */
const DECIMALS = 12;

/** Every number of corners of a polygon (Q19), each with a few vertical scales. */
const CASES = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12].flatMap((corners) =>
  [0.8, 1, 1.25].map((scale) => ({ corners, scale })),
);

describe("unitStretchedPolygon (DERIV-stretched-polygon step 6)", () => {
  it("gives the unit square from its bottom-right vertex, clockwise on screen, its box at the origin", () => {
    expectPoints(
      unitStretchedPolygon(4, 1),
      [
        { x: 1, y: 1 },
        { x: 0, y: 1 },
        { x: 0, y: 0 },
        { x: 1, y: 0 },
      ],
      DECIMALS,
    );
  });

  it("gives the triangle's apex once, on the axis", () => {
    // Base from 0 to 1 at y = √3/2, apex (1/2, 0).
    expectPoints(
      unitStretchedPolygon(3, 1),
      [
        { x: 1, y: Math.sqrt(3) / 2 },
        { x: 0, y: Math.sqrt(3) / 2 },
        { x: 0.5, y: 0 },
      ],
      DECIMALS,
    );
  });

  it("keeps every side of length 1 and the base centered, for every n and a few λ", () => {
    for (const { corners, scale } of CASES) {
      const polygon = unitStretchedPolygon(corners, scale);
      const width = Math.max(...polygon.map((point) => point.x));
      const base = polygon.slice(0, 2).reduce((sum, point) => sum + point.x, 0);

      expect(polygon).toHaveLength(corners);
      expect(edgeLengths(polygon).every((length) => Math.abs(length - 1) < 1e-12)).toBe(true);
      expect(base).toBeCloseTo(width, DECIMALS);
    }
  });

  test.prop([fc.integer({ max: 12, min: 3 }), fc.double({ max: 3, min: -3, noNaN: true })])(
    "closes with sides of length 1 for every λ where it closes (DERIV-stretched-polygon step 2)",
    (corners, log) => {
      const lengths = edgeLengths(unitStretchedPolygon(corners, Math.exp(log)));

      // An odd polygon does not close at some small λ: NaN coordinates, then NaN lengths.
      expect(lengths.every((length) => Number.isNaN(length) || Math.abs(length - 1) < 1e-9)).toBe(
        true,
      );
    },
  );
});
