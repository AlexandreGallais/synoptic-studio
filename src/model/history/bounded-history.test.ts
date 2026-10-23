import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_HISTORY, historyState, keptRectangleAt } from "../../../tests";

import { boundedHistory } from "./bounded-history";
import { historySize } from "./history-size";
import { undoneHistory } from "./undone-history";

import type { History } from "./history";
import type { KeptRectangle } from "../../../tests";

/** B, shared by every state. */
const SHAPE_B = keptRectangleAt(200);

/** A nudged twice, A selected: A and B, A′ and B past, A″ and B present; 2 504 bytes. */
const TWICE: History<KeptRectangle> = {
  future: [],
  past: [
    historyState([keptRectangleAt(10), SHAPE_B], [0]),
    historyState([keptRectangleAt(11), SHAPE_B], [0]),
  ],
  present: historyState([keptRectangleAt(12), SHAPE_B], [0]),
};

describe("boundedHistory (F40)", () => {
  it("[F40.AC4] drops A and B, the oldest, for 2 000 bytes and 1 step kept: 1 252 bytes left", () => {
    // 2 504 > 2 000; without A and B, A′ 1 228, 2 references 16, 1 index 8: 1 252 ≤ 2 000.
    const bounded = boundedHistory(TWICE, { size: 2000, steps: 1 });

    expect(bounded.past).toHaveLength(1);
    expect(bounded.past[0]).toBe(TWICE.past[1]);
    expect(bounded.present).toBe(TWICE.present);
    expect(historySize(bounded)).toBe(1252);
  });

  it("[F40.AC4] drops nothing below 5 steps kept, 2 504 bytes over 2 000", () => {
    expect(boundedHistory(TWICE, { size: 2000, steps: 5 })).toBe(TWICE);
  });

  it("[F40.AC4] drops nothing below 2 steps kept, the history's own count, even for 0 bytes", () => {
    expect(boundedHistory(TWICE, { size: 0, steps: 2 })).toBe(TWICE);
  });

  it("[F40.AC4] drops both past states for 1 000 bytes and 0 steps, a step larger than the size", () => {
    // Without A and B, 1 252 > 1 000: A′ and B goes too, 0 bytes left.
    const bounded = boundedHistory(TWICE, { size: 1000, steps: 0 });

    expect(bounded.past).toEqual([]);
    expect(historySize(bounded)).toBe(0);
  });

  it.each([
    { size: 3000, title: "3 000 bytes, over its size" },
    { size: 2504, title: "2 504 bytes, its size exactly" },
  ])("[F40.AC4] keeps the history itself for $title", ({ size }) => {
    expect(boundedHistory(TWICE, { size, steps: 0 })).toBe(TWICE);
  });

  it("[F40.AC4] drops the oldest for 2 503 bytes, one byte under its size", () => {
    expect(boundedHistory(TWICE, { size: 2503, steps: 0 }).past).toEqual([TWICE.past[1]]);
  });

  it("[F40.AC4] counts the state to come but never drops it", () => {
    // Undone once: A past, A″ to come, 2 504 bytes; the past dropped, A″ and B left: 1 252.
    const undone = undoneHistory(TWICE);

    for (const size of [2000, 1000]) {
      const bounded = boundedHistory(undone, { size, steps: 0 });

      expect(bounded.past).toEqual([]);
      expect(bounded.future).toBe(undone.future);
      expect(historySize(bounded)).toBe(1252);
    }
  });

  test.prop({
    bound: fc.record({ size: fc.nat({ max: 200 }), steps: fc.nat({ max: 6 }) }),
    history: ANY_HISTORY,
  })(
    "[F40.AC4] drops the fewest oldest past states for the size, never below the steps kept",
    ({ bound, history }) => {
      const before = JSON.stringify(history);
      const bounded = boundedHistory(history, bound);
      const dropped = history.past.length - bounded.past.length;
      const kept = Math.min(history.past.length, bound.steps);

      expect(bounded.past).toEqual(history.past.slice(dropped));
      expect(bounded.present).toBe(history.present);
      expect(bounded.future).toBe(history.future);
      expect(bounded.past.length).toBeGreaterThanOrEqual(kept);
      expect(historySize(bounded) <= bound.size || bounded.past.length === kept).toBe(true);

      expect(
        dropped === 0 ||
          historySize({ ...history, past: history.past.slice(dropped - 1) }) > bound.size,
      ).toBe(true);

      expect(JSON.stringify(history)).toBe(before);
    },
  );
});
