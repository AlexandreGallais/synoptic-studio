import { isInIntegerRange } from "../../math";
import { isShapeAngle } from "../placement";

import { isFontName } from "./is-font-name";
import { isTextColor } from "./is-text-color";
import { TEXT_OPTIONS } from "./text-options";
import { TEXT_RANGES } from "./text-ranges";

import type { Text } from "./text";

/**
 * Whether a text can enter the model: a size from 1 to 100 000, a weight from 100 to 900, an
 * opacity from 0 to 100, a letter and a word spacing from −100 000 to 100 000, all integers; a
 * color of six hexadecimal digits; an integer position (ADR-0003); an angle from 0 to 359
 * (ADR-0035); a case, an alignment and a direction among their values; a font name and a
 * fallback name that are not empty, trimmed and free of control characters (`isFontName`). Any
 * content is allowed, the empty one included, which draws nothing.
 *
 * @kind domain
 * @param text - the text to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#text
 */
export function isValidText(text: Text): boolean {
  const { opacity, size, spacing, weight } = TEXT_RANGES;
  const { alignments, cases, directions } = TEXT_OPTIONS;

  return [
    isInIntegerRange(text.size, size.first, size.last),
    isInIntegerRange(text.weight, weight.first, weight.last),
    isInIntegerRange(text.opacity, opacity.first, opacity.last),
    ...[text.letterSpacing, text.wordSpacing].map((value) =>
      isInIntegerRange(value, spacing.first, spacing.last),
    ),
    isTextColor(text.color),
    isShapeAngle(text.angle),
    [text.x, text.y].every((value) => Number.isSafeInteger(value)),
    cases.includes(text.textCase),
    alignments.includes(text.alignment),
    directions.includes(text.direction),
    isFontName(text.font),
    isFontName(text.fallback),
  ].every(Boolean);
}
