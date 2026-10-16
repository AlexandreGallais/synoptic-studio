import { listenToExport } from "../document";
import { listenToFonts } from "../installed-fonts";
import { selectElement } from "../page";
import { listenToTextPosition } from "../text-placement";

import { pressBold } from "./press-bold";
import { readTextModel } from "./read-text-model";
import { writeFittedBox } from "./write-fitted-box";

/**
 * Listens to the choice of Fit content or Box on the choice itself, so that the box's fields take
 * the text's block rounded up before the options redraw the text (`writeFittedBox`, US-055): a
 * browser sends `input`, then `change`, each reaching the choice before its group. A width or
 * height typed marks the box as the designer's (`data-box-typed`), kept from then on (US-057).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F17-US-057-box-size-kept.md
 */
function listenToBoxChoice(document: Document): void {
  const fit = selectElement(document, "text-fit");

  for (const type of ["input", "change"]) {
    fit.addEventListener(type, () => {
      writeFittedBox(document);
    });
  }

  for (const id of ["text-box-width", "text-box-height"]) {
    selectElement(document, id).addEventListener("input", () => {
      selectElement(document, "text-options").dataset["boxTyped"] = "true";
    });
  }
}

/**
 * Listens to the bold button, which gives 700 and gives back the weight before (`pressBold`,
 * US-037), and to the weight list, a weight chosen forgetting the weight held.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F05-US-037-bold-gives-back-the-weight-and-no-sentence-case.md
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
 * a value typed in X or Y redraws the text (`listenToTextPosition`, US-038); Box chosen after Fit
 * content first writes the block rounded up in the box's fields (`writeFittedBox`, US-055), unless
 * the designer typed the box, which then comes back (US-057).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function listenToTextFields(document: Document, update: () => void): void {
  const options = selectElement(document, "text-options");
  const italic = selectElement(document, "text-italic");

  listenToBoxChoice(document);
  listenToFonts(document, update);
  listenToExport(document, () => [readTextModel(document)]);
  options.addEventListener("input", update);
  options.addEventListener("change", update);

  listenToBold(document, update);
  listenToTextPosition(document, update);

  italic.addEventListener("click", () => {
    italic.setAttribute("aria-pressed", String(italic.getAttribute("aria-pressed") !== "true"));
    update();
  });
}
