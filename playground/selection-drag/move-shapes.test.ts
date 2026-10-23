// @vitest-environment happy-dom
import { beforeEach, describe, expect, it, vi } from "vitest";

import { isDrawnText, shapePanelValues } from "../../src";
import { HELLO_TEXT } from "../../tests";
import { readFontSession, writeFontSession } from "../fonts";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readSelectedShapes,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";
import {
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readShapeModel,
  readValue,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

import type { Point, Shape } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** Shift held. */
const SHIFT: PointerKeys = { altKey: false, shiftKey: true };

/** Where A is pressed, inside it: A's box runs from (10, 20) to (110, 70). */
const IN_A: Point = { x: 50, y: 40 };

/** Where B is pressed: its middle, (250, 45). */
const IN_B: Point = { x: 250, y: 45 };

/**
 * Releases the pointer, then the `click` the browser fires on the same element.
 *
 * @param at - the point released
 * @param keys - the keys held at the release
 */
function release(at: Point, keys: PointerKeys): void {
  dispatchDrawingPointer("pointerup", at, keys);
  dispatchDrawingPointer("click", at, keys);
}

/**
 * Clicks A as a mouse does: pressed and released at (50, 40).
 *
 * @param keys - the keys held
 */
function clickA(keys: PointerKeys): void {
  dispatchDrawingPointer("pointerdown", IN_A, keys);
  release(IN_A, keys);
}

/**
 * Drags on the drawing by an offset, then releases, the same keys held throughout.
 *
 * @param from - the point pressed
 * @param by - how far the pointer goes
 * @param keys - the keys held at the press, the move and the release
 */
function dragBy(from: Point, by: Point, keys: PointerKeys): void {
  const to = { x: from.x + by.x, y: from.y + by.y };

  dispatchDrawingPointer("pointerdown", from, keys);
  dispatchDrawingPointer("pointermove", to, keys);
  release(to, keys);
}

/**
 * Types the box of the rectangle the panel edits, sharp, at an angle.
 *
 * @param box - its x, y, width, height and angle
 */
function typeRectangle(box: readonly [string, string, string, string, string]): void {
  const [x, y, width, height, angle] = box;

  typeValue("angle", "0");
  typeValue("radius", "0");
  typeValue("width", width);
  typeValue("height", height);
  typeValue("shape-x", x);
  typeValue("shape-y", y);
  typeValue("angle", angle);
}

/**
 * Draws A, 100 × 50 at (10, 20), straight, then a second rectangle 100 × 50 at (200, 20), turned
 * about its middle (250, 45): B by 30°, D by 0°; the second edited.
 *
 * @param angle - the second's angle
 */
function drawTwo(angle: string): void {
  typeRectangle(["10", "20", "100", "50", "0"]);
  clickElement("tool-rectangle");
  dispatchDrawingPointer("click", { x: 400, y: 300 }, NO_KEY);
  typeRectangle(["200", "20", "100", "50", angle]);
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
 * Asserts where a shape's position is, to 5 decimals, as the panel shows it.
 *
 * @param shape - a shape of the drawing
 * @param position - its position computed by hand
 */
function expectAt(shape: Shape, position: Point): void {
  expect(shape.x).toBeCloseTo(position.x, 5);
  expect(shape.y).toBeCloseTo(position.y, 5);
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

/**
 * Reads a path of the layer above the drawing.
 *
 * @param name - its class
 * @returns its path data; undefined for none
 */
function readOverlayPath(name: string): string | undefined {
  return document.querySelector(`#overlay svg path.${name}`)?.getAttribute("d") ?? undefined;
}

describe("a drag moves the shapes selected (US-072)", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    drawTwo("30");
  });

  it("[F39.AC2] draws A where the pointer takes it, at (40.6, 39.4), with its band", () => {
    clickA(NO_KEY);

    const band = readOverlayPath("selection");

    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);

    // (10, 20) + (30.6, 19.4) = (40.6, 39.4), not landed yet.
    expectAt(readShapeModel(), { x: 40.6, y: 39.4 });
    expect(document.querySelectorAll("#overlay svg path.selection")).toHaveLength(1);
    expect(readOverlayPath("selection")).not.toBe(band);
  });

  it("[F39.AC2] lands A at (41, 39) at the release, its size and radius kept", () => {
    clickA(NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 }, NO_KEY);

    // 40.6 rounds to 41, 39.4 to 39.
    expect(readShapeModel()).toMatchObject({ height: 50, width: 100, x: 41, y: 39 });
    expect(readValue("radius")).toBe("0");
    expect(readValue("shape-x")).toBe("41");
    expect(readValue("shape-y")).toBe("39");
    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readKeptDrawing(document)).toHaveLength(2);
  });

  it("[F39.AC2] lands B's straight box at (205, 2), its position (210.80127, 23.65064), 30°", () => {
    dragBy(IN_B, { x: 10.6, y: 3.2 }, NO_KEY);

    // 194.19873 + 10.6 = 204.79873 rounds to 205, −1.65064 + 3.2 = 1.54936 to 2: a shift of
    // (10.80127, 3.65064) from (200, 20).
    expectAt(readShapeModel(), { x: 210.80127, y: 23.65064 });
    expect(readShapeModel().angle).toBe(30);
    expect(readValue("shape-x")).toBe("205");
    expect(readValue("shape-y")).toBe("2");
  });

  it("[F39.AC2] moves A and B by one shift, their dashed box's top-left landing on (21, 2)", () => {
    clickA(SHIFT);
    dragBy(IN_A, { x: 10.6, y: 3.2 }, NO_KEY);

    // (10, −1.65064) + (10.6, 3.2) = (20.6, 1.54936) rounds to (21, 2): a shift of (11, 3.65064),
    // A to (21, 23.65064), B to (211, 23.65064), its straight box to (205.19873, 2).
    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expectAt(readShape(0), { x: 21, y: 23.65064 });
    expectAt(readShape(1), { x: 211, y: 23.65064 });
    expectStraightBoxAt(readShape(1), { x: 205.19873, y: 2 });
  });

  it("[F39.AC3] selects A alone, pressed while B is selected, and moves it alone", () => {
    dragBy(IN_A, { x: 30.6, y: 19.4 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readShapeModel()).toMatchObject({ x: 41, y: 39 });
    expectAt(readShape(1), { x: 200, y: 20 });
  });

  it("[F39.AC3] adds B to A, pressed with Shift, both moved by (31, 19.65064)", () => {
    clickA(NO_KEY);
    // Shift held at the press only, so that the move keeps both axes.
    dispatchDrawingPointer("pointerdown", IN_B, SHIFT);
    dispatchDrawingPointer("pointermove", { x: 280.6, y: 64.4 }, NO_KEY);
    release({ x: 280.6, y: 64.4 }, NO_KEY);

    // (10, −1.65064) + (30.6, 19.4) = (40.6, 17.74936) rounds to (41, 18): a shift of
    // (31, 19.65064), A to (41, 39.65064), B to (231, 39.65064), its straight box at
    // (225.19873, 18).
    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expectAt(readShape(0), { x: 41, y: 39.65064 });
    expectAt(readShape(1), { x: 231, y: 39.65064 });
    expectStraightBoxAt(readShape(1), { x: 225.19873, y: 18 });
  });

  it("[F39.AC3] keeps A and B, A pressed with Shift at the press only, both moved", () => {
    clickA(SHIFT);
    dispatchDrawingPointer("pointerdown", IN_A, SHIFT);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);
    release({ x: 80.6, y: 59.4 }, NO_KEY);

    // A pressed with Shift is one of the shapes selected: both stay selected. (10, −1.65064) +
    // (30.6, 19.4) = (40.6, 17.74936) rounds to (41, 18): a shift of (31, 19.65064).
    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expectAt(readShape(0), { x: 41, y: 39.65064 });
    expectAt(readShape(1), { x: 231, y: 39.65064 });
  });

  it("[F39.AC2] keeps A and B still down with Shift held throughout, only across landed", () => {
    clickA(SHIFT);
    dragBy(IN_A, { x: 30.6, y: 19.4 }, SHIFT);

    // Shift held at the release keeps the larger axis, across: 10 + 30.6 = 40.6 lands on 41, a
    // shift of 31; down is kept still, the dashed box's y −1.65064 not landed (2026-10-23).
    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expectAt(readShape(0), { x: 41, y: 20 });
    expectAt(readShape(1), { x: 231, y: 20 });
    expectStraightBoxAt(readShape(1), { x: 225.19873, y: -1.65064 });
  });

  it("[F39.AC2] leaves a text never measured where it is, selected, out of the anchor", () => {
    const never = { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn", x: 5, y: -50 } };

    // A and B kept below, then a text kept by hand at (5, −50), never drawn: no family measures
    // it, and none did before. Ctrl + A selects the three.
    clickA(SHIFT);
    writeFontSession(document, { ...readFontSession(document), kept: [] });

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      ...readStoredDrawing(document, DRAWING_KEYS.below),
      never,
    ]);

    selectPageElement("body").dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ctrlKey: true, key: "a" }),
    );

    dragBy(IN_A, { x: 30.6, y: 19.4 }, NO_KEY);

    // The anchor is A and B's, (10, −1.65064), not the text's (5, −50): a shift of (31, 19.65064).
    expect(readSelectedShapes(document)).toEqual([0, 1, 2]);
    expectAt(readShape(0), { x: 41, y: 39.65064 });
    expect(readKeptDrawing(document)[2]?.shape).toEqual(never.shape);
  });

  it("[F39.AC2] takes a release on the press's integer point for a click, A not moved", () => {
    clickA(SHIFT);
    // (50.3, 40.2) rounds to (50, 40), the press's point: a click, which selects A alone.
    dragBy(IN_A, { x: 0.3, y: 0.2 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
  });

  it("[F39.AC2] takes a release further than 3 pixels from the press for a move, A landing at (14, 20)", () => {
    clickA(NO_KEY);
    // (3.7, 0.2) away is √(13.69 + 0.04) = 3.705 pixels at 100 %, beyond 3 (SP-030's answer 8): a
    // move, A's (13.7, 20.2) landing on (14, 20).
    dragBy(IN_A, { x: 3.7, y: 0.2 }, NO_KEY);

    expect(readShapeModel()).toMatchObject({ x: 14, y: 20 });
  });

  it("[F39.AC2] keeps the axis of the larger displacement with Shift held, A landing at (41, 20)", () => {
    clickA(NO_KEY);
    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);
    release({ x: 80.6, y: 59.4 }, SHIFT);

    expect(readShapeModel()).toMatchObject({ x: 41, y: 20 });
  });

  it("[F39.AC2] frees the other axis again once Shift is released, A landing at (41, 39)", () => {
    clickA(NO_KEY);
    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, SHIFT);

    // Held while moving: across only, (40.6, 20).
    expectAt(readShapeModel(), { x: 40.6, y: 20 });

    release({ x: 80.6, y: 59.4 }, NO_KEY);

    expect(readShapeModel()).toMatchObject({ x: 41, y: 39 });
  });

  it("[F39.AC2] keeps a tie, (20, −20), across with Shift, A landing at (30, 20)", () => {
    clickA(NO_KEY);
    dragBy(IN_A, { x: 20, y: -20 }, SHIFT);

    expect(readShapeModel()).toMatchObject({ x: 30, y: 20 });
  });

  it("[F39.AC2] gives back A where it was and B selected for a drag cancelled", () => {
    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);
    dispatchDrawingPointer("pointercancel", { x: 80.6, y: 59.4 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([1]);
    expectAt(readShape(0), { x: 10, y: 20 });
    expect(readShapeModel().angle).toBe(30);
  });

  it.each([
    { init: {}, key: "Delete" },
    { init: {}, key: "Backspace" },
    { init: {}, key: "Escape" },
    { init: { ctrlKey: true }, key: "a" },
  ])("[F39.AC2] changes nothing on $key $init while A moves, A landed", ({ init, key }) => {
    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);

    selectPageElement("body").dispatchEvent(
      new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true, key }),
    );

    // A selected alone, still drawn where the pointer takes it, (40.6, 39.4); B kept.
    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readKeptDrawing(document)).toHaveLength(2);
    expectAt(readShape(0), { x: 40.6, y: 39.4 });

    release({ x: 80.6, y: 59.4 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readShapeModel()).toMatchObject({ x: 41, y: 39 });
  });

  it("[F39.AC2] makes the canvas take the pointer pressed on A, so that the move goes on beyond it", () => {
    const capture = vi.spyOn(selectPageElement("#canvas"), "setPointerCapture");

    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);

    expect(capture).toHaveBeenCalledWith(1);
  });

  it("[F39.AC2] moves nothing in Points mode", () => {
    clickA(NO_KEY);
    clickElement("mode-points");
    dragBy(IN_A, { x: 30.6, y: 19.4 }, NO_KEY);

    expect(readShapeModel()).toMatchObject({ x: 10, y: 20 });
  });

  it("[F39.AC2] outlines no shape under the pointer while moving, the dashed box following", () => {
    clickA(SHIFT);

    const box = readOverlayPath("selection-box");

    dispatchDrawingPointer("pointerdown", IN_A, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 80.6, y: 59.4 }, NO_KEY);

    expect(readOverlayPath("hover")).toBeUndefined();
    expect(document.querySelectorAll("#overlay svg path.selection")).toHaveLength(2);
    expect(readOverlayPath("selection-box")).not.toBe(box);
  });
});

describe("a press inside the dashed box of several (US-072)", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    // A and D, their dashed box from (10, 20) to (300, 70).
    drawTwo("0");
    clickA(SHIFT);
  });

  it("[F39.AC2] draws a selection rectangle from an empty place between A and D, none selected", () => {
    // From (150, 40) to (181, 40): between A, ending at 110, and D, starting at 200.
    dragBy({ x: 150, y: 40 }, { x: 30.6, y: 0 }, NO_KEY);

    expect(readSelectedShapes(document)).toEqual([]);
    expectAt(readShape(0), { x: 10, y: 20 });
    expectAt(readShape(1), { x: 200, y: 20 });
  });

  it("[F39.AC2] keeps A and D with Shift, neither moved", () => {
    dragBy({ x: 150, y: 40 }, { x: 30.6, y: 0 }, SHIFT);

    expect(readSelectedShapes(document)).toEqual([0, 1]);
    expectAt(readShape(0), { x: 10, y: 20 });
    expectAt(readShape(1), { x: 200, y: 20 });
  });
});
