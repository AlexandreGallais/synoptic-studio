import { commitColorField } from "./commit-color-field";
import { readColorField } from "./read-color-field";
import { readColorInput } from "./read-color-input";
import { writeKeptColor } from "./write-kept-color";

/**
 * Listens to each color's fields: the hexadecimal field keeps its color when it gets the focus,
 * and commits what it holds when it is left or Enter pressed (`commitColorField`), the page
 * redrawn after.
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

    field.addEventListener("change", () => {
      commitColorField(document, ids);
      update();
    });
  }
}
