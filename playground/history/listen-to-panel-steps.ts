import { listenToNewText } from "../new-text";
import { selectElement } from "../page";

import { schedulePanelStep } from "./schedule-panel-step";

/** The color picker's sliders and its plane, which take a step at each key and each release. */
const SLIDERS = '[role="slider"]';

/** The fields of several shapes, which an arrow steps (US-081). */
const SEVERAL_INPUTS = "#several-fields input";

/** The events of the panel that end a change, each on the elements it counts on. */
const PANEL_ENDS = [
  { selector: "*", type: "change" },
  { selector: "button", type: "click" },
  { selector: SEVERAL_INPUTS, type: "keyup" },
  { selector: SLIDERS, type: "keyup" },
  { selector: SLIDERS, type: "pointerup" },
];

/**
 * Asks a step for an event of the panel that reached an element of a kind, itself or inside it
 * (`schedulePanelStep`); none for an event elsewhere.
 *
 * @kind procedure
 * @param document - page document
 * @param event - an event of the panel
 * @param selector - CSS selector of the kind
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function scheduleStepAt(document: Document, event: Event, selector: string): void {
  if (event.target instanceof Element && event.target.closest(selector) !== null) {
    schedulePanelStep(document);
  }
}

/**
 * Listens to the ends of the panel's changes (F40, `PANEL_ENDS`): a step asked at each value
 * applied or choice made in the panel, `change` fired by Enter, by leaving a field typed in or by
 * a choice (`REF-MDN-CHANGE-EVENT`), at each click on one of its buttons, at each key released
 * in a field of several shapes, which an arrow steps (US-081), a key that changes no shape taking
 * none, at each key released on a slider of the color picker and at each release of the pointer on one, or on the plane
 * (SP-029's answer 4); and at the Content field of a new text left with a content typed, its
 * placing and its typing one step (`listenToNewText`, SP-029's answer 5). A value typed back as it was, or a button that changes no shape, takes
 * none, the drawing the same (`historyWithStep`). Heard as the event bubbles up to the panel,
 * after the field's or the button's own listeners, which redraw the page (`schedulePanelStep`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToPanelSteps(document: Document): void {
  const panel = selectElement(document, "panel");

  for (const { selector, type } of PANEL_ENDS) {
    panel.addEventListener(type, (event) => {
      scheduleStepAt(document, event, selector);
    });
  }

  listenToNewText(document, () => {
    schedulePanelStep(document);
  });
}
