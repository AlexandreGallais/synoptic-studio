import { readNumber } from "../page";

import { readFieldValue } from "./read-field-value";

import type { Text, TextLongWord, TextVerticalAlignment, TextWrapStyle } from "../../src";

/** The options of a text's box the Text tool reads (US-033); the background comes with US-034. */
type TextBoxOptions = Pick<
  Text,
  | "boxHeight"
  | "boxWidth"
  | "hasEllipsis"
  | "isInBox"
  | "isTrimmed"
  | "lineHeight"
  | "longWord"
  | "maximumLines"
  | "verticalAlignment"
  | "wrapStyle"
>;

/**
 * Reads the box's fields of the Text tool: Fit content or Box, its width and height, what becomes
 * of a long word, the maximum number of lines, the ellipsis, the vertical alignment, the wrap
 * style, the line height and the trim. They are read whichever kind is chosen, so that the text
 * keeps them when it changes kind (`docs/domain/shapes.md`, "Box"); a number left empty is
 * `NaN`, which the model refuses (`isValidText`).
 *
 * @kind procedure
 * @param document - page document
 * @returns the options of its box, valid or not
 * @see docs/backlog/stories/E15-F14-US-033-put-a-text-in-a-box.md
 */
export function readTextBoxOptions(document: Document): TextBoxOptions {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const longWord = readFieldValue(document, "text-long-word") as TextLongWord;
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const verticalAlignment = readFieldValue(
    document,
    "text-vertical-alignment",
  ) as TextVerticalAlignment;
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the page offers only the values of this type
  const wrapStyle = readFieldValue(document, "text-wrap") as TextWrapStyle;

  return {
    boxHeight: readNumber(document, "text-box-height"),
    boxWidth: readNumber(document, "text-box-width"),
    hasEllipsis: document.querySelector<HTMLInputElement>("#text-ellipsis")?.checked === true,
    isInBox: readFieldValue(document, "text-fit") === "box",
    isTrimmed: document.querySelector<HTMLInputElement>("#text-trim")?.checked === true,
    lineHeight: readNumber(document, "text-line-height"),
    longWord,
    maximumLines: readNumber(document, "text-maximum-lines"),
    verticalAlignment,
    wrapStyle,
  };
}
