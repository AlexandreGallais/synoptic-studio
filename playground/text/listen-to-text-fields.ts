import { listenToExport } from "../document";
import { listenToFonts } from "../installed-fonts";
import { readDrawnTexts } from "../kept-shapes";
import { readNumberField } from "../number-fields";
import { selectElement } from "../page";
import { resetFittedBox, writeFittedBox } from "../text-box";

import { pressBold } from "./press-bold";
import { readTextModel } from "./read-text-model";

/**
 * Listens to the choice of Fit content or Box on the choice itself, so that the box's fields take
 * the text's block rounded up before the options redraw the text (`writeFittedBox`, US-055): a
 * browser sends `input`, then `change`, each reaching the choice before its group. A width or
 * height applied marks the box as the designer's (`data-box-typed`, `applyEditedValue`), kept
 * from then on (US-057).
 *
 * @kind procedure
 * @param document - page document
 * @see US-057
 */
function listenToBoxChoice(document: Document): void {
  const fit = selectElement(document, "text-fit");

  for (const type of ["input", "change"]) {
    fit.addEventListener(type, () => {
      writeFittedBox(document, readTextModel(document));
    });
  }
}

/**
 * Listens to the Text tool's options: a value typed or chosen redraws the text, except a field
 * acting as a number field, the box's Width and Height, whose value is applied on Enter or when it
 * is left (`readNumberField`, `listenToEditedFields`, US-085), redrawing nothing while it is typed.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/domain/interaction.md#_2-selector
 */
function listenToOptions(document: Document, update: () => void): void {
  const options = selectElement(document, "text-options");

  options.addEventListener("input", (event) => {
    if (readNumberField(event.target) === undefined) {
      update();
    }
  });

  options.addEventListener("change", update);
}

/**
 * Listens to the bold button, which gives 700 and gives back the weight before (`pressBold`,
 * US-037), and to the weight list, a weight chosen forgetting the weight held.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see US-037
 */
function listenToBold(document: Document, update: () => void): void {
  selectElement(document, "text-bold").addEventListener("click", () => {
    pressBold(document);
    update();
  });

  selectElement(document, "text-weight").addEventListener("change", () => {
    delete selectElement(document, "text-bold").dataset["heldWeight"];
  });
}

/**
 * Listens to the Text tool's fields: its font controls (`listenToFonts`) and **Export SVG**
 * (`listenToExport`, US-036); a value typed or chosen, the drawing's options among them, redraws
 * the text; the bold button gives 700 and gives back the weight before (`pressBold`, US-037), a
 * weight chosen in the list forgetting the weight held; the italic button turns italic on or off;
 * a field acting as a number field, the box's Width and Height, redraws nothing while it is typed,
 * its value applied on Enter or when it is left (`listenToEditedFields`, US-085); Box chosen after Fit
 * content first writes the block rounded up in the box's fields (`writeFittedBox`, US-055), unless
 * the designer typed the box, which then comes back (US-057); **Reset** puts the box back to that
 * size and forgets the box typed (`resetFittedBox`, US-057).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see US-031
 */
export function listenToTextFields(document: Document, update: () => void): void {
  const italic = selectElement(document, "text-italic");

  listenToBoxChoice(document);
  listenToFonts(document, update);
  listenToExport(document, () => readDrawnTexts(document));
  listenToOptions(document, update);
  listenToBold(document, update);

  selectElement(document, "text-box-reset").addEventListener("click", () => {
    resetFittedBox(document, readTextModel(document));
    update();
  });

  italic.addEventListener("click", () => {
    italic.setAttribute("aria-pressed", String(italic.getAttribute("aria-pressed") !== "true"));
    update();
  });
}
