import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { heldCount, shapeKey } from "../held-shapes";

import { measuredHistory } from "./measured-history";
import { sizedRedoneHistory } from "./sized-redone-history";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** A′, A nudged to (11, 20). */
const SHAPE_A_NUDGED = keptRectangleAt(11);

/** A″, A nudged to (12, 20). */
const SHAPE_A_TWICE = keptRectangleAt(12);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

/** A nudged twice then undone twice: A and B present, A′ and B, A″ and B to come; 2 504 bytes. */
const BACK = measuredHistory({
  future: [
    historyState([SHAPE_A_NUDGED, SHAPE_B], [0]),
    historyState([SHAPE_A_TWICE, SHAPE_B], [0]),
  ],
  past: [],
  present: historyState([SHAPE_A, SHAPE_B], [0]),
});

describe("sizedRedoneHistory (EN-147)", () => {
  it("[F41.AC2] keeps A past and A″ to come once redone: 2 504 bytes", () => {
    // A′ and B removed from those to come: A′ held by none, not in the present, − 1 228 − 24:
    // 1 252; A and B added to the past, A in the present, + 24; A left by the present, held by 1,
    // + 1 228; A′ taken, held by none: 2 504.
    const redone = sizedRedoneHistory(BACK);

    expect(redone.size).toBe(2504);
    expect(redone.present).toBe(BACK.future[0]);

    expect(
      [SHAPE_A, SHAPE_A_NUDGED, SHAPE_A_TWICE].map((shape) =>
        heldCount(redone.held, shapeKey(shape)),
      ),
    ).toEqual([1, 0, 1]);
  });

  it("[F41.AC2] gives the history itself when nothing is to come", () => {
    const ahead = sizedRedoneHistory(sizedRedoneHistory(BACK));

    expect(ahead.size).toBe(2504);
    expect(sizedRedoneHistory(ahead)).toBe(ahead);
  });
});
