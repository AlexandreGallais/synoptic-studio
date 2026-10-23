import { shownHistoryWindow } from "./shown-history-window";
import { stepName } from "./step-name";

import type { HistoryList } from "./history-list";
import type { ShapeFields } from "./shape-fields";
import type { StepPlace } from "./step-place";
import type { History, HistoryState } from "../history";

/**
 * Where the row at an index stands against the present's row.
 *
 * @kind domain
 * @param index - the row's index, the oldest first
 * @param present - the present's index
 * @returns `past` before it, `present` at it, `future` after it
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function stepPlace(index: number, present: number): StepPlace {
  if (index < present) {
    return "past";
  }

  return index === present ? "present" : "future";
}

/**
 * The name of a row: its step's (`stepName`), from the state before it; `Start` for the oldest
 * state of the history, none before it.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param before - the state before the row's; undefined for the oldest
 * @param state - the state after the row's step
 * @param fields - reads the fields of a shape
 * @returns the step's name, or `Start`
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function rowName<T>(
  before: HistoryState<T> | undefined,
  state: HistoryState<T>,
  fields: ShapeFields<T>,
): string {
  return before === undefined ? "Start" : stepName(before.drawing, state.drawing, fields);
}

/**
 * The states the list shows, the oldest first.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history listed
 * @param earlier - how many of the oldest past states are left out
 * @param after - how many states to come are shown
 * @returns the past states shown, the present and the states to come shown
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function shownStates<T>(
  history: History<T>,
  earlier: number,
  after: number,
): readonly HistoryState<T>[] {
  return [...history.past.slice(earlier), history.present, ...history.future.slice(0, after)];
}

/**
 * The list of the history's steps (US-077): the states around the present the window shows
 * (`shownHistoryWindow`), the oldest first, each named by the step that led to it from the state
 * before (`stepName`), the oldest state of the history `Start`, with where each stands against the
 * present (`stepPlace`) and how many states are left out on each side (`rowName`,
 * `shownStates`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history listed
 * @param rows - the most rows shown, the present's included
 * @param fields - reads the fields of a shape
 * @returns the rows shown and the states left out
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function historySteps<T>(
  history: History<T>,
  rows: number,
  fields: ShapeFields<T>,
): HistoryList<T> {
  const shown = shownHistoryWindow(history.past.length, history.future.length, rows);
  const earlier = history.past.length - shown.before;
  const states = shownStates(history, earlier, shown.after);
  const previous = [history.past[earlier - 1], ...states];

  return {
    earlier,
    later: history.future.length - shown.after,
    rows: states.map((state, index) => ({
      name: rowName(previous[index], state, fields),
      place: stepPlace(index, shown.before),
      state,
    })),
  };
}
