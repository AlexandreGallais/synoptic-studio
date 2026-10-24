import { fc, test } from "@fast-check/vitest";
import { describe, expect } from "vitest";

import {
  ANY_HISTORY_OPERATION,
  ANY_HISTORY_STATE,
  operatedHistory,
  statesHolding,
} from "../../../tests";
import { heldCount, shapeKey } from "../held-shapes";
import { boundedHistory, historySize } from "../history";

import { measuredHistory } from "./measured-history";

/** A shape of the properties, as `ANY_HISTORY_STATE` draws them. */
type Drawn = readonly string[];

describe("SizedHistory (EN-147)", () => {
  test.prop({
    operations: fc.array(ANY_HISTORY_OPERATION, { maxLength: 30 }),
    present: ANY_HISTORY_STATE,
  })(
    "[F41.AC2] keeps historySize's bytes and each shape's states through any steps, undos, redos and bounds",
    ({ operations, present }) => {
      const start = measuredHistory<Drawn>({ future: [], past: [], present });

      operations.reduce((history, operation) => {
        const next = operatedHistory(history, operation);
        const shapes = [...next.past, next.present, ...next.future].flatMap(
          (state) => state.drawing,
        );

        expect(next.size).toBe(historySize(next));

        expect(shapes.map((shape) => heldCount(next.held, shapeKey(shape)))).toEqual(
          shapes.map((shape) => statesHolding(next, shape)),
        );

        if (operation.kind === "bound") {
          const { future, past, present: shown } = boundedHistory(history, operation.bound);

          expect([next.past, next.present, next.future]).toStrictEqual([past, shown, future]);
        }

        return next;
      }, start);
    },
  );
});
