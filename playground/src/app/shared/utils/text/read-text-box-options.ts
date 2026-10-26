import { readNumber } from "../page";

import { readFieldValue } from "./read-field-value";

import type { Text, TextVerticalAlignment } from "editor";

/** The options of a text's box the Text tool reads, its background among them. */
type TextBoxOptions = Pick<
  Text,
  | "backgroundColor"
  | "backgroundOpacity"
  | "boxHeight"
  | "boxWidth"
  | "hasBackground"
  | "hasEllipsis"
  | "isInBox"
  | "isTrimmed"
  | "lineHeight"
  | "maximumLines"
  | "verticalAlignment"
>;

/**
 * Reads the box's fields of the Text tool: Fit content or Box, its width and height, the maximum
 * number of lines, the ellipsis, the vertical alignment, the wrap
 * style, the line height, the trim and the background, on or off, with its color and opacity.
 * They are read whichever kind is chosen, so that the text
 * keeps them when it changes kind (`RULE-shapes-box`); a number left empty is
 * `NaN`, which the model refuses (`isValidText`). The page opens the box at 200 × 100, a playground
 * starting value, replaced by the text's block rounded up when Box is chosen after Fit content
 * (`writeFittedBox`); the model's default stays 0 × 0 (`TEXT_BOX_DEFAULTS`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the options of its box, valid or not
 */
export function readTextBoxOptions(document: Document): TextBoxOptions {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const verticalAlignment = readFieldValue(
    document,
    "text-vertical-alignment",
  ) as TextVerticalAlignment;

  return {
    backgroundColor: readFieldValue(document, "text-background-color"),
    backgroundOpacity: readNumber(document, "text-background-opacity"),
    boxHeight: readNumber(document, "text-box-height"),
    boxWidth: readNumber(document, "text-box-width"),
    hasBackground: document.querySelector<HTMLInputElement>("#text-background")?.checked === true,
    hasEllipsis: document.querySelector<HTMLInputElement>("#text-ellipsis")?.checked === true,
    isInBox: readFieldValue(document, "text-fit") === "box",
    isTrimmed: document.querySelector<HTMLInputElement>("#text-trim")?.checked === true,
    lineHeight: readNumber(document, "text-line-height"),
    maximumLines: readNumber(document, "text-maximum-lines"),
    verticalAlignment,
  };
}
