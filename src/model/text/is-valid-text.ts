import { isInIntegerRange } from "../../math";
import { isShapeAngle } from "../placement";

import { isTextColor } from "./is-text-color";
import { TEXT_RANGES } from "./text-ranges";

import type { Text } from "./text";

/**
 * Whether a text can enter the model: a size from 1 to 100 000, a weight from 100 to 900 and an
 * opacity from 0 to 100, all integers; a color of six hexadecimal digits; integer letter and word
 * spacing and position (ADR-0003); an angle from 0 to 359 (ADR-0035). Any content is allowed, the
 * empty one included, which draws nothing.
 *
 * @kind domain
 * @param text - the text to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#text
 */
export function isValidText(text: Text): boolean {
  const { opacity, size, weight } = TEXT_RANGES;

  return [
    isInIntegerRange(text.size, size.first, size.last),
    isInIntegerRange(text.weight, weight.first, weight.last),
    isInIntegerRange(text.opacity, opacity.first, opacity.last),
    isTextColor(text.color),
    isShapeAngle(text.angle),
    ...[text.letterSpacing, text.wordSpacing, text.x, text.y].map((value) =>
      Number.isSafeInteger(value),
    ),
  ].every(Boolean);
}
