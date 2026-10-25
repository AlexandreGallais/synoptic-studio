import type { StepPlace } from "./step-place";
import type { HistoryState } from "../history";

/**
 * A row of the list of the history's steps: the state after a step, the step's name and
 * where it stands against the present.
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type HistoryRow<T> = {
  /** The step's name by its action, `Move`, `Delete`, `Width`, or `Start` for the oldest state. */
  readonly name: string;
  /** Where the state stands against the present. */
  readonly place: StepPlace;
  /** The state after the step, the one a click on the row goes back to. */
  readonly state: HistoryState<T>;
};
