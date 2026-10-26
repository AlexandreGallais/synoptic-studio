import { readDrawnText } from "./read-drawn-text";
import { readPositionFields } from "./read-position-fields";
import { readStraightStart } from "./read-straight-start";
import { readTypedCoordinate } from "./read-typed-coordinate";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { Point } from "editor";

/**
 * Reads where X and Y put the straight box once a text was drawn: on each axis, while the field
 * still shows the start of the text last drawn, at 5 decimals, its exact value;
 * otherwise the integer typed (`readTypedCoordinate`).
 *
 * @kind procedure
 * @param document - page document
 * @param drawn - the text last drawn
 * @returns the start, exact on each axis left as shown
 */
function readShownStart(document: Document, drawn: DrawnTextRecord): Point {
  const start = readStraightStart(drawn);
  const [x, y] = readPositionFields(document);

  return {
    x: x === drawn.fields[0] ? start.x : readTypedCoordinate(document, "shape-x"),
    y: y === drawn.fields[1] ? start.y : readTypedCoordinate(document, "shape-y"),
  };
}

/**
 * Reads where the text's straight box starts from the X and Y fields: exact on each
 * axis left as the text last drawn shows it (`readShownStart`); before a first text, or once
 * typed, the integers typed (`readTypedCoordinate`), a fraction typed refused. At 0° the start is
 * the text's position.
 *
 * @kind procedure
 * @param document - page document
 * @returns where the top-left of its straight box is drawn
 */
export function readTextPosition(document: Document): Point {
  const drawn = readDrawnText(document);

  return drawn === undefined
    ? { x: readTypedCoordinate(document, "shape-x"), y: readTypedCoordinate(document, "shape-y") }
    : readShownStart(document, drawn);
}
