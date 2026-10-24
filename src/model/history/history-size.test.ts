import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_HISTORY, historyState, keptRectangleAt } from "../../../tests";

import { historySize } from "./history-size";
import { historyWithStep } from "./history-with-step";
import { undoneHistory } from "./undone-history";

import type { History } from "./history";
import type { KeptRectangle } from "../../../tests";

/** A and B, A selected, none past nor to come. */
const START: History<KeptRectangle> = {
  future: [],
  past: [],
  present: historyState([keptRectangleAt(10), keptRectangleAt(200)], [0]),
};

/** A nudged once: A and B past, A′ and B present, A selected in both. */
const NUDGED = historyWithStep(
  START,
  historyState([keptRectangleAt(11), keptRectangleAt(200)], [0]),
);

/** A nudged twice: A and B, A′ and B past, A″ and B present. */
const TWICE = historyWithStep(
  NUDGED,
  historyState([keptRectangleAt(12), keptRectangleAt(200)], [0]),
);

describe("historySize (F40)", () => {
  it("[F40.AC4] counts nothing with nothing past nor to come", () => {
    expect(historySize(START)).toBe(0);
  });

  it("[F40.AC4] counts A, its two references and its index after one nudge: 1 252 bytes", () => {
    // A held by the past alone: 614 × 2 = 1 228; B shared with the present, not counted;
    // the past state's references to A and B, 2 × 8 = 16, and its index 0, 8: 1 252.
    expect(historySize(NUDGED)).toBe(1252);
  });

  it("[F40.AC4] counts A and A′, four references and two indices after two nudges: 2 504 bytes", () => {
    // A and A′, 1 228 each: 2 456; B held by the present; 4 × 8 = 32; 2 × 8 = 16: 2 504.
    expect(historySize(TWICE)).toBe(2504);
  });

  it("[F40.AC4] counts the state to come as a past one: 2 504 bytes undone once", () => {
    // A′ and B present: A past, A″ to come, 1 228 each; 4 references, 32; 2 indices, 16.
    expect(historySize(undoneHistory(TWICE))).toBe(2504);
  });

  it("[F40.AC4] counts a shape held by two past states once", () => {
    // A in both past states: 1 228 once; 2 references, 16; no index: 1 244.
    const shape = keptRectangleAt(10);
    const twice: History<KeptRectangle> = {
      future: [],
      past: [historyState([shape], []), historyState([shape], [])],
      present: historyState([keptRectangleAt(11)], []),
    };

    expect(historySize(twice)).toBe(1244);
  });

  it("[F40.AC4] counts a copy of the present's B, not shared, as a shape of its own", () => {
    // B copied: 615 × 2 = 1 230, and one reference, 8: 1 238.
    const copied: History<KeptRectangle> = {
      future: [],
      past: [historyState([keptRectangleAt(200)], [])],
      present: historyState([keptRectangleAt(200)], []),
    };

    expect(historySize(copied)).toBe(1238);
  });

  it("[F40.AC4] counts the references only for a past state the present holds whole", () => {
    // A, shared, and none selected then A selected: two references to one shape, 8 + 8; one index, 8.
    const shape = keptRectangleAt(10);
    const history: History<KeptRectangle> = {
      future: [historyState([shape], [0])],
      past: [historyState([shape], [])],
      present: historyState([shape, keptRectangleAt(200)], [1]),
    };

    expect(historySize(history)).toBe(24);
  });

  test.prop({ history: ANY_HISTORY })(
    "[F40.AC4] counts at least 8 bytes a reference and an index, 0 with nothing past nor to come",
    ({ history }) => {
      const before = JSON.stringify(history);
      const kept = [...history.past, ...history.future];
      const references = kept.reduce(
        (sum, state) => sum + state.drawing.length + state.selection.length,
        0,
      );

      expect(historySize(history)).toBeGreaterThanOrEqual(references * 8);
      expect(historySize({ ...history, future: [], past: [] })).toBe(0);
      expect(JSON.stringify(history)).toBe(before);
    },
  );
});
