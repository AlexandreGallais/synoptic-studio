import { describe, expect, it } from "vitest";

import { changedFields } from "./changed-fields";

describe("changedFields (US-077)", () => {
  it("[F41.AC1] names x alone for a rectangle nudged from (10, 20) to (11, 20)", () => {
    expect(changedFields({ width: 100, x: 10, y: 20 }, { width: 100, x: 11, y: 20 })).toEqual([
      "x",
    ]);
  });

  it("[F41.AC1] names the fields in the order of their names, y before x given", () => {
    expect(changedFields({ y: 20, x: 10 }, { y: 21, x: 11 })).toEqual(["x", "y"]);
  });

  it("[F41.AC1] compares a list of points by its text: one point moved changes corners", () => {
    const before = { corners: [{ x: 0 }, { x: 1 }] };
    const after = { corners: [{ x: 0 }, { x: 0.5 }] };

    expect(changedFields(before, after)).toEqual(["corners"]);
  });

  it("[F41.AC1] names a field only one of the two holds, either way", () => {
    expect(changedFields({ x: 1 }, { radii: [0], x: 1 })).toEqual(["radii"]);
    expect(changedFields({ radii: [0], x: 1 }, { x: 1 })).toEqual(["radii"]);
  });

  it("[F41.AC1] names none for two equal objects, and for two empty ones", () => {
    expect(changedFields({ x: 1, y: [2] }, { x: 1, y: [2] })).toEqual([]);
    expect(changedFields({}, {})).toEqual([]);
  });
});
