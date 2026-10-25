import { isInIntegerRange } from "../../math";
import { isColor } from "../color";

import { TEXT_OPTIONS } from "./text-options";
import { TEXT_RANGES } from "./text-ranges";

import type { Text } from "./text";

/**
 * Whether the options of a text's box can enter the model: its width and height, its
 * maximum number of lines and its line height, integers from 0 to 100 000; its background's color
 * of `#` and six lower-case hexadecimal digits and its opacity, an integer from 0 to 100; its
 * vertical alignment and wrap style among their values; its box, ellipsis, trim and background
 * switches true or false, as read from outside. They are checked whether the text sits in
 * a box or fits its content, so that a text keeps valid options when it changes kind.
 *
 * @kind domain
 * @param text - the text to check
 * @returns `true` when every option of its box is allowed
 * @see RULE-shapes-text
 */
export function isValidTextBox(text: Text): boolean {
  const { boxSide, lineHeight, maximumLines, opacity } = TEXT_RANGES;
  const { verticalAlignments } = TEXT_OPTIONS;

  return [
    ...[text.boxWidth, text.boxHeight].map((side) =>
      isInIntegerRange(side, boxSide.first, boxSide.last),
    ),
    isInIntegerRange(text.maximumLines, maximumLines.first, maximumLines.last),
    isInIntegerRange(text.lineHeight, lineHeight.first, lineHeight.last),
    isInIntegerRange(text.backgroundOpacity, opacity.first, opacity.last),
    isColor(text.backgroundColor),
    verticalAlignments.includes(text.verticalAlignment),
    [text.isInBox, text.hasEllipsis, text.isTrimmed, text.hasBackground].every(
      (isOn) => typeof isOn === "boolean",
    ),
  ].every(Boolean);
}
