import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { heldCount, shapeKey } from "../held-shapes";

import { measuredHistory } from "./measured-history";
import { sizedUndoneHistory } from "./sized-undone-history";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** A′, A nudged to (11, 20). */
const SHAPE_A_NUDGED = keptRectangleAt(11);

/** A″, A nudged to (12, 20). */
const SHAPE_A_TWICE = keptRectangleAt(12);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A nudged twice: A and B, A′ and B past, A″ and B present; 2 504 bytes. */
const TWICE = measuredHistory({
  future: [],
  past: [historyState([SHAPE_A, SHAPE_B], [0]), historyState([SHAPE_A_NUDGED, SHAPE_B], [0])],
  present: historyState([SHAPE_A_TWICE, SHAPE_B], [0]),
});

describe("sizedUndoneHistory (EN-147)", () => {
  it("[F41.AC2] keeps A past and A″ to come once undone: 2 504 bytes", () => {
    // A′ and B removed from the past: A′ held by none, not in the present, − 1 228 − 24: 1 252;
    // A″ and B added to come, A″ in the present, + 24; A″ left by the present, held by 1,
    // + 1 228; A′ taken, held by none: 2 504.
    const undone = sizedUndoneHistory(TWICE);

    expect(undone.size).toBe(2504);
    expect(undone.present).toBe(TWICE.past[1]);

    expect(
      [SHAPE_A, SHAPE_A_NUDGED, SHAPE_A_TWICE, SHAPE_B].map((shape) =>
        heldCount(undone.held, shapeKey(shape)),
      ),
    ).toEqual([1, 0, 1, 2]);
  });

  it("[F41.AC2] keeps A′ and A″ to come once undone twice: 2 504 bytes", () => {
    const back = sizedUndoneHistory(sizedUndoneHistory(TWICE));

    expect(back.size).toBe(2504);
    expect(back.past).toEqual([]);
    expect(heldCount(back.held, shapeKey(SHAPE_A))).toBe(0);
  });

  it("[F41.AC2] gives the history itself when nothing is past", () => {
    const back = sizedUndoneHistory(sizedUndoneHistory(TWICE));

    expect(sizedUndoneHistory(back)).toBe(back);
  });
});
