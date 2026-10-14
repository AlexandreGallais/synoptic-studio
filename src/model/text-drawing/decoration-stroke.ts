import type { LineDecoration } from "./line-decoration";
import type { PositionedLine } from "./positioned-line";

/** Where a decoration's stroke sits, in user units at the text's size. */
export type StrokeMetrics = {
  /** Height of its top above the baseline, negative below it (`REF-OT-POST`, `REF-OT-OS2`). */
  readonly position: number;
  /** Its thickness. */
  readonly thickness: number;
};

/**
 * The rectangle of an underline or a strike-through along a line: as long as the line, from its
 * left edge, its top the font's position above the baseline, the SVG's ordinates growing
 * downwards (`DERIV-text-width`, step 6).
 *
 * Formula: x = left, y = baseline − position, width = w, height = thickness.
 *
 * @kind domain
 * @param line - the line, placed
 * @param stroke - the stroke's position and thickness
 * @returns the stroke's rectangle
 * @see DERIV-text-width
 */
export function decorationStroke(line: PositionedLine, stroke: StrokeMetrics): LineDecoration {
  return {
    height: stroke.thickness,
    width: line.width,
    x: line.left,
    y: line.y - stroke.position,
  };
}
