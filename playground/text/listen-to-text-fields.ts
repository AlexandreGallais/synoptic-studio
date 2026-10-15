import { listenToExport } from "../document";
import { listenToFonts } from "../fonts";
import { selectElement } from "../page";
import { listenToTextPosition } from "../text-placement";

import { pressBold } from "./press-bold";
import { readTextModel } from "./read-text-model";

/**
 * Listens to the Text tool's fields: its font controls (`listenToFonts`) and **Export SVG**
 * (`listenToExport`, US-036); a value typed or chosen, the drawing's options among them, redraws
 * the text; the bold button gives 700 and gives back the weight before (`pressBold`, US-037), a
 * weight chosen in the list forgetting the weight held; the italic button turns italic on or off;
 * a value typed in X or Y redraws the text (`listenToTextPosition`, US-038).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function listenToTextFields(document: Document, update: () => void): void {
  const options = selectElement(document, "text-options");
  const italic = selectElement(document, "text-italic");

  listenToFonts(document, update);
  listenToExport(document, () => [readTextModel(document)]);
  options.addEventListener("input", update);
  options.addEventListener("change", update);

  selectElement(document, "text-bold").addEventListener("click", () => {
    pressBold(document);
    update();
  });

  selectElement(document, "text-weight").addEventListener("change", () => {
    delete selectElement(document, "text-bold").dataset["heldWeight"];
  });

  listenToTextPosition(document, update);

  italic.addEventListener("click", () => {
    italic.setAttribute("aria-pressed", String(italic.getAttribute("aria-pressed") !== "true"));
    update();
  });
}
