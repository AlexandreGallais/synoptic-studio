// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBand, contourPiecesToPathData, shapePickArea } from "../../src";
import { readExportedSvg } from "../document";
import { readDrawnTexts } from "../kept-shapes";
import {
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readShapeModel,
  runResizeObservers,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";
import { PAGE_VIEWS } from "../view";

import type { Point, Shape } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** Shift held. */
const SHIFT: PointerKeys = { altKey: false, shiftKey: true };

/** The longest chord along a curve of the outlines: 2 screen pixels at the view's scale of 1. */
const STEP = 2;

/**
 * Presses the drawing, moves the pointer, releases it, then the `click` the browser fires.
 *
 * @param from - the point pressed
 * @param to - the point released
 * @param keys - the keys held
 */
function dragFrom(from: Point, to: Point, keys: PointerKeys): void {
  dispatchDrawingPointer("pointerdown", from, keys);
  dispatchDrawingPointer("pointermove", to, keys);
  dispatchDrawingPointer("pointerup", to, keys);
  dispatchDrawingPointer("click", to, keys);
}

/**
 * Draws A, B and C, 100 × 100 at (0, 0), (150, 0) and (300, 0), in the page's color: A the
 * rectangle the page opens with, typed 100 × 100; B and C dragged with the Rectangle tool. C is then
 * selected.
 *
 * @returns their models
 */
function drawThreeShapes(): { readonly a: Shape; readonly b: Shape; readonly c: Shape } {
  typeValue("width", "100");
  typeValue("height", "100");

  const a = readShapeModel();

  clickElement("tool-rectangle");
  dragFrom({ x: 150, y: 0 }, { x: 250, y: 100 }, NO_KEY);

  const b = readShapeModel();

  clickElement("tool-rectangle");
  dragFrom({ x: 300, y: 0 }, { x: 400, y: 100 }, NO_KEY);

  return { a, b, c: readShapeModel() };
}

/**
 * Reads the paths of the layer above the drawing, in the order drawn.
 *
 * @returns each one's class and path data
 */
function readOverlayPaths(): readonly { readonly name: string; readonly d: string | null }[] {
  return [...document.querySelectorAll("#overlay svg path")].map((path) => ({
    d: path.getAttribute("d"),
    name: path.getAttribute("class") ?? "",
  }));
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

/**
 * The band expected about a shape's contour as drawn.
 *
 * @param shape - the shape outlined
 * @param half - half the band's width, in user units
 * @param step - the longest chord along a curve, in user units
 * @returns the path data of its band
 */
function bandOf(shape: Shape, half: number, step: number): string {
  return contourBand(shapePickArea(shape).pieces, half, step)
    .map((subpath) => contourPiecesToPathData(subpath))
    .join(" ");
}

/**
 * Reads the fill the page's style gives a path of the layer above the drawing.
 *
 * @param name - its class
 * @returns its computed fill
 */
function readFill(name: string): string {
  return getComputedStyle(selectPageElement(`#overlay svg path.${name}`)).fill;
}

describe("the helps seen on any color (US-080)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), radius 10, whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F43.AC2] outlines A under the pointer by a white band 4 pixels wide under the accent's 2", () => {
    const { a } = drawThreeShapes();

    dispatchDrawingPointer("pointermove", { x: 50, y: 50 }, NO_KEY);

    // At 100 %: the light band's half width 4 / 2 = 2, the accent's 2 / 2 = 1, both chords 2 long,
    // so that 2 − 1 = 1 pixel of white shows on each side of the blue.
    expect(readOverlayPaths().filter(({ name }) => name.startsWith("hover"))).toEqual([
      { d: bandOf(a, 2, STEP), name: "hover-light" },
      { d: bandOf(a, 1, STEP), name: "hover" },
    ]);

    expect(readFill("hover-light")).toBe("#fff");
    expect(readFill("hover")).toBe("#4a7bd0");
  });

  it("[F43.AC2] outlines A selected by the same two bands", () => {
    const { a } = drawThreeShapes();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);

    expect(readOverlayPaths().filter(({ name }) => name.startsWith("selection"))).toEqual([
      { d: bandOf(a, 2, STEP), name: "selection-light" },
      { d: bandOf(a, 1, STEP), name: "selection" },
    ]);

    expect(readFill("selection-light")).toBe("#fff");
  });

  it("[F43.AC2] keeps the bands 4 and 2 pixels wide at 200 %, 2 and 1 units", () => {
    const { a } = drawThreeShapes();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    PAGE_VIEWS.set(document, { origin: { x: 0, y: 0 }, scale: 2 });
    runResizeObservers("#canvas");

    // Half widths 4 / 2 / 2 = 1 and 2 / 2 / 2 = 0.5 unit, chords 2 / 2 = 1 unit.
    expect(readOverlayPath("selection-light")).toBe(bandOf(a, 1, 1));
    expect(readOverlayPath("selection")).toBe(bandOf(a, 0.5, 1));
  });

  it("[F43.AC2] dashes the box of A and B in the accent over a white frame, 1 pixel thick", () => {
    drawThreeShapes();
    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    dragFrom({ x: 200, y: 50 }, { x: 200, y: 50 }, SHIFT);

    // The box from (0, 0) to (250, 100): the white frame from its sides to 1 inward, its hole the
    // other way round, so that the 3 between the dashes, 4 long every 7, show white; drawn after
    // the outlines of A and B selected, before B's, under the pointer.
    const names = readOverlayPaths().map(({ name }) => name);

    expect(readOverlayPath("selection-box-light")).toBe("M0 0H250V100H0Z M1 1V99H249V1Z");
    expect(readOverlayPath("selection-box")?.startsWith("M0 0H4V1H0Z M7 0H11V1H7Z")).toBe(true);
    expect(names.slice(-4, -2)).toEqual(["selection-box-light", "straight-box selection-box"]);
    expect(readFill("selection-box-light")).toBe("#fff");
  });

  it("[F43.AC2] draws the rectangle's band of 1 pixel over a white one of 3, its tint unchanged", () => {
    drawThreeShapes();
    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);

    // The white band 3 wide centred on the sides: from −10 − 1.5 to 200 + 1.5 outside, from
    // −10 + 1.5 to 200 − 1.5 inside; the accent's 1 wide over it, 0.5 each side.
    const names = readOverlayPaths().map(({ name }) => name);

    expect(names.slice(-3)).toEqual([
      "selection-area",
      "selection-rectangle-light",
      "selection-rectangle",
    ]);

    expect(readOverlayPath("selection-rectangle-light")).toBe(
      "M-11.5 -11.5H201.5V111.5H-11.5Z M-8.5 -8.5V108.5H198.5V-8.5Z",
    );

    expect(readOverlayPath("selection-rectangle")).toBe(
      "M-10.5 -10.5H200.5V110.5H-10.5Z M-9.5 -9.5V109.5H199.5V-9.5Z",
    );

    expect(readFill("selection-rectangle-light")).toBe("#fff");
    expect(readFill("selection-area")).toBe("color-mix(in srgb, #4a7bd0 12%, transparent)");
  });

  it("[F43.AC2] draws a rectangle thinner than the white band as that band alone, no hole", () => {
    drawThreeShapes();
    // From (5, 150) to (5, 200): 0 wide, thinner than 3.
    dispatchDrawingPointer("pointerdown", { x: 5, y: 150 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 5, y: 200 }, NO_KEY);

    expect(readOverlayPath("selection-rectangle-light")).toBe("M3.5 148.5H6.5V201.5H3.5Z");
  });

  it("[F43.AC2] exports none of the helps, the file of A, B and C unchanged", () => {
    drawThreeShapes();
    dispatchDrawingPointer("pointermove", { x: 600, y: 300 }, NO_KEY);

    const exported = readExportedSvg(document, readDrawnTexts(document));
    const drawing = selectPageElement("#canvas").getHTML();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    dragFrom({ x: 200, y: 50 }, { x: 200, y: 50 }, SHIFT);
    dispatchDrawingPointer("pointermove", { x: 350, y: 50 }, NO_KEY);
    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);

    expect(readOverlayPath("selection-rectangle-light")).toBeDefined();
    expect(selectPageElement("#canvas").getHTML()).toBe(drawing);
    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);
  });
});
