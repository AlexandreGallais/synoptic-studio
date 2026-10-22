// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBand, contourPiecesToPathData, shapePickArea } from "../../src";
import { readExportedSvg } from "../document";
import { readDrawnTexts } from "../kept-shapes";
import {
  clickAt,
  clickElement,
  openServedPage,
  readText,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

import type { Point, Shape } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner (`DRAWING_MARGIN`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/** Half the width of the outline: 2 screen pixels wide at the view's scale of 1. */
const HALF_WIDTH = 1;

/** The longest chord along a curve of the outline: 2 screen pixels at the view's scale of 1. */
const STEP = 2;

/**
 * Fires a pointer event on the canvas at a point of the drawing, as the browser does.
 *
 * @param type - `pointerdown`, `pointermove`, `pointerup` or `click`
 * @param point - the point of the drawing, possibly fractional
 */
function firePointer(type: string, point: Point): void {
  const init = {
    bubbles: true,
    buttons: type === "pointerdown" ? 1 : 0,
    clientX: point.x + ORIGIN_LEFT,
    clientY: point.y + ORIGIN_TOP,
    pointerId: 1,
  };
  const event = type === "click" ? new MouseEvent(type, init) : new PointerEvent(type, init);

  selectPageElement("#canvas").dispatchEvent(event);
}

/**
 * Moves the pointer over the drawing, no button held.
 *
 * @param point - the point of the drawing it passes over
 */
function hover(point: Point): void {
  firePointer("pointermove", point);
}

/**
 * Places a shape: its tool chosen, then a click on the drawing (F35).
 *
 * @param tool - `rectangle`, `polygon`, `round` or `text`
 * @param point - the point of the drawing clicked
 */
function placeShape(tool: string, point: Point): void {
  clickElement(`tool-${tool}`);
  clickAt("#canvas", { clientX: point.x + ORIGIN_LEFT, clientY: point.y + ORIGIN_TOP });
}

/**
 * Reads the model of the shape the panel edits.
 *
 * @returns the shape
 */
function readModel(): Shape {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the model shown, a shape of the drawing
  return JSON.parse(readText("#model")) as Shape;
}

/**
 * Reads the outline the layer above the drawing holds.
 *
 * @returns its path data; undefined for none
 */
function readOutline(): string | undefined {
  return document.querySelector("#overlay svg path.hover")?.getAttribute("d") ?? undefined;
}

/**
 * The outline expected of a shape: the band about its contour as drawn, 2 pixels wide.
 *
 * @param shape - the shape outlined
 * @returns the path data of its band
 */
function outlineOf(shape: Shape): string {
  return contourBand(shapePickArea(shape).pieces, HALF_WIDTH, STEP)
    .map((subpath) => contourPiecesToPathData(subpath))
    .join(" ");
}

describe("the shape under the pointer outlined in the layer above (US-065)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), radius 10, whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F36.AC2] outlines the shape under the pointer, and nothing once it leaves", () => {
    const first = readModel();

    // Centred on the points clicked: (250, 150) and (450, 150), each 100 × 100.
    placeShape("rectangle", { x: 300, y: 200 });

    const second = readModel();

    placeShape("round", { x: 500, y: 200 });

    const third = readModel();

    hover({ x: 60, y: 40 });
    expect(readOutline()).toBe(outlineOf(first));

    hover({ x: 300, y: 200 });
    expect(readOutline()).toBe(outlineOf(second));

    hover({ x: 500, y: 200 });
    expect(readOutline()).toBe(outlineOf(third));

    // Between the shapes: none.
    hover({ x: 200, y: 300 });
    expect(readOutline()).toBeUndefined();

    hover({ x: 60, y: 40 });
    selectPageElement("#canvas").dispatchEvent(new PointerEvent("pointerleave", { pointerId: 1 }));
    expect(readOutline()).toBeUndefined();
  });

  it("[F36.AC2] outlines the top one of two shapes overlapping", () => {
    const first = readModel();

    // A rectangle 100 × 100 at (50, 10), above the first, 120 × 80 at (0, 0).
    placeShape("rectangle", { x: 100, y: 60 });

    const second = readModel();

    hover({ x: 75, y: 40 });
    expect(readOutline()).toBe(outlineOf(second));

    hover({ x: 25, y: 40 });
    expect(readOutline()).toBe(outlineOf(first));
  });

  it("[F36.AC2] outlines nothing in the cut corner of a rounded rectangle", () => {
    const first = readModel();

    // The corner's arc is centred on (10, 10), radius 10: (1, 1) is 12.7 from it, outside;
    // (4, 4) is 8.5 from it, inside.
    hover({ x: 1, y: 1 });
    expect(readOutline()).toBeUndefined();

    hover({ x: 4, y: 4 });
    expect(readOutline()).toBe(outlineOf(first));
  });

  it("[F36.AC2] exports no outline, the drawing's SVG unchanged by it", () => {
    hover({ x: 200, y: 300 });

    const drawing = selectPageElement("#canvas").getHTML();
    const exported = readExportedSvg(document, readDrawnTexts(document));

    hover({ x: 60, y: 40 });

    expect(readOutline()).toBeDefined();
    expect(selectPageElement("#canvas").getHTML()).toBe(drawing);
    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);
  });

  it("[F36.AC2] lets the pointer through the layer, of the drawing's size and origin", () => {
    hover({ x: 60, y: 40 });

    const overlay = selectPageElement("#overlay");

    expect(getComputedStyle(overlay).pointerEvents).toBe("none");
    // The canvas 800 × 600, the origin 40 from its top-left, as the drawing's.
    expect(overlay.querySelector("svg")?.getAttribute("viewBox")).toBe("-40 -40 800 600");
  });

  it("[F36.AC2] outlines nothing while a tool is armed, a click then placing a shape", () => {
    clickElement("tool-rectangle");
    hover({ x: 60, y: 40 });

    expect(readOutline()).toBeUndefined();
  });

  it("[F36.AC2] follows the shape changed under a pointer standing still", () => {
    hover({ x: 60, y: 40 });
    typeValue("width", "200");

    expect(readModel().width).toBe(200);
    expect(readOutline()).toBe(outlineOf(readModel()));
  });

  it("[F36.AC2] outlines a text anywhere in its block, between its letters too", () => {
    placeShape("text", { x: 400, y: 300 });
    // The text `Text` centred on (400, 300): its middle is in its block, whatever its letters.
    hover({ x: 400, y: 300 });

    expect(readOutline()).toMatch(/^M/u);

    hover({ x: 400, y: 450 });
    expect(readOutline()).toBeUndefined();
  });

  it("[F36.AC2] outlines a shape of width 0 within 4 pixels of it", () => {
    // A drag straight down from (300, 100) to (300, 180) adds a rectangle 0 × 80 (US-064).
    clickElement("tool-rectangle");
    firePointer("pointerdown", { x: 300, y: 100 });
    firePointer("pointermove", { x: 300, y: 180 });
    firePointer("pointerup", { x: 300, y: 180 });
    firePointer("click", { x: 300, y: 180 });

    const flat = readModel();

    expect(flat.width).toBe(0);

    // 2 from it: within 4 pixels; 6 from it: beyond.
    hover({ x: 302, y: 140 });
    expect(readOutline()).toBe(outlineOf(flat));

    hover({ x: 306, y: 140 });
    expect(readOutline()).toBeUndefined();
  });
});
