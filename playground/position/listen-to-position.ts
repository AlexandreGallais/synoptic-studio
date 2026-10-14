import { contourBoundingBox, curveContour } from "../../src";
import { readNumber, selectElement } from "../page";
import { readShape } from "../shape";

import { moveShape } from "./move-shape";
import { writeResizedPosition } from "./write-resized-position";

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
  const box = contourBoundingBox(curveContour(reading.corners));

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
 * before it turns, moves by the difference (ADR-0035). Another value moves nothing. A width or
 * height typed on a turned shape moves its position so that its turned top-left stays in place.
 * Called before the inputs redraw the page.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function listenToPosition(document: Document, update: () => void): void {
  listenToAxis(document, "x", update);
  listenToAxis(document, "y", update);

  // Before the page redraws: a turned shape keeps its turned top-left when resized (US-019).
  for (const id of ["width", "height"]) {
    selectElement(document, id).addEventListener("input", () => {
      writeResizedPosition(document);
    });
  }

  // Once a field is left, it shows the straight box again, a value that moved nothing included.
  for (const id of ["shape-x", "shape-y"]) {
    selectElement(document, id).addEventListener("change", update);
  }
}
