import { describe, expect, it } from "vitest";

import { effectiveRadii, roundedContour } from "../../geometry";

import { rectangleCorners } from "./rectangle-corners";

describe("rectangleCorners", () => {
  it("[F03.AC1] gives the four vertices of the contour the same radius when the radii are equal", () => {
    expect(
      rectangleCorners({ angle: 0, x: 0, y: 0, height: 50, radii: [10, 10, 10, 10], width: 100 }),
    ).toEqual([
      { point: { x: 0, y: 0 }, radius: 10 },
      { point: { x: 100, y: 0 }, radius: 10 },
      { point: { x: 100, y: 50 }, radius: 10 },
      { point: { x: 0, y: 50 }, radius: 10 },
    ]);
  });
});

describe("rectangleCorners (one radius per vertex)", () => {
  it("[F03.AC1] rounds only the top-left corner of a 100 × 100 square with radii 50, 0, 0, 0", () => {
    // One quarter circle of radius 50 centered at (50, 50), from (0, 50) to (50, 0); the other
    // three corners stay sharp, joined by segments.
    const pieces = roundedContour(
      rectangleCorners({ angle: 0, x: 0, y: 0, height: 100, radii: [50, 0, 0, 0], width: 100 }),
    );
    // Within rounding: the setback is 50 · tan(45°). Each arc as start, end, center, radius.
    const arcs = pieces.flatMap((piece) =>
      piece.kind === "arc"
        ? [
            [
              piece.start.x,
              piece.start.y,
              piece.end.x,
              piece.end.y,
              piece.center.x,
              piece.center.y,
              piece.radius,
            ],
          ]
        : [],
    );

    expect(arcs).toHaveLength(1);

    for (const [index, expected] of [0, 50, 50, 0, 50, 50, 50].entries()) {
      expect(arcs[0]?.[index]).toBeCloseTo(expected, 9);
    }
  });

  it("[F03.AC1] shares a short edge between unequal radii as DERIV-local-radius-clamp says", () => {
    // Equal within rounding: a setback is r · tan(45°), not exactly r in floating point.
    // 100 × 20, radii 30, 10, 0, 0: left edge 20 < 30, f = 2/3, top-left 20; top-right 10 fits.
    expect(
      effectiveRadii(
        rectangleCorners({ angle: 0, x: 0, y: 0, height: 20, radii: [30, 10, 0, 0], width: 100 }),
      ),
    ).toEqual([expect.closeTo(20, 9), expect.closeTo(10, 9), 0, 0]);

    // 20 × 100, same radii: top edge 20 < 40, f = 1/2 for both its corners: 15 and 5.
    expect(
      effectiveRadii(
        rectangleCorners({ angle: 0, x: 0, y: 0, height: 100, radii: [30, 10, 0, 0], width: 20 }),
      ),
    ).toEqual([expect.closeTo(15, 9), expect.closeTo(5, 9), 0, 0]);

    // 100 × 30, radii 40, 20, 0, 0: left edge 30 < 40, f = 3/4, top-left 30; top-right 20 fits.
    expect(
      effectiveRadii(
        rectangleCorners({ angle: 0, x: 0, y: 0, height: 30, radii: [40, 20, 0, 0], width: 100 }),
      ),
    ).toEqual([expect.closeTo(30, 9), expect.closeTo(20, 9), 0, 0]);

    // 30 × 20, radii 30, 10, 0, 0: left f = 2/3, top f = 3/4; top-left takes the smaller of its
    // two edges, 20; top-right 7.5, leaving 2.5 of the top edge unused (known limit).
    expect(
      effectiveRadii(
        rectangleCorners({ angle: 0, x: 0, y: 0, height: 20, radii: [30, 10, 0, 0], width: 30 }),
      ),
    ).toEqual([expect.closeTo(20, 9), expect.closeTo(7.5, 9), 0, 0]);
  });
});
