// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBand, contourPiecesToPathData, shapePickArea } from "../../src";
import { readExportedSvg } from "../document";
import { readBoxPathData } from "../drawing";
import { readDrawnTexts } from "../kept-shapes";
import {
  clickElement,
  clickPoint,
  openServedPage,
  readShapeModel,
  runResizeObservers,
  selectPageElement,
  setElementBox,
  typeAndEnter,
} from "../user-actions";

import { PAGE_VIEWS } from "./page-views";

import type { Point, View } from "../../src";

// The canvas sits at (100, 50) in the window, 1 200 × 800: the offset (x, y) on the canvas is the
// window's (x + 100, y + 50), over the drawing's point origin + (x, y) / scale.

/** The view of the examples at 200 %, (180, 130) at the canvas's top-left. */
const ZOOMED: View = { origin: { x: 180, y: 130 }, scale: 2 };

/** The pointer of the examples, (440, 340) on the canvas, in the window. */
const POINTER: Point = { x: 540, y: 390 };

/**
 * Shows a view, as the page would after a zoom: kept, then the page redrawn as the canvas resized.
 *
 * @param view - the view shown
 */
function showView(view: View): void {
  PAGE_VIEWS.set(document, view);
  runResizeObservers("#canvas");
}

/**
 * Sends a pointer event of the main button to the canvas, as the browser does.
 *
 * @param type - `pointerdown`, `pointermove`, `pointerup` or `click`
 * @param client - where the pointer is, in the window
 */
function sendPointer(type: string, client: Point): void {
  const init = {
    bubbles: true,
    button: 0,
    buttons: type === "pointerdown" || type === "pointermove" ? 1 : 0,
    clientX: client.x,
    clientY: client.y,
    pointerId: 1,
  };

  selectPageElement("#canvas").dispatchEvent(
    type === "click" ? new MouseEvent(type, init) : new PointerEvent(type, init),
  );
}

/**
 * Presses the main button, moves the pointer, releases it and clicks, as a mouse does.
 *
 * @param from - where it is pressed, in the window
 * @param to - where it is released, in the window
 */
function pressAndRelease(from: Point, to: Point): void {
  sendPointer("pointerdown", from);
  sendPointer("pointermove", to);
  sendPointer("pointerup", to);
  sendPointer("click", to);
}

/**
 * Types the box of the rectangle the panel edits, sharp, at an angle.
 *
 * @param box - its x, y, width, height and angle
 */
function typeRectangle(box: readonly [string, string, string, string, string]): void {
  const [x, y, width, height, angle] = box;

  typeAndEnter("angle", "0");
  typeAndEnter("radius", "0");
  typeAndEnter("width", width);
  typeAndEnter("height", height);
  typeAndEnter("shape-x", x);
  typeAndEnter("shape-y", y);
  typeAndEnter("angle", angle);
}

/**
 * Reads a path of an SVG of the page.
 *
 * @param selector - CSS selector of the path
 * @returns its path data; undefined for none
 */
function readPath(selector: string): string | undefined {
  return document.querySelector(selector)?.getAttribute("d") ?? undefined;
}

describe("a shape placed and moved at any zoom (US-078)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    typeRectangle(["10", "20", "100", "50", "0"]);
  });

  it("[F42.AC1] places a rectangle 100 × 100 at (350, 250), clicked over (400, 300) at 200 %", () => {
    showView(ZOOMED);
    clickElement("tool-rectangle");
    pressAndRelease(POINTER, POINTER);

    // (180 + 440 / 2, 130 + 340 / 2) = (400, 300), less half of 100 × 100.
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 350, y: 250 });

    // The offset (240, 140), over (180 + 240 / 2, 130 + 140 / 2) = (300, 200); (200, 100) at 100 %.
    clickElement("tool-rectangle");
    pressAndRelease({ x: 340, y: 190 }, { x: 340, y: 190 });
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 250, y: 150 });
  });

  it("places 100 × 100 centred on the press at 10 %, the pointer trembling by 1 pixel", () => {
    showView({ origin: { x: -40, y: -40 }, scale: 0.1 });
    clickElement("tool-rectangle");
    pressAndRelease(POINTER, { x: POINTER.x + 1, y: POINTER.y });

    // Pressed over (−40 + 440 / 0.1, −40 + 340 / 0.1) = (4 360, 3 360); 1 pixel is 10 units, within
    // 3 pixels: a click, centred on the press, (4 310, 3 310).
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 4310, y: 3310 });
  });

  it("draws 1 × 0 by a drag of 7 pixels at 800 %, places 100 × 100 by one of 3", () => {
    showView({ origin: { x: 0, y: 0 }, scale: 8 });

    // The offset (80, 80) is over (10, 10); 7 pixels across is (10.875, 10), rounded to (11, 10).
    const press = { x: 180, y: 130 };

    clickElement("tool-rectangle");
    pressAndRelease(press, { x: press.x + 7, y: press.y });
    expect(readShapeModel()).toMatchObject({ height: 0, width: 1, x: 10, y: 10 });

    // 3 pixels across, (10.375, 10): a click, 100 × 100 centred on (10, 10).
    clickElement("tool-rectangle");
    pressAndRelease(press, { x: press.x + 3, y: press.y });
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: -40, y: -40 });
  });

  it("moves A by the Selector only beyond 3 pixels from the press", () => {
    // A's (50, 40) is the offset (90, 80) at the opening view, (190, 130) in the window.
    const press = { x: 190, y: 130 };

    pressAndRelease(press, { x: press.x + 3, y: press.y });
    expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
    expect(document.querySelector("#overlay svg path.selection")).not.toBeNull();

    pressAndRelease(press, { x: press.x + 4, y: press.y });
    expect(readShapeModel()).toMatchObject({ x: 14, y: 20 });
  });

  it("keeps A's band 2 pixels wide at 200 %, its chords at most 2 pixels", () => {
    const area = shapePickArea(readShapeModel());

    showView({ origin: { x: 0, y: 0 }, scale: 2 });

    // A half width of 1 / 2 = 0.5 unit, chords of 2 / 2 = 1 unit.
    const band = contourBand(area.pieces, 0.5, 1)
      .map((subpath) => contourPiecesToPathData(subpath))
      .join(" ");

    expect(readPath("#overlay svg path.selection")).toBe(band);

    // Hovered over A's (50, 40), the offset (100, 80).
    sendPointer("pointermove", { x: 200, y: 130 });
    expect(readPath("#overlay svg path.hover")).toBe(band);
  });

  it("draws a turned shape's frame 0.5 wide and its dashes 2 in 3.5 at 200 %", () => {
    typeRectangle(["10", "20", "100", "50", "30"]);
    showView(ZOOMED);

    // The straight box of A turned by 30° about (60, 45) starts at (60 − 43.30127 − 12.5,
    // 45 − 25 − 21.65064) = (4.19873, −1.65064): dashes 2 long, 0.5 thick, every 3.5.
    expect(readPath("#canvas path.straight-box")).toMatch(
      /^M4\.19873 -1\.65064H6\.19873V-1\.15064H4\.19873Z M7\.69873 -1\.65064H9\.69873V-1\.15064H7\.69873Z /u,
    );

    const model = readShapeModel();

    expect(readPath("#canvas path.turned-box")).toBe(
      `${readBoxPathData(model, 0)} ${readBoxPathData(model, 0.5)}`,
    );
  });

  it("[F42.AC4] exports the same file at 100 % and at 200 %, its viewBox the opening view's", () => {
    const opening = readExportedSvg(document, readDrawnTexts(document));

    showView(ZOOMED);

    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(opening);
    expect(opening).toContain('viewBox="-40 -40 1200 800"');
  });
});

describe("the handles of the points at 200 % (US-078)", () => {
  beforeEach(() => {
    // Step 1: the rectangle 120 × 80, radius 10, point 1 at (0, 0); Points mode, point 1 selected.
    openServedPage("?step=1");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    clickElement("mode-points");
    clickPoint(1, false);
    showView(ZOOMED);
  });

  it("draws a vertex's handle of radius 8 / 2 = 4, its halo 13 / 2 = 6.5", () => {
    expect(readPath("#canvas path.handle.first")).toBe("M-4 0 A4 4 0 1 1 4 0 A4 4 0 1 1 -4 0 Z");

    expect(readPath("#canvas path.halo")).toBe(
      "M-6.5 0 A6.5 6.5 0 1 1 6.5 0 A6.5 6.5 0 1 1 -6.5 0 Z",
    );
  });

  it("draws a curve's handle square 8 / 2 = 4 units", () => {
    // Point 2 at (120, 0), its handle after at (150, −15), before mirrored at (90, 15).
    typeAndEnter("radius", "0");
    clickPoint(2, false);
    clickElement("handles-switch");
    typeAndEnter("handle-after-x", "150");
    typeAndEnter("handle-after-y", "-15");

    expect(
      [...document.querySelectorAll("#canvas path.curve-handle")].map(
        (mark) => mark.getAttribute("d")?.split(" M", 2)[1],
      ),
    ).toEqual(
      expect.arrayContaining([
        "148 -17 L152 -17 L152 -13 L148 -13 Z",
        "88 13 L92 13 L92 17 L88 17 Z",
      ]),
    );
  });
});
