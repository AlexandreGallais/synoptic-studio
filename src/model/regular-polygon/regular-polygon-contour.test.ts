import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { expectPoints } from "../../../tests";
import { ALIGNMENTS, boundingBox, stretchedPolygon } from "../../geometry";
import { EPSILON } from "../../math";
import { resizeRadii } from "../corner-radius";

import { regularPolygonContour } from "./regular-polygon-contour";
import { regularPolygonCorners } from "./regular-polygon-corners";

import type { Anchor } from "../../geometry";

/** F02's placement: centered on both axes (F07.AC1). */
const CENTER: Anchor = { horizontal: "mid", vertical: "mid" };

/** Decimals compared: the 5 of the SVG output (Q10), and more. */
const DECIMALS = 9;

/** √3 / 2 · 100 = 86.60254…: height of the triangle and the hexagon fitted in 100 × 100. */
const HEIGHT_3 = 50 * Math.sqrt(3);

describe("regularPolygonContour (DERIV-regular-polygon-fit, checks)", () => {
  it("[F02.AC1] fits a triangle in 100 × 100: width reached, centered vertically", () => {
    // s = 100/√3, height 1.5 s = 86.60254, top margin (100 − 86.60254)/2 = 6.69873.
    const margin = (100 - HEIGHT_3) / 2;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 3,
        height: 100,
        radii: [0, 0, 0],
        width: 100,
      }),
      [
        { x: 100, y: 100 - margin },
        { x: 0, y: 100 - margin },
        { x: 50, y: margin },
      ],
      DECIMALS,
    );
  });

  it("[F02.AC1] fits a square in 100 × 50: height reached, not width", () => {
    // s = min(100/√2, 50/√2): a 50 × 50 square, left margin 25.
    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 4,
        height: 50,
        radii: [0, 0, 0, 0],
        width: 100,
      }),
      [
        { x: 75, y: 50 },
        { x: 25, y: 50 },
        { x: 25, y: 0 },
        { x: 75, y: 0 },
      ],
      DECIMALS,
    );
  });

  it("[F02.AC3] fits a hexagon in 100 × 100 with flat top and bottom, from the right end of its base", () => {
    // s = 50: 100 wide, 50√3 = 86.60254 high; top and bottom edges at 6.69873 and 93.30127.
    const margin = (100 - HEIGHT_3) / 2;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 6,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0],
        width: 100,
      }),
      [
        { x: 75, y: 100 - margin },
        { x: 25, y: 100 - margin },
        { x: 0, y: 50 },
        { x: 25, y: margin },
        { x: 75, y: margin },
        { x: 100, y: 50 },
      ],
      DECIMALS,
    );
  });

  it("[F02.AC3] fits a pentagon in 100 × 100 from the right end of its base (odd n above 3, AUD-002)", () => {
    // Unit span 2 cos(π/10) × (1 + cos(π/5)); s = 100 / (2 cos(π/10)) = 52.57311: width reached.
    const s = 50 / Math.cos(Math.PI / 10);
    const top = (100 - s * (1 + Math.cos(Math.PI / 5))) / 2;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 5,
        height: 100,
        radii: [0, 0, 0, 0, 0],
        width: 100,
      }),
      [
        { x: 50 + s * Math.sin(Math.PI / 5), y: top + s * (1 + Math.cos(Math.PI / 5)) },
        { x: 50 - s * Math.sin(Math.PI / 5), y: top + s * (1 + Math.cos(Math.PI / 5)) },
        { x: 0, y: top + s * (1 - Math.sin(Math.PI / 10)) },
        { x: 50, y: top },
        { x: 100, y: top + s * (1 - Math.sin(Math.PI / 10)) },
      ],
      DECIMALS,
    );
  });

  it("[F02.AC1] fits a polygon of 12 corners, the largest allowed (Q19)", () => {
    // n = 12 is divisible by 4: flat sides on all four axes, both sizes reached.
    const box = boundingBox(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 12,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        width: 100,
      }),
    );

    expect(box.maxX - box.minX).toBeCloseTo(100, DECIMALS);
    expect(box.maxY - box.minY).toBeCloseTo(100, DECIMALS);
  });

  it("[F02.AC1] fits an octagon in 100 × 100: both sizes reached", () => {
    // Unit span 2 cos(π/8) on both axes; first vertex, the right end of the base, at
    // x = 50 + 50 tan(π/8) = 70.71068, y = 100.
    const contour = regularPolygonContour({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 8,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0, 0, 0],
      width: 100,
    });
    const box = boundingBox(contour);

    expect(contour[0]?.x).toBeCloseTo(50 + 50 * Math.tan(Math.PI / 8), DECIMALS);
    expect(contour[0]?.y).toBeCloseTo(100, DECIMALS);
    expect(box.maxX - box.minX).toBeCloseTo(100, DECIMALS);
    expect(box.maxY - box.minY).toBeCloseTo(100, DECIMALS);
  });

  it("collapses a polygon of width 0 to the center of its box (Q15)", () => {
    expect(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 5,
        height: 40,
        radii: [0, 0, 0, 0, 0],
        width: 0,
      }),
    ).toEqual(Array.from({ length: 5 }, () => ({ x: 0, y: 20 })));
  });

  it("collapses a polygon of height 0 to the middle of its top edge (Q15)", () => {
    expect(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 3,
        height: 0,
        radii: [0, 0, 0],
        width: 40,
      }),
    ).toEqual(Array.from({ length: 3 }, () => ({ x: 20, y: 0 })));
  });

  it("derives the vertices without changing the model (ADR-0003)", () => {
    const polygon = Object.freeze({
      anchor: CENTER,
      stretched: false,
      angle: 0,
      x: 0,
      y: 0,
      corners: 6,
      height: 100,
      radii: [10, 10, 10, 10, 10, 10],
      width: 100,
    });

    expect(regularPolygonContour(polygon)).toEqual(regularPolygonContour(polygon));

    expect(polygon).toEqual({
      anchor: CENTER,
      stretched: false,
      angle: 0,
      corners: 6,
      x: 0,
      y: 0,
      height: 100,
      radii: [10, 10, 10, 10, 10, 10],
      width: 100,
    });
  });
});

describe("regularPolygonContour (anchored, F07)", () => {
  it("[F07.AC1] draws F02's triangle when centered on both axes", () => {
    // Top margin (100 − 86.60254)/2 = 6.69873, as F02.
    const margin = (100 - HEIGHT_3) / 2;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 3,
        height: 100,
        radii: [0, 0, 0],
        width: 100,
      }),
      [
        { x: 100, y: 100 - margin },
        { x: 0, y: 100 - margin },
        { x: 50, y: margin },
      ],
      DECIMALS,
    );
  });

  it("[F07.AC2] puts the triangle's base on the bottom of 100 × 100: apex at 13.39746", () => {
    // Room 100 − 86.60254 = 13.39746, all of it above the triangle.
    const anchor = { horizontal: "mid", vertical: "max" } as const;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor,
        stretched: false,
        corners: 3,
        height: 100,
        radii: [0, 0, 0],
        width: 100,
      }),
      [
        { x: 100, y: 100 },
        { x: 0, y: 100 },
        { x: 50, y: 100 - HEIGHT_3 },
      ],
      DECIMALS,
    );
  });

  it("[F07.AC2] puts the hexagon's base on the bottom of 100 × 100 (DERIV check row)", () => {
    // Shifted down by 6.69873 from its centered place: (25, 13.39746) … (0, 56.69873).
    const anchor = { horizontal: "mid", vertical: "max" } as const;
    const top = 100 - HEIGHT_3;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor,
        stretched: false,
        corners: 6,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0],
        width: 100,
      }),
      [
        { x: 75, y: 100 },
        { x: 25, y: 100 },
        { x: 0, y: top + HEIGHT_3 / 2 },
        { x: 25, y: top },
        { x: 75, y: top },
        { x: 100, y: top + HEIGHT_3 / 2 },
      ],
      DECIMALS,
    );
  });

  it("[F07.AC2] puts the square on the right of 100 × 50", () => {
    // Room 100 − 50 = 50, all of it on the left of the square.
    const anchor = { horizontal: "max", vertical: "mid" } as const;

    expectPoints(
      regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor,
        stretched: false,
        corners: 4,
        height: 50,
        radii: [0, 0, 0, 0],
        width: 100,
      }),
      [
        { x: 100, y: 50 },
        { x: 50, y: 50 },
        { x: 50, y: 0 },
        { x: 100, y: 0 },
      ],
      DECIMALS,
    );
  });

  it("[F07.AC3] keeps the triangle in place when only its horizontal alignment changes", () => {
    // The width is reached: no room on that axis. The anchor itself is kept by the model type.
    const left = {
      anchor: { horizontal: "min", vertical: "max" },
      stretched: false,
      angle: 0,
      corners: 3,
      x: 0,
      y: 0,
      height: 100,
      radii: [0, 0, 0],
      width: 100,
    } as const;
    const right = { ...left, anchor: { horizontal: "max", vertical: "max" } } as const;

    expectPoints(regularPolygonContour(left), regularPolygonContour(right), DECIMALS);
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
        regularPolygonContour({
          angle: 0,
          x: 0,
          y: 0,
          anchor: CENTER,
          stretched: false,
          corners,
          height,
          radii: Array.from({ length: corners }, () => 0),
          width,
        }),
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

describe("regularPolygonContour (numbered from its base, F08)", () => {
  it("[F08.AC5] starts a triangle and a square at the right end of their base, clockwise", () => {
    const triangle = regularPolygonContour({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 3,
      height: 100,
      radii: [0, 0, 0],
      width: 100,
    });
    const square = regularPolygonContour({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 4,
      height: 100,
      radii: [0, 0, 0, 0],
      width: 100,
    });

    expect(
      triangle.map((point) => [point.x, point.y].map((value) => Number(value.toFixed(5)))),
    ).toEqual([
      [100, 93.30127],
      [0, 93.30127],
      [50, 6.69873],
    ]);

    expect(
      square.map((point) => [point.x, point.y].map((value) => Number(value.toFixed(5)))),
    ).toEqual([
      [100, 100],
      [0, 100],
      [0, 0],
      [100, 0],
    ]);
  });

  it("[F08.AC5] keeps the radii of the base on the base when the number of corners changes", () => {
    // Points 1 and 2 are the base at any number of corners: radii kept by number stay there.
    const radii = resizeRadii([10, 10, 0, 0, 0, 0], 5);
    const corners = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 5,
      height: 100,
      radii,
      width: 100,
    });
    const lowest = Math.max(...corners.map((corner) => corner.point.y));

    expect(
      corners.filter((corner) => corner.radius === 10).map((corner) => corner.point.y),
    ).toEqual([lowest, lowest]);
  });

  test.prop({
    anchor: fc.record({
      horizontal: fc.constantFrom(...ALIGNMENTS),
      vertical: fc.constantFrom(...ALIGNMENTS),
    }),
    corners: fc.integer({ max: 12, min: 3 }),
    size: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F08.AC5] puts points 1 and 2 at the two ends of the flat base",
    ({ anchor, corners, size }) => {
      const contour = regularPolygonContour({
        angle: 0,
        x: 0,
        y: 0,
        anchor,
        stretched: false,
        corners,
        height: size,
        radii: Array.from({ length: corners }, () => 0),
        width: size,
      });
      const [right = { x: NaN, y: NaN }, left = { x: NaN, y: NaN }] = contour;
      const lowest = Math.max(...contour.map((point) => point.y));
      const tolerance = EPSILON * Math.max(1, size);

      expect(Math.abs(right.y - lowest)).toBeLessThan(tolerance);
      expect(Math.abs(left.y - lowest)).toBeLessThan(tolerance);
      expect(right.x - left.x).toBeGreaterThan(0);
    },
  );
});

describe("regularPolygonContour (at a position, F04)", () => {
  it("[F04.AC3] places the polygon's box at its position: the F07 contour moved by (10, 20)", () => {
    // A triangle anchored at the bottom of 100 × 100 has its base at y = 100, from x = 0 to 100
    // (the width is reached); at (10, 20) the base runs from (10, 120) to (110, 120).
    const contour = regularPolygonContour({
      anchor: { horizontal: "mid", vertical: "max" },
      stretched: false,
      angle: 0,
      corners: 3,
      height: 100,
      radii: [0, 0, 0],
      width: 100,
      x: 10,
      y: 20,
    });

    expect(contour[0]?.x).toBeCloseTo(110, 12);
    expect(contour[0]?.y).toBeCloseTo(120, 12);
    expect(contour[1]?.x).toBeCloseTo(10, 12);
    expect(contour[1]?.y).toBeCloseTo(120, 12);
  });
});

describe("regularPolygonContour (stretched, F06)", () => {
  it("[F06.AC2] [F06.AC3] stretches the hexagon to its box at its position, its anchor without effect", () => {
    // DERIV-stretched-polygon, checks: the hexagon in 100 × 100, every side 54.85838, moved by
    // (10, 20); the anchor bottom left would leave room only for a regular hexagon.
    const expected = stretchedPolygon(6, { height: 100, width: 100 });

    for (const anchor of [CENTER, { horizontal: "min", vertical: "max" } as const]) {
      const contour = regularPolygonContour({
        anchor,
        angle: 0,
        corners: 6,
        height: 100,
        radii: [0, 0, 0, 0, 0, 0],
        stretched: true,
        width: 100,
        x: 10,
        y: 20,
      });

      expectPoints(
        contour,
        expected.map((point) => ({ x: point.x + 10, y: point.y + 20 })),
        DECIMALS,
      );
    }
  });

  it("[F06.AC1] keeps the regular polygon of F02 when not stretched: the hexagon leaves room", () => {
    const contour = regularPolygonContour({
      anchor: CENTER,
      angle: 0,
      corners: 6,
      height: 100,
      radii: [0, 0, 0, 0, 0, 0],
      stretched: false,
      width: 100,
      x: 0,
      y: 0,
    });

    // Regular hexagon fitted in 100 × 100: height 86.60254, centered, so its top is at 6.69873.
    expect(boundingBox(contour).minY).toBeCloseTo(50 - HEIGHT_3 / 2, DECIMALS);
  });
});
