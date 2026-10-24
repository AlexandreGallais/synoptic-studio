import { cyclicItem } from "../contour";

import { circleArcs } from "./circle-arcs";
import { pieceSamples } from "./piece-samples";
import { segmentBand } from "./segment-band";

import type { ContourPiece, Segment } from "../contour";

/**
 * The band about a contour, a fill standing for a stroke with round joins (`REF-SVG2-PAINT`):
 * for each piece, a disc at its start (`circleArcs`), then the band about each chord between its
 * points (`pieceSamples`, `segmentBand`); each a closed subpath turning in the positive direction,
 * so that the nonzero rule fills them as one. A curve is followed by its chords, as the source
 * leaves a drawing leeway.
 *
 * @kind geometry
 * @param pieces - the contour, each piece starting where the previous one ends
 * @param halfWidth - the half width of the band, in user units, > 0: half a stroke's width, the
 * radius of its round join, a sector "of diameter stroke-width" (`REF-SVG2-PAINT`, whose
 * sentence on a dash says stroke-width where half is meant)
 * @param step - the longest chord wanted along a curve, in user units, > 0
 * @returns its closed subpaths, the disc first for each piece; none for no piece
 * @see REF-SVG2-PAINT
 */
export function contourBand(
  pieces: readonly ContourPiece[],
  halfWidth: number,
  step: number,
): readonly (readonly ContourPiece[])[] {
  return pieces.flatMap((piece) => {
    const samples = pieceSamples(piece, step);
    const chords: readonly Segment[] = samples.slice(0, -1).map((start, index) => ({
      end: cyclicItem(samples, index + 1, start),
      kind: "segment",
      start,
    }));
    const bands = chords
      .map((chord) => segmentBand(chord, halfWidth))
      .filter((band) => band.length > 0);

    return [circleArcs(piece.start, halfWidth), ...bands];
  });
}
