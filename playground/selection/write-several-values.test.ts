// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { HELLO_TEXT } from "../../tests";
import { readFontSession, writeFontSession } from "../fonts";
import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";
import { selectElement, writeEditedKind } from "../page";
import {
  clickAt,
  clickElement,
  openServedPage,
  pressAndClick,
  pressKey,
  readDisplay,
  readValue,
  selectInput,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

import { writeSeveralValues } from "./write-several-values";

import type { Point } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner (`DRAWING_MARGIN`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** The four fields of several shapes, X, Y, W and H. */
const FIELDS = ["several-x", "several-y", "several-width", "several-height"];

/**
 * The window's point of a point of the drawing.
 *
 * @param point - the point of the drawing
 * @returns its `clientX` and `clientY`
 */
function toWindow(point: Point): { readonly clientX: number; readonly clientY: number } {
  return { clientX: point.x + 140, clientY: point.y + 90 };
}

/**
 * Places a shape: its tool chosen, then a click on the drawing (F35).
 *
 * @param tool - `rectangle` or `text`
 * @param point - the point of the drawing clicked
 */
function placeShape(tool: string, point: Point): void {
  clickElement(`tool-${tool}`);
  clickAt("#canvas", toWindow(point));
}

/**
 * Clicks the drawing with the Selector as a mouse does, Shift held or not.
 *
 * @param point - the point of the drawing clicked
 * @param isShift - whether Shift is held
 */
function pick(point: Point, isShift: boolean): void {
  const at = toWindow(point);

  if (!isShift) {
    pressAndClick("#canvas", at);

    return;
  }

  const canvas = selectPageElement("#canvas");

  canvas.dispatchEvent(new PointerEvent("pointerdown", { ...at, bubbles: true, buttons: 1 }));
  canvas.dispatchEvent(new PointerEvent("pointerup", { ...at, bubbles: true, buttons: 0 }));
  canvas.dispatchEvent(new MouseEvent("click", { ...at, bubbles: true, shiftKey: true }));
}

/**
 * Reads what the four fields of several shapes show: each value, or its placeholder in brackets.
 *
 * @returns X, Y, W and H as shown
 */
function readSeveralFields(): readonly string[] {
  return FIELDS.map((id) => {
    const field = selectInput(id);

    return field.value === "" ? `[${field.placeholder}]` : field.value;
  });
}

/**
 * Types the box of the rectangle the panel edits.
 *
 * @param box - its x, y, width and height
 */
function typeBox(box: readonly [string, string, string, string]): void {
  const [x, y, width, height] = box;

  typeValue("width", width);
  typeValue("height", height);
  typeValue("shape-x", x);
  typeValue("shape-y", y);
}

/**
 * Draws A at (10, 20), 100 × 50, then B at (200, 20), 40 × 40, both of radius 10, whose arcs
 * reach their sides; B edited. (50, 40) is inside A, (220, 40) inside B.
 */
function drawAAndB(): void {
  typeBox(["10", "20", "100", "50"]);
  placeShape("rectangle", { x: 220, y: 40 });
  typeBox(["200", "20", "40", "40"]);
}

/**
 * Sets the rectangle the panel edits to A, sharp, 100 × 50 at (10, 20), then turns it by 30°.
 */
function turnA(): void {
  typeValue("angle", "0");
  typeValue("radius", "0");
  typeBox(["10", "20", "100", "50"]);
  typeValue("angle", "30");
}

/**
 * Presses Ctrl + A outside the fields, which selects every shape (US-068).
 */
function selectAll(): void {
  selectPageElement("body").dispatchEvent(
    new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ctrlKey: true, key: "a" }),
  );
}

describe("the panel shows several shapes' position and size (US-070)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), radius 10, whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F38.AC1] shows A and B as X Mixed, Y 20, W Mixed, H Mixed, read-only", () => {
    drawAAndB();
    pick({ x: 50, y: 40 }, false);
    pick({ x: 220, y: 40 }, true);

    expect(readSeveralFields()).toEqual(["[Mixed]", "20", "[Mixed]", "[Mixed]"]);
    expect(FIELDS.map((id) => selectInput(id).readOnly)).toEqual([true, true, true, true]);
    expect(readDisplay("several-fields")).not.toBe("none");
    expect(readDisplay("shape-panel")).toBe("none");
  });

  it("[F38.AC1] shows Y Mixed once B is set to Y 30", () => {
    drawAAndB();
    typeValue("shape-y", "30");
    pick({ x: 50, y: 40 }, true);

    expect(readSeveralFields()).toEqual(["[Mixed]", "[Mixed]", "[Mixed]", "[Mixed]"]);
  });

  it("[F38.AC1] shows the values common to two rectangles alike, without a placeholder", () => {
    typeBox(["10", "20", "100", "50"]);
    placeShape("rectangle", { x: 300, y: 300 });
    typeBox(["10", "200", "100", "50"]);
    pick({ x: 50, y: 40 }, true);

    expect(readSeveralFields()).toEqual(["10", "[Mixed]", "100", "50"]);
    expect(selectInput("several-x").placeholder).toBe("");
  });

  it("[F38.AC1] hides the fields with one shape selected, the panel editing it as before", () => {
    expect(readDisplay("several-fields")).toBe("none");

    drawAAndB();
    pick({ x: 50, y: 40 }, false);
    pick({ x: 220, y: 40 }, true);
    pick({ x: 50, y: 40 }, false);

    expect(readDisplay("several-fields")).toBe("none");
    expect(readDisplay("shape-panel")).not.toBe("none");
    expect(readValue("width")).toBe("100");

    pressKey("body", "Escape");

    expect(readDisplay("several-fields")).toBe("none");
  });

  it("[F38.AC1] shows a text by its frame, two texts alike at one height giving its Y, W and H", () => {
    placeShape("text", { x: 250, y: 140 });

    const y = readValue("shape-y");

    placeShape("text", { x: 450, y: 140 });
    pick({ x: 250, y: 140 }, true);

    const [x, shownY, width, height] = readSeveralFields();

    // At 0° a text's straight box is its frame, whose top the panel shows for it alone.
    expect(x).toBe("[Mixed]");
    expect(shownY).toBe(y);
    expect(width).toMatch(/^\d/u);
    expect(height).toMatch(/^\d/u);
  });

  it("[F38.AC1] leaves out a text never measured: with the rectangle, the rectangle's values", () => {
    // A text kept by hand at (0, 0), never drawn: no family measures it, and none did before.
    writeFontSession(document, { ...readFontSession(document), kept: [] });

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn", x: 500, y: 500 } },
    ]);

    selectAll();

    // The rectangle the page opens with: 120 × 80 at (0, 0).
    expect(readSeveralFields()).toEqual(["0", "0", "120", "80"]);
  });

  it("[F38.AC1] shows no placeholder when no shape selected has values", () => {
    const never = { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn" } };

    writeFontSession(document, { ...readFontSession(document), kept: [] });
    writeStoredDrawing(document, DRAWING_KEYS.below, [never, never]);
    writeStoredDrawing(document, DRAWING_KEYS.edited, []);
    writeEditedKind(document, "several");
    selectElement(document, "canvas").dataset[DRAWING_KEYS.selected] = "0 1";
    writeSeveralValues(document);

    expect(readSeveralFields()).toEqual(["[]", "[]", "[]", "[]"]);
  });

  it("[F38.AC1] shows A turned by 30° by its straight box's x and y, as for one shape", () => {
    // A sharp, 100 × 50 at (10, 20), turned by 30° about its middle (60, 45): half sizes
    // 50 cos 30° + 25 sin 30° = 55.80127 and 50 sin 30° + 25 cos 30° = 46.65064, so its straight
    // box starts at (60 − 55.80127, 45 − 46.65064) = (4.19873, −1.65064).
    turnA();

    const alone = [readValue("shape-x"), readValue("shape-y")];

    placeShape("rectangle", { x: 500, y: 400 });
    turnA();
    selectAll();

    expect(alone).toEqual(["4.19873", "-1.65064"]);
    expect(readSeveralFields()).toEqual(["4.19873", "-1.65064", "100", "50"]);
  });
});
