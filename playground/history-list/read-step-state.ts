import { STEP_BUTTON_STATES } from "./step-button-states";

import type { HistoryState } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads the state a click on the list of steps goes back to: the state kept by the
 * button clicked, or holding the element clicked (`STEP_BUTTON_STATES`).
 *
 * @kind procedure
 * @param event - a click inside the list
 * @returns the state; undefined for a click outside a step's button
 */
export function readStepState(event: Event): HistoryState<KeptShape> | undefined {
  if (!(event.target instanceof Element)) {
    return undefined;
  }

  return STEP_BUTTON_STATES.get(event.target.closest("button") ?? event.target);
}
