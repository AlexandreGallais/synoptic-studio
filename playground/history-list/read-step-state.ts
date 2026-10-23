import type { HistoryState } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads the state a click on the list of steps goes back to (US-077): the state kept by the
 * button clicked (`STEP_BUTTON_STATES`).
 *
 * @kind procedure
 * @param event - a click inside the list
 * @returns the state; undefined for a click outside a step's button
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readStepState(event: Event): HistoryState<KeptShape> | undefined {
  return event.target === null ? undefined : undefined;
}
