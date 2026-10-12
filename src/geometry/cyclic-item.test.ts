import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { cyclicItem } from "./cyclic-item";

/** Rectangle 100 × 50 of US-001, clockwise from the top-left vertex. */
const RECTANGLE = [
  { x: 0, y: 0 },
  { x: 100, y: 0 },
  { x: 100, y: 50 },
  { x: 0, y: 50 },
];

/** Fallback returned for an empty contour. */
const FALLBACK = { x: -1, y: -1 };

describe("cyclicItem", () => {
  it("reads the vertex at an index inside the contour", () => {
    expect(cyclicItem(RECTANGLE, 2, FALLBACK)).toEqual({ x: 100, y: 50 });
  });

  it("wraps around: the vertex before the first one is the last one (Q11)", () => {
    expect(cyclicItem(RECTANGLE, -1, FALLBACK)).toEqual({ x: 0, y: 50 });
  });

  it("wraps around: the vertex after the last one is the first one (Q11)", () => {
    expect(cyclicItem(RECTANGLE, 4, FALLBACK)).toEqual({ x: 0, y: 0 });
  });

  it("returns the fallback for an empty contour", () => {
    expect(cyclicItem([], 0, FALLBACK)).toEqual(FALLBACK);
  });

  it("wraps around for any integer index, even several turns away", () => {
    // −5 mod 4 = 3 and 9 mod 4 = 1, with a non-negative modulo (ADR-0025).
    expect(cyclicItem(RECTANGLE, -5, FALLBACK)).toEqual({ x: 0, y: 50 });
    expect(cyclicItem(RECTANGLE, 9, FALLBACK)).toEqual({ x: 100, y: 0 });
  });

  it("works for one value per vertex as well", () => {
    // Radii of the four corners; the radius before the first corner is the last one.
    expect(cyclicItem([10, 20, 30, 40], -1, 0)).toBe(40);
  });
});

describe("cyclicItem (properties)", () => {
  test.prop({
    index: fc.integer({ max: 1000, min: -1000 }),
    items: fc.array(fc.integer(), { minLength: 1 }),
  })("does not change after a whole turn", ({ index, items }) => {
    expect(cyclicItem(items, index + items.length, 0)).toBe(cyclicItem(items, index, 0));
  });
});
