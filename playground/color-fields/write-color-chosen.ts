import { readColorInput } from "./read-color-input";

import type { ColorField } from "./color-field";

/**
 * Marks a color as chosen by the designer when its hexadecimal or its opacity is typed or picked:
 * a color that follows the text's, an underline's or a strike-through's, stops following it, its
 * check box unticked, and keeps what was typed or picked from then on (US-041); a color that
 * follows none is left as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param ids - the color's fields
 * @see docs/backlog/stories/E15-F16-US-041-underline-and-strike-through-colors.md
 */
export function writeColorChosen(document: Document, ids: ColorField): void {
  if (ids.follows !== "") {
    readColorInput(document, ids.follows).checked = false;
  }
}
