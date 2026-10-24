import { test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { ANY_HISTORY, historyState, keptRectangleAt, statesHolding } from "../../../tests";
import { heldCount, shapeKey } from "../held-shapes";
import { historySize } from "../history";

import { measuredHistory } from "./measured-history";

/** A at (10, 20), 614 characters, 1 228 bytes. */
const SHAPE_A = keptRectangleAt(10);

/** B at (200, 20), 615 characters. */
const SHAPE_B = keptRectangleAt(200);

describe("measuredHistory (EN-147)", () => {
  it("[F41.AC2] counts nothing with nothing past nor to come", () => {
    const present = historyState([SHAPE_A], [0]);

    expect(measuredHistory({ future: [], past: [], present })).toMatchObject({
      future: [],
      past: [],
      present,
      size: 0,
    });
  });

  it("[F41.AC2] counts A and A″, four references and two indices, A′ present: 2 504 bytes", () => {
    // A past, A″ to come, 1 228 each; B held by both and the present; 4 × 8 + 2 × 8 = 48.
    const nudged = keptRectangleAt(12);
    const history = measuredHistory({
      future: [historyState([nudged, SHAPE_B], [0])],
      past: [historyState([SHAPE_A, SHAPE_B], [0])],
      present: historyState([keptRectangleAt(11), SHAPE_B], [0]),
    });

    expect(history.size).toBe(2504);

    expect(
      [SHAPE_A, SHAPE_B, nudged].map((shape) => heldCount(history.held, shapeKey(shape))),
    ).toEqual([1, 2, 1]);
  });

  test.prop({ history: ANY_HISTORY })(
    "[F41.AC2] counts historySize's bytes and the states holding each shape, its states those given",
    ({ history }) => {
      const measured = measuredHistory(history);
      const shapes = [...history.past, history.present, ...history.future].flatMap(
        (state) => state.drawing,
      );

      expect(measured.size).toBe(historySize(history));

      expect(shapes.map((shape) => heldCount(measured.held, shapeKey(shape)))).toEqual(
        shapes.map((shape) => statesHolding(history, shape)),
      );

      expect([measured.past, measured.present, measured.future]).toEqual([
        history.past,
        history.present,
        history.future,
      ]);
    },
  );
});
