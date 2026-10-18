import { readColorField } from "./read-color-field";
import { readShownColor } from "./read-shown-color";

/**
 * Paints each swatch with its field's color, or, while what is typed there is no color, the color
 * the field kept (`readShownColor`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function writeColorSwatches(document: Document): void {
  for (const swatch of document.querySelectorAll<HTMLElement>(".swatch")) {
    swatch.style.backgroundColor = readShownColor(document, readColorField(swatch).color);
  }
}
