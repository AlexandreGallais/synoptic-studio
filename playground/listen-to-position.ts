import { contourBoundingBox, roundedContour } from "../src";

import { moveShape } from "./move-shape";
import { readNumber } from "./read-number";
import { readShape } from "./read-shape";
import { selectElement } from "./select-element";

/** Axis of a position field. */
type Axis = "x" | "y";

/**
 * Reads where the shape's straight box starts on an axis.
 *
 * @kind procedure
 * @param document - page document
 * @param axis - `x` for its left, `y` for its top
 * @returns the start, nothing for a refused shape
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function readStraightStart(document: Document, axis: Axis): number | undefined {
  const reading = readShape(document);
  const box = contourBoundingBox(roundedContour(reading.corners));

  if (!reading.isValid) {
    return undefined;
  }

  return axis === "x" ? box.minX : box.minY;
}

/**
 * Reads how far the shape must move for its straight box to start at the integer typed.
 *
 * @kind procedure
 * @param document - page document
 * @param axis - axis of the field typed
 * @returns the move, nothing when the value typed is not an integer or the shape is refused
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function readTypedShift(document: Document, axis: Axis): number | undefined {
  const typed = readNumber(document, `shape-${axis}`);
  const start = readStraightStart(document, axis);

  return start !== undefined && Number.isSafeInteger(typed) ? typed - start : undefined;
}

/**
 * Makes one position field move the shape.
 *
 * @kind procedure
 * @param document - page document
 * @param axis - axis of the field
 * @param update - redraws the page
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
function listenToAxis(document: Document, axis: Axis, update: () => void): void {
  selectElement(document, `shape-${axis}`).addEventListener("input", () => {
    const shift = readTypedShift(document, axis);

    if (shift === undefined) {
      return;
    }

    moveShape(document, axis === "x" ? { x: shift, y: 0 } : { x: 0, y: shift });
    update();
  });
}

/**
 * Makes the x and y fields move the shape (F04): an integer typed puts the top-left of the
 * shape's straight box there, as in Inkscape; the stored position, the top-left of the box
 * before it turns, moves by the difference (ADR-0035). Another value moves nothing.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function listenToPosition(document: Document, update: () => void): void {
  listenToAxis(document, "x", update);
  listenToAxis(document, "y", update);
}
