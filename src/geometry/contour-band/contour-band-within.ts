import { EPSILON, integerQuotient } from "../../math";
import { isPieceMeetingBox } from "../box-overlap";
import { isPointInBox, widenedBox } from "../placement";

import { circleArcs } from "./circle-arcs";
import { pieceLengthBound } from "./piece-length-bound";
import { pieceSample } from "./piece-sample";
import { pieceStepCount } from "./piece-step-count";
import { segmentBand } from "./segment-band";

import type { BandSize } from "./band-size";
import type { ContourPiece, Segment } from "../contour";
import type { BoundingBox } from "../placement";

/** A piece cut into chords, and the box its chords are kept within. */
type ChordRun = {
  /** The piece. */
  readonly piece: ContourPiece;
  /** Its number of chords n (`pieceStepCount`). */
  readonly count: number;
  /** At least each chord's length, the piece's length bound over n (`pieceLengthBound`). */
  readonly spacing: number;
  /** The box asked widened by the band's half width. */
  readonly reach: BoundingBox;
};

/**
 * The chord between two consecutive samples of a piece, when it meets the box widened by the half
 * width (`isPieceMeetingBox`, `DERIV-band-within-box` step 3), drawn whole.
 *
 * @kind geometry
 * @param run - the piece, its chords and the box
 * @param first - the chord's first sample's index
 * @returns the chord; none when it misses the box
 * @see DERIV-box-overlap
 */
function meetingChord(run: ChordRun, first: number): readonly Segment[] {
  const chord: Segment = {
    end: pieceSample(run.piece, run.count, first + 1),
    kind: "segment",
    start: pieceSample(run.piece, run.count, first),
  };

  return isPieceMeetingBox(chord, run.reach) ? [chord] : [];
}

/**
 * The chords of a run of a piece's samples that meet the box widened by the half width
 * (`DERIV-band-within-box` steps 4 and 5): a run of one chord kept when it meets it
 * (`meetingChord`); a longer one left out when its middle sample lies outside that box widened
 * by the run's reach, `EPSILON` more for the samples' rounding errors, otherwise cut there into two
 * runs, so that no sample out of sight is computed.
 *
 * Formula: m = ⌊(i₀ + i₁) / 2⌋, ρ = max(m − i₀, i₁ − m) · λ; left out when Pₘ ∉ B_h widened by ρ.
 *
 * @kind geometry
 * @param run - the piece, its chords and the box
 * @param first - the run's first sample's index i₀
 * @param last - its last sample's index i₁, > i₀
 * @returns the chords kept, in their order
 * @see DERIV-band-within-box
 */
function keptChords(run: ChordRun, first: number, last: number): readonly Segment[] {
  if (last - first === 1) {
    return meetingChord(run, first);
  }

  const middle = integerQuotient(first + last, 2);
  const radius = Math.max(middle - first, last - middle) * run.spacing;
  const sample = pieceSample(run.piece, run.count, middle);

  if (!isPointInBox(widenedBox(run.reach, radius + EPSILON), sample)) {
    return [];
  }

  return [...keptChords(run, first, middle), ...keptChords(run, middle, last)];
}

/**
 * The band within a box of one piece (`DERIV-band-within-box` step 3): its disc when its start is
 * in the box widened by the half width (`isPointInBox`, `circleArcs`), then the band of each chord
 * meeting that box (`keptChords`, `segmentBand`), a chord of length 0 giving none.
 *
 * @kind geometry
 * @param piece - a segment, an arc or a cubic of a contour
 * @param size - the band's half width and its longest chord
 * @param reach - the box asked widened by the half width
 * @returns its closed subpaths, the disc first
 * @see DERIV-band-within-box
 */
function pieceBandWithin(
  piece: ContourPiece,
  size: BandSize,
  reach: BoundingBox,
): readonly (readonly ContourPiece[])[] {
  const count = pieceStepCount(piece, size.step);
  const run = { count, piece, reach, spacing: pieceLengthBound(piece) / count };
  const bands = keptChords(run, 0, count)
    .map((chord) => segmentBand(chord, size.halfWidth))
    .filter((band) => band.length > 0);

  return isPointInBox(reach, piece.start)
    ? [circleArcs(piece.start, size.halfWidth), ...bands]
    : bands;
}

/**
 * The band about a contour computed only within a box (`DERIV-band-within-box`): the whole band's
 * subpaths (`contourBand`), in their order, less those that paint no point of the box, so that it
 * paints the box as the whole band does. With the box widened by the half width (`widenedBox`), a
 * piece's disc is kept when its start is in it (`isPointInBox`), a chord's band when the chord
 * meets it (`isPieceMeetingBox`), drawn whole; the runs of chords are cut by halves, a run left out
 * without computing its samples when its middle sample lies farther from that box than the run's
 * reach (`pieceSample`, `pieceLengthBound`), so that nothing is sampled out of sight.
 *
 * @kind geometry
 * @param pieces - the contour, each piece starting where the previous one ends
 * @param size - the band's half width and its longest chord, in user units
 * @param box - the box the band is computed within, in user units
 * @returns its closed subpaths, each piece's disc first; none for no piece
 * @see DERIV-band-within-box
 */
export function contourBandWithin(
  pieces: readonly ContourPiece[],
  size: BandSize,
  box: BoundingBox,
): readonly (readonly ContourPiece[])[] {
  const reach = widenedBox(box, size.halfWidth);

  return pieces.flatMap((piece) => pieceBandWithin(piece, size, reach));
}
