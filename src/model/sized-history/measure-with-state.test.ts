import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { EMPTY_HELD_COUNTS, heldCount, shapeKey } from "../held-shapes";

import { measureWithState } from "./measure-with-state";

import type { HistoryMeasure } from "./history-measure";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** The present: A′, A nudged to (11, 20), and B. */
const PRESENT = new Set([keptRectangleAt(11), SHAPE_B]);

/** The measure of no state. */
const NONE: HistoryMeasure = { held: EMPTY_HELD_COUNTS, size: 0 };

/** A and B, A selected, added once. */
const ONCE = measureWithState(NONE, historyState([SHAPE_A, SHAPE_B], [0]), PRESENT);

describe("measureWithState (EN-147)", () => {
  it("[F41.AC2] adds A, not in the present, and three slots: 1 252 bytes", () => {
    // A 0 → 1, not in the present: 614 × 2 = 1 228; B in the present: 0; 3 × 8 = 24: 1 252.
    expect(ONCE.size).toBe(1252);

    expect([
      heldCount(ONCE.held, shapeKey(SHAPE_A)),
      heldCount(ONCE.held, shapeKey(SHAPE_B)),
    ]).toEqual([1, 1]);
  });

  it("[F41.AC2] adds the slots alone for a state whose shapes another holds: 1 276 bytes", () => {
    // A 1 → 2, B 1 → 2: no text; 3 × 8 = 24: 1 252 + 24 = 1 276.
    const twice = measureWithState(ONCE, historyState([SHAPE_A, SHAPE_B], [0]), PRESENT);

    expect(twice.size).toBe(1276);

    expect([
      heldCount(twice.held, shapeKey(SHAPE_A)),
      heldCount(twice.held, shapeKey(SHAPE_B)),
    ]).toEqual([2, 2]);
  });

  it("[F41.AC2] counts a shape a state holds twice once, its two references both: 1 244 bytes", () => {
    // A 0 → 1 once: 1 228; two references, 16: 1 244.
    const doubled = measureWithState(NONE, historyState([SHAPE_A, SHAPE_A], []), PRESENT);

    expect(doubled.size).toBe(1244);
    expect(heldCount(doubled.held, shapeKey(SHAPE_A))).toBe(1);
  });

  it("[F41.AC2] adds nothing for an empty state", () => {
    expect(measureWithState(ONCE, historyState([], []), PRESENT)).toEqual(ONCE);
  });
});
