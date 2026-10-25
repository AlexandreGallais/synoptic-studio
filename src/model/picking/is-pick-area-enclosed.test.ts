import { describe, expect, it } from "vitest";

import { isPickAreaEnclosed } from "./is-pick-area-enclosed";

import type { PickArea } from "./pick-area";

/** The corners of a box 100 × 100 at (0, 0). */
const CORNERS = [
  { x: 0, y: 0 },
  { x: 100, y: 0 },
  { x: 100, y: 100 },
  { x: 0, y: 100 },
];

/** That box with an area: its four sides. */
const BOX: PickArea = {
  corners: CORNERS,
  isFlat: false,
  pieces: [
    { end: { x: 100, y: 0 }, kind: "segment", start: { x: 0, y: 0 } },
    { end: { x: 100, y: 100 }, kind: "segment", start: { x: 100, y: 0 } },
    { end: { x: 0, y: 100 }, kind: "segment", start: { x: 100, y: 100 } },
    { end: { x: 0, y: 0 }, kind: "segment", start: { x: 0, y: 100 } },
  ],
};

/** A box 0 × 80 at (10, 10): its corners on its line, no contour. */
const FLAT: PickArea = {
  corners: [
    { x: 10, y: 10 },
    { x: 10, y: 10 },
    { x: 10, y: 90 },
    { x: 10, y: 90 },
  ],
  isFlat: true,
  pieces: [],
};

describe("isPickAreaEnclosed", () => {
  it("encloses a shape whose contour lies within the rectangle, its sides included", () => {
    // [0, 100]² holds the contour exactly; [1, 100] × [0, 100] leaves out its side x = 0.
    expect(isPickAreaEnclosed(BOX, { maxX: 100, maxY: 100, minX: 0, minY: 0 })).toBe(true);
    expect(isPickAreaEnclosed(BOX, { maxX: 100, maxY: 100, minX: 1, minY: 0 })).toBe(false);
  });

  it("encloses a flat shape by its corners' box", () => {
    // Its line from (10, 10) to (10, 90): within [0, 20] × [0, 100], not within [0, 20] × [0, 50].
    expect(isPickAreaEnclosed(FLAT, { maxX: 20, maxY: 100, minX: 0, minY: 0 })).toBe(true);
    expect(isPickAreaEnclosed(FLAT, { maxX: 20, maxY: 50, minX: 0, minY: 0 })).toBe(false);
  });

  it("encloses a flat shape by its corners, not by a contour it may not have", () => {
    // Flat, with the box's sides as pieces: its line is within [0, 20] × [0, 100].
    expect(
      isPickAreaEnclosed(
        { ...BOX, corners: FLAT.corners, isFlat: true },
        { maxX: 20, maxY: 100, minX: 0, minY: 0 },
      ),
    ).toBe(true);
  });
});
