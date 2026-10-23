import { contourBandWithin, contourPiecesToPathData } from "../../src";

import { HELPS_FRAMES } from "./helps-frames";
import { startHelpsFrame } from "./start-helps-frame";

import type { HelpsFrame } from "./helps-frame";
import type { BandSize, ContourPiece } from "../../src";

/**
 * Reads the frame the helps of this redraw are computed in: the one started (`HELPS_FRAMES`), or
 * one started now (`startHelpsFrame`).
 *
 * @kind procedure
 * @param document - page document
 * @returns its box, its scale and the bands drawn in it
 * @see docs/domain/interaction.md#_2-selector
 */
function readHelpsFrame(document: Document): HelpsFrame {
  return HELPS_FRAMES.get(document) ?? startHelpsFrame(document);
}

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
  const frame = readHelpsFrame(document);
  const key = JSON.stringify({ pieces, size });
  const data =
    frame.bands.get(key) ??
    frame.previous.get(key) ??
    contourBandWithin(pieces, size, frame.box)
      .map((subpath) => contourPiecesToPathData(subpath))
      .join(" ");

  HELPS_FRAMES.set(document, { ...frame, bands: new Map([...frame.bands, [key, data]]) });

  return data;
}
