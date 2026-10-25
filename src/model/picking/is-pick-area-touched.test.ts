import { describe, expect, it } from "vitest";

import { isPickAreaTouched } from "./is-pick-area-touched";

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

/**
 * The box and the flat shape moved right, as a contour computed with a rounding error lands.
 *
 * @param offset - how far, in user units
 * @returns the box, then the flat shape
 */
function moved(offset: number): readonly [PickArea, PickArea] {
  return [
    {
      ...BOX,
      pieces: BOX.pieces.map((piece) => ({
        ...piece,
        end: { ...piece.end, x: piece.end.x + offset },
        start: { ...piece.start, x: piece.start.x + offset },
      })),
    },
    { ...FLAT, corners: FLAT.corners.map((corner) => ({ ...corner, x: corner.x + offset })) },
  ];
}

describe("isPickAreaTouched", () => {
  it("touches a shape with an area inside it or across its contour", () => {
    // [40, 60]² lies inside, no side meeting it; [90, 110] × [40, 60] crosses x = 100.
    expect(isPickAreaTouched(BOX, { maxX: 60, maxY: 60, minX: 40, minY: 40 })).toBe(true);
    expect(isPickAreaTouched(BOX, { maxX: 110, maxY: 60, minX: 90, minY: 40 })).toBe(true);
    expect(isPickAreaTouched(BOX, { maxX: 120, maxY: 60, minX: 102, minY: 40 })).toBe(false);
  });

  it("touches a flat shape by its line, with no tolerance", () => {
    // [5, 15] × [40, 50] crosses x = 10; [12, 20] × [0, 100] is 2 from it.
    expect(isPickAreaTouched(FLAT, { maxX: 15, maxY: 50, minX: 5, minY: 40 })).toBe(true);
    expect(isPickAreaTouched(FLAT, { maxX: 20, maxY: 100, minX: 12, minY: 0 })).toBe(false);
  });

  it("touches a side computed a rounding error away, not one further (ADR-0003, EPSILON)", () => {
    // Both moved right by 10⁻¹²: the box's left side to x = 10⁻¹², the line to x = 10 + 10⁻¹²;
    // [−10, 0] × [40, 60] widened reaches x = 10⁻⁹ ≥ 10⁻¹², [0, 10] × [40, 60] reaches 10 + 10⁻⁹.
    // Moved by 10⁻⁶, beyond EPSILON, neither, the corner (−10, 40) outside the box.
    const [near, nearLine] = moved(1e-12);
    const [far, farLine] = moved(1e-6);

    expect(isPickAreaTouched(near, { maxX: 0, maxY: 60, minX: -10, minY: 40 })).toBe(true);
    expect(isPickAreaTouched(nearLine, { maxX: 10, maxY: 60, minX: 0, minY: 40 })).toBe(true);
    expect(isPickAreaTouched(far, { maxX: 0, maxY: 60, minX: -10, minY: 40 })).toBe(false);
    expect(isPickAreaTouched(farLine, { maxX: 10, maxY: 60, minX: 0, minY: 40 })).toBe(false);
  });

  it("touches a flat shape by its corners, not by a contour it may not have", () => {
    // Flat, with the box's sides as pieces: [40, 60]² is 30 from the line x = 10.
    expect(
      isPickAreaTouched(
        { ...BOX, corners: FLAT.corners, isFlat: true },
        { maxX: 60, maxY: 60, minX: 40, minY: 40 },
      ),
    ).toBe(false);
  });
});
