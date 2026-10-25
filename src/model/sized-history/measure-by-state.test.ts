import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { EMPTY_HELD_COUNTS, heldCount, shapeKey } from "../held-shapes";

import { measureByState } from "./measure-by-state";

import type { HistoryMeasure } from "./history-measure";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A and B, A selected. */
const STATE = historyState([SHAPE_A, SHAPE_B], [0]);

/** The present: A′, A nudged to (11, 20), and B. */
const PRESENT = new Set([keptRectangleAt(11), SHAPE_B]);

/** The measure of no state. */
const NONE: HistoryMeasure = { held: EMPTY_HELD_COUNTS, size: 0 };

/** A and B, A selected, added once. */
const ONCE = measureByState(NONE, { by: 1, state: STATE }, PRESENT);

/** The state added twice: A and B held by 2, 1 276 bytes. */
const TWICE = measureByState(ONCE, { by: 1, state: STATE }, PRESENT);

/**
 * How many states hold A and B.
 *
 * @param measure - the measure read
 * @returns A's count, then B's
 */
function heldAB(measure: HistoryMeasure): readonly number[] {
  return [heldCount(measure.held, shapeKey(SHAPE_A)), heldCount(measure.held, shapeKey(SHAPE_B))];
}

describe("measureByState", () => {
  it("[F41.AC2] adds A, not in the present, and three slots: 1 252 bytes", () => {
    // A 0 → 1, not in the present: 614 × 2 = 1 228; B in the present: 0; 3 × 8 = 24: 1 252.
    expect(ONCE.size).toBe(1252);
    expect(heldAB(ONCE)).toEqual([1, 1]);
  });

  it("[F41.AC2] adds the slots alone for a state whose shapes another holds: 1 276 bytes", () => {
    // A 1 → 2, B 1 → 2: no text; 3 × 8 = 24: 1 252 + 24 = 1 276.
    expect(TWICE.size).toBe(1276);
    expect(heldAB(TWICE)).toEqual([2, 2]);
  });

  it("[F41.AC2] counts a shape a state holds twice once, its two references both: 1 244 bytes", () => {
    // A 0 → 1 once: 1 228; two references, 16: 1 244.
    const doubled = measureByState(
      NONE,
      { by: 1, state: historyState([SHAPE_A, SHAPE_A], []) },
      PRESENT,
    );

    expect(doubled.size).toBe(1244);
    expect(heldCount(doubled.held, shapeKey(SHAPE_A))).toBe(1);
  });

  it("[F41.AC2] adds nothing for an empty state", () => {
    expect(measureByState(ONCE, { by: 1, state: historyState([], []) }, PRESENT)).toEqual(ONCE);
  });

  it("[F41.AC2] takes the slots alone away while another state holds its shapes: 1 252 bytes", () => {
    // A 2 → 1, B 2 → 1: 1 276 − 24 = 1 252.
    const once = measureByState(TWICE, { by: -1, state: STATE }, PRESENT);

    expect(once.size).toBe(1252);
    expect(heldAB(once)).toEqual([1, 1]);
  });

  it("[F41.AC2] takes A away with the last state holding it: 0 bytes", () => {
    // A 1 → 0, not in the present: 1 252 − 1 228 − 24 = 0.
    const none = measureByState(ONCE, { by: -1, state: STATE }, PRESENT);

    expect(none.size).toBe(0);
    expect(heldAB(none)).toEqual([0, 0]);
  });

  it("[F41.AC2] keeps the text of a shape the present holds: B alone, 8 bytes to 0", () => {
    // B in the present: its reference alone, 8, added then taken away.
    const alone = historyState([SHAPE_B], []);
    const added = measureByState(NONE, { by: 1, state: alone }, PRESENT);

    expect(added.size).toBe(8);
    expect(measureByState(added, { by: -1, state: alone }, PRESENT).size).toBe(0);
  });
});
