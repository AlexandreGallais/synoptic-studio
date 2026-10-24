import { describe, expect, it } from "vitest";

import { historyState, keptRectangleAt, keptShapeFields } from "../../../tests";
import { undoneHistory } from "../history";

import { HISTORY_LIST_ROWS } from "./history-list-rows";
import { historySteps } from "./history-steps";

import type { HistoryList } from "./history-list";
import type { KeptRectangle } from "../../../tests";
import type { History } from "../history";

/** A at (10, 20). */
const SHAPE_A = keptRectangleAt(10);

/** A′, A nudged to (11, 20). */
const A_NUDGED = keptRectangleAt(11);

/** A″, A′ 150 wide. */
const A_WIDE = { ...A_NUDGED, shape: { ...A_NUDGED.shape, width: 150 } };

/** B at (200, 20). */
const SHAPE_B = keptRectangleAt(200);

/** A and B drawn, then A nudged, B deleted and 150 typed in A's W. */
const STATES = [
  historyState([SHAPE_A, SHAPE_B], []),
  historyState([A_NUDGED, SHAPE_B], [0]),
  historyState([A_NUDGED], []),
  historyState([A_WIDE], [0]),
] as const;

/** The four states, the last the present. */
const TAKEN: History<KeptRectangle> = {
  future: [],
  past: [STATES[0], STATES[1], STATES[2]],
  present: STATES[3],
};

/**
 * The names and places of a list's rows, and what it leaves out.
 *
 * @param list - a list of the history's steps
 * @returns each row's name and place, the counts left out
 */
function summary(list: HistoryList<KeptRectangle>): object {
  return {
    earlier: list.earlier,
    later: list.later,
    rows: list.rows.map((row) => `${row.name} ${row.place}`),
  };
}

describe("historySteps (US-077)", () => {
  it("[F41.AC1] names A nudged, B deleted and 150 typed: Move, Delete, Width, the last present", () => {
    const list = historySteps(TAKEN, HISTORY_LIST_ROWS, keptShapeFields);

    expect(summary(list)).toEqual({
      earlier: 0,
      later: 0,
      rows: ["Start past", "Move past", "Delete past", "Width present"],
    });

    expect(list.rows.map((row) => row.state)).toEqual(STATES);
    expect(list.rows[3]?.state).toBe(STATES[3]);
  });

  it("[F41.AC1] lists Width to come once undone, Delete present", () => {
    const undone = undoneHistory(TAKEN);

    expect(summary(historySteps(undone, HISTORY_LIST_ROWS, keptShapeFields))).toEqual({
      earlier: 0,
      later: 0,
      rows: ["Start past", "Move past", "Delete present", "Width future"],
    });
  });

  it("[F41.AC1] names the first row shown from the state before it: 2 rows, Delete and Width", () => {
    // One row beside the present, none to come: the past state before it, Delete, from A′ and SHAPE_B.
    expect(summary(historySteps(TAKEN, 2, keptShapeFields))).toEqual({
      earlier: 2,
      later: 0,
      rows: ["Delete past", "Width present"],
    });
  });

  it("[F41.AC1] leaves the last state to come out of 3 rows, Move present", () => {
    const back = undoneHistory(undoneHistory(TAKEN));

    // Two rows beside the present, at most one after it: Start before, Delete after.
    expect(summary(historySteps(back, 3, keptShapeFields))).toEqual({
      earlier: 0,
      later: 1,
      rows: ["Start past", "Move present", "Delete future"],
    });
  });

  it("[F41.AC1] lists a history with nothing past nor to come as Start, present", () => {
    const alone = { future: [], past: [], present: STATES[0] };

    expect(summary(historySteps(alone, HISTORY_LIST_ROWS, keptShapeFields))).toEqual({
      earlier: 0,
      later: 0,
      rows: ["Start present"],
    });
  });
});
