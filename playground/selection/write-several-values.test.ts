// @vitest-environment happy-dom
import { beforeEach, describe, expect, it, vi } from "vitest";

import { isDrawnText } from "../../src";
import { HELLO_TEXT } from "../../tests";
import { readFontSession, writeFontSession } from "../fonts";
import { PAGE_HISTORIES } from "../history";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";
import { selectElement, writeEditedKind } from "../page";
import { readTextBlock } from "../picking";
import {
  chooseOption,
  clickAt,
  clickElement,
  commitField,
  openServedPage,
  pressAndClick,
  pressKey,
  pressPointer,
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
import type { KeptShape } from "../kept-shapes";

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

  typeAndEnter("width", width);
  typeAndEnter("height", height);
  typeAndEnter("shape-x", x);
  typeAndEnter("shape-y", y);
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
  typeAndEnter("text-box-width", "100");
  typeAndEnter("text-box-height", "20");
  typeAndEnter("shape-x", "47");
  typeAndEnter("shape-y", "61");
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
    typeAndEnter("shape-y", "30");
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
 * Reads the shapes kept in the drawing, every one kept below while several are selected (F37),
 * the one edited and those above it too while one is (`readKeptDrawing`).
 *
 * @returns each shape, in the order drawn
 */
function readDrawing(): readonly DrawingShape[] {
  return readKeptDrawing(document).map(({ shape }) => shape);
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

/**
 * Presses a key in a field of several shapes, then releases it, as the browser sends them:
 * bubbling, cancelable.
 *
 * @param id - the field, without `#`
 * @param init - the key, as `KeyboardEvent.key` names it, and the modifiers held
 * @returns the key pressed, telling whether its default was prevented
 */
function pressIn(id: string, init: KeyboardEventInit): KeyboardEvent {
  const field = selectPageElement(`#${id}`);
  const down = new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true });

  field.dispatchEvent(down);
  field.dispatchEvent(new KeyboardEvent("keyup", { ...init, bubbles: true, cancelable: true }));

  return down;
}

/** Presses Ctrl + Z outside the fields, which undoes a step (US-074). */
function undo(): void {
  pressIn("canvas", { ctrlKey: true, key: "z" });
}

/**
 * Reads the drawing of the history's present, what the last step recorded.
 *
 * @returns the shapes the present holds; none without a history
 */
function readPresent(): readonly KeptShape[] {
  return PAGE_HISTORIES.get(document)?.present.drawing ?? [];
}

describe("the arrows in a field of several shapes (US-081)", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    selectAAndB();
  });

  it("[F43.AC3] makes A 101 wide and B 41 for Up in W, W still Mixed, in one step", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });

    const up = pressIn("several-width", { key: "ArrowUp" });

    // The step taken once no arrow follows within 500 ms (US-081, T2).
    vi.advanceTimersByTime(500);
    vi.useRealTimers();

    // 100 + 1 and 40 + 1: each shape's own width stepped, as +1 typed.
    expectBoxes([
      [10, 20, 101, 50],
      [200, 20, 41, 40],
    ]);

    expect(up.defaultPrevented).toBe(true);
    expect(readSeveralFields()).toEqual(["[Mixed]", "20", "[Mixed]", "[Mixed]"]);
    expect(readPresent()).toEqual(readStoredDrawing(document, DRAWING_KEYS.below));

    undo();

    expectBoxes([
      [10, 20, 100, 50],
      [200, 20, 40, 40],
    ]);
  });

  it("[F43.AC3] makes them 110 and 50 for Shift + Up, then 99 and 39 for Down from 100 and 40", () => {
    pressIn("several-width", { key: "ArrowUp", shiftKey: true });

    // 100 + 10 and 40 + 10.
    expectBoxes([
      [10, 20, 110, 50],
      [200, 20, 50, 40],
    ]);

    undo();
    pressIn("several-width", { key: "ArrowDown" });

    // 100 − 1 and 40 − 1.
    expectBoxes([
      [10, 20, 99, 50],
      [200, 20, 39, 40],
    ]);

    pressIn("several-width", { key: "ArrowDown", shiftKey: true });

    // 99 − 10 and 39 − 10.
    expectBoxes([
      [10, 20, 89, 50],
      [200, 20, 29, 40],
    ]);
  });

  it("[F43.AC3] moves both 1 down for Up in Y, which then shows 21, and 1 right for Up in X", () => {
    pressIn("several-y", { key: "ArrowUp" });

    // 20 + 1 for both, the y axis pointing down.
    expectBoxes([
      [10, 21, 100, 50],
      [200, 21, 40, 40],
    ]);

    expect(readSeveralFields()).toEqual(["[Mixed]", "21", "[Mixed]", "[Mixed]"]);

    pressIn("several-x", { key: "ArrowUp" });

    // 10 + 1 and 200 + 1: X goes right as a number field's Up adds 1.
    expectBoxes([
      [11, 21, 100, 50],
      [201, 21, 40, 40],
    ]);
  });

  it.each([
    { init: { ctrlKey: true, key: "ArrowUp" }, name: "Ctrl + Up" },
    { init: { key: "ArrowUp", metaKey: true }, name: "Cmd + Up" },
    { init: { altKey: true, key: "ArrowUp" }, name: "Alt + Up" },
    { init: { key: "ArrowLeft" }, name: "Left" },
    { init: { key: "ArrowRight" }, name: "Right" },
    { init: { key: "5" }, name: "5" },
  ])("[F43.AC3] leaves $name to the field, changing nothing", ({ init }) => {
    const down = pressIn("several-width", init);

    expectBoxes([
      [10, 20, 100, 50],
      [200, 20, 40, 40],
    ]);

    expect(down.defaultPrevented).toBe(false);
  });

  it("[F43.AC3] refuses Down in H for both when B is 0 high, no step taken", () => {
    pick({ x: 220, y: 40 }, false);
    typeAndEnter("height", "0");
    pick({ x: 50, y: 40 }, true);

    const present = readPresent();

    pressIn("several-height", { key: "ArrowDown" });

    // B's 0 − 1 is below 0: the value is refused for A as well, as -1 typed is (US-071).
    expectBoxes([
      [10, 20, 100, 50],
      [200, 20, 40, 0],
    ]);

    expect(readPresent()).toEqual(present);
    expect(readSeveralFields()).toEqual(["[Mixed]", "20", "[Mixed]", "[Mixed]"]);

    pressIn("several-height", { key: "ArrowUp" });

    // Up is taken: 50 + 1 and 0 + 1.
    expectBoxes([
      [10, 20, 100, 51],
      [200, 20, 40, 1],
    ]);
  });

  it("[F43.AC3] leaves a width of 100 000 at 100 000 for Up, capped (Q20)", () => {
    typeAndEnter("several-width", "100000");
    pressIn("several-width", { key: "ArrowUp" });

    expectBoxes([
      [10, 20, 100_000, 50],
      [200, 20, 100_000, 40],
    ]);

    pressIn("several-width", { key: "ArrowDown" });

    // Down is taken: 100 000 − 1.
    expectBoxes([
      [10, 20, 99_999, 50],
      [200, 20, 99_999, 40],
    ]);
  });
});

/**
 * Places, beside A, the epic's rectangle 120 × 80 at (0, 0), a hexagon by a click at (300, 50),
 * centred: 100 × 100 at (250, 0); then a round shape by a click at (500, 50), 100 × 100 at
 * (450, 0), typed 60 × 40, its top-left kept, being straight.
 */
function placeHexagonAndRound(): void {
  placeShape("polygon", { x: 300, y: 50 });
  placeShape("round", { x: 500, y: 50 });
  typeAndEnter("width", "60");
  typeAndEnter("height", "40");
}

/**
 * Applies 150 typed in W, the field then left by a press on the drawing as a mouse does: the
 * press, the field's `change` as it loses the focus, the release and the click.
 *
 * @param point - the point of the drawing pressed
 */
function applyWidthByClick(point: Point): void {
  const at = toWindow(point);

  typeValue("several-width", "150");
  pressPointer("#canvas", "pointerdown", { ...at, buttons: 1 });
  commitField("several-width");
  pressPointer("#canvas", "pointerup", { ...at, buttons: 0 });
  clickAt("#canvas", at);
}

describe("several shapes' width and height, the cases tried (US-082)", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F43.AC4] makes A, a hexagon and a round shape 150 wide, then 10 higher for +10 in H", () => {
    placeHexagonAndRound();
    selectAll();
    typeAndEnter("several-width", "150");

    expectBoxes([
      [0, 0, 150, 80],
      [250, 0, 150, 100],
      [450, 0, 150, 40],
    ]);

    expect(readSeveralFields()).toEqual(["[Mixed]", "0", "150", "[Mixed]"]);

    typeAndEnter("several-height", "+10");

    // 80 + 10, 100 + 10, 40 + 10.
    expectBoxes([
      [0, 0, 150, 90],
      [250, 0, 150, 110],
      [450, 0, 150, 50],
    ]);
  });

  it("[F43.AC4] takes -100 in W, 0 in H, then 30 in W, for each shape", () => {
    placeHexagonAndRound();
    selectAll();
    typeAndEnter("several-width", "150");
    // 150 − 100 = 50 for each.
    typeAndEnter("several-width", "-100");

    expectBoxes([
      [0, 0, 50, 80],
      [250, 0, 50, 100],
      [450, 0, 50, 40],
    ]);

    typeAndEnter("several-height", "0");
    typeAndEnter("several-width", "30");

    expectBoxes([
      [0, 0, 30, 0],
      [250, 0, 30, 0],
      [450, 0, 30, 0],
    ]);

    expect(readSeveralFields()).toEqual(["[Mixed]", "0", "30", "0"]);
  });

  it("[F43.AC4] keeps A's turned top-left corner at (28.03848, −24.64102), typed 150 wide", () => {
    // A turned by 30° about its middle (60, 40): its top-left corner is
    // (60 − 60 cos 30° + 40 sin 30°, 40 − 60 sin 30° − 40 cos 30°) = (28.03848, −24.64102).
    // 150 wide, its middle is that corner + (75 cos 30° − 40 sin 30°, 75 sin 30° + 40 cos 30°)
    // = (72.99038, 47.5), its position (72.99038 − 75, 47.5 − 40) = (−2.00962, 7.5).
    typeValue("angle", "30");
    placeHexagonAndRound();
    selectAll();
    typeAndEnter("several-width", "150");

    const [turned, hexagon, round] = readBoxes();

    expect(turned?.[0]).toBeCloseTo(-2.00962, 5);
    expect(turned?.[1]).toBeCloseTo(7.5, 9);
    expect(turned?.[2]).toBe(150);

    expect([hexagon, round]).toEqual([
      [250, 0, 150, 100],
      [450, 0, 150, 40],
    ]);
  });

  it("[F43.AC4] sets a text turned by 23° in a box 100 × 20 to 150 × 60, its turned corner kept", () => {
    // Its middle (97, 71), its top-left corner (97 − 50 cos 23° + 10 sin 23°,
    // 71 − 50 sin 23° − 10 cos 23°) = (54.88207, 42.25840); 150 × 60, its middle that corner +
    // (75 cos 23° − 30 sin 23°, 75 sin 23° + 30 cos 23°) = (112.19800, 99.17838), its position
    // (112.19800 − 75, 99.17838 − 30) = (37.19800, 69.17838).
    placeHexagonAndRound();
    placeTurnedText({ x: 450, y: 300 });
    selectAll();
    typeAndEnter("several-width", "150");
    typeAndEnter("several-height", "60");

    const text = readDrawing()[3];

    expect(text).toMatchObject({ boxHeight: 60, boxWidth: 150 });
    expect(text?.x).toBeCloseTo(37.198, 5);
    expect(text?.y).toBeCloseTo(69.17838, 5);
  });

  it.each([
    {
      name: "Enter",
      way: (): void => {
        typeAndEnter("several-width", "150");
      },
    },
    {
      name: "Tab",
      way: (): void => {
        typeValue("several-width", "150");
        commitField("several-width");
        selectInput("several-height").focus();
      },
    },
    {
      name: "a click on the drawing's empty place",
      way: (): void => {
        applyWidthByClick({ x: 600, y: 400 });
      },
    },
    {
      name: "a click on A",
      way: (): void => {
        applyWidthByClick({ x: 60, y: 40 });
      },
    },
    {
      name: "a click on the hexagon",
      way: (): void => {
        applyWidthByClick({ x: 300, y: 50 });
      },
    },
  ])("[F43.AC4] gives the same widths for W applied by $name", ({ way }) => {
    placeHexagonAndRound();
    selectAll();
    way();

    expectBoxes([
      [0, 0, 150, 80],
      [250, 0, 150, 100],
      [450, 0, 150, 40],
    ]);
  });
});
