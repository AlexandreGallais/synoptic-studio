import { isInIntegerRange } from "../../math";

import { isTextColor } from "./is-text-color";
import { TEXT_OPTIONS } from "./text-options";
import { TEXT_RANGES } from "./text-ranges";

import type { Text } from "./text";

/**
 * Whether the options of a text's box (F14) can enter the model: its width and height, its
 * maximum number of lines and its line height, integers from 0 to 100 000; its background's color
 * of six hexadecimal digits and its opacity, an integer from 0 to 100; its long-word option,
 * vertical alignment and wrap style among their values. They are checked whether the text sits in
 * a box or fits its content, so that a text keeps valid options when it changes kind.
 *
 * @kind domain
 * @param text - the text to check
 * @returns `true` when every option of its box is allowed
 * @see docs/domain/shapes.md#text
 */
export function isValidTextBox(text: Text): boolean {
  const { boxSide, lineHeight, maximumLines, opacity } = TEXT_RANGES;
  const { longWords, verticalAlignments, wrapStyles } = TEXT_OPTIONS;

  return [
    ...[text.boxWidth, text.boxHeight].map((side) =>
      isInIntegerRange(side, boxSide.first, boxSide.last),
    ),
    isInIntegerRange(text.maximumLines, maximumLines.first, maximumLines.last),
    isInIntegerRange(text.lineHeight, lineHeight.first, lineHeight.last),
    isInIntegerRange(text.backgroundOpacity, opacity.first, opacity.last),
    isTextColor(text.backgroundColor),
    longWords.includes(text.longWord),
    verticalAlignments.includes(text.verticalAlignment),
    wrapStyles.includes(text.wrapStyle),
  ].every(Boolean);
}
