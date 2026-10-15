import {
  commitColorField,
  readColorField,
  readColorInput,
  writeColorChosen,
  writeKeptColor,
} from "../color-fields";
import { showColorPicker, updateColorPicker } from "../color-picker";

/**
 * Listens to each color's fields: the hexadecimal field keeps its color when it gets the focus,
 * and commits what it holds when it is left or Enter pressed (`commitColorField`), the page
 * redrawn after, and an open picker following the color and its opacity; a hexadecimal or an
 * opacity typed marks a color that follows the text's as chosen (`writeColorChosen`, US-041),
 * before the page is redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function listenToColorFields(document: Document, update: () => void): void {
  for (const swatch of document.querySelectorAll<HTMLElement>(".swatch")) {
    const ids = readColorField(swatch);
    const field = readColorInput(document, ids.color);

    field.addEventListener("focus", () => {
      writeKeptColor(document, ids.color);
    });

    field.addEventListener("input", () => {
      writeColorChosen(document, ids);
    });

    field.addEventListener("change", () => {
      commitColorField(document, ids);
      update();
      updateColorPicker(document);
    });

    readColorInput(document, ids.opacity).addEventListener("input", () => {
      writeColorChosen(document, ids);
      showColorPicker(document);
    });
  }
}
