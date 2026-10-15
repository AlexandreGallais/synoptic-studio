import { parseHexColor } from "../../src";

import { readColorInput } from "./read-color-input";

/**
 * Keeps the color a hexadecimal field holds, if it is one, as the color it shows again when what
 * is typed next is refused: on focus, and on each color written or picked.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the hexadecimal field's id
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function writeKeptColor(document: Document, id: string): void {
  const field = readColorInput(document, id);
  const typed = parseHexColor(field.value);

  if (typed !== undefined) {
    field.dataset["kept"] = typed.color;
  }
}
