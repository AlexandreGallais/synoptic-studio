import { pairKerning } from "../kerning";

import { measuredLine } from "./measured-line";

import type { LineMeasure } from "./line-measure";
import type { FontMetrics } from "../font";
import type { Kerning } from "../kerning";

/** The end of a line with no character measured: no character, which kerns with nothing. */
const NO_CHARACTER = -1;

/** A combining mark or an invisible formatting character opening a string. */
const JOINING_START = /^[\p{M}\p{Cf}]/u;

/**
 * The advance of two lines joined, in font units: both advances, and the kerning of the left
 * one's last character with the right one's first, the new pair the join makes; none when the
 * left one is empty (`DERIV-text-width`, step 2).
 *
 * Formula: adv(a + b) = adv(a) + adv(b) + kern(aₙ, b₁), kern(∅, b₁) = 0.
 *
 * @kind domain
 * @param kerning - the font's kerning
 * @param left - the measure of the line before
 * @param right - the measure of the line after, with a character measured
 * @returns the advance and kerning of the joined line
 * @see DERIV-text-width
 */
function joinedAdvance(kerning: Kerning, left: LineMeasure, right: LineMeasure): number {
  const joint = left.last === NO_CHARACTER ? 0 : pairKerning(kerning, left.last, right.first);

  return left.advance + right.advance + joint;
}

/**
 * The measure of two lines joined, from their measures: their advances, units and separators
 * added with the kerning of the left one's last character and the right one's first, so that the
 * width is the one the joined line would be measured to
 * (`DERIV-text-width`, step 2). A right line opening with a combining mark or a formatting
 * character may compose or count with the left one's last character, and a right line with no
 * character measured kerns with nothing, so the joined line is then measured again
 * (`measuredLine`).
 *
 * @kind domain
 * @param metrics - the font face's metrics
 * @param left - the measure of the line before
 * @param right - the measure of the line after
 * @returns the joined line with its advance, units, separators and ends
 * @see DERIV-text-width
 */
export function joinedLineMeasure(
  metrics: FontMetrics,
  left: LineMeasure,
  right: LineMeasure,
): LineMeasure {
  if (right.first === NO_CHARACTER || JOINING_START.test(right.text)) {
    return measuredLine(metrics, left.text + right.text);
  }

  return {
    advance: joinedAdvance(metrics.kerning, left, right),
    first: left.first === NO_CHARACTER ? right.first : left.first,
    last: right.last,
    separatorCount: left.separatorCount + right.separatorCount,
    text: left.text + right.text,
    unitCount: left.unitCount + right.unitCount,
  };
}
