import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { polygonSegments } from "./polygon-segments";
import { windingNumber } from "./winding-number";

/** A box 100 × 100 at (0, 0), clockwise on screen from its top-left (Q11). */
const SQUARE = polygonSegments([
  { x: 0, y: 0 },
  { x: 100, y: 0 },
  { x: 100, y: 100 },
  { x: 0, y: 100 },
]);

/** The same box, counter-clockwise on screen. */
const REVERSED = polygonSegments([
  { x: 0, y: 0 },
  { x: 0, y: 100 },
  { x: 100, y: 100 },
  { x: 100, y: 0 },
]);

describe("windingNumber (DERIV-painted-area steps 1 and 5)", () => {
  it("is +1 inside a box drawn clockwise on screen", () => {
    // On y = 50 the right side, y growing, crosses at x = 100 > 50: +1; the left one at x = 0.
    expect(windingNumber(SQUARE, { x: 50, y: 50 })).toBe(1);
  });

  it("is −1 inside the same box drawn the other way", () => {
    expect(windingNumber(REVERSED, { x: 50, y: 50 })).toBe(-1);
  });

  it("is 0 outside it, beside, above and below", () => {
    // Beside: both sides cross left of 150; above and below: no side spans the level.
    expect(windingNumber(SQUARE, { x: 150, y: 50 })).toBe(0);
    expect(windingNumber(SQUARE, { x: 50, y: -10 })).toBe(0);
    expect(windingNumber(SQUARE, { x: 50, y: 110 })).toBe(0);
  });

  it("is 2 inside a box travelled twice, and 0 for a box and its reverse", () => {
    expect(windingNumber([...SQUARE, ...SQUARE], { x: 50, y: 50 })).toBe(2);
    expect(windingNumber([...SQUARE, ...REVERSED], { x: 50, y: 50 })).toBe(0);
  });

  it("is 0 for no piece", () => {
    expect(windingNumber([], { x: 50, y: 50 })).toBe(0);
  });

  it("counts a ray through a vertex once: (−10, 0) beside the top-left corner", () => {
    // The left side ends at (0, 0), y falling: its end counts, at x = 0 > −10, −1; the right
    // side starts at (100, 0), y growing: its start counts, +1; the top is flat. Outside: 0.
    expect(windingNumber(SQUARE, { x: -10, y: 0 })).toBe(0);
  });
});

describe("windingNumber (properties)", () => {
  test.prop({
    height: fc.integer({ max: 100, min: 1 }),
    width: fc.integer({ max: 100, min: 1 }),
    x: fc.integer({ max: 200, min: -100 }),
    y: fc.integer({ max: 200, min: -100 }),
  })("is 1 strictly inside a box and 0 outside it", ({ height, width, x, y }) => {
    // The point at half-integers, never on a side.
    const point = { x: x + 0.5, y: y + 0.5 };
    const box = polygonSegments([
      { x: 0, y: 0 },
      { x: width, y: 0 },
      { x: width, y: height },
      { x: 0, y: height },
    ]);
    const isInside = point.x > 0 && point.x < width && point.y > 0 && point.y < height;

    expect(windingNumber(box, point)).toBe(isInside ? 1 : 0);
  });
});
