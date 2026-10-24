import { describe, expect, it } from "vitest";

import {
  DEFAULT_HISTORY_BOUND,
  measuredHistory,
  sizedBoundedHistory,
  sizedHistoryAt,
  sizedHistoryWithStep,
} from "../src";

import { historyState, keptRectangleAt } from "./history-fixtures";

import type { KeptRectangle } from "./history-fixtures";
import type { HistoryState, SizedHistory } from "../src";

// The timings of a history kept fast (F41, EN-147, AUD-043): opt-in, skipped by `npm run test`,
// run by `npx vitest run tests/history-timings.test.ts --mode history-timings`. Each limit bounds
// a test by Vitest's timeout, the clock never read (EN-129), and its measure alone is the median
// of 5 runs of this file alone, each test's own duration.

/** The run's mode: `history-timings` runs these tests. */
const { MODE } = import.meta.env;

/** The tests run only when asked. */
const isAsked = MODE === "history-timings";

/** The moves of AUD-042's history, about the 80 380 the default 100 MB holds. */
const MOVES = 80_000;

/** The shapes of a wide drawing, the 1 000 symbols the default size is read from (SP-029). */
const WIDE = 1000;

/** The steps a test takes, or the steps back it goes. */
const STEPS = 10;

/**
 * The history of `MOVES` past states, each a new rectangle moved, as AUD-042's bench builds it,
 * its size measured once.
 *
 * @returns the history, nothing to come
 */
function longHistory(): SizedHistory<KeptRectangle> {
  return measuredHistory({
    future: [],
    past: Array.from({ length: MOVES }, (unused, index) =>
      historyState([keptRectangleAt(-index - 1)], [0]),
    ),
    present: historyState([keptRectangleAt(10)], [0]),
  });
}

/**
 * A state of the wide drawing with one shape moved.
 *
 * @param drawing - the drawing before
 * @param step - the step's number, the shape moved the one at its index modulo the drawing's length
 * @returns the state, the shape moved selected
 */
function movedState(drawing: readonly KeptRectangle[], step: number): HistoryState<KeptRectangle> {
  const index = step % drawing.length;

  return historyState(drawing.with(index, keptRectangleAt(5000 + step)), [index]);
}

/**
 * Takes steps on a history, each bounded by the default bound, as the page records them.
 *
 * @param history - the history before
 * @param steps - the steps' numbers, each moving a shape (`movedState`)
 * @returns the history after them
 */
function steppedHistory(
  history: SizedHistory<KeptRectangle>,
  steps: readonly number[],
): SizedHistory<KeptRectangle> {
  return steps.reduce(
    (before, step) =>
      sizedBoundedHistory(
        sizedHistoryWithStep(before, movedState(before.present.drawing, step)),
        DEFAULT_HISTORY_BOUND,
      ),
    history,
  );
}

/**
 * The numbers of `STEPS` steps from a first one.
 *
 * @param first - the first step's number
 * @returns the steps' numbers
 */
function stepNumbers(first: number): readonly number[] {
  return Array.from({ length: STEPS }, (unused, index) => first + index);
}

/**
 * The wide drawing of `WIDE` rectangles, then 20 steps each moving one, bounded as the page bounds
 * them.
 *
 * @returns the history, 20 states past
 */
function wideHistory(): SizedHistory<KeptRectangle> {
  const drawing = Array.from({ length: WIDE }, (unused, index) => keptRectangleAt(index * 3));
  const start = measuredHistory({ future: [], past: [], present: historyState(drawing, [0]) });

  return steppedHistory(start, [...stepNumbers(0), ...stepNumbers(STEPS)]);
}

/** A history of nothing, for a run not asking the timings. */
const NO_HISTORY = measuredHistory<KeptRectangle>({
  future: [],
  past: [],
  present: historyState([], []),
});

/** The long history, built before the tests are timed, only when they run. */
const LONG = isAsked ? longHistory() : NO_HISTORY;

/** The wide history, built before the tests are timed, only when they run. */
const WIDE_HISTORY = isAsked ? wideHistory() : NO_HISTORY;

describe.runIf(isAsked)("The timings of a history kept fast (EN-147, AUD-043)", () => {
  // alone: 24 ms, 10 steps of about 2.4 ms; 160 ms is 6.7 times it, and 16 ms a step, F41's
  // criterion 2.
  it(
    "[F41.AC2] takes 10 steps on a history of 80 000 moves within 16 ms each",
    { tags: ["timing"], timeout: 160 },
    () => {
      const after = steppedHistory(LONG, stepNumbers(0));

      // The default 100 MB full, the oldest dropped as each step adds one.
      expect(after.size).toBeLessThanOrEqual(DEFAULT_HISTORY_BOUND.size);
      expect(after.present.drawing[0]?.shape.x).toBe(5009);
    },
  );

  // alone: 109 ms, 10 steps of about 11 ms, each counting every shape of the present (AUD-043);
  // 450 ms is 4.1 times it.
  it(
    "takes 10 steps in a drawing of 1 000 shapes within 45 ms each",
    { tags: ["timing"], timeout: 450 },
    () => {
      const after = steppedHistory(WIDE_HISTORY, stepNumbers(2 * STEPS));

      expect(after.past).toHaveLength(WIDE_HISTORY.past.length + STEPS);
    },
  );

  // alone: 134 ms, 10 undos of about 13 ms; 14 steps back took 138 ms at the audit; 550 ms is
  // 4.1 times it.
  it(
    "goes back 10 steps in a drawing of 1 000 shapes within 550 ms",
    { tags: ["timing"], timeout: 550 },
    () => {
      const state = WIDE_HISTORY.past.at(-STEPS) ?? WIDE_HISTORY.present;
      const back = sizedHistoryAt(WIDE_HISTORY, state);

      expect(back.present).toBe(state);
      expect(back.future).toHaveLength(STEPS);
    },
  );
});
