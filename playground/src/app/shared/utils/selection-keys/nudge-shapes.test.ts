import { isDrawnText, shapePanelValues } from "editor";
// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  HELLO_TEXT,
  clickElement,
  dispatchDrawingPointer,
  openPage,
  readShapeModel,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
} from "../../../../../testing";
import { readFontSession, writeFontSession } from "../fonts";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readSelectedShapes,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";

import type { PointerKeys } from "../../../../../testing";
import type { Point, Shape } from "editor";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** Shift held. */
const SHIFT: PointerKeys = { altKey: false, shiftKey: true };

/** Where A is pressed, inside it: A's box runs from (10, 20) to (110, 70). */
const IN_A: Point = { x: 50, y: 40 };

/**
 * Presses a key on an element of the page, as the browser sends it: bubbling, cancelable.
 *
 * @param selector - CSS selector of the element the focus is on, `body` for none
 * @param key - the key, as `KeyboardEvent.key` names it
 * @param init - the modifiers held and whether the key repeats
 * @returns the event, telling whether its default was prevented
 */
function pressOn(selector: string, key: string, init: KeyboardEventInit): KeyboardEvent {
  const event = new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true, key });

  selectPageElement(selector).dispatchEvent(event);

  return event;
}

/**
 * Presses an arrow with the focus outside the fields.
 *
 * @param key - the arrow, `ArrowRight`
 * @param isShift - whether Shift is held
 * @returns the event, telling whether its default was prevented
 */
function pressArrow(key: string, isShift: boolean): KeyboardEvent {
  return pressOn("body", key, { shiftKey: isShift });
}

/**
 * Clicks A as a mouse does: pressed and released at (50, 40).
 *
 * @param keys - the keys held
 */
function clickA(keys: PointerKeys): void {
  dispatchDrawingPointer("pointerdown", IN_A, keys);
  dispatchDrawingPointer("pointerup", IN_A, keys);
  dispatchDrawingPointer("click", IN_A, keys);
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
 * Draws A, 100 × 50 at (10, 20), turned about its middle (60, 45) by an angle, then B, 100 × 50
 * at (200, 20), turned by 30° about its middle (250, 45), its straight box at
 * (194.19873, −1.65064); B edited.
 *
 * @param angle - A's angle: 0 for A, 30 for A′
 */
function drawAAndB(angle: string): void {
  typeRectangle(["10", "20", "100", "50", angle]);
  clickElement("tool-rectangle");
  dispatchDrawingPointer("click", { x: 400, y: 300 }, NO_KEY);
  typeRectangle(["200", "20", "100", "50", "30"]);
}

/**
 * Reads a shape of the drawing, in the order drawn.
 *
 * @param index - its index
 * @returns the shape a tool made, at that index
 * @throws {TypeError} when no shape a tool made is at that index
 */
function readShape(index: number): Shape {
  const shape = readKeptDrawing(document)[index]?.shape;

  if (shape === undefined || isDrawnText(shape)) {
    throw new TypeError(`No shape ${index}.`);
  }

  return shape;
}

/**
 * Asserts where a shape's straight box starts, to 5 decimals, as the panel shows it.
 *
 * @param shape - a shape of the drawing
 * @param start - the top-left of its straight box, computed by hand
 */
function expectStraightBoxAt(shape: Shape, start: Point): void {
  const { x, y } = shapePanelValues(shape);

  expect(x).toBeCloseTo(start.x, 5);
  expect(y).toBeCloseTo(start.y, 5);
}

describe("the arrows nudge the shapes selected", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    drawAAndB("0");
  });

  it.each([
    // (10, 20) + (1, 0), + (10, 0), + (0, −1), + (−10, 0).
    { at: ["11", "20"], isShift: false, key: "ArrowRight" },
    { at: ["20", "20"], isShift: true, key: "ArrowRight" },
    { at: ["10", "19"], isShift: false, key: "ArrowUp" },
    { at: ["0", "20"], isShift: true, key: "ArrowLeft" },
    { at: ["9", "20"], isShift: false, key: "ArrowLeft" },
    { at: ["10", "30"], isShift: true, key: "ArrowDown" },
  ])("[F39.AC4] moves A to $at on $key, Shift $isShift, the panel showing it", (each) => {
    clickA(NO_KEY);

    const event = pressArrow(each.key, each.isShift);

    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(each.at);
    expect(readShapeModel()).toMatchObject({ angle: 0, height: 50, width: 100 });
    expect(readSelectedShapes(document)).toEqual([0]);
    expect(event.defaultPrevented).toBe(true);
  });

  it("[F46.AC1] moves the shape the panel edits, B, its straight box landed on (195, −2)", () => {
    pressArrow("ArrowRight", false);

    // 194.19873 + 1 = 195.19873 rounds to 195, a shift of 0.80127; −1.65064 rounds to −2, a
    // shift of −0.34936: its position (200, 20) to (200.80127, 19.65064), its angle kept.
    expect(readValue("shape-x")).toBe("195");
    expect(readValue("shape-y")).toBe("-2");
    expect(readShapeModel()).toMatchObject({ angle: 30 });
    expect(readShapeModel().x).toBeCloseTo(200.80127, 5);
    expect(readShapeModel().y).toBeCloseTo(19.65064, 5);
  });

  it("[F46.AC1] moves A and B by their dashed box landed on (11, −2), both still selected", () => {
    clickA(SHIFT);
    pressArrow("ArrowRight", false);

    // The dashed box from (10, −1.65064), + (1, 0) = (11, −1.65064), lands on (11, −2): both go
    // by (1, −0.34936), A from (10, 20) to (11, 19.65064), B's straight box to (195.19873, −2).
    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expect(readShape(0).x).toBe(11);
    expect(readShape(0).y).toBeCloseTo(19.65064, 5);
    expectStraightBoxAt(readShape(1), { x: 195.19873, y: -2 });
  });

  it("[F39.AC4] moves A one step at each key the browser sends, its repeats included", () => {
    clickA(NO_KEY);
    pressOn("body", "ArrowRight", {});
    pressOn("body", "ArrowRight", { repeat: true });
    pressOn("body", "ArrowRight", { repeat: true });

    // 10 + 1 + 1 + 1.
    expect(readShapeModel()).toMatchObject({ x: 13, y: 20 });
  });

  it("[F39.AC4] leaves a text never measured where it is, the others moved", () => {
    const never = { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn", x: 5, y: -50 } };

    clickA(SHIFT);
    writeFontSession(document, { ...readFontSession(document), kept: [] });

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      ...readStoredDrawing(document, DRAWING_KEYS.below),
      never,
    ]);

    pressOn("body", "a", { ctrlKey: true });
    pressArrow("ArrowDown", true);

    // The dashed box of A and B from (10, −1.65064), + (0, 10) = (10, 8.34936), lands on (10, 8):
    // A from (10, 20) to (10, 29.65064); the text kept at (5, −50).
    expect(readSelectedShapes(document)).toEqual([0, 1, 2]);
    expect(readShape(0).x).toBe(10);
    expect(readShape(0).y).toBeCloseTo(29.65064, 5);
    expect(readKeptDrawing(document)[2]?.shape).toEqual(never.shape);
  });

  it.each(["#shape-x", "#width", 'input[name="anchor"]', "#color-hue", "#color-plane"])(
    "[F39.AC4] leaves the arrow to %s, which has the focus, no shape moved",
    (selector) => {
      clickA(NO_KEY);
      pressOn(selector, "ArrowRight", {});

      expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
    },
  );

  it.each([{ ctrlKey: true }, { metaKey: true }, { altKey: true }])(
    "[F39.AC4] leaves the arrow to the browser with %o, no shape moved",
    (init) => {
      clickA(NO_KEY);

      const event = pressOn("body", "ArrowRight", init);

      expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
      expect(event.defaultPrevented).toBe(false);
    },
  );

  it("[F39.AC4] leaves the arrow to the browser with no shape selected", () => {
    pressOn("body", "Escape", {});

    const event = pressArrow("ArrowRight", false);

    expect(readShape(0)).toMatchObject({ x: 10, y: 20 });
    expect(readShape(1)).toMatchObject({ x: 200, y: 20 });
    expect(event.defaultPrevented).toBe(false);
  });

  it("[F39.AC4] leaves the arrow to the browser for a shape refused in the panel", () => {
    typeAndEnter("angle", "");

    const event = pressArrow("ArrowRight", false);

    expect(readShape(0)).toMatchObject({ x: 10, y: 20 });
    expect(readValue("angle")).toBe("");
    expect(event.defaultPrevented).toBe(false);
  });

  it("[F39.AC4] leaves the arrow to the browser in Points mode, no shape moved", () => {
    clickA(NO_KEY);
    clickElement("mode-points");

    const event = pressArrow("ArrowRight", false);

    expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
    expect(event.defaultPrevented).toBe(false);
  });

  it("[F39.AC4] leaves a key other than an arrow to the others", () => {
    clickA(NO_KEY);

    const event = pressOn("body", "PageDown", {});

    expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
    expect(event.defaultPrevented).toBe(false);
  });

  it("[F39.AC4] moves nothing while a press with the Selector moves A, landed at the release", () => {
    clickA(NO_KEY);
    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);
    pressArrow("ArrowRight", false);

    // Still where the pointer takes it, (10, 20) + (30.6, 19.4), not back at the press plus 1.
    expect(readShapeModel().x).toBeCloseTo(40.6, 9);
    expect(readShapeModel().y).toBeCloseTo(39.4, 9);

    dispatchDrawingPointer("pointerup", { x: 80.6, y: 59.4 }, NO_KEY);

    // Landed as dragged by (30.6, 19.4) alone: (41, 39).
    expect(readShapeModel()).toMatchObject({ x: 41, y: 39 });
  });
});

describe("the arrows land a turned shape on integers", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    drawAAndB("30");
    clickA(NO_KEY);
  });

  it.each([
    // A′'s straight box at (60 − 55.80127, 45 − 46.65064) = (4.19873, −1.65064), moved then
    // landed: 5.19873 rounds to 5, 3.19873 to 3, −5.80127 to −6, and Y −1.65064 to −2; Up takes
    // Y to −2.65064, rounding to −3, X 4.19873 to 4.
    { isShift: false, key: "ArrowRight", x: "5", y: "-2" },
    { isShift: false, key: "ArrowLeft", x: "3", y: "-2" },
    { isShift: true, key: "ArrowLeft", x: "-6", y: "-2" },
    { isShift: false, key: "ArrowUp", x: "4", y: "-3" },
  ])("[F46.AC1] lands A′ from (4.19873, −1.65064) on ($x, $y) for $key, Shift $isShift", (each) => {
    expect(readValue("shape-x")).toBe("4.19873");

    pressArrow(each.key, each.isShift);

    expect(readValue("shape-x")).toBe(each.x);
    expect(readValue("shape-y")).toBe(each.y);
  });

  it("[F46.AC1] moves A′ on integers by exactly 1 at the next arrow, as a shape on integers", () => {
    const body = selectPageElement("body");

    for (const [key, isShift] of [
      ["ArrowRight", false],
      ["ArrowRight", false],
      ["ArrowDown", true],
    ] as const) {
      pressArrow(key, isShift);
      body.dispatchEvent(new KeyboardEvent("keyup", { bubbles: true, key }));
    }

    // Each pressed and released: landed on (5, −2), then (6, −2), then (6, 8), 1 across, then 10
    // down, exactly.
    expect(readValue("shape-x")).toBe("6");
    expect(readValue("shape-y")).toBe("8");
  });

  it("[F46.AC1] lands A′ held on Right for 30 keys one step of the history, Ctrl + Z giving it back", () => {
    const body = selectPageElement("body");
    const sent = Array.from({ length: 30 }).keys();

    for (const index of sent) {
      body.dispatchEvent(
        new KeyboardEvent("keydown", {
          bubbles: true,
          cancelable: true,
          key: "ArrowRight",
          repeat: index > 0,
        }),
      );
    }

    body.dispatchEvent(new KeyboardEvent("keyup", { bubbles: true, key: "ArrowRight" }));

    // Landed on (5, −2) at the first key, then 29 × 1 across: (34, −2).
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["34", "-2"]);

    pressOn("body", "z", { ctrlKey: true });

    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["4.19873", "-1.65064"]);
  });

  it("[F39.AC4] lands A′ moved by Right at the next drag, its X 5 landing on 9", () => {
    pressArrow("ArrowRight", false);
    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 53.7, y: 40 }, NO_KEY);
    dispatchDrawingPointer("pointerup", { x: 53.7, y: 40 }, NO_KEY);

    // 3.7 pixels, beyond 3: a drag. 5 + 3.7 = 8.7 rounds to 9; −2 + 0 stays −2.
    expect(readValue("shape-x")).toBe("9");
    expect(readValue("shape-y")).toBe("-2");
  });
});
