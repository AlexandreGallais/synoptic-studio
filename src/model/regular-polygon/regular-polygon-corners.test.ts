import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { edgeLengths, effectiveRadii, roundedContour } from "../../geometry";
import { EPSILON } from "../../math";
import { effectiveCornerRadius } from "../corner-radius";

import { regularPolygonContour } from "./regular-polygon-contour";
import { regularPolygonCorners } from "./regular-polygon-corners";

import type { Anchor, Arc, ContourPiece, Corner } from "../../geometry";

/** F02's placement: centered on both axes (F07.AC1). */
const CENTER: Anchor = { horizontal: "mid", vertical: "mid" };

/** Decimals compared, from `EPSILON = 1e-9` (Q10). */
const DECIMALS = 9;

/**
 * Arcs of a rounded contour.
 *
 * @param pieces - segments and arcs
 * @returns the arcs only, in drawing order
 */
function arcsOf(pieces: readonly ContourPiece[]): readonly Arc[] {
  return pieces.filter((piece): piece is Arc => piece.kind === "arc");
}

/**
 * Circumradius `s` of a regular polygon, from its edge `a = 2 s sin(π/n)`
 * (`REF-MATHWORLD-REGULAR-POLYGON` (2)).
 *
 * @param corners - corners of the polygon, in drawing order
 * @returns the radius of its circumscribed circle
 */
function circumradius(corners: readonly Corner[]): number {
  const [edge = 0] = edgeLengths(corners.map((corner) => corner.point));

  return edge / (2 * Math.sin(Math.PI / corners.length));
}

/**
 * The one radius drawn on corners asking the same radius, through the clamp of F01.
 *
 * @param corners - points and radii of a shape
 * @returns the radius drawn, or `undefined` when they differ
 */
function radiusOf(corners: readonly Corner[]): number | undefined {
  return effectiveCornerRadius(effectiveRadii(corners));
}

describe("regularPolygonCorners (DERIV-regular-polygon-fit step 6)", () => {
  it("[F02.AC2] turns a hexagon in 100 × 100 with radius 1000 into its incircle: diameter 86.60254", () => {
    // s = 50, inradius 50 cos(π/6) = 43.30127; every arc centered at the center of the box.
    const corners = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 6,
      height: 100,
      radii: [1000, 1000, 1000, 1000, 1000, 1000],
      width: 100,
    });
    const pieces = roundedContour(corners);

    expect(radiusOf(corners)).toBeCloseTo(25 * Math.sqrt(3), DECIMALS);
    expect(arcsOf(pieces)).toHaveLength(6);
    expect(pieces.every((piece) => piece.kind === "arc")).toBe(true);

    const arcs = arcsOf(pieces);

    for (const arc of arcs) {
      expect(arc.center.x).toBeCloseTo(50, DECIMALS);
      expect(arc.center.y).toBeCloseTo(50, DECIMALS);
    }
  });

  it("[F02.AC2] gives a triangle in 100 × 100 with radius 1000 its incircle, below the center of the box", () => {
    // s = 100/√3, inradius s cos(π/3) = 28.86751; center 28.86751 above the base 93.30127: y = 64.43376.
    const corners = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 3,
      height: 100,
      radii: [1000, 1000, 1000],
      width: 100,
    });
    const inradius = 50 / Math.sqrt(3);
    const base = 50 + 25 * Math.sqrt(3);

    expect(radiusOf(corners)).toBeCloseTo(inradius, DECIMALS);

    const pieces = roundedContour(corners);
    const arcs = arcsOf(pieces);

    expect(arcs).toHaveLength(3);
    expect(pieces.every((piece) => piece.kind === "arc")).toBe(true);

    for (const arc of arcs) {
      expect(arc.center.x).toBeCloseTo(50, DECIMALS);
      expect(arc.center.y).toBeCloseTo(base - inradius, DECIMALS);
    }
  });

  it("[F02.AC2] keeps radius 10 on a hexagon in 100 × 100: two setbacks of 5.7735 fit an edge of 50", () => {
    // Turn of π/3: setback 10 · tan(π/6) = 5.7735; 2 × 5.7735 = 11.547 ≤ 50, no reduction.
    const corners = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 6,
      height: 100,
      radii: [10, 10, 10, 10, 10, 10],
      width: 100,
    });

    expect(radiusOf(corners)).toBeCloseTo(10, DECIMALS);
  });

  it("keeps the requested radius on a polygon of size 0, which draws nothing (Q17)", () => {
    const corners = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 6,
      height: 0,
      radii: [10, 10, 10, 10, 10, 10],
      width: 0,
    });

    expect(radiusOf(corners)).toBe(10);
    expect(roundedContour(corners)).toEqual([]);
  });

  it("[F02.AC2] keeps the requested radius on every corner (Q8)", () => {
    const corners = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners: 5,
      height: 100,
      radii: [1000, 1000, 1000, 1000, 1000],
      width: 100,
    });

    expect(corners.map((corner) => corner.radius)).toEqual([1000, 1000, 1000, 1000, 1000]);
  });
});

describe("regularPolygonCorners (anchored, F07)", () => {
  it("[F07.AC4] keeps the right side of a square anchored right at x = 100 with radius 10", () => {
    // A flat face keeps its straight part on the edge: the segment from (100, 10) to (100, 40).
    const anchor = { horizontal: "max", vertical: "mid" } as const;
    const pieces = roundedContour(
      regularPolygonCorners({
        angle: 0,
        x: 0,
        y: 0,
        anchor,
        stretched: false,
        corners: 4,
        height: 50,
        radii: [10, 10, 10, 10],
        width: 100,
      }),
    );
    const right = pieces.filter((piece) => piece.kind === "segment" && piece.start.x > 99);

    expect(right).toHaveLength(1);
    expect(right[0]?.start.x).toBeCloseTo(100, DECIMALS);
    expect(right[0]?.start.y).toBeCloseTo(10, DECIMALS);
    expect(right[0]?.end.x).toBeCloseTo(100, DECIMALS);
    expect(right[0]?.end.y).toBeCloseTo(40, DECIMALS);
  });

  it("[F07.AC4] moves the rounded apex of a triangle anchored top to y = 10 with radius 10", () => {
    // Interior angle 60°: the arc's center is r / sin(30°) = 20 below the apex, its top r = 10 below.
    const anchor = { horizontal: "mid", vertical: "min" } as const;
    const arcs = arcsOf(
      roundedContour(
        regularPolygonCorners({
          angle: 0,
          x: 0,
          y: 0,
          anchor,
          stretched: false,
          corners: 3,
          height: 100,
          radii: [10, 10, 10],
          width: 100,
        }),
      ),
    );
    // The path starts where the arc of point 1, the right end of the base, ends, and closes with it (DERIV-fillet-arc step 7).
    const [apex, ...others] = arcs.filter((arc) => arc.center.y < 50);

    expect(others).toHaveLength(0);

    const { center, radius } = apex ?? {
      center: { x: NaN, y: NaN },
      radius: NaN,
    };

    expect(center.x).toBeCloseTo(50, DECIMALS);
    expect(center.y).toBeCloseTo(20, DECIMALS);
    expect(radius).toBeCloseTo(10, DECIMALS);

    // The apex arc starts and ends at y = 15 and turns over the top, center.y − radius = 10.
    expect(Math.min(...arcs.map((arc) => arc.center.y - arc.radius))).toBeCloseTo(10, DECIMALS);
    expect(Math.min(...arcs.flatMap((arc) => [arc.start.y, arc.end.y]))).toBeCloseTo(15, DECIMALS);
  });
});

describe("regularPolygonCorners (properties)", () => {
  test.prop({
    corners: fc.integer({ max: 12, min: 3 }),
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F02.AC2] at the maximal radius, gives every corner the inradius s · cos(π/n), one common center",
    ({ corners, height, width }) => {
      const polygon = regularPolygonCorners({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners,
        height,
        radii: Array.from({ length: corners }, () => 100_000),
        width,
      });
      const arcs = arcsOf(roundedContour(polygon));
      const [first] = arcs;
      const tolerance = 1e-9 * Math.max(width, height);
      const offsets = arcs.map((arc) =>
        Math.hypot(arc.center.x - (first?.center.x ?? 0), arc.center.y - (first?.center.y ?? 0)),
      );

      expect(radiusOf(polygon)).toBeCloseTo(
        circumradius(polygon) * Math.cos(Math.PI / corners),
        DECIMALS,
      );

      expect(arcs).toHaveLength(corners);
      expect(Math.max(...offsets)).toBeLessThan(tolerance);
    },
  );

  test.prop({
    corners: fc.integer({ max: 12, min: 3 }),
    radius: fc.integer({ max: 2000, min: 0 }),
    size: fc.integer({ max: 1000, min: 1 }),
  })("never exceeds the requested radius", ({ corners, radius, size }) => {
    const polygon = regularPolygonCorners({
      angle: 0,
      x: 0,
      y: 0,
      anchor: CENTER,
      stretched: false,
      corners,
      height: size,
      radii: Array.from({ length: corners }, () => radius),
      width: size,
    });

    expect(radiusOf(polygon)).toBeLessThanOrEqual(radius * (1 + 1e-9));
  });

  test.prop({
    corners: fc.integer({ max: 12, min: 3 }),
    radius: fc.integer({ max: 2000, min: 0 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })("gives every corner the same effective radius (AUD-002)", ({ corners, radius, width }) => {
    const radii = effectiveRadii(
      regularPolygonCorners({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners,
        height: width,
        radii: Array.from({ length: corners }, () => radius),
        width,
      }),
    );

    expect(Math.max(...radii) - Math.min(...radii)).toBeLessThan(1e-9 * Math.max(1, radius));
  });
});

describe("regularPolygonCorners (anchored, properties)", () => {
  test.prop({
    corners: fc.integer({ max: 12, min: 3 }),
    height: fc.integer({ max: 1000, min: 1 }),
    radius: fc.integer({ max: 2000, min: 0 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F07.AC4] keeps the flat base on the bottom of the box at any radius",
    ({ corners, height, radius, width }) => {
      // Every polygon has a flat base; its fillets stop on it, and at the maximal radius the
      // incircle still touches it at its middle (DERIV-regular-polygon-fit step 6).
      const anchor = { horizontal: "mid", vertical: "max" } as const;
      const pieces = roundedContour(
        regularPolygonCorners({
          angle: 0,
          x: 0,
          y: 0,
          anchor,
          stretched: false,
          corners,
          height,
          radii: Array.from({ length: corners }, () => radius),
          width,
        }),
      );
      const lowest = Math.max(...pieces.flatMap((piece) => [piece.start.y, piece.end.y]));

      expect(Math.abs(lowest - height)).toBeLessThan(EPSILON * Math.max(1, width, height));
    },
  );
});

describe("regularPolygonCorners (one radius per vertex)", () => {
  it("[F03.AC1] gives every vertex the same radius as F02 when the radii are equal", () => {
    const polygon = {
      anchor: CENTER,
      stretched: false,
      angle: 0,
      corners: 6,
      height: 100,
      radii: [7, 7, 7, 7, 7, 7],
      x: 0,
      y: 0,
      width: 100,
    };

    expect(regularPolygonCorners(polygon)).toEqual(
      regularPolygonContour(polygon).map((point) => ({ point, radius: 7 })),
    );
  });

  it("[F06.AC3] gives each vertex of a stretched pentagon its own requested radius", () => {
    const polygon = {
      anchor: CENTER,
      angle: 0,
      corners: 5,
      height: 50,
      radii: [1, 2, 3, 4, 5],
      stretched: true,
      width: 300,
      x: 0,
      y: 0,
    };

    expect(regularPolygonCorners(polygon)).toEqual(
      regularPolygonContour(polygon).map((point, index) => ({ point, radius: index + 1 })),
    );
  });

  it("[F03.AC1] rounds only vertex 1 of a square with radii 20, 0, 0, 0", () => {
    // A square with a flat base fills 100 × 100; vertex 1 is the right end of its base (Q18,
    // F08): one quarter circle of radius 20 centered at (80, 80) — at (20, 80) it would be vertex 2.
    const pieces = roundedContour(
      regularPolygonCorners({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: 4,
        height: 100,
        radii: [20, 0, 0, 0],
        width: 100,
      }),
    );
    const [arc, ...others] = arcsOf(pieces);

    expect(others).toEqual([]);
    expect(arc?.radius).toBe(20);
    expect(arc?.center.x).toBeCloseTo(80, DECIMALS);
    expect(arc?.center.y).toBeCloseTo(80, DECIMALS);
  });
});

describe("regularPolygonCorners (one radius per vertex, properties)", () => {
  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    radii: fc.array(fc.nat({ max: 2000 }), { maxLength: 12, minLength: 12 }),
    sides: fc.integer({ max: 12, min: 3 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F03.AC1] keeps the flat base on the bottom of the box whatever the radius of each corner",
    ({ height, radii, sides, width }) => {
      // The anchor places the sharp polygon; fillets only cut inwards, and the base's straight
      // part, or the point where its two fillets meet, stays on it (DERIV-local-radius-clamp).
      const anchor = { horizontal: "mid", vertical: "max" } as const;
      const pieces = roundedContour(
        regularPolygonCorners({
          angle: 0,
          x: 0,
          y: 0,
          anchor,
          stretched: false,
          corners: sides,
          height,
          radii: radii.slice(0, sides),
          width,
        }),
      );
      const lowest = Math.max(...pieces.flatMap((piece) => [piece.start.y, piece.end.y]));

      expect(Math.abs(lowest - height)).toBeLessThan(EPSILON * Math.max(1, width, height));
    },
  );
});

describe("regularPolygonCorners (unequal radii, closed outline)", () => {
  test.prop({
    radii: fc.array(fc.nat({ max: 2000 }), { maxLength: 12, minLength: 12 }),
    sides: fc.integer({ max: 12, min: 3 }),
    size: fc.integer({ max: 1000, min: 0 }),
  })("[F03.AC1] is a closed chain whatever the radius of each corner", ({ radii, sides, size }) => {
    const pieces = roundedContour(
      regularPolygonCorners({
        angle: 0,
        x: 0,
        y: 0,
        anchor: CENTER,
        stretched: false,
        corners: sides,
        height: size,
        radii: radii.slice(0, sides),
        width: size,
      }),
    );

    // Each piece starts where the previous one ends: the arcs of one edge never overlap.
    for (const [index, piece] of pieces.entries()) {
      const next = pieces[(index + 1) % pieces.length] ?? piece;

      expect(Math.hypot(next.start.x - piece.end.x, next.start.y - piece.end.y)).toBeLessThan(
        EPSILON * Math.max(1, size),
      );
    }
  });
});
