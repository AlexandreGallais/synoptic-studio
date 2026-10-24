import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../../tests";
import { NO_HANDLES } from "../../geometry";
import { rectangleAsFreeShape } from "../rectangle";

import { shapeMovedBy } from "./shape-moved-by";

import type { FreeShape } from "../free-shape";

/** B, 100 × 50 at (200, 20), sharp, turned by 30° about its middle (250, 45). */
const SHAPE_B = rectangleAsFreeShape({
  angle: 30,
  height: 50,
  radii: [0, 0, 0, 0],
  width: 100,
  x: 200,
  y: 20,
});

/** A free shape 10.5 wide and 20.25 high at (3.5, 7), its points holding fractions. */
const FRACTIONAL: FreeShape = {
  angle: 0,
  corners: [
    { handles: NO_HANDLES, point: { x: 0.25, y: 0 }, radius: 0 },
    { handles: NO_HANDLES, point: { x: 10.75, y: 0.5 }, radius: 2 },
    { handles: NO_HANDLES, point: { x: 5.5, y: 20.75 }, radius: 0 },
  ],
  height: 20.25,
  width: 10.5,
  x: 3.5,
  y: 7,
};

describe("shapeMovedBy (US-072)", () => {
  it("[F39.AC2] moves B by (10.80127, 3.65064), its angle, size and corners kept", () => {
    // Its position (200, 20) goes to (210.80127, 23.65064), its straight box to (205, 2).
    const moved = shapeMovedBy(SHAPE_B, { x: 10.80127, y: 3.65064 });

    expect(moved).toEqual({ ...SHAPE_B, x: 210.80127, y: 23.65064 });
  });

  it("[F39.AC2] keeps a free shape 10.5 wide, its points' fractions kept, only its place moved", () => {
    // (3.5, 7) + (31, 19) = (34.5, 26).
    expect(shapeMovedBy(FRACTIONAL, { x: 31, y: 19 })).toEqual({ ...FRACTIONAL, x: 34.5, y: 26 });
  });

  it("[F39.AC2] moves a text the same way, its content and options kept", () => {
    // (300, 20) + (−20, 45.5) = (280, 65.5).
    expect(shapeMovedBy({ ...HELLO_TEXT, x: 300, y: 20 }, { x: -20, y: 45.5 })).toEqual({
      ...HELLO_TEXT,
      x: 280,
      y: 65.5,
    });
  });

  it("keeps a shape moved by (0, 0) as it is", () => {
    expect(shapeMovedBy(FRACTIONAL, { x: 0, y: 0 })).toEqual(FRACTIONAL);
  });

  test.prop({
    x: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
    y: fc.double({ max: 1e5, min: -1e5, noNaN: true }),
  })("moves the position by the shift and nothing else", (shift) => {
    const moved = shapeMovedBy(FRACTIONAL, shift);

    expect(moved).toEqual({ ...FRACTIONAL, x: FRACTIONAL.x + shift.x, y: FRACTIONAL.y + shift.y });
  });
});
