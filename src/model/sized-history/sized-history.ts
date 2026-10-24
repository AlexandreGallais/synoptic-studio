import type { HistoryMeasure } from "./history-measure";
import type { History } from "../history";

/**
 * A history with its size kept (EN-147): its states, and what they hold beyond the present
 * (`HistoryMeasure`), updated by each step, undo, redo and bound from the states and the shapes it
 * moves alone, so that a step costs what it changes, not what the history holds (ADR-0097).
 *
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 */
export type SizedHistory<T> = History<T> & HistoryMeasure;
