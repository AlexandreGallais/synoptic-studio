import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { shapeSelectionAfterPress } from "./shape-selection-after-press";

// A, B and C drawn in that order: their indices 0, 1 and 2.
describe("the shapes selected once a press on a shape moves (US-072)", () => {
  it("[F39.AC3] selects A alone, pressed while B is selected", () => {
    expect(shapeSelectionAfterPress([1], 0, false)).toEqual([0]);
    expect(shapeSelectionAfterPress([], 2, false)).toEqual([2]);
  });

  it("[F39.AC2] keeps A and B, A pressed among them, so that both move", () => {
    expect(shapeSelectionAfterPress([0, 1], 0, false)).toEqual([0, 1]);
    expect(shapeSelectionAfterPress([0, 1, 2], 2, false)).toEqual([0, 1, 2]);
  });

  it("[F39.AC3] adds B to A, pressed with Shift, in the order drawn", () => {
    expect(shapeSelectionAfterPress([0], 1, true)).toEqual([0, 1]);
    expect(shapeSelectionAfterPress([2], 0, true)).toEqual([0, 2]);
    expect(shapeSelectionAfterPress([], 1, true)).toEqual([1]);
  });

  it("[F39.AC2] keeps A and B, A pressed with Shift among them, never taking it out", () => {
    expect(shapeSelectionAfterPress([0, 1], 0, true)).toEqual([0, 1]);
  });

  it("keeps the selection for a press on no shape", () => {
    expect(shapeSelectionAfterPress([0, 2], -1, false)).toEqual([0, 2]);
    expect(shapeSelectionAfterPress([0, 2], -1, true)).toEqual([0, 2]);
  });

  test.prop({
    index: fc.nat({ max: 11 }),
    isShift: fc.boolean(),
    selected: fc.uniqueArray(fc.nat({ max: 11 })),
  })(
    "holds the shape pressed, ascending without repeat, every shape selected kept with Shift",
    ({ index, isShift, selected }) => {
      const before = selected.toSorted((left, right) => left - right);
      const after = shapeSelectionAfterPress(before, index, isShift);

      expect(after).toContain(index);
      expect(after).toEqual([...new Set(after)].toSorted((left, right) => left - right));

      if (isShift || before.includes(index)) {
        expect(after).toEqual(expect.arrayContaining([...before]));
      } else {
        expect(after).toEqual([index]);
      }
    },
  );
});
