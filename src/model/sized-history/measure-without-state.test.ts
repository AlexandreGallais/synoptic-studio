import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { EMPTY_HELD_COUNTS, heldCount, shapeKey } from "../held-shapes";

import { measureWithState } from "./measure-with-state";
import { measureWithoutState } from "./measure-without-state";

import type { HistoryMeasure } from "./history-measure";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A and B, A selected. */
const STATE = historyState([SHAPE_A, SHAPE_B], [0]);

/** The present: A′, A nudged to (11, 20), and B. */
const PRESENT = new Set([keptRectangleAt(11), SHAPE_B]);

/** The state added twice: A and B held by 2, 1 276 bytes. */
const TWICE = [STATE, STATE].reduce<HistoryMeasure>(
  (measure, state) => measureWithState(measure, state, PRESENT),
  { held: EMPTY_HELD_COUNTS, size: 0 },
);

describe("measureWithoutState (EN-147)", () => {
  it("[F41.AC2] takes the slots alone away while another state holds its shapes: 1 252 bytes", () => {
    // A 2 → 1, B 2 → 1: 1 276 − 24 = 1 252.
    const once = measureWithoutState(TWICE, STATE, PRESENT);

    expect(once.size).toBe(1252);
    expect(heldCount(once.held, shapeKey(SHAPE_A))).toBe(1);
  });

  it("[F41.AC2] takes A away with the last state holding it: 0 bytes", () => {
    // A 1 → 0, not in the present: 1 252 − 1 228 − 24 = 0.
    const none = measureWithoutState(measureWithoutState(TWICE, STATE, PRESENT), STATE, PRESENT);

    expect(none.size).toBe(0);

    expect([
      heldCount(none.held, shapeKey(SHAPE_A)),
      heldCount(none.held, shapeKey(SHAPE_B)),
    ]).toEqual([0, 0]);
  });

  it("[F41.AC2] keeps the text of a shape the present holds: B alone, 8 bytes to 0", () => {
    // B in the present: its reference alone, 8, added then taken away.
    const alone = historyState([SHAPE_B], []);
    const added = measureWithState({ held: EMPTY_HELD_COUNTS, size: 0 }, alone, PRESENT);

    expect(added.size).toBe(8);
    expect(measureWithoutState(added, alone, PRESENT).size).toBe(0);
  });
});
