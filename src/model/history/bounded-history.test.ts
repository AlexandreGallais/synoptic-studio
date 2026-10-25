import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_HISTORY, historyState, keptRectangleAt } from "../../../tests";

import { boundedHistory } from "./bounded-history";
import { historySize } from "./history-size";
import { historyWithStep } from "./history-with-step";
import { undoneHistory } from "./undone-history";

import type { History } from "./history";
import type { HistoryBound } from "./history-bound";
import type { KeptRectangle } from "../../../tests";

/** B, shared by every state. */
const SHAPE_B = keptRectangleAt(200);

/**
 * Counts how many times a history is measured as it is bounded: each measure (`historySize`) reads
 * the selection of every past state it keeps once, so that the newest, never dropped here, is read
 * once a measure.
 *
 * @param history - the history bounded, at least one state past, never all dropped
 * @param bound - the memory size and the steps kept
 * @returns how many times `historySize` read it
 */
function countedMeasures(history: History<KeptRectangle>, bound: HistoryBound): number {
  const reads: string[] = [];
  const newest = history.past.at(-1) ?? historyState([], []);
  const counted = new Proxy(newest, {
    get: (state, key): unknown => {
      reads.push(String(key));

      return Reflect.get(state, key);
    },
  });

  boundedHistory({ ...history, past: [...history.past.slice(0, -1), counted] }, bound);

  return reads.filter((key) => key === "selection").length;
}

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

  it.each([
    { measures: 1, size: 2504, title: "once within its size" },
    { measures: 2, size: 2000, title: "twice over it by its oldest state" },
  ])("[F40.AC4] measures the history $title", ({ measures, size }) => {
    // 2 504 ≤ 2 504: measured once, nothing dropped; 2 504 > 2 000, then 1 252 ≤ 2 000 with 1
    // dropped (`doubledRange`): twice, the range from 1 to 1 needing no halving.
    expect(countedMeasures(TWICE, { size, steps: 0 })).toBe(measures);
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

/**
 * Moves A again and again, from x 10 to x 10 + count, each move a step bounded as the page bounds
 * it, A selected and drawn first, the other shapes shared by every state.
 *
 * @param others - the shapes drawn after A, never moved
 * @param count - how many moves
 * @param size - the memory size, 0 steps kept
 * @returns the history after the last move
 */
function movedHistory(
  others: readonly KeptRectangle[],
  count: number,
  size: number,
): History<KeptRectangle> {
  const start: History<KeptRectangle> = {
    future: [],
    past: [],
    present: historyState([keptRectangleAt(10), ...others], [0]),
  };
  const moves = Array.from({ length: count })
    .keys()
    .map((index) => 11 + index);

  return moves.reduce((history, x) => {
    const moved = historyState([keptRectangleAt(x), ...others], [0]);

    return boundedHistory(historyWithStep(history, moved), { size, steps: 0 });
  }, start);
}

describe("boundedHistory, A moved again and again", () => {
  it("[F40.AC4] keeps 10 moves of A alone in 12 440 bytes, 1 244 bytes each", () => {
    // A as it was, 614 characters, 1 228 bytes, its reference 8 and its index 8: 1 244 a move;
    // 10 × 1 244 = 12 440, within the size.
    const history = movedHistory([], 10, 12_440);

    expect(history.past).toHaveLength(10);
    expect(historySize(history)).toBe(12_440);
  });

  it("[F40.AC4] drops the first move at the eleventh, the oldest first", () => {
    // 11 × 1 244 = 13 684 > 12 440: A at x 10, the first state, dropped; A at x 11 the oldest.
    const history = movedHistory([], 11, 12_440);

    expect(history.past).toHaveLength(10);
    expect(history.past[0]?.drawing[0]).toEqual(keptRectangleAt(11));
    expect(historySize(history)).toBe(12_440);
  });

  it("[F40.AC4] keeps 2 moves of A in a drawing of 100 rectangles in 4 072 bytes, 2 036 each", () => {
    // A as it was, 1 228, the 100 references of its state, 800, its index, 8: 2 036 a move;
    // 3 × 2 036 = 6 108 > 4 072 = 2 × 2 036; the 99 others shared with the present, not counted.
    const indices = Array.from({ length: 99 }).keys();
    const others = indices.map((index) => keptRectangleAt(200 + index)).toArray();
    const history = movedHistory(others, 3, 4072);

    expect(history.past).toHaveLength(2);
    expect(historySize(history)).toBe(4072);
  });
});

describe("boundedHistory, a size just set smaller", () => {
  it("[F40.AC4] measures 10 moves of A 7 times to drop 7 for 3 732 bytes", () => {
    // 12 440 > 3 732: 1; doubling, 1, 2 and 4 dropped leave 9, 8 and 6 moves, over, 8 leaves 2,
    // within: 4 more; halving from 4 + 1 = 5 to 8, 6 leaves 4, over, then 7 leaves 3, within:
    // 2 more, 7 in all.
    const history = movedHistory([], 10, 12_440);

    expect(countedMeasures(history, { size: 3732, steps: 0 })).toBe(7);
  });

  it("[F40.AC4] drops 7 of 10 moves of A at once for 3 732 bytes, 3 moves", () => {
    // 10 × 1 244 = 12 440 > 3 732 = 3 × 1 244: the 7 oldest dropped, A at x 17 the oldest left.
    const history = boundedHistory(movedHistory([], 10, 12_440), { size: 3732, steps: 0 });

    expect(history.past).toHaveLength(3);
    expect(history.past[0]?.drawing[0]).toEqual(keptRectangleAt(17));
    expect(historySize(history)).toBe(3732);
  });

  it.each([
    { size: 3731, want: 2, why: "one byte under 3 moves" },
    { size: 12_439, want: 9, why: "one byte under 10 moves" },
    { size: 1243, want: 0, why: "one byte under 1 move" },
  ])("[F40.AC4] keeps $want of 10 moves for $size bytes, $why", ({ size, want }) => {
    const history = boundedHistory(movedHistory([], 10, 12_440), { size, steps: 0 });

    expect(history.past).toHaveLength(want);
    expect(historySize(history)).toBe(want * 1244);
  });

  it("[F40.AC4] keeps 4 of 10 moves for 0 bytes and 4 steps kept", () => {
    const history = boundedHistory(movedHistory([], 10, 12_440), { size: 0, steps: 4 });

    expect(history.past).toHaveLength(4);
  });
});
