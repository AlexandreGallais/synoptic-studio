import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { rectangleContour } from "./rectangle-contour";

describe("rectangleContour", () => {
  it("[F01.AC4] lists the vertices clockwise on screen from the top-left one (Q11)", () => {
    // y points down: top-left, top-right, bottom-right, bottom-left.
    expect(rectangleContour({ height: 50, radii: [0, 0, 0, 0], width: 100 })).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ]);
  });

  it("keeps integer coordinates for integer sizes (ADR-0003)", () => {
    const vertices = rectangleContour({ height: 7, radii: [0, 0, 0, 0], width: 13 });

    expect(
      vertices.every((vertex) => Number.isSafeInteger(vertex.x) && Number.isSafeInteger(vertex.y)),
    ).toBe(true);
  });

  it("accepts a degenerate rectangle of height 0 (Q15)", () => {
    // Height 0: the four vertices lie on the top edge.
    expect(rectangleContour({ height: 0, radii: [0, 0, 0, 0], width: 10 })).toEqual([
      { x: 0, y: 0 },
      { x: 10, y: 0 },
      { x: 10, y: 0 },
      { x: 0, y: 0 },
    ]);
  });
});

describe("rectangleContour (properties)", () => {
  test.prop({
    height: fc.integer({ max: 1000, min: 1 }),
    width: fc.integer({ max: 1000, min: 1 }),
  })(
    "[F01.AC4] turns clockwise on screen: positive signed area with y down",
    ({ height, width }) => {
      const contour = rectangleContour({ height, radii: [0, 0, 0, 0], width });
      // Shoelace sum Σ (xᵢ yᵢ₊₁ − xᵢ₊₁ yᵢ) = 2 · width · height > 0 for a clockwise turn on screen.
      const doubleArea = contour.reduce((sum, vertex, index) => {
        const next = contour[(index + 1) % contour.length] ?? vertex;

        return sum + vertex.x * next.y - next.x * vertex.y;
      }, 0);

      expect(doubleArea).toBe(2 * width * height);
    },
  );
});
