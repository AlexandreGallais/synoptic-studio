import { fontUnitsAtSize } from "../font";
import { textStrokeFill } from "../text-stroke";

import { decorationStroke } from "./decoration-stroke";
import { isLineWithLength } from "./is-line-with-length";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { TextStrokeKind } from "../text-stroke";
import type { StrokeMetrics } from "./decoration-stroke";
import type { DrawnStroke } from "./drawn-stroke";
import type { PositionedLine } from "../text-layout";

/** A stroke a text asks for, with the font's position and thickness for it. */
type AskedStroke = StrokeMetrics & {
  /** The underline or the strike-through. */
  readonly kind: TextStrokeKind;
};

/**
 * The strokes a text asks for, in font units: its underline from the font's `post` table when it
 * is underlined, its strike-through from `OS/2` when it is struck through.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param text - whether it is underlined and struck through
 * @returns its underline then its strike-through, those it asks for
 * @see DERIV-text-width
 */
function askedStrokes(metrics: FontMetrics, text: Text): readonly AskedStroke[] {
  const underline: AskedStroke = {
    kind: "underline",
    position: metrics.underlinePosition,
    thickness: metrics.underlineThickness,
  };
  const strikeThrough: AskedStroke = {
    kind: "strikeThrough",
    position: metrics.strikeoutPosition,
    thickness: metrics.strikeoutSize,
  };

  return [
    ...(text.isUnderlined ? [underline] : []),
    ...(text.isStruckThrough ? [strikeThrough] : []),
  ];
}

/**
 * A decoration's position and thickness, from font units to the text's size.
 *
 * @kind domain
 * @param metrics - the font face's metrics, for its units per em
 * @param size - the text's size
 * @param units - the position and thickness in font units
 * @returns them in user units
 * @see DERIV-text-width
 */
function strokeAtSize(metrics: FontMetrics, size: number, units: StrokeMetrics): StrokeMetrics {
  return {
    position: fontUnitsAtSize(units.position, size, metrics.unitsPerEm),
    thickness: fontUnitsAtSize(units.thickness, size, metrics.unitsPerEm),
  };
}

/**
 * The strokes along one line of a text: those it asks for (`askedStrokes`), drawn as rectangles
 * rather than `text-decoration` (ADR-0052), each in its own color and opacity once chosen, else
 * in the text's (`textStrokeFill`, US-041). A line with no length, empty or tightened past
 * nothing, has none, and a stroke the font gives no thickness, 0 or negative, is not drawn.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param text - its size, its decorations and their colors
 * @param line - the line, placed
 * @returns its underline then its strike-through, those it has, each with its color
 * @see DERIV-text-width
 */
export function lineDecorations(
  metrics: FontMetrics,
  text: Text,
  line: PositionedLine,
): readonly DrawnStroke[] {
  const strokes = askedStrokes(metrics, text).filter((stroke) => stroke.thickness > 0);

  return isLineWithLength(line)
    ? strokes.map((stroke) => ({
        ...decorationStroke(line, strokeAtSize(metrics, text.size, stroke)),
        ...textStrokeFill(text, stroke.kind),
      }))
    : [];
}
