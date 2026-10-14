import { selectElement } from "../page";

import { readFieldValue } from "./read-field-value";

/** Weights of the bold button, pressed and released (`REF-CSS-FONTS-4`, §2.2: bold 700, normal 400). */
const WEIGHTS = { bold: "700", normal: "400" };

/**
 * Listens to the Text tool's fields: a value typed or chosen redraws the text; the bold button
 * sets the weight to 700, or back to 400 when it already is 700; the italic button turns italic
 * on or off.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function listenToTextFields(document: Document, update: () => void): void {
  const options = selectElement(document, "text-options");
  const weight = selectElement(document, "text-weight");
  const italic = selectElement(document, "text-italic");

  options.addEventListener("input", update);
  options.addEventListener("change", update);

  selectElement(document, "text-bold").addEventListener("click", () => {
    const isBold = readFieldValue(document, "text-weight") === WEIGHTS.bold;

    if (weight instanceof HTMLSelectElement) {
      weight.value = isBold ? WEIGHTS.normal : WEIGHTS.bold;
    }

    update();
  });

  italic.addEventListener("click", () => {
    italic.setAttribute("aria-pressed", String(italic.getAttribute("aria-pressed") !== "true"));
    update();
  });
}
