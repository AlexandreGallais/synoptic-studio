import { describe, expect, it } from "vitest";

import { isPickAreaAt } from "./is-pick-area-at";

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

describe("isPickAreaAt (EN-143)", () => {
  it("reaches a shape with an area inside it only, whatever the tolerance", () => {
    // (50, 50) inside; (102, 50) is 2 from its side, beyond it: the tolerance is not used.
    expect(isPickAreaAt(BOX, { x: 50, y: 50 }, 4)).toBe(true);
    expect(isPickAreaAt(BOX, { x: 102, y: 50 }, 4)).toBe(false);
  });

  it("reaches a flat shape within the tolerance of its line", () => {
    // (12, 50) is 2 from x = 10, (14, 50) 4, (15, 50) 5.
    expect(isPickAreaAt(FLAT, { x: 12, y: 50 }, 4)).toBe(true);
    expect(isPickAreaAt(FLAT, { x: 14, y: 50 }, 4)).toBe(true);
    expect(isPickAreaAt(FLAT, { x: 15, y: 50 }, 4)).toBe(false);
  });

  it("reaches a flat shape by its corners, not by a contour it may not have", () => {
    // Flat, with the box's sides as pieces: (50, 50) is 40 from the line, beyond 4.
    expect(isPickAreaAt({ ...BOX, corners: FLAT.corners, isFlat: true }, { x: 50, y: 50 }, 4)).toBe(
      false,
    );
  });
});
