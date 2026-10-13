import {
  cyclicItem,
  freeShapeCorners,
  freeShapeWithHandles,
  handlesAfterTyping,
  pastedControlPoints,
  subtract,
  turnedCorners,
} from "../../src";
import { selectElement, writeText } from "../page";
import { readEditedFreeShape, readShape, writeStoredFreeShape } from "../shape";

import { readSelectedCorner } from "./read-selected-corner";

import type { CurveCorner, FreeShape, PastedCurve, Point } from "../../src";
import type { ShapeReading } from "../shape";
import type { SelectedCorner } from "./read-selected-corner";

/** A handle of a point to set so that it reaches a control point. */
type HandleTarget = {
  /** The control point to reach, as drawn. */
  readonly control: Point;
  /** Which handle. */
  readonly end: "after" | "before";
  /** The point's number, from 0. */
  readonly index: number;
};

/** A curve read from the paste field, and the number of the side's end point. */
type PasteReading = {
  /** The control points, or why the text was refused. */
  readonly curve: PastedCurve;
  /** Number of the point at the side's end. */
  readonly next: number;
};

/**
 * Corners of a free shape as drawn on the page.
 *
 * @kind procedure
 * @param shape - free shape
 * @returns its corners, turned
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
function readDrawnCorners(shape: FreeShape): readonly CurveCorner[] {
  return turnedCorners(freeShapeCorners(shape), shape);
}

/**
 * A free shape with one handle of one point set to reach a control point: the point in handles
 * mode, its other handle following its mode (`handlesAfterTyping`).
 *
 * @kind procedure
 * @param shape - free shape
 * @param target - the point's number, which handle, and the control point to reach, as drawn
 * @returns the free shape with that handle
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
function readShapeWithHandleTo(shape: FreeShape, target: HandleTarget): FreeShape {
  const corner = cyclicItem(readDrawnCorners(shape), target.index, undefined);

  return corner === undefined
    ? shape
    : freeShapeWithHandles(
        shape,
        target.index,
        handlesAfterTyping(
          { ...corner.handles, isOn: true },
          target.end,
          subtract(target.control, corner.point),
        ),
      );
}

/**
 * Reads the paste field against the side after the point selected.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - the shape read from the inputs, its corners as drawn
 * @param selected - the point selected
 * @returns the curve read and the number of the side's end point
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
function readPastedCurve(
  document: Document,
  reading: ShapeReading,
  selected: SelectedCorner,
): PasteReading {
  const next = (selected.index + 1) % reading.corners.length;
  const end = cyclicItem(reading.corners, next, selected.corner).point;
  const input = selectElement(document, "handle-paste");
  const text = input instanceof HTMLInputElement ? input.value : "";

  return { curve: pastedControlPoints(text, selected.corner.point, end), next };
}

/**
 * Shows why a pasted text was refused, or hides the line when it was taken.
 *
 * @kind procedure
 * @param document - page document
 * @param curve - the curve read
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
function showPasteMessage(document: Document, curve: PastedCurve): void {
  const isRefused = "refusal" in curve;

  selectElement(document, "paste-message").hidden = !isRefused;
  writeText(document, "paste-message", isRefused ? curve.refusal : "");
}

/**
 * Applies the curve pasted to the side after the point selected: its handle after and the next
 * point's handle before (`pastedControlPoints`), or a line saying why the text was refused.
 *
 * @kind procedure
 * @param document - page document
 * @param update - the page's redraw
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
function applyPastedCurve(document: Document, update: () => void): void {
  const shape = readEditedFreeShape(document);
  const reading = readShape(document);
  const selected = readSelectedCorner(document, reading);

  if (shape === undefined || selected === undefined) {
    return;
  }

  writePastedCurve(document, shape, {
    index: selected.index,
    ...readPastedCurve(document, reading, selected),
  });

  update();
}

/**
 * Stores the curve pasted, or shows why it was refused.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - the free shape edited
 * @param pasted - the curve read, the side's start and end point numbers
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
function writePastedCurve(
  document: Document,
  shape: FreeShape,
  pasted: PasteReading & { readonly index: number },
): void {
  const { curve } = pasted;

  showPasteMessage(document, curve);

  if ("refusal" in curve) {
    return;
  }

  const first = readShapeWithHandleTo(shape, {
    control: curve.first,
    end: "after",
    index: pasted.index,
  });

  writeStoredFreeShape(
    document,
    readShapeWithHandleTo(first, { control: curve.second, end: "before", index: pasted.next }),
  );
}

/**
 * Listens to the button applying a pasted curve to the side after the point selected (US-025).
 *
 * @kind procedure
 * @param document - page document
 * @param update - the page's redraw
 * @see docs/backlog/stories/E14-F12-US-025-handles-as-share-or-data.md
 */
export function listenToPaste(document: Document, update: () => void): void {
  selectElement(document, "paste-curve").addEventListener("click", () => {
    applyPastedCurve(document, update);
  });
}
