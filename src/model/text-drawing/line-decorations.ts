import { fontUnitsAtSize } from "../font";

import { decorationStroke } from "./decoration-stroke";

import type { FontMetrics } from "../font";
import type { Text } from "../text";
import type { StrokeMetrics } from "./decoration-stroke";
import type { LineDecoration } from "./line-decoration";
import type { PositionedLine } from "./positioned-line";

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
 * The strokes along one line of a text: its underline from the font's `post` table when it is
 * underlined, its strike-through from `OS/2` when it is struck through, drawn as rectangles
 * rather than `text-decoration` (ADR-0052). A line with no length, empty or tightened past
 * nothing, has none, and a stroke the font gives no thickness, 0 or negative, is not drawn.
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param text - its size and decorations
 * @param line - the line, placed
 * @returns its underline then its strike-through, those it has
 * @see DERIV-text-width
 */
export function lineDecorations(
  metrics: FontMetrics,
  text: Text,
  line: PositionedLine,
): readonly LineDecoration[] {
  const underline = { position: metrics.underlinePosition, thickness: metrics.underlineThickness };
  const strikeout = { position: metrics.strikeoutPosition, thickness: metrics.strikeoutSize };
  const strokes = [
    ...(text.isUnderlined ? [underline] : []),
    ...(text.isStruckThrough ? [strikeout] : []),
  ].filter((stroke) => stroke.thickness > 0);

  return line.width > 0
    ? strokes.map((stroke) => decorationStroke(line, strokeAtSize(metrics, text.size, stroke)))
    : [];
}
