import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt, statesRecordingReads } from "../../../tests";
import { heldCount, shapeKey } from "../held-shapes";
import { DEFAULT_HISTORY_BOUND } from "../history";

import { measuredHistory } from "./measured-history";
import { sizedBoundedHistory } from "./sized-bounded-history";
import { sizedHistoryWithStep } from "./sized-history-with-step";

import type { HistoryBound } from "../history";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A and B, A selected, none past nor to come: 0 bytes. */
const START = measuredHistory({
  future: [],
  past: [],
  present: historyState([SHAPE_A, SHAPE_B], [0]),
});

/** A nudged once, B read again as a new object: A and B past, A′ and B present. */
const NUDGED = sizedHistoryWithStep(
  START,
  historyState([keptRectangleAt(11), keptRectangleAt(200)], [0]),
);

/**
 * A moved 1 000 times, each past state a new A at (10, 20), 614 characters, and its index: 1 000 ×
 * (1 228 + 16) = 1 244 000 bytes; A′ at (11, 20) present.
 */
const MOVED = measuredHistory({
  future: [],
  past: Array.from({ length: 1000 }, () => historyState([keptRectangleAt(10)], [0])),
  present: historyState([keptRectangleAt(11)], [0]),
});

/**
 * Takes a step to A″ at (12, 20) and bounds the history moved 1 000 times, its past states
 * recording each property read.
 *
 * @param bound - the bound applied after the step
 * @returns the indices of the past states read, the oldest 0
 */
function readAtStep(bound: HistoryBound): readonly number[] {
  const reads: number[] = [];
  const history = { ...MOVED, past: statesRecordingReads(MOVED.past, reads) };
  const stepped = sizedHistoryWithStep(history, historyState([keptRectangleAt(12)], [0]));

  sizedBoundedHistory(stepped, bound);

  return [...new Set(reads)];
}

describe("sizedHistoryWithStep (EN-147)", () => {
  it("[F41.AC2] keeps A, two references and an index after one nudge: 1 252 bytes", () => {
    // The present added to the past: A and B in the present, 3 × 8 = 24; then A left by the
    // present, held by 1: + 1 228 = 1 252; B shared.
    expect(NUDGED.size).toBe(1252);
    expect(NUDGED.past).toEqual([START.present]);
    expect(NUDGED.present.drawing[1]).toBe(SHAPE_B);
  });

  it("[F41.AC2] keeps A and A′ after two nudges: 2 504 bytes", () => {
    // A′ and B added, 24; A′ left, + 1 228: 1 252 + 1 252 = 2 504.
    const twice = sizedHistoryWithStep(NUDGED, historyState([keptRectangleAt(12), SHAPE_B], [0]));

    expect(twice.size).toBe(2504);
  });

  it("[F41.AC2] keeps the measure when the selection alone is replaced", () => {
    const selected = sizedHistoryWithStep(START, historyState([SHAPE_A, SHAPE_B], [1]));

    expect(selected).toEqual({ ...START, present: historyState([SHAPE_A, SHAPE_B], [1]) });
    expect(selected.held).toBe(START.held);
    expect(selected.past).toBe(START.past);
  });

  it("[F41.AC2] gives the history itself when nothing changed", () => {
    const again = historyState([keptRectangleAt(11), SHAPE_B], [0]);

    expect(sizedHistoryWithStep(NUDGED, again)).toBe(NUDGED);
  });

  it("[F41.AC2] drops A″ with the state to come a step removes: 2 504 bytes", () => {
    // A and B past, A′ and B present, A″ and B to come: 2 504. A‴ recorded: A′ and B added, 24;
    // A″ held by none, not in the present, − 1 228 − 24; A′ left by the present, + 1 228: 2 504.
    const nudged = keptRectangleAt(12);
    const undone = measuredHistory({
      future: [historyState([nudged, SHAPE_B], [0])],
      past: [historyState([SHAPE_A, SHAPE_B], [0])],
      present: historyState([keptRectangleAt(11), SHAPE_B], [0]),
    });
    const stepped = sizedHistoryWithStep(undone, historyState([keptRectangleAt(13), SHAPE_B], [0]));

    expect(stepped.size).toBe(2504);
    expect(stepped.future).toEqual([]);
    expect(heldCount(stepped.held, shapeKey(nudged))).toBe(0);
  });

  it.each([
    // A′ added to the past, 16, and left by the present, 1 228: 1 245 244 bytes, within 100 MB.
    { bound: DEFAULT_HISTORY_BOUND, read: [], title: "no past state within the size" },
    // 1 245 244 > 1 244 000; the oldest dropped, − 1 244: 1 244 000, within the size.
    {
      bound: { size: 1_244_000, steps: 5 },
      read: [0],
      title: "the oldest alone over the size by it",
    },
  ])("[F41.AC2] reads $title at a step of 1 000 moves", ({ bound, read }) => {
    expect(readAtStep(bound)).toEqual(read);
  });
});
