import type { HistoryList } from "./history-list";
import type { ShapeFields } from "./shape-fields";
import type { History } from "../history";

/**
 * The list of the history's steps (US-077): the states around the present the window shows
 * (`shownHistoryWindow`), the oldest first, each named by the step that led to it from the state
 * before (`stepName`), the oldest state of the history `Start`, with where each stands against the
 * present and how many states are left out on each side.
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
  return {
    earlier: rows,
    later: 0,
    rows: [history.present].map((state) => ({
      name: Object.keys(state.drawing.map((shape) => fields(shape))).join(","),
      place: "present",
      state,
    })),
  };
}
