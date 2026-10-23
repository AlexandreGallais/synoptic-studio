import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { keptRectangleAt } from "../../../tests";

import { areSameItems } from "./are-same-items";

/** A, a rectangle at (10, 20), as a state holds it. */
const SHAPE_A = keptRectangleAt(10);

/** B, a rectangle at (200, 20), as a state holds it. */
const SHAPE_B = keptRectangleAt(200);

describe("areSameItems (F40)", () => {
  it.each([
    {
      first: [SHAPE_A, SHAPE_B],
      second: [SHAPE_A, SHAPE_B],
      title: "the same objects in the same order",
      want: true,
    },
    {
      first: [SHAPE_A, SHAPE_B],
      second: [SHAPE_B, SHAPE_A],
      title: "the same objects in another order",
      want: false,
    },
    {
      first: [SHAPE_A, SHAPE_B],
      second: [SHAPE_A],
      title: "a list longer than the other",
      want: false,
    },
    {
      first: [SHAPE_A],
      second: [SHAPE_A, SHAPE_B],
      title: "a list shorter than the other",
      want: false,
    },
    { first: [], second: [], title: "two empty lists", want: true },
  ])("[F40.AC3] tells $title: $want", ({ first, second, want }) => {
    expect(areSameItems(first, second)).toBe(want);
  });

  it("[F40.AC3] tells a copy equal by its text from the object itself", () => {
    expect(areSameItems([SHAPE_A, SHAPE_B], [SHAPE_A, keptRectangleAt(200)])).toBe(false);
  });

  it.each([
    { first: [0], second: [0], want: true },
    { first: [0, 1], second: [0, 1], want: true },
    { first: [0, 1], second: [1, 0], want: false },
    { first: [1], second: [2], want: false },
  ])("[F40.AC3] tells the selection $first from $second: $want", ({ first, second, want }) => {
    expect(areSameItems(first, second)).toBe(want);
  });

  test.prop({ list: fc.array(fc.integer(), { maxLength: 6 }) })(
    "[F40.AC3] finds a list the same as its copy, and not with one more item",
    ({ list }) => {
      expect(areSameItems(list, [...list])).toBe(true);
      expect(areSameItems(list, [...list, 0])).toBe(false);
    },
  );
});
