import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { EMPTY_HELD_COUNTS } from "../held-shapes";

import { measureByState } from "./measure-by-state";
import { measureWithPresent } from "./measure-with-present";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** A′, A nudged to (11, 20), 614 characters. */
const SHAPE_A_NUDGED = keptRectangleAt(11);

/** A″, A nudged to (12, 20), 614 characters. */
const SHAPE_A_TWICE = keptRectangleAt(12);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A and B past, A′ and B present: 1 252 bytes. */
const NUDGED = measureByState(
  { held: EMPTY_HELD_COUNTS, size: 0 },
  { by: 1, state: historyState([SHAPE_A, SHAPE_B], [0]) },
  new Set([SHAPE_A_NUDGED, SHAPE_B]),
);

describe("measureWithPresent (EN-147)", () => {
  it("[F41.AC2] takes A away once the present holds it: 1 252 to 24 bytes", () => {
    // A taken by the present, held by 1: 1 252 − 1 228 = 24; A′ left, held by none: nothing.
    const back = measureWithPresent(NUDGED, [SHAPE_A_NUDGED, SHAPE_B], [SHAPE_A, SHAPE_B]);

    expect(back).toEqual({ held: NUDGED.held, size: 24 });
  });

  it("[F41.AC2] adds A back once the present leaves it: 24 to 1 252 bytes", () => {
    // The measure of A and B past and present: 24 bytes; A left, held by 1: + 1 228.
    const shared = { held: NUDGED.held, size: 24 };

    expect(measureWithPresent(shared, [SHAPE_A, SHAPE_B], [SHAPE_A_NUDGED, SHAPE_B]).size).toBe(
      1252,
    );
  });

  it("[F41.AC2] keeps the measure for shapes no state holds: A′ to A″", () => {
    expect(measureWithPresent(NUDGED, [SHAPE_A_NUDGED, SHAPE_B], [SHAPE_A_TWICE, SHAPE_B])).toBe(
      NUDGED,
    );
  });

  it("[F41.AC2] keeps the measure for the same drawing, a selection replaced alone", () => {
    expect(measureWithPresent(NUDGED, [SHAPE_A_NUDGED, SHAPE_B], [SHAPE_A_NUDGED, SHAPE_B])).toBe(
      NUDGED,
    );
  });
});
