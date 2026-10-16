import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";
import { readTurnedCorner } from "./read-turned-corner";
import { readTypedCoordinate } from "./read-typed-coordinate";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { Point } from "../../src";

/**
 * Reads the corner X and Y show once a text was drawn: on each axis, while the field still shows
 * the corner of the text last drawn, at 5 decimals, its exact value (US-038); otherwise the
 * integer typed (`readTypedCoordinate`).
 *
 * @kind procedure
 * @param document - page document
 * @param drawn - the text last drawn
 * @returns the corner, exact on each axis left as shown
 * @see docs/backlog/stories/E15-F17-US-054-turned-text-position-shows-its-corner.md
 */
function readShownCorner(document: Document, drawn: DrawnTextRecord): Point {
  const corner = readTurnedCorner(drawn);
  const [x, y] = readPositionFields(document);

  return {
    x: x === drawn.fields[0] ? corner.x : readTypedCoordinate(document, "shape-x"),
    y: y === drawn.fields[1] ? corner.y : readTypedCoordinate(document, "shape-y"),
  };
}

/**
 * Reads the turned top-left corner of the text from the X and Y fields (US-054): exact on each
 * axis left as the text last drawn shows it (`readShownCorner`); before a first text, or once
 * typed, the integers typed (`readTypedCoordinate`), a fraction typed refused. At 0° the corner is
 * the text's position.
 *
 * @kind procedure
 * @param document - page document
 * @returns where the top-left of its block, or its box, is drawn
 * @see docs/backlog/stories/E15-F17-US-054-turned-text-position-shows-its-corner.md
 */
export function readTextPosition(document: Document): Point {
  const drawn = readDrawnText(document);

  return drawn === undefined
    ? { x: readTypedCoordinate(document, "shape-x"), y: readTypedCoordinate(document, "shape-y") }
    : readShownCorner(document, drawn);
}
