import { freeShapeCorners, freeShapeWithPoint, turnedCorners } from "../src";

import { listenToDeleteKey } from "./listen-to-delete-key";
import { readEditedFreeShape } from "./read-edited-free-shape";
import { readNumber } from "./read-number";
import { readSelection } from "./read-selection";
import { selectElement } from "./select-element";
import { writeAddedMidpoint } from "./write-added-midpoint";
import { writeConvertedShape } from "./write-converted-shape";
import { writeDeletedPoints } from "./write-deleted-points";
import { writeStoredFreeShape } from "./write-stored-free-shape";

import type { FreeShape, Point } from "../src";

/** Which point moves, and along which axis. */
type Target = {
  /** The coordinate typed. */
  readonly axis: "x" | "y";
  /** Number of the point, from 0. */
  readonly index: number;
};

/**
 * Reads where the point goes: its drawn place, the coordinate typed replacing one of its two
 * (ADR-0033), when the value typed is an integer.
 *
 * @kind procedure
 * @param document - page document
 * @param axis - `x` or `y`, the coordinate typed
 * @param drawn - the point where it is drawn, `undefined` when there is none
 * @returns its new place, `undefined` when nothing moves
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function readTypedPoint(
  document: Document,
  axis: "x" | "y",
  drawn: Point | undefined,
): Point | undefined {
  const typed = readNumber(document, `point-${axis}`);

  return drawn !== undefined && Number.isSafeInteger(typed)
    ? { ...drawn, [axis]: typed }
    : undefined;
}

/**
 * Moves a point of a free shape to the coordinate typed and keeps the shape on the page.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - free shape shown (the rectangle included)
 * @param target - the point and the coordinate typed
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function writeMovedPoint(document: Document, shape: FreeShape, target: Target): void {
  const point = readTypedPoint(
    document,
    target.axis,
    turnedCorners(freeShapeCorners(shape), shape).at(target.index)?.point,
  );

  if (point !== undefined) {
    writeStoredFreeShape(document, freeShapeWithPoint(shape, target.index, point));
  }
}

/**
 * Reads the number of the point selected, when exactly one is.
 *
 * @kind procedure
 * @param document - page document
 * @returns its number from 0, `undefined` when none or several are selected
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function readOnlySelected(document: Document): number | undefined {
  const { selected } = readSelection(document);

  return selected.length === 1 ? selected[0] : undefined;
}

/**
 * Moves the one point selected of a valid free shape to the coordinate typed (F08.AC2).
 *
 * @kind procedure
 * @param document - page document
 * @param axis - `x` or `y`, the field typed in
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function movePointTyped(document: Document, axis: "x" | "y"): void {
  const shape = readEditedFreeShape(document);
  const index = readOnlySelected(document);

  if (shape === undefined || index === undefined) {
    return;
  }

  writeMovedPoint(document, shape, { axis, index });
}

/**
 * Listens to the x and y fields of the point selected and to the buttons converting a polygon
 * adding and deleting points, and to the Delete key, then calls back (F08.AC1 to F08.AC4).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
export function listenToPointFields(document: Document, update: () => void): void {
  for (const axis of ["x", "y"] as const) {
    // On "change" (Enter or leaving the field): typing "-" or clearing it moves nothing yet.
    selectElement(document, `point-${axis}`).addEventListener("change", () => {
      movePointTyped(document, axis);
      update();
    });
  }

  selectElement(document, "convert").addEventListener("click", () => {
    writeConvertedShape(document);
    update();
  });

  selectElement(document, "add-midpoint").addEventListener("click", () => {
    writeAddedMidpoint(document);
    update();
  });

  selectElement(document, "delete-points").addEventListener("click", () => {
    writeDeletedPoints(document);
    update();
  });

  listenToDeleteKey(document, update);
}
