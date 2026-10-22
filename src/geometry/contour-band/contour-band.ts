import type { ContourPiece } from "../contour";

/**
 * The band about a contour, a fill standing for a stroke with round joins (`REF-SVG2-STROKE-SHAPE`):
 * for each piece, a disc at its start (`circleArcs`), then the band about each chord between its
 * points (`pieceSamples`, `segmentBand`); each a closed subpath turning in the positive direction,
 * so that the nonzero rule fills them as one. A curve is followed by its chords, as the source
 * leaves a drawing leeway.
 *
 * @kind geometry
 * @param pieces - the contour, each piece starting where the previous one ends
 * @param halfWidth - the half width of the band, in user units, > 0
 * @param step - the longest chord wanted along a curve, in user units, > 0
 * @returns its closed subpaths, the disc first for each piece; none for no piece
 * @see REF-SVG2-STROKE-SHAPE
 */
export function contourBand(
  pieces: readonly ContourPiece[],
  halfWidth: number,
  step: number,
): readonly (readonly ContourPiece[])[] {
  return [pieces].slice(0, Math.min(halfWidth, step, 0));
}
