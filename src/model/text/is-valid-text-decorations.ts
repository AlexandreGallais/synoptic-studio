import { isInIntegerRange } from "../../math";
import { isColor } from "../color";

import { TEXT_RANGES } from "./text-ranges";

import type { Text } from "./text";

/**
 * Whether the colors of a text's underline and strike-through can enter the model (US-041): each
 * color of `#` and six lower-case hexadecimal digits (`isColor`), each opacity an integer from 0
 * to 100, and whether each was chosen true or false, as read from outside. They are checked
 * whether chosen or not, so that a text keeps valid colors when one is chosen again.
 *
 * @kind domain
 * @param text - the text to check
 * @returns `true` when both colors, both opacities and both switches are allowed
 * @see docs/domain/shapes.md#text
 */
export function isValidTextDecorations(text: Text): boolean {
  const { opacity } = TEXT_RANGES;

  return [
    ...[text.underlineColor, text.strikeThroughColor].map((color) => isColor(color)),
    ...[text.underlineOpacity, text.strikeThroughOpacity].map((value) =>
      isInIntegerRange(value, opacity.first, opacity.last),
    ),
    [text.hasUnderlineColor, text.hasStrikeThroughColor].every(
      (isChosen) => typeof isChosen === "boolean",
    ),
  ].every(Boolean);
}
