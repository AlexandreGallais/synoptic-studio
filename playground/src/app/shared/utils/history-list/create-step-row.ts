import { STEP_BUTTON_STATES } from "./step-button-states";

import type { KeptShape } from "../kept-shapes";
import type { HistoryRow } from "editor";

/**
 * Creates the row of a step of the list: a button named by the step's action, its place
 * against the present in `data-place`, the present's marked as the current step
 * (`aria-current`), the state it goes back to kept by the button (`STEP_BUTTON_STATES`).
 *
 * @kind procedure
 * @param document - page document
 * @param row - the step's name, place and state (`historySteps`)
 * @returns the list item holding the button
 */
export function createStepRow(document: Document, row: HistoryRow<KeptShape>): HTMLLIElement {
  const item = document.createElement("li");
  const button = document.createElement("button");

  button.type = "button";
  button.textContent = row.name;
  button.dataset["place"] = row.place;

  if (row.place === "present") {
    button.setAttribute("aria-current", "step");
  }

  STEP_BUTTON_STATES.set(button, row.state);
  item.append(button);

  return item;
}
