import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { doubleSignedArea } from "../../tests";

import { rectangleContour } from "./rectangle-contour";

describe("rectangleContour", () => {
  it("[F01.AC4] lists the vertices clockwise on screen from the top-left one (Q11)", () => {
    // y points down: top-left, top-right, bottom-right, bottom-left.
    expect(
      rectangleContour({ angle: 0, x: 0, y: 0, height: 50, radii: [0, 0, 0, 0], width: 100 }),
    ).toEqual([
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ]);
  });

  it("keeps integer coordinates for integer sizes (ADR-0003)", () => {
    const vertices = rectangleContour({
      angle: 0,
      x: 0,
      y: 0,
      height: 7,
      radii: [0, 0, 0, 0],
      width: 13,
    });

    expect(
      vertices.every((vertex) => Number.isSafeInteger(vertex.x) && Number.isSafeInteger(vertex.y)),
    ).toBe(true);
  });

  it("accepts a degenerate rectangle of height 0 (Q15)", () => {
    // Height 0: the four vertices lie on the top edge.
    expect(
      rectangleContour({ angle: 0, x: 0, y: 0, height: 0, radii: [0, 0, 0, 0], width: 10 }),
    ).toEqual([
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
      const contour = rectangleContour({
        angle: 0,
        x: 0,
        y: 0,
        height,
        radii: [0, 0, 0, 0],
        width,
      });

      // Shoelace sum Σ (xᵢ yᵢ₊₁ − xᵢ₊₁ yᵢ) = 2 · width · height > 0 for a clockwise turn on screen.
      expect(doubleSignedArea(contour)).toBe(2 * width * height);
    },
  );
});
