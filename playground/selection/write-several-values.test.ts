// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { isDrawnText } from "../../src";
import { HELLO_TEXT } from "../../tests";
import { readFontSession, writeFontSession } from "../fonts";
import { DRAWING_KEYS, readStoredDrawing, writeStoredDrawing } from "../kept-shapes";
import { selectElement, writeEditedKind } from "../page";
import { readTextBlock } from "../picking";
import {
  chooseOption,
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
  typeAndEnter,
  typeTextContent,
  typeValue,
} from "../user-actions";

import { writeSeveralValues } from "./write-several-values";

import type { DrawingShape, Point } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner at the opening view (`OPENING_VIEW`): the drawing's point (x, y) is the window's (x + 140, y + 90).

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
 * Places a text by a click, then types `Text` in it and leaves its Content field (US-075).
 *
 * @param point - the point of the drawing clicked
 */
function placeText(point: Point): void {
  placeShape("text", point);
  typeTextContent("Text");
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
 * Places a text, then puts it in a box 100 × 20 at (47, 61) and turns it by 23° (ADR-0057).
 *
 * @param point - the point of the drawing clicked to place it
 */
function placeTurnedText(point: Point): void {
  placeText(point);
  chooseOption("text-fit", "box");
  typeValue("text-box-width", "100");
  typeValue("text-box-height", "20");
  typeValue("shape-x", "47");
  typeValue("shape-y", "61");
  typeValue("angle", "23");
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

  it("[F38.AC1] shows A and B as X Mixed, Y 20, W Mixed, H Mixed", () => {
    drawAAndB();
    pick({ x: 50, y: 40 }, false);
    pick({ x: 220, y: 40 }, true);

    expect(readSeveralFields()).toEqual(["[Mixed]", "20", "[Mixed]", "[Mixed]"]);
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
    placeText({ x: 250, y: 140 });

    const y = readValue("shape-y");

    placeText({ x: 450, y: 140 });
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

  it("[F38.AC1] shows a text turned by 23° by its straight box, alone, picked and among several", () => {
    // ADR-0057's box 100 × 20 at (47, 61), turned by 23° about its middle (97, 71): half spans
    // (100 cos 23° + 20 sin 23°) / 2 = (92.05049 + 7.81462) / 2 = 49.93255 and
    // (100 sin 23° + 20 cos 23°) / 2 = (39.07311 + 18.41010) / 2 = 28.74161, so its straight box
    // starts at (97 − 49.93255, 71 − 28.74161) = (47.06745, 42.2584), as a shape's (AUD-040).
    // The rectangle the page opens with, under the text, removed first.
    pick({ x: 60, y: 40 }, false);
    pressKey("body", "Delete");
    placeTurnedText({ x: 250, y: 140 });

    const alone = [readValue("shape-x"), readValue("shape-y")];

    pressKey("body", "Escape");
    pick({ x: 97, y: 71 }, false);

    const picked = [readValue("shape-x"), readValue("shape-y")];

    placeTurnedText({ x: 450, y: 300 });
    selectAll();

    expect(alone).toEqual(["47.06745", "42.2584"]);
    expect(picked).toEqual(["47.06745", "42.2584"]);
    expect(readSeveralFields()).toEqual(["47.06745", "42.2584", "100", "20"]);
  });
});

/**
 * Reads the shapes kept in the drawing, every one kept below while several are selected (F37).
 *
 * @returns each shape, in the order drawn
 */
function readDrawing(): readonly DrawingShape[] {
  return readStoredDrawing(document, DRAWING_KEYS.below).map(({ shape }) => shape);
}

/**
 * Reads the box of each shape kept, its position and its own size, the first drawn first.
 *
 * @returns x, y, width and height of each, a text's x and y alone
 */
function readBoxes(): readonly (readonly number[])[] {
  return readDrawing().map((shape) =>
    "content" in shape ? [shape.x, shape.y] : [shape.x, shape.y, shape.width, shape.height],
  );
}

/**
 * Asserts the box of each shape kept, within `EPSILON`: a rounded rectangle's x typed keeps the
 * rounding error of its arcs' extremes (ADR-0003).
 *
 * @param expected - x, y, width and height of each, computed by hand
 */
function expectBoxes(expected: readonly (readonly number[])[]): void {
  const boxes = readBoxes();

  expect(boxes).toHaveLength(expected.length);

  for (const [index, box] of boxes.entries()) {
    for (const [at, value] of box.entries()) {
      expect(value).toBeCloseTo(expected[index]?.[at] ?? NaN, 9);
    }
  }
}

/** Draws A and B, then selects both, B by Shift + click. */
function selectAAndB(): void {
  drawAAndB();
  pick({ x: 50, y: 40 }, false);
  pick({ x: 220, y: 40 }, true);
}

describe("a value typed for several shapes, relative or fixed (US-071)", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F38.AC2] makes A 150 wide and B 90 for +50 in W, W still Mixed", () => {
    selectAAndB();
    typeAndEnter("several-width", "+50");

    expectBoxes([
      [10, 20, 150, 50],
      [200, 20, 90, 40],
    ]);

    expect(readSeveralFields()).toEqual(["[Mixed]", "20", "[Mixed]", "[Mixed]"]);
  });

  it("[F38.AC2] moves A to x −10 and B to x 180 for -20 in X", () => {
    selectAAndB();
    typeAndEnter("several-x", "-20");

    expectBoxes([
      [-10, 20, 100, 50],
      [180, 20, 40, 40],
    ]);
  });

  it("[F38.AC2] makes both 460 wide for 460 in W, then puts both at x 50 for 50 in X", () => {
    selectAAndB();
    typeAndEnter("several-width", "460");
    typeAndEnter("several-x", "50");

    expectBoxes([
      [50, 20, 460, 50],
      [50, 20, 460, 40],
    ]);

    expect(readSeveralFields()).toEqual(["50", "20", "460", "[Mixed]"]);
  });

  it("[F38.AC2] applies a value when the field is left, not while it is typed", () => {
    selectAAndB();
    typeValue("several-height", "70");

    expectBoxes([
      [10, 20, 100, 50],
      [200, 20, 40, 40],
    ]);

    expect(FIELDS.map((id) => selectInput(id).readOnly)).toEqual([false, false, false, false]);
  });

  it("[F38.AC2] moves A turned by 30° by its straight box, its x then 55.80127", () => {
    // A's straight box starts at x 4.19873: 50 typed moves it by 45.80127, from 10 to 55.80127.
    turnA();
    placeShape("rectangle", { x: 400, y: 300 });
    typeBox(["200", "20", "40", "40"]);
    selectAll();
    typeAndEnter("several-x", "50");

    const [turned, straight] = readBoxes();

    expect(turned?.[0]).toBeCloseTo(55.80127, 5);
    expect(straight?.[0]).toBe(50);
    expect(readSeveralFields()[0]).toBe("50");
  });

  it("[F38.AC2] keeps the size of a text fitting its content, the rectangle made 460 wide", () => {
    // The rectangle the page opens with, 120 × 80 at (0, 0), then a text placed beside it.
    placeText({ x: 250, y: 140 });
    selectAll();

    const [, text] = readDrawing();

    typeAndEnter("several-width", "460");

    expect(text).toMatchObject({ isInBox: false });
    expect(readDrawing()).toEqual([expect.objectContaining({ width: 460, x: 0 }), text]);
  });

  it("[F38.AC2] changes nothing for a value refused, the fields shown again", () => {
    selectAAndB();

    for (const typed of ["50.5", "abc", "", "-50"]) {
      // -50: B, 40 wide, would be −10 wide; A is not changed either.
      typeAndEnter("several-width", typed);

      expectBoxes([
        [10, 20, 100, 50],
        [200, 20, 40, 40],
      ]);

      expect(readSeveralFields()).toEqual(["[Mixed]", "20", "[Mixed]", "[Mixed]"]);
    }
  });

  it("[F38.AC2] leaves a text never measured as it is, the rectangle made 460 wide", () => {
    const never = {
      kind: "text",
      shape: { ...HELLO_TEXT, content: "Never drawn", x: 500, y: 500 },
    };

    writeFontSession(document, { ...readFontSession(document), kept: [] });
    writeStoredDrawing(document, DRAWING_KEYS.below, [never]);
    selectAll();
    typeAndEnter("several-width", "460");

    // The rectangle the page opens with: 120 × 80 at (0, 0).
    expect(readDrawing()).toEqual([never.shape, expect.objectContaining({ width: 460 })]);
  });

  it("[F38.AC2] moves a text drawn by 50 in X, a text never measured left as it is", () => {
    const never = {
      kind: "text",
      shape: { ...HELLO_TEXT, content: "Never drawn", x: 500, y: 500 },
    };

    // The rectangle the page opens with, 120 × 80 at (0, 0), a text placed beside it, then a
    // text kept by hand, never drawn: no family measures it, and none did before.
    placeText({ x: 250, y: 140 });
    writeFontSession(document, { ...readFontSession(document), kept: [] });
    selectAll();

    const [, text] = readDrawing();

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      ...readStoredDrawing(document, DRAWING_KEYS.below),
      never,
    ]);

    selectAll();
    typeAndEnter("several-x", "50");

    // At 0° a text's straight box is its frame, its x its position: both put at x 50.
    expect(readDrawing()).toEqual([
      expect.objectContaining({ width: 120, x: 50 }),
      { ...text, x: 50 },
      never.shape,
    ]);
  });

  it("[F38.AC2] leaves a shape not selected as it is", () => {
    // C, 30 × 30 at (400, 300), drawn last and left out of the selection.
    drawAAndB();
    placeShape("rectangle", { x: 400, y: 300 });
    typeBox(["400", "300", "30", "30"]);
    pick({ x: 50, y: 40 }, false);
    pick({ x: 220, y: 40 }, true);
    typeAndEnter("several-width", "460");

    expectBoxes([
      [10, 20, 460, 50],
      [200, 20, 460, 40],
      [400, 300, 30, 30],
    ]);
  });

  it("[F38.AC2] still draws a text moved whose font is no longer read, by its kept metrics", () => {
    placeText({ x: 250, y: 140 });
    selectAll();
    // No family measures it any more: it is drawn by the metrics it was last drawn with.
    writeFontSession(document, { ...readFontSession(document), kept: [] });
    typeAndEnter("several-x", "+10");

    const [, text] = readDrawing();

    expect(
      text !== undefined && isDrawnText(text) ? readTextBlock(document, text) : undefined,
    ).toBeDefined();

    expect(readSeveralFields()[1]).not.toBe("[]");
  });
});
