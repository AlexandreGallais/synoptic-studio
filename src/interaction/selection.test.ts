import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { NO_VERTEX_SELECTION } from "./no-vertex-selection";
import { selectionAfterClick } from "./selection-after-click";
import { selectionAfterHandleClick } from "./selection-after-handle-click";
import { selectionAfterRange } from "./selection-after-range";
import { selectionAfterRowClick } from "./selection-after-row-click";
import { selectionAfterToggle } from "./selection-after-toggle";

describe("vertex selection", () => {
  it("[F03.AC5] selects the vertex clicked alone, the start of the next range", () => {
    expect(selectionAfterClick(2)).toEqual({ selected: [2], start: 2 });
  });

  it("[F03.AC5] adds a vertex with Ctrl + click, and removes it with a second one", () => {
    const two = selectionAfterToggle(selectionAfterClick(4), 1);

    expect(two).toEqual({ selected: [1, 4], start: 1 });
    expect(selectionAfterToggle(two, 4)).toEqual({ selected: [1], start: 4 });
  });

  it("[F03.AC5] selects a range from its start, in either direction, replacing the selection", () => {
    // Click vertex 2, Ctrl + click vertex 5 (start 5), Shift + click vertex 3: 3 to 5 only.
    const start = selectionAfterToggle(selectionAfterClick(2), 5);

    expect(selectionAfterRange(start, 3)).toEqual({ selected: [3, 4, 5], start: 5 });

    expect(selectionAfterRange(selectionAfterClick(1), 3)).toEqual({
      selected: [1, 2, 3],
      start: 1,
    });
  });

  it("[F03.AC5] acts as a plain click for a range without a start", () => {
    expect(selectionAfterRange(NO_VERTEX_SELECTION, 3)).toEqual({ selected: [3], start: 3 });
  });

  test.prop([fc.uniqueArray(fc.nat({ max: 11 })), fc.nat({ max: 11 })])(
    "keeps the selection ascending and without repeat after a toggle",
    (selected, vertex) => {
      const sorted = selected.toSorted((left, right) => left - right);
      const after = selectionAfterToggle({ selected: sorted, start: undefined }, vertex).selected;

      expect(after).toEqual([...new Set(after)].toSorted((left, right) => left - right));
      expect(after.includes(vertex)).toBe(!sorted.includes(vertex));
    },
  );
});

describe("vertex selection by click", () => {
  const plain = { shift: false, toggle: false };
  const withShift = { shift: true, toggle: false };
  const withToggle = { shift: false, toggle: true };
  const fromTwo = selectionAfterClick(2);

  it("[F03.AC5] in the list: a plain click, Ctrl for one more, Shift for a range", () => {
    expect(selectionAfterRowClick(fromTwo, 4, plain)).toEqual({ selected: [4], start: 4 });
    expect(selectionAfterRowClick(fromTwo, 4, withToggle)).toEqual({ selected: [2, 4], start: 4 });

    expect(selectionAfterRowClick(fromTwo, 4, withShift)).toEqual({
      selected: [2, 3, 4],
      start: 2,
    });
  });

  it("[F03.AC5] on the drawing: Shift or Ctrl adds or removes, no range", () => {
    expect(selectionAfterHandleClick(fromTwo, 4, plain)).toEqual({ selected: [4], start: 4 });

    expect(selectionAfterHandleClick(fromTwo, 4, withToggle)).toEqual({
      selected: [2, 4],
      start: 4,
    });

    expect(selectionAfterHandleClick(fromTwo, 4, withShift)).toEqual({
      selected: [2, 4],
      start: 4,
    });
  });
});
