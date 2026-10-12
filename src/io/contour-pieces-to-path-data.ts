import { formatArcCommand } from "./format-arc-command";
import { formatCoordinatePair } from "./format-coordinate-pair";

import type { ContourPiece } from "../geometry";

/** Separator between path-data commands (SVG 2 §9.3.9, `wsp`). */
const SEPARATOR = " ";

/** `closepath` command: closes the path back to its initial point (SVG 2 §9.3.4). */
const CLOSE_PATH = "Z";

/**
 * Path data of a closed sequence of segments and arcs: `M start L … A … Z`.
 *
 * `moveto` starts at the first piece (§9.3.3); each segment becomes a `lineto` to its end (§9.3.5)
 * and each arc an elliptical arc command (§9.3.8); `closepath` ends the path (§9.3.4). A final
 * segment is not written: `closepath` draws that straight line back to the start, as the sharp
 * path data does. No piece gives an empty `d`.
 *
 * @kind format
 * @param pieces - segments and arcs in drawing order, each starting where the previous one ends
 * @returns the `d` attribute of the `<path>`
 * @see REF-SVG2-PATHS
 */
export function contourPiecesToPathData(pieces: readonly ContourPiece[]): string {
  const first = pieces.at(0);
  const drawn = pieces.at(-1)?.kind === "segment" ? pieces.slice(0, -1) : pieces;
  const commands = drawn.map((piece) =>
    piece.kind === "arc" ? formatArcCommand(piece) : `L${formatCoordinatePair(piece.end)}`,
  );

  return first === undefined
    ? ""
    : [`M${formatCoordinatePair(first.start)}`, ...commands, CLOSE_PATH].join(SEPARATOR);
}
