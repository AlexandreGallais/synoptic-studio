import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_HISTORY, ANY_HISTORY_STATE, historyState, keptRectangleAt } from "../../../tests";

import { historyWithStep } from "./history-with-step";
import { redoneHistory } from "./redone-history";
import { undoneHistory } from "./undone-history";

import type { History } from "./history";
import type { KeptRectangle } from "../../../tests";

/** A at (10, 20), as the present holds it. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), as the present holds it. */
const SHAPE_B = keptRectangleAt(200);

/** A and B, A selected, none past nor to come. */
const START: History<KeptRectangle> = {
  future: [],
  past: [],
  present: historyState([SHAPE_A, SHAPE_B], [0]),
};

/** A′, A nudged to (11, 20), as the page reads it after the nudge. */
const SHAPE_A_NUDGED = keptRectangleAt(11);

/** START once A′ and B is recorded, B read again as a new object. */
const NUDGED = historyWithStep(START, historyState([SHAPE_A_NUDGED, keptRectangleAt(200)], [0]));

describe("historyWithStep (F40)", () => {
  it("[F40.AC1] pushes A and B onto the past, A′ and B the present, B shared", () => {
    expect(NUDGED.past).toHaveLength(1);
    expect(NUDGED.past[0]).toBe(START.present);
    expect(NUDGED.present).toEqual(historyState([keptRectangleAt(11), keptRectangleAt(200)], [0]));
    expect(NUDGED.present.drawing[0]).toBe(SHAPE_A_NUDGED);
    expect(NUDGED.present.drawing[1]).toBe(SHAPE_B);
    expect(NUDGED.future).toEqual([]);
  });

  it("[F40.AC3] keeps the history itself for the same drawing and the same selection", () => {
    const again = historyState([keptRectangleAt(10), keptRectangleAt(200)], [0]);

    expect(historyWithStep(START, again)).toBe(START);
  });

  it("[F40.AC3] keeps the history itself for two shapes of equal texts read again", () => {
    // A twice, two objects of the same text, both selected: the same drawing read again.
    const twice: History<KeptRectangle> = {
      future: [],
      past: [],
      present: historyState([keptRectangleAt(10), keptRectangleAt(10)], [0, 1]),
    };
    const again = historyState([keptRectangleAt(10), keptRectangleAt(10)], [0, 1]);

    expect(historyWithStep(twice, again)).toBe(twice);
  });

  it("[F40.AC3] takes no step for another selection, the present's selection replaced", () => {
    const selected = historyWithStep(
      NUDGED,
      historyState([keptRectangleAt(11), keptRectangleAt(200)], [1]),
    );

    expect(selected.past).toBe(NUDGED.past);
    expect(selected.future).toBe(NUDGED.future);
    expect(selected.present.drawing).toBe(NUDGED.present.drawing);
    expect(selected.present.selection).toEqual([1]);
  });

  it("[F40.AC3] keeps the states to come when only the selection changes", () => {
    const undone = undoneHistory(NUDGED);
    const selected = historyWithStep(undone, historyState([SHAPE_A, SHAPE_B], []));

    expect(selected.future).toBe(undone.future);
    expect(selected.present.selection).toEqual([]);
  });

  it("[F40.AC1] keeps the selection as it was when the step was taken, given back by undo and redo", () => {
    // B selected alone, no step; then B deleted: the drawing A′, none selected.
    const selected = historyWithStep(
      NUDGED,
      historyState([keptRectangleAt(11), keptRectangleAt(200)], [1]),
    );
    const deleted = historyWithStep(selected, historyState([keptRectangleAt(11)], []));

    expect(deleted.past.at(-1)).toEqual(historyState([SHAPE_A_NUDGED, SHAPE_B], [1]));
    expect(deleted.present.drawing[0]).toBe(SHAPE_A_NUDGED);
    expect(undoneHistory(deleted).present).toEqual(historyState([SHAPE_A_NUDGED, SHAPE_B], [1]));

    expect(redoneHistory(undoneHistory(deleted)).present).toEqual(
      historyState([SHAPE_A_NUDGED], []),
    );
  });

  it("[F40.AC1] drops the state undone when a step is taken after an undo", () => {
    // Undone: A and B present, A′ and B to come; then B moved to (201, 20).
    const moved = keptRectangleAt(201);
    const after = historyWithStep(
      undoneHistory(NUDGED),
      historyState([keptRectangleAt(10), moved], [1]),
    );

    expect(after.future).toEqual([]);
    expect(after.past).toEqual([historyState([SHAPE_A, SHAPE_B], [0])]);
    expect(after.present.drawing[0]).toBe(SHAPE_A);
    expect(after.present.drawing[1]).toBe(moved);
  });

  it.each([
    {
      drawing: [keptRectangleAt(200), keptRectangleAt(10)],
      title: "the same shapes in another order",
    },
    { drawing: [keptRectangleAt(10)], title: "a shape deleted" },
    {
      drawing: [keptRectangleAt(10), keptRectangleAt(200), keptRectangleAt(10)],
      title: "a shape added",
    },
    { drawing: [], title: "every shape deleted" },
  ])("[F40.AC1] takes a step for $title", ({ drawing }) => {
    const after = historyWithStep(START, historyState(drawing, [0]));

    expect(after.past).toEqual([START.present]);
    expect(after.present.drawing).toEqual(drawing);
  });

  test.prop({ history: ANY_HISTORY, state: ANY_HISTORY_STATE })(
    "[F40.AC1] gives the state recorded as the present, past and future kept or stepped",
    ({ history, state }) => {
      const before = JSON.stringify({ history, state });
      const after = historyWithStep(history, state);
      const isStep = after.past.length > history.past.length;

      expect(after.present).toEqual(state);
      expect(after.past).toEqual(isStep ? [...history.past, history.present] : history.past);
      expect(after.future).toEqual(isStep ? [] : history.future);
      expect(JSON.stringify({ history, state })).toBe(before);
    },
  );

  test.prop({ history: ANY_HISTORY })(
    "[F40.AC3] takes no step for a copy of the present, each shape a new object of the same text",
    ({ history }) => {
      const { drawing, selection } = history.present;
      const copy = historyState(
        drawing.map((shape) => [...shape]),
        [...selection],
      );

      expect(historyWithStep(history, copy)).toBe(history);
    },
  );
});
