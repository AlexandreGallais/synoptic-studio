import { characterUnits, isNoBreakAfter } from "../line-characters";
import { joinedLineMeasure, measuredLine } from "../text-block";
import { isMeasureFitting, withoutTrailingSpaces } from "../text-wrap";

import { textEllipsis } from "./text-ellipsis";

import type { LineMeasure } from "../text-block";
import type { TextInBox } from "../text-wrap";

/** The space U+0020, which never ends what is kept before the ellipsis. */
const SPACE = " ";

/** A line being read for its cut: the measure of its start read so far, and the longest start kept. */
type EllipsisCut = {
  /** The measure of the line's start read so far. */
  readonly start: LineMeasure;
  /** The longest start that fits with the ellipsis, or the first one a line may end with; undefined before it. */
  readonly kept: string | undefined;
};

/** What a cut is read with: the text, its face's metrics, and the measure of its ellipsis. */
type CutContext = TextInBox & {
  /** The measure of the ellipsis. */
  readonly ellipsis: LineMeasure;
};

/**
 * A line's cut read one unit further: its start grows by the unit, measured once
 * (`joinedLineMeasure`); a start a line may end with — its last unit neither a space nor a
 * no-break space (`isNoBreakAfter`) — is kept when it is the first such, which always stays, or
 * when it fits with the ellipsis after it.
 *
 * @kind domain
 * @param cut - the cut read so far
 * @param unit - the line's next unit
 * @param context - the text, its face's metrics and its ellipsis's measure
 * @returns the cut read with the unit
 * @see DERIV-text-box
 */
function cutWithUnit(cut: EllipsisCut, unit: string, context: CutContext): EllipsisCut {
  const { metrics, text } = context;
  const start = joinedLineMeasure(metrics, cut.start, measuredLine(metrics, unit));
  const isKept =
    unit !== SPACE &&
    !isNoBreakAfter(unit) &&
    (cut.kept === undefined ||
      isMeasureFitting(metrics, joinedLineMeasure(metrics, start, context.ellipsis), text));

  return { kept: isKept ? start.text : cut.kept, start };
}

/**
 * What a line keeps before its ellipsis: the line without its trailing spaces, then without its
 * last units, with the spaces before them, until it and its ellipsis (`textEllipsis`) fit the box;
 * never a start ending with a no-break space, which stays with the unit after it; the first unit
 * other than a space always stays, so that a box narrower than it and the ellipsis shows both,
 * overflowing (`DERIV-text-box`, step 6, provisional). The line's start is measured once, unit by
 * unit, and the longest start that fits is kept, which is where removing units from the end first
 * fits.
 *
 * @kind domain
 * @param context - the text and the metrics of its face
 * @param line - a line shown, in the text's case
 * @returns the start kept, the ellipsis to come after it; empty for a line of spaces
 * @see REF-CSS-OVERFLOW-3
 */
export function ellipsisCut(context: TextInBox, line: string): string {
  const cutContext = {
    ...context,
    ellipsis: measuredLine(context.metrics, textEllipsis(context.metrics)),
  };
  const cut = characterUnits(withoutTrailingSpaces(line)).reduce<EllipsisCut>(
    (state, unit) => cutWithUnit(state, unit, cutContext),
    { kept: undefined, start: measuredLine(context.metrics, "") },
  );

  return cut.kept ?? "";
}
