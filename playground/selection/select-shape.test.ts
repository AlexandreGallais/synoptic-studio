// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBand, contourPiecesToPathData, shapePickArea } from "../../src";
import { readExportedSvg } from "../document";
import { readDrawnTexts } from "../kept-shapes";
import {
  clickAt,
  clickElement,
  openServedPage,
  pressAndClick,
  pressPointer,
  readDisplay,
  readShapeModel,
  readText,
  selectPageElement,
  setElementBox,
} from "../user-actions";

import type { Point, Shape } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner (`DRAWING_MARGIN`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/**
 * The window's point of a point of the drawing.
 *
 * @param point - the point of the drawing
 * @returns its `clientX` and `clientY`
 */
function toWindow(point: Point): { readonly clientX: number; readonly clientY: number } {
  return { clientX: point.x + ORIGIN_LEFT, clientY: point.y + ORIGIN_TOP };
}

/**
 * Places a shape: its tool chosen, then a click on the drawing (F35).
 *
 * @param tool - `rectangle`, `polygon`, `round` or `text`
 * @param point - the point of the drawing clicked
 */
function placeShape(tool: string, point: Point): void {
  clickElement(`tool-${tool}`);
  clickAt("#canvas", toWindow(point));
}

/**
 * Clicks the drawing with the selector armed, as a mouse does: pressed, released, clicked.
 *
 * @param point - the point of the drawing clicked
 */
function pick(point: Point): void {
  pressAndClick("#canvas", toWindow(point));
}

/**
 * Reads the path data of the shapes drawn, the editor's frames, boxes and handles left out.
 *
 * @returns each shape's `d`, the first drawn first
 */
function readShapePaths(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas svg > path:not([class])")].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Reads the outline of the shape selected in the layer above the drawing.
 *
 * @returns its path data; undefined for none
 */
function readSelectionOutline(): string | undefined {
  return document.querySelector("#overlay svg path.selection")?.getAttribute("d") ?? undefined;
}

/**
 * The outline expected of a shape: the band about its contour as drawn, 2 pixels wide, a chord
 * at most 2 pixels long, as the hover's (US-065).
 *
 * @param shape - the shape outlined
 * @returns the path data of its band
 */
function outlineOf(shape: Shape): string {
  return contourBand(shapePickArea(shape).pieces, 1, 2)
    .map((subpath) => contourPiecesToPathData(subpath))
    .join(" ");
}

/**
 * Draws A, B and C in that order, C edited: A the rectangle 120 × 80 at (0, 0) the page opens
 * with, radius 10; B a polygon 100 × 100 at (250, 150); C a round shape 100 × 100 at (450, 150).
 *
 * @returns their models, and their paths in the order drawn
 */
function drawThreeShapes(): {
  readonly a: Shape;
  readonly b: Shape;
  readonly c: Shape;
  readonly paths: readonly string[];
} {
  const a = { model: readShapeModel(), path: readText("#path-data") };

  placeShape("polygon", { x: 300, y: 200 });

  const b = { model: readShapeModel(), path: readText("#path-data") };

  placeShape("round", { x: 500, y: 200 });

  const c = { model: readShapeModel(), path: readText("#path-data") };

  return { a: a.model, b: b.model, c: c.model, paths: [a.path, b.path, c.path] };
}

describe("a click with the selector selects the shape under it (US-067)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), radius 10, whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F36.AC3] edits A clicked, selected above, drawn under B and C; a new shape goes on top", () => {
    const { a, c, paths } = drawThreeShapes();

    // C, the last placed, is the shape edited, and shown selected.
    expect(readSelectionOutline()).toBe(outlineOf(c));

    // (60, 40) is in A alone.
    pick({ x: 60, y: 40 });

    expect(readShapeModel()).toEqual(a);
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readText("#path-data")).toBe(paths[0]);
    expect(readSelectionOutline()).toBe(outlineOf(a));
    // A keeps its place, under B and C.
    expect(readShapePaths()).toEqual(paths);

    placeShape("rectangle", { x: 300, y: 450 });

    expect(readShapePaths()).toEqual([...paths, readText("#path-data")]);
  });

  it("[F36.AC3] edits B picked from the middle, A below it and C above, then C again", () => {
    const { b, c, paths } = drawThreeShapes();

    // (300, 200), the middle of B's box, is in the hexagon.
    pick({ x: 300, y: 200 });

    expect(readShapeModel()).toEqual(b);
    expect(readText("#shape-title")).toBe("Polygon");
    expect(readShapePaths()).toEqual(paths);

    pick({ x: 500, y: 200 });

    expect(readShapeModel()).toEqual(c);
    expect(readText("#shape-title")).toBe("Round shape");
    expect(readShapePaths()).toEqual(paths);
  });

  it("[F36.AC3] keeps the selection shown while a tool is armed", () => {
    const { a } = drawThreeShapes();

    pick({ x: 60, y: 40 });
    clickElement("tool-rectangle");

    expect(readSelectionOutline()).toBe(outlineOf(a));
  });

  it("[F36.AC3] selects none on a click on the empty drawing, the panel saying so", () => {
    const { paths } = drawThreeShapes();

    // (200, 300) is in none of the three shapes.
    pick({ x: 200, y: 300 });

    expect(readText("#shape-title")).toBe("No shape selected");
    expect(readDisplay("shape-panel")).toBe("none");
    expect(readSelectionOutline()).toBeUndefined();
    // Every shape still drawn, in the order drawn, the one edited before among them.
    expect(readShapePaths()).toEqual(paths);

    // A redraw keeps none selected.
    globalThis.dispatchEvent(new Event("resize"));

    expect(readText("#shape-title")).toBe("No shape selected");
    expect(readShapePaths()).toEqual(paths);

    placeShape("rectangle", { x: 300, y: 450 });

    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readDisplay("shape-panel")).not.toBe("none");
    expect(readShapePaths()).toEqual([...paths, readText("#path-data")]);
  });

  it("[F36.AC3] exports no selection outline, the drawing's SVG the same whichever is selected", () => {
    drawThreeShapes();

    const exported = readExportedSvg(document, readDrawnTexts(document));

    pick({ x: 60, y: 40 });

    expect(readSelectionOutline()).toBeDefined();
    expect(selectPageElement("#canvas").querySelector(".selection")).toBeNull();
    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);

    pick({ x: 200, y: 300 });

    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);
  });

  it("[F36.AC3] picks nothing on the click that ends a drag with a tool", () => {
    // A drag from (200, 150) to (100, 60) draws a rectangle 100 × 90 at (100, 60), radius 10 as
    // the last rectangle: its corner (100, 60) is cut, 14.1 from the arc's center (110, 70), while
    // A, 120 × 80 at (0, 0), holds it. The click that follows the release would pick A.
    clickElement("tool-rectangle");
    pressPointer("#canvas", "pointerdown", { ...toWindow({ x: 200, y: 150 }), buttons: 1 });
    pressPointer("#canvas", "pointermove", { ...toWindow({ x: 100, y: 60 }), buttons: 1 });
    pressPointer("#canvas", "pointerup", { ...toWindow({ x: 100, y: 60 }), buttons: 0 });
    clickAt("#canvas", toWindow({ x: 100, y: 60 }));

    expect(readShapeModel()).toMatchObject({ height: 90, width: 100, x: 100, y: 60 });
  });
});
