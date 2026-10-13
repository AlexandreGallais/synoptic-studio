import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { integerPoint } from "../../../tests";

import { stretchedHandles } from "./stretched-handles";

/** Handles on, in mode none, with their two vectors given. */
const HANDLES = {
  after: { x: 30, y: -15 },
  before: { x: -10, y: 4 },
  isOn: true,
  mode: "none",
} as const;

/** Vector with integer components. */
const VECTOR = integerPoint(1e3);

describe("stretchedHandles (REF-WIKI-BEZIER)", () => {
  it("[F12.AC3] scales each handle on each axis as its point is", () => {
    // Stored 1 × 1, drawn 200 × 100: (30, −15) → (6000, −1500), (−10, 4) → (−2000, 400).
    expect(stretchedHandles(HANDLES, { x: 200, y: 100 })).toEqual({
      ...HANDLES,
      after: { x: 6000, y: -1500 },
      before: { x: -2000, y: 400 },
    });
  });

  it("leaves the handles as they are at factors 1", () => {
    expect(stretchedHandles(HANDLES, { x: 1, y: 1 })).toEqual(HANDLES);
  });

  test.prop({ after: VECTOR, first: VECTOR, second: VECTOR })(
    "stretched twice is stretched once by the product of the factors",
    ({ after, first, second }) => {
      const handles = { ...HANDLES, after };
      const twice = stretchedHandles(stretchedHandles(handles, first), second);
      const once = stretchedHandles(handles, { x: first.x * second.x, y: first.y * second.y });

      expect(twice.after.x).toBeCloseTo(once.after.x, 6);
      expect(twice.after.y).toBeCloseTo(once.after.y, 6);
    },
  );
});
