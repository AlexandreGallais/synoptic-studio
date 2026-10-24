import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { shapeSelectionAfterClick } from "./shape-selection-after-click";

// A, B and C drawn in that order: their indices 0, 1 and 2; −1 is the empty drawing.
describe("the shapes selected after a click with the Selector (US-068)", () => {
  it("[F37.AC2] adds B to A with Shift, and takes B out again with Shift", () => {
    expect(shapeSelectionAfterClick([0], 1, true)).toEqual([0, 1]);
    expect(shapeSelectionAfterClick([0, 1], 1, true)).toEqual([0]);
  });

  it("[F37.AC2] keeps the indices in the order drawn, A added after C", () => {
    expect(shapeSelectionAfterClick([2], 0, true)).toEqual([0, 2]);
  });

  it("[F37.AC2] selects the shape clicked alone without Shift, a selected one included", () => {
    expect(shapeSelectionAfterClick([0, 1], 2, false)).toEqual([2]);
    expect(shapeSelectionAfterClick([0, 1], 1, false)).toEqual([1]);
    expect(shapeSelectionAfterClick([], 0, false)).toEqual([0]);
  });

  it("[F37.AC2] keeps the selection on the empty drawing with Shift, and selects none without", () => {
    expect(shapeSelectionAfterClick([0, 1], -1, true)).toEqual([0, 1]);
    expect(shapeSelectionAfterClick([0, 1], -1, false)).toEqual([]);
    expect(shapeSelectionAfterClick([], -1, true)).toEqual([]);
  });

  it("[F37.AC2] takes the last shape out with Shift, leaving none", () => {
    expect(shapeSelectionAfterClick([1], 1, true)).toEqual([]);
  });

  test.prop({
    index: fc.integer({ max: 11, min: -1 }),
    isShift: fc.boolean(),
    selected: fc.uniqueArray(fc.nat({ max: 11 })),
  })(
    "gives indices ascending without repeat, the shape clicked toggled with Shift",
    ({ index, isShift, selected }) => {
      const sorted = selected.toSorted((left, right) => left - right);
      const after = shapeSelectionAfterClick(sorted, index, isShift);

      expect(after).toEqual([...new Set(after)].toSorted((left, right) => left - right));

      if (!isShift) {
        expect(after).toEqual(index < 0 ? [] : [index]);

        return;
      }

      // Every other shape keeps its state; the one clicked changes it, none on the empty drawing.
      const others = after.filter((other) => other !== index);

      expect(others).toEqual(sorted.filter((other) => other !== index));
      expect(after.includes(index)).toBe(index >= 0 && !sorted.includes(index));
    },
  );
});
