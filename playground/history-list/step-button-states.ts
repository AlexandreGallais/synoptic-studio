import type { HistoryState } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * The state each button of the list of steps goes back to (US-077), by the button, so that a
 * click finds the state the history holds, whatever the history recorded since the list was shown.
 */
export const STEP_BUTTON_STATES = new WeakMap<Element, HistoryState<KeptShape>>();
