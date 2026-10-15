import { parseHexColor } from "../../src";

import { readColorField } from "./read-color-field";
import { readColorInput } from "./read-color-input";
import { readKeptColor } from "./read-kept-color";

/**
 * Paints each swatch with its field's color, or, while what is typed there is no color, the color
 * the field kept.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function writeColorSwatches(document: Document): void {
  for (const swatch of document.querySelectorAll<HTMLElement>(".swatch")) {
    const field = readColorInput(document, readColorField(swatch).color);

    swatch.style.backgroundColor = parseHexColor(field.value)?.color ?? readKeptColor(field);
  }
}
