import { parseHexColor } from "../../src";
import { writeInputValue, writeText } from "../page";

import { readColorInput } from "./read-color-input";
import { readKeptColor } from "./read-kept-color";
import { writeColorChosen } from "./write-color-chosen";
import { writeKeptColor } from "./write-kept-color";

import type { ColorField } from "./color-field";

/**
 * Commits what a hexadecimal field holds when it is left, or Enter pressed: a color the library
 * reads (`parseHexColor`) is shown in its stored form, `#` and six lower-case digits, its alpha,
 * if any, written in the opacity field, and a color that follows the text's marked chosen
 * (`writeColorChosen`, US-041); anything else is refused, the field showing the color it kept
 * again and the line under it saying why, a color that follows the text's still following it
 * (`docs/domain/interaction.md` §5).
 *
 * @kind procedure
 * @param document - page document
 * @param ids - the color's fields
 * @see US-035
 */
export function commitColorField(document: Document, ids: ColorField): void {
  const field = readColorInput(document, ids.color);
  const typed = parseHexColor(field.value);

  if (typed === undefined) {
    const kept = readKeptColor(field);

    writeText(
      document,
      ids.message,
      `“${field.value}” is no hexadecimal color: ${kept} kept. Type 3, 4, 6 or 8 hexadecimal digits, with or without #.`,
    );

    field.value = kept;

    return;
  }

  writeText(document, ids.message, "");
  field.value = typed.color;
  writeKeptColor(document, ids.color);
  writeColorChosen(document, ids);

  if (typed.opacity !== undefined) {
    writeInputValue(document, ids.opacity, typed.opacity);
  }
}
