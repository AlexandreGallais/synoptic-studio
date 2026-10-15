import { TEXT_BOX_DEFAULTS, toStoredColors } from "../../src";
import { readFontSession } from "../fonts";
import { readNumber, selectElement } from "../page";
import { readAngle } from "../position";
import { readTextPosition } from "../text-placement";

import { readFieldValue } from "./read-field-value";
import { readTextBoxOptions } from "./read-text-box-options";

import type { Text, TextAlignment, TextCase, TextDirection } from "../../src";

/** The family named after the font, when a reader lacks it (F05 refinement, round 2). */
const FALLBACK = "sans-serif";

/**
 * Reads the Text tool's switches: the italic button, the underline and strike-through check
 * boxes.
 *
 * @kind procedure
 * @param document - page document
 * @returns whether each is on
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
function readTextSwitches(
  document: Document,
): Pick<Text, "isItalic" | "isStruckThrough" | "isUnderlined"> {
  return {
    isItalic: selectElement(document, "text-italic").getAttribute("aria-pressed") === "true",
    isStruckThrough: document.querySelector<HTMLInputElement>("#text-strike")?.checked === true,
    isUnderlined: document.querySelector<HTMLInputElement>("#text-underline")?.checked === true,
  };
}

/** The colors of a text's underline and strike-through the Text tool reads (US-041). */
type TextStrokeColors = Pick<
  Text,
  | "hasStrikeThroughColor"
  | "hasUnderlineColor"
  | "strikeThroughColor"
  | "strikeThroughOpacity"
  | "underlineColor"
  | "underlineOpacity"
>;

/**
 * Reads the Text tool's underline and strike-through colors: each chosen while its "follows the
 * text's color" box is unticked, with the color and the opacity of its fields, read whether
 * chosen or not, so that the text keeps them; a number left empty is `NaN`, which the model
 * refuses (`isValidText`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the strokes' colors, valid or not
 * @see docs/backlog/stories/E15-F16-US-041-underline-and-strike-through-colors.md
 */
function readTextStrokeColors(document: Document): TextStrokeColors {
  return {
    hasStrikeThroughColor:
      document.querySelector<HTMLInputElement>("#text-strike-follows")?.checked === false,
    hasUnderlineColor:
      document.querySelector<HTMLInputElement>("#text-underline-follows")?.checked === false,
    strikeThroughColor: readFieldValue(document, "text-strike-color"),
    strikeThroughOpacity: readNumber(document, "text-strike-opacity"),
    underlineColor: readFieldValue(document, "text-underline-color"),
    underlineOpacity: readNumber(document, "text-underline-opacity"),
  };
}

/**
 * Reads the text typed in the Text tool's fields, its font the family measured with (US-036), its box's among them (`readTextBoxOptions`), its underline's and strike-through's colors (`readTextStrokeColors`, US-041), its colors in their stored form (`toStoredColors`); its position and its angle are the shape's X, Y
 * and angle, its exact position while X and Y show the one last drawn (`readTextPosition`, US-038), the angle kept from 0 to 359 (`readAngle`). The choices are read as the page offers them, so they hold one
 * of their values.
 *
 * @kind procedure
 * @param document - page document
 * @returns the text, valid or not
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function readTextModel(document: Document): Text {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const alignment = readFieldValue(document, "text-alignment") as TextAlignment;
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const textCase = readFieldValue(document, "text-case") as TextCase;
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const direction = readFieldValue(document, "text-direction") as TextDirection;

  return toStoredColors({
    ...TEXT_BOX_DEFAULTS,
    ...readTextBoxOptions(document),
    alignment,
    angle: readAngle(document),
    color: readFieldValue(document, "text-color"),
    content: readFieldValue(document, "text-content"),
    direction,
    fallback: FALLBACK,
    font: readFontSession(document).measured,
    ...readTextSwitches(document),
    ...readTextStrokeColors(document),
    letterSpacing: readNumber(document, "text-letter-spacing"),
    opacity: readNumber(document, "text-opacity"),
    size: readNumber(document, "text-size"),
    textCase,
    weight: Number(readFieldValue(document, "text-weight")),
    wordSpacing: readNumber(document, "text-word-spacing"),
    ...readTextPosition(document),
  });
}
