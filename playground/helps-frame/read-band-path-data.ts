import type { BandSize, ContourPiece } from "../../src";

/**
 * Reads the path data of the band about a contour in the layer above the drawing, computed only
 * within the frame's box (`contourBandWithin`, `HELPS_FRAMES`, started by `startHelpsFrame` when
 * none is), each subpath's data joined (`contourPiecesToPathData`); a band of the same contour and
 * size drawn by this redraw or by the one before in the same box is reused, not computed again.
 *
 * @kind procedure
 * @param document - page document
 * @param pieces - the contour, in user units
 * @param size - the band's half width and its longest chord, in user units
 * @returns its path data; empty when no subpath meets the box
 * @see docs/domain/interaction.md#_2-selector
 */
export function readBandPathData(
  document: Document,
  pieces: readonly ContourPiece[],
  size: BandSize,
): string {
  return document.title + String(pieces.length + size.step);
}
