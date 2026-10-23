// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { readExportedSvg } from "../document";
import { readDrawnTexts, readSelectedShapes } from "../kept-shapes";
import {
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readShapeModel,
  readText,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

import type { Point, Shape } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/**
 * Presses the drawing and moves the pointer, not released.
 *
 * @param from - the point pressed
 * @param to - the point the pointer is moved to
 */
function pressAndMove(from: Point, to: Point): void {
  dispatchDrawingPointer("pointerdown", from, NO_KEY);
  dispatchDrawingPointer("pointermove", to, NO_KEY);
}

/**
 * Releases the pointer, then the `click` the browser fires on the same element.
 *
 * @param at - the point released
 * @param keys - the keys held at the release
 */
function release(at: Point, keys: PointerKeys): void {
  dispatchDrawingPointer("pointerup", at, keys);
  dispatchDrawingPointer("click", at, keys);
}

/**
 * Drags on the drawing from one point to another, then releases with keys held: the browser's
 * `pointerdown`, `pointermove`, `pointerup`, then the `click` it fires on the same element.
 *
 * @param from - the point pressed
 * @param to - the point released
 * @param keys - the keys held at the release
 */
function dragFrom(from: Point, to: Point, keys: PointerKeys): void {
  pressAndMove(from, to);
  release(to, keys);
}

/**
 * Draws a shape with its tool by a drag, its box from one point to the other (US-064).
 *
 * @param tool - `rectangle`, `polygon`, `round` or `text`
 * @param from - the point pressed
 * @param to - the point released
 */
function drawWithTool(tool: string, from: Point, to: Point): void {
  clickElement(`tool-${tool}`);
  dragFrom(from, to, NO_KEY);
}

/**
 * Draws A, B and C in that order, C then selected: A the rectangle the page opens with, typed
 * 100 × 100 at (0, 0), radius 10; B a rectangle dragged from (150, 0) to (250, 100), radius 10
 * as the last rectangle; C a round shape dragged from (300, 0) to (400, 100), a circle of
 * diameter 100 centred on (350, 50).
 *
 * @returns their models
 */
function drawThreeShapes(): { readonly a: Shape; readonly b: Shape; readonly c: Shape } {
  typeValue("width", "100");
  typeValue("height", "100");

  const a = readShapeModel();

  drawWithTool("rectangle", { x: 150, y: 0 }, { x: 250, y: 100 });

  const b = readShapeModel();

  drawWithTool("round", { x: 300, y: 0 }, { x: 400, y: 100 });

  return { a, b, c: readShapeModel() };
}

/**
 * Reads a path of the layer above the drawing.
 *
 * @param name - its class
 * @returns its path data; undefined for none
 */
function readOverlayPath(name: string): string | undefined {
  return document.querySelector(`#overlay svg path.${name}`)?.getAttribute("d") ?? undefined;
}

describe("a selection rectangle selects shapes (US-069)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), radius 10, whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F37.AC3] draws the rectangle in the layer above from the press to the pointer", () => {
    drawThreeShapes();
    pressAndMove({ x: -10, y: -10 }, { x: 200, y: 110 });

    // The area from (−10, −10) to (200, 110); its outline 1 pixel wide centred on its sides, from
    // −10 − 0.5 to 200 + 0.5 outside and from −10 + 0.5 to 200 − 0.5 inside, the hole the other
    // way round so that the nonzero rule leaves it empty.
    expect(readOverlayPath("selection-area")).toBe("M-10 -10H200V110H-10Z");

    expect(readOverlayPath("selection-rectangle")).toBe(
      "M-10.5 -10.5H200.5V110.5H-10.5Z M-9.5 -9.5V109.5H199.5V-9.5Z",
    );

    expect(selectPageElement("#canvas").querySelector(".selection-rectangle")).toBeNull();
  });

  it("[F37.AC3] draws the same rectangle dragged the other way, on the nearest integers", () => {
    drawThreeShapes();
    // (200.4, 110.5) rounds to (200, 111), a tie rounded up; (−10.4, −9.6) to (−10, −10).
    pressAndMove({ x: 200.4, y: 110.5 }, { x: -10.4, y: -9.6 });

    expect(readOverlayPath("selection-area")).toBe("M-10 -10H200V111H-10Z");
  });

  it("[F37.AC3] draws a rectangle without width as its outline alone, 1 pixel wide", () => {
    drawThreeShapes();
    // From (5, 150) to (5, 200): 0 wide, thinner than the band, so no hole.
    pressAndMove({ x: 5, y: 150 }, { x: 5, y: 200 });

    expect(readOverlayPath("selection-rectangle")).toBe("M4.5 149.5H5.5V200.5H4.5Z");
  });

  it("[F37.AC3] draws nothing while the pointer stays on the press's integer point", () => {
    drawThreeShapes();
    // (−10.3, −9.8) rounds to (−10, −10), the press's point.
    pressAndMove({ x: -10, y: -10 }, { x: -10.3, y: -9.8 });

    expect(readOverlayPath("selection-area")).toBeUndefined();
  });

  it("[F37.AC3] draws the rectangle once the pointer has left the press's point by one unit", () => {
    drawThreeShapes();
    // (−9, −10) is one unit right of the press: 1 wide and 0 high, a sum of 1 > 0.
    pressAndMove({ x: -10, y: -10 }, { x: -9, y: -10 });

    expect(readOverlayPath("selection-area")).toBe("M-10 -10H-9V-10H-10Z");
  });

  it("[F37.AC3] hides the outline of the shape under the pointer while the rectangle is drawn", () => {
    drawThreeShapes();
    pressAndMove({ x: -10, y: -10 }, { x: 50, y: 50 });

    expect(readOverlayPath("selection-area")).toBeDefined();
    expect(readOverlayPath("hover")).toBeUndefined();
  });

  it("[F37.AC3] takes the rectangle away at the release, and on a drag cancelled", () => {
    drawThreeShapes();
    dragFrom({ x: -10, y: -10 }, { x: 200, y: 110 }, NO_KEY);

    expect(readOverlayPath("selection-area")).toBeUndefined();
    expect(readOverlayPath("selection-rectangle")).toBeUndefined();

    pressAndMove({ x: 110, y: 10 }, { x: 140, y: 90 });
    dispatchDrawingPointer("pointercancel", { x: 140, y: 90 }, NO_KEY);

    // Cancelled: nothing selected anew, A and B kept.
    expect(readOverlayPath("selection-area")).toBeUndefined();
    expect(readSelectedShapes(document)).toEqual([0, 1]);
  });

  it("[F37.AC3] selects A and B, the shapes whose painted area the rectangle touches", () => {
    drawThreeShapes();
    dragFrom({ x: -10, y: -10 }, { x: 200, y: 110 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expect(readText("#shape-title")).toBe("2 shapes selected");
    expect(document.querySelectorAll("#overlay svg path.selection")).toHaveLength(2);
    expect(readOverlayPath("selection-box")).toBeDefined();
  });

  it("[F37.AC3] selects the same dragged from (200, 110) to (−10, −10)", () => {
    drawThreeShapes();
    dragFrom({ x: 200, y: 110 }, { x: -10, y: -10 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([0, 1]);
  });

  it("[F37.AC3] selects A alone with Alt, the shape the rectangle encloses, the panel editing it", () => {
    const { a } = drawThreeShapes();

    dragFrom({ x: -10, y: -10 }, { x: 200, y: 110 }, { altKey: true, shiftKey: false });

    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readShapeModel()).toEqual(a);
  });

  it("[F37.AC3] leaves C out of a rectangle over its box's corner, outside the circle", () => {
    drawThreeShapes();
    // The circle's nearest point to (390, 90) lies √(40² + 40²) = 56.6 from (350, 50), beyond 50.
    dragFrom({ x: 390, y: 90 }, { x: 420, y: 120 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([]);
    expect(readText("#shape-title")).toBe("No shape selected");
  });

  it("[F37.AC3] adds B to A with Shift; without Shift selects B alone, the panel editing it", () => {
    const { b } = drawThreeShapes();
    const shift = { altKey: false, shiftKey: true };

    dragFrom({ x: -10, y: -10 }, { x: 200, y: 110 }, { altKey: true, shiftKey: false });
    // From (140, −10) to (260, 110): B alone, A ending at 100 and C starting at 300.
    dragFrom({ x: 140, y: -10 }, { x: 260, y: 110 }, shift);

    expect(readSelectedShapes(document)).toEqual([0, 1]);

    dragFrom({ x: 140, y: -10 }, { x: 260, y: 110 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([1]);
    expect(readShapeModel()).toEqual(b);
  });

  it("[F37.AC3] selects none for a rectangle taking none; with Shift keeps the selection", () => {
    drawThreeShapes();
    dragFrom({ x: -10, y: -10 }, { x: 200, y: 110 }, NO_KEY);
    // From (110, 10) to (140, 90): between A, ending at 100, and B, starting at 150.
    dragFrom({ x: 110, y: 10 }, { x: 140, y: 90 }, { altKey: false, shiftKey: true });

    expect(readSelectedShapes(document)).toEqual([0, 1]);

    dragFrom({ x: 110, y: 10 }, { x: 140, y: 90 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([]);
  });

  it("[F37.AC3] draws no rectangle from a press on A, which moves A over B (US-072)", () => {
    drawThreeShapes();
    pressAndMove({ x: 50, y: 50 }, { x: 200, y: 50 });

    expect(readOverlayPath("selection-area")).toBeUndefined();

    release({ x: 200, y: 50 }, NO_KEY);

    // A selected alone and moved by (150, 0), its box from (0, 0) to (150, 0); the click the
    // release brings picks nothing, or it would pick A, drawn under B, no more than B.
    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readShapeModel()).toMatchObject({ x: 150, y: 0 });
  });

  it("[F37.AC3] takes no drag for a click once moved, back on the press's integer point", () => {
    drawThreeShapes();
    pressAndMove({ x: 50, y: 50 }, { x: 200, y: 50 });
    dispatchDrawingPointer("pointermove", { x: 50, y: 50 }, NO_KEY);
    release({ x: 50, y: 50 }, NO_KEY);

    // A moved and brought back, selected alone by its move (US-072), where it was.
    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readShapeModel()).toMatchObject({ x: 0, y: 0 });
  });

  it("[F37.AC3] takes a release on the press's integer point for a click, which selects A", () => {
    const { a } = drawThreeShapes();

    // (50.3, 50.2) rounds to (50, 50), the press's point.
    dragFrom({ x: 50, y: 50 }, { x: 50.3, y: 50.2 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readShapeModel()).toEqual(a);
  });

  it("[F37.AC3] draws a rectangle with a tool armed, never a selection rectangle", () => {
    drawThreeShapes();
    clickElement("tool-rectangle");
    pressAndMove({ x: 500, y: 0 }, { x: 600, y: 100 });

    expect(readOverlayPath("selection-area")).toBeUndefined();

    release({ x: 600, y: 100 }, NO_KEY);

    // 600 − 500 = 100 wide, 100 − 0 = 100 high, a fourth shape, selected alone.
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 500, y: 0 });
    expect(readSelectedShapes(document)).toEqual([3]);
  });

  it("[F37.AC3] draws no rectangle in Points mode, the selection unchanged", () => {
    drawThreeShapes();
    clickElement("mode-points");
    pressAndMove({ x: -10, y: -10 }, { x: 200, y: 110 });

    expect(readOverlayPath("selection-area")).toBeUndefined();

    release({ x: 200, y: 110 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([2]);
  });

  it("[F37.AC3] exports A, B and C, never the rectangle", () => {
    drawThreeShapes();

    const exported = readExportedSvg(document, readDrawnTexts(document));

    pressAndMove({ x: -10, y: -10 }, { x: 200, y: 110 });

    expect(readOverlayPath("selection-area")).toBeDefined();
    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);
  });
});
