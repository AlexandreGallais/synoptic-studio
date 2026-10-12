import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { edgeLengths } from "./edge-lengths";

describe("edgeLengths", () => {
  it("gives the length of each edge, the last one closing the contour (Q11)", () => {
    // Rectangle 100 × 50 clockwise from the top-left vertex: top, right, bottom, left.
    const rectangle = [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 50 },
      { x: 0, y: 50 },
    ];

    expect(edgeLengths(rectangle)).toEqual([100, 50, 100, 50]);
  });

  it("gives 0 for the edges of a size of 0 (Q15)", () => {
    // Rectangle 0 × 50: the top and bottom edges have no length.
    const flat = [
      { x: 0, y: 0 },
      { x: 0, y: 0 },
      { x: 0, y: 50 },
      { x: 0, y: 50 },
    ];

    expect(edgeLengths(flat)).toEqual([0, 50, 0, 50]);
  });

  it("gives nothing for an empty contour", () => {
    expect(edgeLengths([])).toEqual([]);
  });
});

describe("edgeLengths (properties)", () => {
  test.prop({
    height: fc.integer({ max: 1000, min: 0 }),
    width: fc.integer({ max: 1000, min: 0 }),
  })("sums to the perimeter 2 · (width + height) of a rectangle", ({ height, width }) => {
    const lengths = edgeLengths([
      { x: 0, y: 0 },
      { x: width, y: 0 },
      { x: width, y: height },
      { x: 0, y: height },
    ]);

    expect(lengths.reduce((sum, length) => sum + length, 0)).toBe(2 * (width + height));
  });
});
