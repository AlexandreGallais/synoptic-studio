import type { TextStrokeKind } from "./text-stroke-kind";
import type { ColorFill } from "../color";
import type { Text } from "../text";

/**
 * The color and the opacity an underline or a strike-through is drawn in (US-041): its own once
 * the designer chose one, kept when the text's color changes; else the text's, which it follows,
 * so that a text of F05 draws as before (agent's reading, provisional, to confirm at VAL-016).
 *
 * @kind domain
 * @param text - the text, its color and its strokes' colors
 * @param kind - the underline or the strike-through
 * @returns the stroke's color and opacity
 * @see docs/domain/shapes.md#text
 */
export function textStrokeFill(text: Text, kind: TextStrokeKind): ColorFill {
  const own =
    kind === "underline"
      ? {
          color: text.underlineColor,
          isChosen: text.hasUnderlineColor,
          opacity: text.underlineOpacity,
        }
      : {
          color: text.strikeThroughColor,
          isChosen: text.hasStrikeThroughColor,
          opacity: text.strikeThroughOpacity,
        };

  return own.isChosen
    ? { color: own.color, opacity: own.opacity }
    : { color: text.color, opacity: text.opacity };
}
