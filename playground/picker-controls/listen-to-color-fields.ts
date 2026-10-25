import {
  commitColorField,
  commitOpacityField,
  readColorField,
  readColorInput,
  writeKeptColor,
} from "../color-fields";
import { showColorPicker, updateColorPicker } from "../color-picker";

/**
 * Listens to each color's fields: the hexadecimal field keeps its color when it gets the focus,
 * and commits what it holds when it is left or Enter pressed (`commitColorField`), the page
 * redrawn after, and an open picker following the color and its opacity; an opacity applied, on
 * Enter or when it is left (US-087), and accepted marks a color that follows the text's as chosen
 * (`commitOpacityField`, US-041), before the page is redrawn.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see US-035
 */
export function listenToColorFields(document: Document, update: () => void): void {
  for (const swatch of document.querySelectorAll<HTMLElement>(".swatch")) {
    const ids = readColorField(swatch);
    const field = readColorInput(document, ids.color);
    const opacity = readColorInput(document, ids.opacity);

    field.addEventListener("focus", () => {
      writeKeptColor(document, ids.color);
    });

    field.addEventListener("change", () => {
      commitColorField(document, ids);
      update();
      updateColorPicker(document);
    });

    // Applied on Enter or when it is left, as every number field (US-087).
    opacity.addEventListener("change", () => {
      commitOpacityField(document, ids);
      showColorPicker(document);
    });
  }
}
