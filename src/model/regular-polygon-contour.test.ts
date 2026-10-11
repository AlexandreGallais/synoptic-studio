import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { boundingBox } from "../geometry";
import { EPSILON } from "../math";

import { regularPolygonContour } from "./regular-polygon-contour";

import type { Anchor } from "../geometry";
import type { Point } from "../math";

/** F02's placement: centered on both axes (F07.AC1). */
const CENTER: Anchor = { horizontal: "mid", vertical: "mid" };

/** Decimals compared: the 5 of the SVG output (Q10), and more. */
const DECIMALS = 9;

/**
 * Checks two lists of points coordinate by coordinate.
 *
 * @param actual - computed points
 * @param expected - hand-computed points (DERIV-regular-polygon-fit, checks)
 */
function expectPoints(actual: readonly Point[], expected: readonly Point[]): void {
  expect(actual).toHaveLength(expected.length);

  for (const [index, point] of expected.entries()) {
    expect(actual[index]?.x).toBeCloseTo(point.x, DECIMALS);
    expect(actual[index]?.y).toBeCloseTo(point.y, DECIMALS);
  }
}

/** √3 / 2 · 100 = 86.60254…: height of the triangle and the hexagon fitted in 100 × 100. */
const HEIGHT_3 = 50 * Math.sqrt(3);

describe("regularPolygonContour (DERIV-regular-polygon-fit, checks)", () => {
  it("[F02.AC1] fits a triangle in 100 × 100: width reached, centered vertically", () => {
    // s = 100/√3, height 1.5 s = 86.60254, top margin (100 − 86.60254)/2 = 6.69873.
    const margin = (100 - HEIGHT_3) / 2;

    expectPoints(
      regularPolygonContour({ anchor: CENTER, corners: 3, height: 100, radius: 0, width: 100 }),
      [
        { x: 50, y: margin },
        { x: 100, y: 100 - margin },
        { x: 0, y: 100 - margin },
      ],
    );
  });

  it("[F02.AC1] fits a square in 100 × 50: height reached, not width", () => {
    // s = min(100/√2, 50/√2): a 50 × 50 square, left margin 25.
    expectPoints(
      regularPolygonContour({ anchor: CENTER, corners: 4, height: 50, radius: 0, width: 100 }),
      [
        { x: 25, y: 0 },
        { x: 75, y: 0 },
        { x: 75, y: 50 },
        { x: 25, y: 50 },
      ],
    );
  });

  it("[F02.AC3] fits a hexagon in 100 × 100 with flat top and bottom, from the top-left vertex", () => {
    // s = 50: 100 wide, 50√3 = 86.60254 high; top and bottom edges at 6.69873 and 93.30127.
    const margin = (100 - HEIGHT_3) / 2;

    expectPoints(
      regularPolygonContour({ anchor: CENTER, corners: 6, height: 100, radius: 0, width: 100 }),
      [
        { x: 25, y: margin },
        { x: 75, y: margin },
        { x: 100, y: 50 },
        { x: 75, y: 100 - margin },
        { x: 25, y: 100 - margin },
        { x: 0, y: 50 },
      ],
    );
  });

  it("[F02.AC3] fits a pentagon in 100 × 100 from its apex (odd n above 3, AUD-002)", () => {
    // Unit span 2 cos(π/10) × (1 + cos(π/5)); s = 100 / (2 cos(π/10)) = 52.57311: width reached.
    const s = 50 / Math.cos(Math.PI / 10);
    const top = (100 - s * (1 + Math.cos(Math.PI / 5))) / 2;

    expectPoints(
      regularPolygonContour({ anchor: CENTER, corners: 5, height: 100, radius: 0, width: 100 }),
      [
        { x: 50, y: top },
        { x: 100, y: top + s * (1 - Math.sin(Math.PI / 10)) },
        { x: 50 + s * Math.sin(Math.PI / 5), y: top + s * (1 + Math.cos(Math.PI / 5)) },
        { x: 50 - s * Math.sin(Math.PI / 5), y: top + s * (1 + Math.cos(Math.PI / 5)) },
        { x: 0, y: top + s * (1 - Math.sin(Math.PI / 10)) },
      ],
    );
  });

  it("[F02.AC1] fits a polygon of 12 corners, the largest allowed (Q19)", () => {
    // n = 12 is divisible by 4: flat sides on all four axes, both sizes reached.
    const box = boundingBox(
      regularPolygonContour({ anchor: CENTER, corners: 12, height: 100, radius: 0, width: 100 }),
    );

    expect(box.maxX - box.minX).toBeCloseTo(100, DECIMALS);
    expect(box.maxY - box.minY).toBeCloseTo(100, DECIMALS);
  });

  it("[F02.AC1] fits an octagon in 100 × 100: both sizes reached", () => {
    // Unit span 2 cos(π/8) on both axes; first vertex at x = 50 − 50 tan(π/8) = 29.28932.
    const contour = regularPolygonContour({
      anchor: CENTER,
      corners: 8,
      height: 100,
      radius: 0,
      width: 100,
    });
    const box = boundingBox(contour);

    expect(contour[0]?.x).toBeCloseTo(50 - 50 * Math.tan(Math.PI / 8), DECIMALS);
    expect(contour[0]?.y).toBeCloseTo(0, DECIMALS);
    expect(box.maxX - box.minX).toBeCloseTo(100, DECIMALS);
    expect(box.maxY - box.minY).toBeCloseTo(100, DECIMALS);
  });

  it("collapses a polygon of width 0 to the center of its box (Q15)", () => {
    expect(
      regularPolygonContour({ anchor: CENTER, corners: 5, height: 40, radius: 0, width: 0 }),
    ).toEqual(Array.from({ length: 5 }, () => ({ x: 0, y: 20 })));
  });

  it("collapses a polygon of height 0 to the middle of its top edge (Q15)", () => {
    expect(
      regularPolygonContour({ anchor: CENTER, corners: 3, height: 0, radius: 0, width: 40 }),
    ).toEqual(Array.from({ length: 3 }, () => ({ x: 20, y: 0 })));
  });

  it("derives the vertices without changing the model (ADR-0003)", () => {
    const polygon = Object.freeze({
      anchor: CENTER,
      corners: 6,
      height: 100,
      radius: 10,
      width: 100,
    });

    expect(regularPolygonContour(polygon)).toEqual(regularPolygonContour(polygon));
    expect(polygon).toEqual({ anchor: CENTER, corners: 6, height: 100, radius: 10, width: 100 });
  });
});

describe("regularPolygonContour (anchored, F07)", () => {
  it("[F07.AC1] draws F02's triangle when centered on both axes", () => {
    // Top margin (100 − 86.60254)/2 = 6.69873, as F02.
    const margin = (100 - HEIGHT_3) / 2;

    expectPoints(
      regularPolygonContour({ anchor: CENTER, corners: 3, height: 100, radius: 0, width: 100 }),
      [
        { x: 50, y: margin },
        { x: 100, y: 100 - margin },
        { x: 0, y: 100 - margin },
      ],
    );
  });

  it("[F07.AC2] puts the triangle's base on the bottom of 100 × 100: apex at 13.39746", () => {
    // Room 100 − 86.60254 = 13.39746, all of it above the triangle.
    const anchor = { horizontal: "mid", vertical: "max" } as const;

    expectPoints(
      regularPolygonContour({ anchor, corners: 3, height: 100, radius: 0, width: 100 }),
      [
        { x: 50, y: 100 - HEIGHT_3 },
        { x: 100, y: 100 },
        { x: 0, y: 100 },
      ],
    );
  });

  it("[F07.AC2] puts the hexagon's base on the bottom of 100 × 100 (DERIV check row)", () => {
    // Shifted down by 6.69873 from its centered place: (25, 13.39746) … (0, 56.69873).
    const anchor = { horizontal: "mid", vertical: "max" } as const;
    const top = 100 - HEIGHT_3;

    expectPoints(
      regularPolygonContour({ anchor, corners: 6, height: 100, radius: 0, width: 100 }),
      [
        { x: 25, y: top },
        { x: 75, y: top },
        { x: 100, y: top + HEIGHT_3 / 2 },
        { x: 75, y: 100 },
        { x: 25, y: 100 },
        { x: 0, y: top + HEIGHT_3 / 2 },
      ],
    );
  });

  it("[F07.AC2] puts the square on the right of 100 × 50", () => {
    // Room 100 − 50 = 50, all of it on the left of the square.
    const anchor = { horizontal: "max", vertical: "mid" } as const;

    expectPoints(regularPolygonContour({ anchor, corners: 4, height: 50, radius: 0, width: 100 }), [
      { x: 50, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 50, y: 50 },
    ]);
  });

  it("[F07.AC3] keeps the triangle in place when only its horizontal alignment changes, the anchor still stored", () => {
    // The width is reached: no room on that axis.
    const left = Object.freeze({
      anchor: { horizontal: "min", vertical: "max" },
      corners: 3,
      height: 100,
      radius: 0,
      width: 100,
    } as const);
    const right = { ...left, anchor: { horizontal: "max", vertical: "max" } } as const;

    expectPoints(regularPolygonContour(left), regularPolygonContour(right));
    expect(left.anchor).toEqual({ horizontal: "min", vertical: "max" });
  });
});

describe("regularPolygonContour (properties)", () => {
  test.prop({
    corners: fc.integer({ max: 12, min: 3 }),
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F02.AC1] stays in its box, centered, reaching its width or its height",
    ({ corners, height, width }) => {
      const box = boundingBox(
        regularPolygonContour({ anchor: CENTER, corners, height, radius: 0, width }),
      );
      const tolerance = EPSILON * Math.max(width, height);

      expect(box.minX).toBeGreaterThanOrEqual(-tolerance);
      expect(box.minY).toBeGreaterThanOrEqual(-tolerance);
      expect(box.maxX).toBeLessThanOrEqual(width + tolerance);
      expect(box.maxY).toBeLessThanOrEqual(height + tolerance);
      expect(Math.abs(box.minX + box.maxX - width)).toBeLessThan(2 * tolerance);
      expect(Math.abs(box.minY + box.maxY - height)).toBeLessThan(2 * tolerance);

      expect(
        Math.abs(box.maxX - box.minX - width) < tolerance ||
          Math.abs(box.maxY - box.minY - height) < tolerance,
      ).toBe(true);
    },
  );
});
