import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt, statesRecordingReads } from "../../../tests";
import { heldCount, shapeKey } from "../held-shapes";

import { measuredHistory } from "./measured-history";
import { sizedBoundedHistory } from "./sized-bounded-history";
import { sizedUndoneHistory } from "./sized-undone-history";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A nudged twice: A and B, A′ and B past, A″ and B present; 2 504 bytes. */
const TWICE = measuredHistory({
  future: [],
  past: [historyState([SHAPE_A, SHAPE_B], [0]), historyState([keptRectangleAt(11), SHAPE_B], [0])],
  present: historyState([keptRectangleAt(12), SHAPE_B], [0]),
});

/**
 * Bounds TWICE, its past states recording each property read, with no step kept.
 *
 * @param size - the most bytes the history holds
 * @returns the indices of the past states read, the oldest 0
 */
function readStates(size: number): readonly number[] {
  const reads: number[] = [];
  const past = statesRecordingReads(TWICE.past, reads);

  sizedBoundedHistory({ ...TWICE, past }, { size, steps: 0 });

  return [...new Set(reads)];
}

describe("sizedBoundedHistory", () => {
  it("[F41.AC2] drops A and B, the oldest, for 2 000 bytes and 1 step kept: 1 252 bytes left", () => {
    // 2 504 > 2 000; A and B removed: A held by none, not in the present, − 1 228 − 24: 1 252.
    const bounded = sizedBoundedHistory(TWICE, { size: 2000, steps: 1 });

    expect(bounded.size).toBe(1252);
    expect(bounded.past).toEqual([TWICE.past[1]]);
    expect(heldCount(bounded.held, shapeKey(SHAPE_A))).toBe(0);
  });

  it("[F41.AC2] keeps the steps kept over the size: 1 step of 1 252 bytes for 0", () => {
    expect(sizedBoundedHistory(TWICE, { size: 0, steps: 1 }).size).toBe(1252);
  });

  it("[F41.AC2] drops every past state for 0 bytes and no step kept", () => {
    const bounded = sizedBoundedHistory(TWICE, { size: 0, steps: 0 });

    expect(bounded).toMatchObject({ future: [], past: [], present: TWICE.present, size: 0 });
  });

  it.each([
    { bound: { size: 2504, steps: 0 }, title: "within its size" },
    { bound: { size: 0, steps: 2 }, title: "within its steps kept" },
  ])("[F41.AC2] gives the history itself $title", ({ bound }) => {
    expect(sizedBoundedHistory(TWICE, bound)).toBe(TWICE);
  });

  it("[F41.AC2] never drops the states to come: A″ and B left, 1 252 bytes", () => {
    // Undone: A and B past, A′ and B present, A″ and B to come, 2 504; A and B dropped: 1 252.
    const bounded = sizedBoundedHistory(sizedUndoneHistory(TWICE), { size: 0, steps: 0 });

    expect(bounded.past).toEqual([]);
    expect(bounded.future).toHaveLength(1);
    expect(bounded.size).toBe(1252);
  });

  it.each([
    { read: [], size: 2504, title: "none within its size" },
    { read: [0], size: 2000, title: "the oldest alone to drop it" },
    { read: [0, 1], size: 0, title: "both to drop both" },
  ])("[F41.AC2] reads $title", ({ read, size }) => {
    expect(readStates(size)).toEqual(read);
  });
});
