// States of the drawing shared by the tests of `src/model/history/` (EN-146). Tests only: never
// part of the library.

import { fc } from "@fast-check/vitest";

import { rectangleAsFreeShape } from "../src";

import type { FreeShape, History, HistoryState } from "../src";

/** A rectangle as the playground keeps it (`KeptShape`): the kind the panel showed, the shape. */
export type KeptRectangle = {
  /** The kind the panel showed it as. */
  readonly kind: "rectangle";
  /** The rectangle as the model holds it, a free shape (ADR-0032). */
  readonly shape: FreeShape;
};

/**
 * A new object at each call: a rectangle 100 × 50 at (x, 20) of radius 0, kept as the playground
 * keeps it; its JSON text is 614 characters at x 10, 11 or 12, 615 at x 200.
 *
 * @param x - the left side of its box, in user units
 * @returns a new object, equal by its text to every other at the same x
 */
export function keptRectangleAt(x: number): KeptRectangle {
  return {
    kind: "rectangle",
    shape: rectangleAsFreeShape({
      angle: 0,
      height: 50,
      radii: [0, 0, 0, 0],
      width: 100,
      x,
      y: 20,
    }),
  };
}

/**
 * A state of the drawing, for the examples.
 *
 * @template T - type of the shapes
 * @param drawing - its shapes, in the order drawn
 * @param selection - the indices of the shapes selected
 * @returns the state holding both
 */
export function historyState<T>(
  drawing: readonly T[],
  selection: readonly number[],
): HistoryState<T> {
  return { drawing, selection };
}

/**
 * Six shapes, the objects any history of the properties draws from, three pairs equal by their
 * text and not the same object.
 */
const SHAPE_POOL: readonly (readonly string[])[] = [
  ["a"],
  ["b"],
  ["c", "d"],
  ["a"],
  ["b"],
  ["c", "d"],
];

/** The most shapes, and indices selected, a state of the properties holds. */
const MOST_IN_STATE = 4;

/** The most states past, and to come, a history of the properties holds. */
const MOST_STATES = 5;

/** Any state drawn from the pool of shapes, its indices selected below the most shapes. */
export const ANY_HISTORY_STATE: fc.Arbitrary<HistoryState<readonly string[]>> = fc.record({
  drawing: fc.array(fc.constantFrom(...SHAPE_POOL), { maxLength: MOST_IN_STATE }),
  selection: fc.array(fc.nat({ max: MOST_IN_STATE - 1 }), { maxLength: MOST_IN_STATE }),
});

/** Any history drawn from the pool of shapes. */
export const ANY_HISTORY: fc.Arbitrary<History<readonly string[]>> = fc.record({
  future: fc.array(ANY_HISTORY_STATE, { maxLength: MOST_STATES }),
  past: fc.array(ANY_HISTORY_STATE, { maxLength: MOST_STATES }),
  present: ANY_HISTORY_STATE,
});
