import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { shapeSelectionAfterBox } from "./shape-selection-after-box";

/**
 * Orders two indices ascending.
 *
 * @param left - one index
 * @param right - another
 * @returns their difference, negative when left comes first
 */
function ascending(left: number, right: number): number {
  return left - right;
}

// A, B and C drawn in that order: their indices 0, 1 and 2.
describe("the shapes selected after a selection rectangle", () => {
  it("[F37.AC3] selects the shapes taken alone without Shift, those selected before dropped", () => {
    expect(shapeSelectionAfterBox([2], [0, 1], false)).toEqual([0, 1]);
    expect(shapeSelectionAfterBox([0], [1], false)).toEqual([1]);
  });

  it("[F37.AC3] adds the shapes taken to those selected with Shift, in the order drawn", () => {
    // A selected, B taken: A and B. C selected, A taken: A before C.
    expect(shapeSelectionAfterBox([0], [1], true)).toEqual([0, 1]);
    expect(shapeSelectionAfterBox([2], [0], true)).toEqual([0, 2]);
  });

  it("[F37.AC3] never takes a shape out with Shift, one taken and selected kept once", () => {
    expect(shapeSelectionAfterBox([0, 1], [1, 2], true)).toEqual([0, 1, 2]);
  });

  it("[F37.AC3] selects none for a rectangle taking none, and keeps the selection with Shift", () => {
    expect(shapeSelectionAfterBox([0, 2], [], false)).toEqual([]);
    expect(shapeSelectionAfterBox([0, 2], [], true)).toEqual([0, 2]);
    expect(shapeSelectionAfterBox([], [], true)).toEqual([]);
  });

  test.prop({
    isShift: fc.boolean(),
    selected: fc.uniqueArray(fc.nat({ max: 11 })),
    taken: fc.uniqueArray(fc.nat({ max: 11 })),
  })(
    "gives indices ascending without repeat: the shapes taken, and with Shift those selected",
    ({ isShift, selected, taken }) => {
      const before = selected.toSorted(ascending);
      const inBox = taken.toSorted(ascending);
      const after = shapeSelectionAfterBox(before, inBox, isShift);

      expect(after).toEqual([...new Set(after)].toSorted(ascending));

      expect(after.filter((index) => !inBox.includes(index))).toEqual(
        isShift ? before.filter((index) => !inBox.includes(index)) : [],
      );

      expect(inBox.every((index) => after.includes(index))).toBe(true);
    },
  );
});
