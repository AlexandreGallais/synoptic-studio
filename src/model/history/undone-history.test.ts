import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_HISTORY, historyState, keptRectangleAt } from "../../../tests";

import { redoneHistory } from "./redone-history";
import { undoneHistory } from "./undone-history";

import type { History } from "./history";
import type { KeptRectangle } from "../../../tests";

/** A′ and B present, A selected, A and B past, A selected. */
const NUDGED: History<KeptRectangle> = {
  future: [],
  past: [historyState([keptRectangleAt(10), keptRectangleAt(200)], [0])],
  present: historyState([keptRectangleAt(11), keptRectangleAt(200)], [0]),
};

describe("undoneHistory (F40)", () => {
  it("[F40.AC1] makes A and B the present, nothing past, A′ and B to come", () => {
    const undone = undoneHistory(NUDGED);

    expect(undone.past).toEqual([]);
    expect(undone.present).toBe(NUDGED.past[0]);
    expect(undone.future).toHaveLength(1);
    expect(undone.future[0]).toBe(NUDGED.present);
  });

  it("[F40.AC1] puts the present before the states already to come", () => {
    // A nudged twice: A and B, A′ and B past, A″ and B present; undone twice.
    const twice: History<KeptRectangle> = {
      ...NUDGED,
      past: [...NUDGED.past, NUDGED.present],
      present: historyState([keptRectangleAt(12), keptRectangleAt(200)], [0]),
    };
    const undone = undoneHistory(undoneHistory(twice));

    expect(undone.present).toBe(NUDGED.past[0]);
    expect(undone.future).toHaveLength(2);
    expect(undone.future[0]).toBe(NUDGED.present);
    expect(undone.future[1]).toBe(twice.present);
  });

  it("[F40.AC1] gives the history itself with nothing past", () => {
    const undone = undoneHistory(NUDGED);

    expect(undoneHistory(undone)).toBe(undone);
  });

  test.prop({ history: ANY_HISTORY })(
    "[F40.AC1] is undone by a redo, when something is past",
    ({ history }) => {
      fc.pre(history.past.length > 0);

      const before = JSON.stringify(history);

      expect(redoneHistory(undoneHistory(history))).toEqual(history);
      expect(JSON.stringify(history)).toBe(before);
    },
  );
});
