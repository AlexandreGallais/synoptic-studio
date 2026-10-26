import type { KeptShape } from "../kept-shapes";
import type { HistoryState } from "editor";

/**
 * The state each button of the list of steps goes back to, by the button, so that a
 * click finds the state the history holds, whatever the history recorded since the list was shown.
 */
export const STEP_BUTTON_STATES = new WeakMap<Element, HistoryState<KeptShape>>();
