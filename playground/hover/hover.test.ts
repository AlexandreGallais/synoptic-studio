// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBand, contourPiecesToPathData, shapePickArea } from "../../src";
import { HELLO_TEXT } from "../../tests";
import { readExportedSvg } from "../document";
import { readFontSession, writeFontSession } from "../fonts";
import { DRAWING_KEYS, readDrawnTexts, writeStoredDrawing } from "../kept-shapes";
import {
  clickAt,
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readShapeModel,
  selectPageElement,
  setElementBox,
  typeTextContent,
  typeValue,
} from "../user-actions";

import type { Point, Shape } from "../../src";
import type { PointerKeys } from "../user-actions";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner at the opening view (`OPENING_VIEW`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/** Half the width of the outline: 2 screen pixels wide at the view's scale of 1. */
const HALF_WIDTH = 1;

/** The longest chord along a curve of the outline: 2 screen pixels at the view's scale of 1. */
const STEP = 2;

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/**
 * Moves the pointer over the drawing, no press on: the page reads no button.
 *
 * @param point - the point of the drawing it passes over
 */
function hover(point: Point): void {
  dispatchDrawingPointer("pointermove", point, NO_KEY);
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
    const first = readShapeModel();

    // Centred on the points clicked: (250, 150) and (450, 150), each 100 × 100.
    placeShape("rectangle", { x: 300, y: 200 });

    const second = readShapeModel();

    placeShape("round", { x: 500, y: 200 });

    const third = readShapeModel();

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
    const first = readShapeModel();

    // A rectangle 100 × 100 at (50, 10), above the first, 120 × 80 at (0, 0).
    placeShape("rectangle", { x: 100, y: 60 });

    const second = readShapeModel();

    hover({ x: 75, y: 40 });
    expect(readOutline()).toBe(outlineOf(second));

    hover({ x: 25, y: 40 });
    expect(readOutline()).toBe(outlineOf(first));
  });

  it("[F36.AC2] outlines nothing in the cut corner of a rounded rectangle", () => {
    const first = readShapeModel();

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

    expect(readShapeModel().width).toBe(200);
    expect(readOutline()).toBe(outlineOf(readShapeModel()));
  });

  it("[F36.AC2] outlines a text anywhere in its block, between its letters too", () => {
    placeShape("text", { x: 400, y: 300 });
    typeTextContent("Text");
    // The text `Text` from (400, 281), 64.65625 wide in Inter 32 (US-075): (430, 300) is in its
    // block, whatever its letters.
    hover({ x: 430, y: 300 });

    expect(readOutline()).toMatch(/^M/u);

    hover({ x: 400, y: 450 });
    expect(readOutline()).toBeUndefined();
  });

  it("[F36.AC2] outlines a kept text whose font's metrics are no longer read, as it is drawn", () => {
    placeShape("text", { x: 400, y: 300 });
    typeTextContent("Text");
    placeShape("rectangle", { x: 600, y: 300 });
    hover({ x: 400, y: 300 });

    const outline = readOutline();

    // No family measures the kept text any more (US-061, US-056): it is drawn with the metrics
    // it was last drawn with, and outlined with them.
    writeFontSession(document, { ...readFontSession(document), kept: [] });
    placeShape("round", { x: 600, y: 500 });
    hover({ x: 400, y: 300 });

    expect(outline).toMatch(/^M/u);
    expect(readOutline()).toBe(outline);
  });

  it("[F36.AC2] outlines no text that no metrics ever measured, as it is not drawn", () => {
    // A text kept by hand at (0, 0), never drawn: no family measures it, and none did before.
    writeFontSession(document, { ...readFontSession(document), kept: [] });

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn" } },
    ]);

    placeShape("round", { x: 500, y: 300 });
    // In its block were it measured, and in the cut corner of the first rectangle, radius 10.
    hover({ x: 1, y: 1 });

    expect(readOutline()).toBeUndefined();
  });

  it("[F36.AC2] outlines a shape of width 0 within 4 pixels of it", () => {
    // A drag straight down from (300, 100) to (300, 180) adds a rectangle 0 × 80 (US-064).
    clickElement("tool-rectangle");
    dispatchDrawingPointer("pointerdown", { x: 300, y: 100 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 300, y: 180 }, NO_KEY);
    dispatchDrawingPointer("pointerup", { x: 300, y: 180 }, NO_KEY);
    dispatchDrawingPointer("click", { x: 300, y: 180 }, NO_KEY);

    const flat = readShapeModel();

    expect(flat.width).toBe(0);

    // 2 from it: within 4 pixels; 6 from it: beyond.
    hover({ x: 302, y: 140 });
    expect(readOutline()).toBe(outlineOf(flat));

    hover({ x: 306, y: 140 });
    expect(readOutline()).toBeUndefined();
  });
});
