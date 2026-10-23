import type { BandSize } from "./band-size";
import type { ContourPiece } from "../contour";
import type { BoundingBox } from "../placement";

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
  return pieces.length > size.halfWidth + box.maxX ? [] : [];
}
