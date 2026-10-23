// States of the drawing shared by the tests of `src/model/history/` (EN-146). Tests only: never
// part of the library.

import { fc } from "@fast-check/vitest";

import {
  rectangleAsFreeShape,
  sizedBoundedHistory,
  sizedHistoryWithStep,
  sizedRedoneHistory,
  sizedUndoneHistory,
} from "../src";

import type { FreeShape, History, HistoryBound, HistoryState, SizedHistory } from "../src";

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
 * The fields a step's name is read from for a shape kept as the playground keeps it, the shape's
 * own and its kind (`ShapeFields`, US-077).
 *
 * @param kept - a shape as the page keeps it, its kind possibly changed
 * @param kept.kind - the kind the panel shows it as
 * @param kept.shape - the shape as the model holds it
 * @returns its shape's fields and its kind
 */
export function keptShapeFields(kept: {
  readonly kind: string;
  readonly shape: object;
}): Readonly<Record<string, unknown>> {
  return { ...kept.shape, kind: kept.kind };
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

/**
 * How many past and future states of a history hold a shape, each counting it once, as the held
 * counts count them (EN-147).
 *
 * @template T - type of the shapes
 * @param history - the history read
 * @param shape - the shape looked for, by `===`
 * @returns the count of states holding it
 */
export function statesHolding<T>(history: History<T>, shape: T): number {
  return [...history.past, ...history.future].filter((state) => state.drawing.includes(shape))
    .length;
}

/**
 * The same states, each recording the reads of its properties: its index pushed onto the list
 * given at each read, so that a test sees which states a function read (EN-147).
 *
 * @template T - type of the shapes
 * @param states - the states watched
 * @param reads - the list each read pushes its state's index onto
 * @returns a proxy of each state, in the same order
 */
export function statesRecordingReads<T>(
  states: readonly HistoryState<T>[],
  reads: number[],
): readonly HistoryState<T>[] {
  return states.map(
    (state, index) =>
      new Proxy(state, {
        get: (target, key): unknown => {
          reads.push(index);

          return Reflect.get(target, key);
        },
      }),
  );
}

/** An operation of the properties on a history. */
export type HistoryOperation =
  | { readonly bound: HistoryBound; readonly kind: "bound" }
  | { readonly kind: "redo" }
  | { readonly kind: "step"; readonly state: HistoryState<readonly string[]> }
  | { readonly kind: "undo" };

/** Any operation: a step to a state drawn, an undo, a redo, or a bound of up to 120 bytes. */
export const ANY_HISTORY_OPERATION: fc.Arbitrary<HistoryOperation> = fc.oneof(
  ANY_HISTORY_STATE.map((state) => ({ kind: "step" as const, state })),
  fc.constant({ kind: "undo" as const }),
  fc.constant({ kind: "redo" as const }),
  fc
    .record({ size: fc.nat({ max: 120 }), steps: fc.nat({ max: 4 }) })
    .map((bound) => ({ bound, kind: "bound" as const })),
);

/**
 * A history once an operation applied, its size kept.
 *
 * @param history - the history before
 * @param operation - the operation applied
 * @returns the history after
 */
export function operatedHistory(
  history: SizedHistory<readonly string[]>,
  operation: HistoryOperation,
): SizedHistory<readonly string[]> {
  switch (operation.kind) {
    case "bound": {
      return sizedBoundedHistory(history, operation.bound);
    }

    case "redo": {
      return sizedRedoneHistory(history);
    }

    case "step": {
      return sizedHistoryWithStep(history, operation.state);
    }

    case "undo": {
      return sizedUndoneHistory(history);
    }
  }
}
