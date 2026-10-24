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

describe("redoneHistory (F40)", () => {
  it("[F40.AC1] makes A′ and B the present again, A and B past, nothing to come", () => {
    const redone = redoneHistory(undoneHistory(NUDGED));

    expect(redone).toEqual(NUDGED);
    expect(redone.present).toBe(NUDGED.present);
    expect(redone.past[0]).toBe(NUDGED.past[0]);
  });

  it("[F40.AC1] gives the history itself with nothing to come", () => {
    expect(redoneHistory(NUDGED)).toBe(NUDGED);
  });

  test.prop({ history: ANY_HISTORY })(
    "[F40.AC1] is undone by an undo, when something is to come",
    ({ history }) => {
      fc.pre(history.future.length > 0);

      const before = JSON.stringify(history);

      expect(undoneHistory(redoneHistory(history))).toEqual(history);
      expect(JSON.stringify(history)).toBe(before);
    },
  );
});
