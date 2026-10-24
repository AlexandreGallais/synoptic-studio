// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { DEFAULT_FONT } from "../../src";
import { ARIAL_METRICS } from "../../tests";
import { readFontSession, writeFontSession } from "../fonts";
import {
  chooseOption,
  chooseRadio,
  clickAt,
  clickElement,
  openServedPage,
  pressAndClick,
  readCheckedRadio,
  readModel,
  readPressedState,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeTextContent,
} from "../user-actions";

import type { FontFamily, Point } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner at the opening view (`OPENING_VIEW`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/** A family Roboto of four faces, regular and bold, upright and italic, measured as Arial. */
const ROBOTO: FontFamily = {
  faces: [400, 700].flatMap((weight) =>
    [false, true].map((isItalic) => ({
      file: undefined,
      isItalic,
      metrics: ARIAL_METRICS,
      weight,
    })),
  ),
  name: "Roboto",
  version: "",
};

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
 * Types the content of the Text tool's text area.
 *
 * @param content - the text typed
 */
function typeContent(content: string): void {
  const area = selectPageElement("#text-content");

  if (!(area instanceof HTMLTextAreaElement)) {
    return;
  }

  area.value = content;
  area.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Reads whether a check box of the page is ticked.
 *
 * @param id - its `id` attribute, without `#`
 * @returns whether it is ticked
 */
function readChecked(id: string): boolean {
  return document.querySelector<HTMLInputElement>(`#${id}`)?.checked === true;
}

describe("the panel loads the shape picked, per kind (US-067)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), radius 10, whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F36.AC3] shows a rectangle picked as a Rectangle, its box and its radius 10", () => {
    // Centred on (300, 200): 100 × 100 at (250, 150), the radius of the last rectangle, 10.
    placeShape("rectangle", { x: 300, y: 200 });

    const rectangle = readModel();

    placeShape("polygon", { x: 550, y: 200 });
    typeAndEnter("radius", "4");
    typeAndEnter("width", "60");
    pick({ x: 300, y: 200 });

    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readValue("shape")).toBe("rectangle");

    expect(["shape-x", "shape-y", "width", "height", "radius"].map((id) => readValue(id))).toEqual([
      "250",
      "150",
      "100",
      "100",
      "10",
    ]);

    expect(readModel()).toEqual(rectangle);
  });

  it("[F36.AC3] gives a turned rectangle picked back its angle, in place", () => {
    placeShape("rectangle", { x: 300, y: 200 });
    typeAndEnter("angle", "30");

    const rectangle = readModel();

    placeShape("round", { x: 550, y: 200 });
    pick({ x: 300, y: 200 });

    expect(readValue("angle")).toBe("30");
    expect(readModel()).toEqual(rectangle);
  });

  it("[F36.AC3] shows a polygon picked with its corners, radius, anchor and stretched option", () => {
    // A pentagon in the box 100 × 100 at (250, 150), anchored at its top-left: 100 wide, about
    // 95 high, so that (300, 200), 50 from its left and its top, is inside.
    placeShape("polygon", { x: 300, y: 200 });
    typeAndEnter("corners", "5");
    typeAndEnter("radius", "4");
    chooseRadio("anchor", "min min");

    const pentagon = readModel();

    placeShape("polygon", { x: 550, y: 200 });
    clickElement("stretched");
    typeAndEnter("corners", "8");
    typeAndEnter("radius", "0");
    chooseRadio("anchor", "mid mid");
    pick({ x: 300, y: 200 });

    expect(readText("#shape-title")).toBe("Polygon");

    expect([readValue("corners"), readValue("radius"), readCheckedRadio("anchor")]).toEqual([
      "5",
      "4",
      "min min",
    ]);

    expect(readModel()).toEqual(pentagon);
  });

  it("[F36.AC3] keeps the settings of the shape the panel leaves for the next of its kind", () => {
    placeShape("polygon", { x: 300, y: 200 });
    typeAndEnter("corners", "3");
    // A, the rectangle the page opens with, 120 × 80 at (0, 0), holds (60, 40).
    pick({ x: 60, y: 40 });
    placeShape("polygon", { x: 550, y: 200 });

    // The triangle left by the click, the last polygon the panel showed (SP-026).
    expect(readValue("corners")).toBe("3");

    // A polygon of 50 corners is refused: left by a click, it leaves nothing for the next.
    typeAndEnter("corners", "50");
    pick({ x: 60, y: 40 });
    placeShape("polygon", { x: 550, y: 450 });

    expect(readValue("corners")).toBe("3");
  });

  it("[F36.AC3] shows a free shape picked as a free shape", () => {
    placeShape("polygon", { x: 300, y: 200 });
    clickElement("convert");

    const free = readModel();

    placeShape("round", { x: 550, y: 200 });
    pick({ x: 300, y: 200 });

    expect(readText("#shape-title")).toBe("Free shape");
    expect(readModel()).toEqual(free);
  });

  it("[F36.AC3] gives a turned text picked back its place, its fractions kept", () => {
    // The text `Text` typed from (200, 281), 64.65625 × 38.71875 in Inter 32 (US-075), then turned
    // about its middle, (232.328125, 300.359375), which stays there.
    placeShape("text", { x: 200, y: 300 });
    typeTextContent("Text");
    typeAndEnter("angle", "30");

    const turned = readModel();

    placeShape("rectangle", { x: 550, y: 200 });
    pick({ x: 232, y: 300 });

    expect(readValue("angle")).toBe("30");
    expect(readModel()).toEqual(turned);
  });

  it("[F36.AC3] loads a text picked with its whole style, the font session taking its family", () => {
    writeFontSession(document, { ...readFontSession(document), kept: [DEFAULT_FONT, ROBOTO] });
    placeShape("text", { x: 200, y: 300 });
    chooseOption("text-font", "Roboto");
    clickElement("text-italic");
    chooseOption("text-weight", "700");
    clickElement("text-strike");
    typeAndEnter("text-size", "40");
    typeContent("Pump");

    const pump = readModel();

    // The next text takes the last text's style (SP-026); it is set back to Inter 400, upright.
    placeShape("text", { x: 500, y: 450 });
    chooseOption("text-font", "Inter");
    clickElement("text-italic");
    chooseOption("text-weight", "400");
    clickElement("text-strike");
    typeAndEnter("text-size", "24");
    typeContent("Valve");

    expect(readFontSession(document).measured).toBe("Inter");

    // 5 right of and below the top-left of the Pump text's block, inside it.
    pick({ x: pump.x + 5, y: pump.y + 5 });

    expect(readText("#shape-title")).toBe("Text");

    expect([readValue("text-font"), readValue("text-weight"), readValue("text-size")]).toEqual([
      "Roboto",
      "700",
      "40",
    ]);

    expect([readPressedState("text-italic"), readChecked("text-strike")]).toEqual(["true", true]);
    expect(readFontSession(document).measured).toBe("Roboto");
    expect(readModel()).toMatchObject({ content: "Pump" });
    expect(readModel()).toEqual(pump);
  });
});
