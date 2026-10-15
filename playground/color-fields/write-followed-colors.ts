import { readColorField } from "./read-color-field";
import { readColorInput } from "./read-color-input";
import { readShownColor } from "./read-shown-color";
import { writeKeptColor } from "./write-kept-color";

/**
 * Shows the text's color and opacity in the fields of each color that follows it, the
 * underline's and the strike-through's while their box is ticked (`data-follows`), so that its
 * swatch, its fields and its picker show the color it is drawn in, and a stroke whose box is
 * unticked starts from it (US-041); the text's color is the one its field stands for
 * (`readShownColor`).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F16-US-041-underline-and-strike-through-colors.md
 */
export function writeFollowedColors(document: Document): void {
  const color = readShownColor(document, "text-color");
  const opacity = readColorInput(document, "text-opacity").value;
  const following = [...document.querySelectorAll<HTMLElement>(".swatch[data-follows]")]
    .map((swatch) => readColorField(swatch))
    .filter((ids) => readColorInput(document, ids.follows).checked);

  for (const ids of following) {
    readColorInput(document, ids.color).value = color;
    readColorInput(document, ids.opacity).value = opacity;
    writeKeptColor(document, ids.color);
  }
}
