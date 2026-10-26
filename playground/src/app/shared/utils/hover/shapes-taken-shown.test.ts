import { contourPiecesToPathData, shapePickArea } from "editor";
// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  clickElement,
  dispatchDrawingPointer,
  openPage,
  readShapeModel,
  setElementBox,
  typeAndEnter,
} from "../../../../../testing";
import { readSelectedShapes } from "../kept-shapes";

import type { PointerKeys } from "../../../../../testing";
import type { Point, Shape } from "editor";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** Alt held. */
const ALT: PointerKeys = { altKey: true, shiftKey: false };

/** Shift held. */
const SHIFT: PointerKeys = { altKey: false, shiftKey: true };

/**
 * Presses the drawing, moves the pointer, releases it, then the `click` the browser fires.
 *
 * @param from - the point pressed
 * @param to - the point released
 */
function dragFrom(from: Point, to: Point): void {
  dispatchDrawingPointer("pointerdown", from, NO_KEY);
  dispatchDrawingPointer("pointermove", to, NO_KEY);
  dispatchDrawingPointer("pointerup", to, NO_KEY);
  dispatchDrawingPointer("click", to, NO_KEY);
}

/**
 * Draws A, B and C, 100 × 100 at (0, 0), (150, 0) and (300, 0): A the rectangle the page opens
 * with, typed 100 × 100; B and C dragged with the Rectangle tool. C is then selected.
 *
 * @returns their models
 */
function drawThreeShapes(): { readonly a: Shape; readonly b: Shape; readonly c: Shape } {
  typeAndEnter("width", "100");
  typeAndEnter("height", "100");

  const a = readShapeModel();

  clickElement("tool-rectangle");
  dragFrom({ x: 150, y: 0 }, { x: 250, y: 100 });

  const b = readShapeModel();

  clickElement("tool-rectangle");
  dragFrom({ x: 300, y: 0 }, { x: 400, y: 100 });

  return { a, b, c: readShapeModel() };
}

/**
 * Reads the paths of the layer above the drawing of a class and of its white line.
 *
 * @param name - the class, `hover` or `selection`
 * @returns their path data, in the order drawn
 */
function readOutlines(name: string): readonly (string | null)[] {
  return [
    ...document.querySelectorAll(`#overlay svg path.${name}, #overlay svg path.${name}-light`),
  ].map((path) => path.getAttribute("d"));
}

/**
 * The two paths expected of a shape outlined: its white line, then its black dashes, both on its
 * contour as drawn.
 *
 * @param shape - the shape outlined
 * @returns the path data of its white line, then of its dashes
 */
function bandsOf(shape: Shape): readonly string[] {
  const contour = contourPiecesToPathData(shapePickArea(shape).pieces);

  return [contour, contour];
}

describe("the shapes a selection rectangle takes outlined while it is dragged", () => {
  beforeEach(() => {
    // The page as it opens, a rectangle 120 × 80 at (0, 0), radius 10.
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F43.AC2] outlines A and B, the shapes it touches, as under the pointer, C not", () => {
    const { a, b } = drawThreeShapes();

    // From (−10, −10) to (200, 110): it touches A and B, C from 300 lying beyond 200.
    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);

    expect(readOutlines("hover")).toEqual([...bandsOf(a), ...bandsOf(b)]);
    // Selected at the release only: C still selected meanwhile.
    expect(readSelectedShapes(document)).toEqual([2]);
  });

  it("[F43.AC2] outlines A alone with Alt held, the one it encloses, Alt read at each move", () => {
    const { a, b } = drawThreeShapes();

    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, ALT);

    // A from (0, 0) to (100, 100) lies within the rectangle, B to 250 does not.
    expect(readOutlines("hover")).toEqual(bandsOf(a));

    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);
    expect(readOutlines("hover")).toEqual([...bandsOf(a), ...bandsOf(b)]);
  });

  it("[F43.AC2] stops outlining B once the pointer is back to (100, 50)", () => {
    const { a } = drawThreeShapes();

    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);
    // From (−10, −10) to (100, 50): it touches A, B starting at 150.
    dispatchDrawingPointer("pointermove", { x: 100, y: 50 }, NO_KEY);

    expect(readOutlines("hover")).toEqual(bandsOf(a));
  });

  it("[F43.AC2] selects A and B at the release, outlined as selected, none as under the pointer", () => {
    const { a, b } = drawThreeShapes();

    dragFrom({ x: -10, y: -10 }, { x: 200, y: 110 });

    // Released on (200, 110), under no shape.
    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expect(readOutlines("selection")).toEqual([...bandsOf(a), ...bandsOf(b)]);
    expect(readOutlines("hover")).toEqual([]);
  });

  it("[F43.AC2] keeps C's selection outlined with Shift, A and B outlined as under the pointer", () => {
    const { a, b, c } = drawThreeShapes();

    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, SHIFT);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, SHIFT);

    expect(readOutlines("selection")).toEqual(bandsOf(c));
    expect(readOutlines("hover")).toEqual([...bandsOf(a), ...bandsOf(b)]);
  });

  it("[F43.AC2] outlines nothing while the pointer stays within 3 pixels of the press", () => {
    drawThreeShapes();

    // (50, 100.4) is 1.6 pixels from the press at (50, 102), outside A, which ends at 100: no
    // rectangle yet, though the one from (50, 100), rounded, to (50, 102) would touch A's side.
    dispatchDrawingPointer("pointerdown", { x: 50, y: 102 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 50, y: 100.4 }, NO_KEY);

    expect(readOutlines("hover")).toEqual([]);
  });
});
