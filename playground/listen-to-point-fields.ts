import { freeShapeCorners, freeShapeWithPoint, regularPolygonAsFreeShape } from "../src";

import { readNumber } from "./read-number";
import { readSelection } from "./read-selection";
import { readShape } from "./read-shape";
import { selectElement } from "./select-element";
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
    freeShapeCorners(shape).at(target.index)?.point,
  );

  if (point !== undefined) {
    writeStoredFreeShape(document, freeShapeWithPoint(shape, target.index, point));
  }
}

/**
 * Moves the one point selected to the coordinate typed; a regular polygon is converted first,
 * its fields greyed (F08.AC2).
 *
 * @kind procedure
 * @param document - page document
 * @param axis - `x` or `y`, the field typed in
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function movePointTyped(document: Document, axis: "x" | "y"): void {
  const { model } = readShape(document);
  const index = readSelection(document).selected.at(0) ?? -1;

  if (!("anchor" in model)) {
    writeMovedPoint(document, model, { axis, index });
  }
}

/**
 * Converts the regular polygon shown into a free shape, unmoved, and keeps it on the page
 * (F08.AC1, ADR-0033).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
function writeConvertedPolygon(document: Document): void {
  const { model } = readShape(document);

  if ("anchor" in model) {
    writeStoredFreeShape(document, regularPolygonAsFreeShape(model));
  }
}

/**
 * Listens to the x and y fields of the point selected and to the conversion button, then calls
 * back (F08.AC1, F08.AC2).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F08-US-014-convert-and-move.md
 */
export function listenToPointFields(document: Document, update: () => void): void {
  for (const axis of ["x", "y"] as const) {
    selectElement(document, `point-${axis}`).addEventListener("input", () => {
      movePointTyped(document, axis);
      update();
    });
  }

  selectElement(document, "convert").addEventListener("click", () => {
    writeConvertedPolygon(document);
    update();
  });
}
