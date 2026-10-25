import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt } from "../../../tests";
import { historySize } from "../history";

import { measuredHistory } from "./measured-history";
import { sizedHistoryAt } from "./sized-history-at";
import { sizedRedoneHistory } from "./sized-redone-history";
import { sizedUndoneHistory } from "./sized-undone-history";

/** B at (200, 20). */
const SHAPE_B = keptRectangleAt(200);

/** A drawn at (10, 20) then nudged to (11, 20) and (12, 20), B beside it, A selected. */
const [FIRST, SECOND, THIRD] = [10, 11, 12].map((x) =>
  historyState([keptRectangleAt(x), SHAPE_B], [0]),
);

/** A nudged twice: the first two states past, the third present; 2 504 bytes. */
const TWICE = measuredHistory({
  future: [],
  past: [FIRST, SECOND].filter((state) => state !== undefined),
  present: THIRD ?? historyState([], []),
});

/** TWICE undone twice: the first state present, the two others to come. */
const BACK = sizedUndoneHistory(sizedUndoneHistory(TWICE));

describe("sizedHistoryAt", () => {
  it("[F41.AC1] goes back to the first state by two undos, the two after it kept to redo", () => {
    const moved = sizedHistoryAt(TWICE, BACK.present);

    expect(moved).toEqual(BACK);
    expect(moved.future).toEqual([SECOND, THIRD]);
    // The size kept equals the size counted again: A and A′ held, 2 × 1 228 + 2 × 24 = 2 504.
    expect([moved.size, historySize(moved)]).toEqual([2504, 2504]);
  });

  it("[F41.AC1] goes back one state by one undo", () => {
    expect(sizedHistoryAt(TWICE, SECOND ?? TWICE.present)).toEqual(sizedUndoneHistory(TWICE));
  });

  it("[F41.AC1] goes forward to the last state to come by two redos", () => {
    const moved = sizedHistoryAt(BACK, TWICE.present);

    expect(moved).toEqual(sizedRedoneHistory(sizedRedoneHistory(BACK)));
    expect(moved.present).toBe(TWICE.present);
    expect(moved.future).toEqual([]);
  });

  it("[F41.AC1] gives the history itself for its present, and for a state it does not hold", () => {
    expect(sizedHistoryAt(TWICE, TWICE.present)).toBe(TWICE);

    const copy = historyState([keptRectangleAt(10), SHAPE_B], [0]);

    expect(sizedHistoryAt(TWICE, copy)).toBe(TWICE);
  });
});
