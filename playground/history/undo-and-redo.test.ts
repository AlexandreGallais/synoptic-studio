// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { isDrawnText } from "../../src";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";
import {
  clickElement,
  commitField,
  dispatchDrawingPointer,
  openServedPage,
  readDisabledState,
  readPressedState,
  readShapeModel,
  readText,
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

/** Where C is pressed: its middle, (450, 45). */
const IN_C: Point = { x: 450, y: 45 };

/** Ctrl + Z. */
const UNDO: KeyboardEventInit = { ctrlKey: true, key: "z" };

/** Ctrl + Shift + Z, the key read as a capital. */
const REDO: KeyboardEventInit = { ctrlKey: true, key: "Z", shiftKey: true };

/**
 * Presses a key on an element of the page then releases it, as the browser sends them: bubbling,
 * cancelable.
 *
 * @param selector - CSS selector of the element the focus is on, `body` for none
 * @param init - the key, as `KeyboardEvent.key` names it, and the modifiers held
 * @returns the key pressed, telling whether its default was prevented
 */
function press(selector: string, init: KeyboardEventInit): KeyboardEvent {
  const element = selectPageElement(selector);
  const down = new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true });

  element.dispatchEvent(down);
  element.dispatchEvent(new KeyboardEvent("keyup", { ...init, bubbles: true, cancelable: true }));

  return down;
}

/**
 * Holds Right with the focus outside the fields: pressed, repeated, then released.
 *
 * @param count - how many keys the browser sends, the first and its repeats
 */
function holdRight(count: number): void {
  const body = selectPageElement("body");
  const key = "ArrowRight";
  const sent = Array.from({ length: count }).keys();

  for (const index of sent) {
    const init = { bubbles: true, cancelable: true, key, repeat: index > 0 };

    body.dispatchEvent(new KeyboardEvent("keydown", init));
  }

  body.dispatchEvent(new KeyboardEvent("keyup", { bubbles: true, cancelable: true, key }));
}

/**
 * Clicks the drawing as a mouse does: pressed, released, then the `click`.
 *
 * @param at - the point clicked
 * @param keys - the keys held
 */
function clickAt(at: Point, keys: PointerKeys): void {
  dispatchDrawingPointer("pointerdown", at, keys);
  dispatchDrawingPointer("pointerup", at, keys);
  dispatchDrawingPointer("click", at, keys);
}

/**
 * Presses the drawing and moves the pointer by an offset, not released.
 *
 * @param from - the point pressed
 * @param by - how far the pointer goes
 * @returns where the pointer is
 */
function pressAndMove(from: Point, by: Point): Point {
  const to = { x: from.x + by.x, y: from.y + by.y };

  dispatchDrawingPointer("pointerdown", from, NO_KEY);
  dispatchDrawingPointer("pointermove", to, NO_KEY);

  return to;
}

/**
 * Drags on the drawing by an offset, then releases, the `click` following.
 *
 * @param from - the point pressed
 * @param by - how far the pointer goes
 */
function dragBy(from: Point, by: Point): void {
  const to = pressAndMove(from, by);

  dispatchDrawingPointer("pointerup", to, NO_KEY);
  dispatchDrawingPointer("click", to, NO_KEY);
}

/**
 * Types the box of the rectangle the panel edits, straight and sharp.
 *
 * @param x - its x, its y 20
 */
function typeRectangle(x: string): void {
  typeValue("angle", "0");
  typeValue("radius", "0");
  typeValue("width", "100");
  typeValue("height", "50");
  typeValue("shape-x", x);
  typeValue("shape-y", "20");
}

/**
 * Draws A, 100 × 50 at (10, 20), from the shape the page opens with, then B and C placed by the
 * Rectangle tool and typed 100 × 50 at (200, 20) and (400, 20); then Escape, none selected.
 */
function drawShapes(): void {
  typeRectangle("10");

  for (const x of ["200", "400"]) {
    clickElement("tool-rectangle");
    dispatchDrawingPointer("click", { x: 600, y: 450 }, NO_KEY);
    typeRectangle(x);
  }

  press("body", { key: "Escape" });
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
 * Asserts the places of the shapes of the drawing, in the order drawn.
 *
 * @param places - the x and y of each, computed by hand
 */
function expectPlaces(places: readonly Point[]): void {
  expect(readKeptDrawing(document)).toHaveLength(places.length);

  for (const [index, place] of places.entries()) {
    expect(readShape(index)).toMatchObject(place);
  }
}

/** A, B and C where they are drawn. */
const DRAWN = [
  { x: 10, y: 20 },
  { x: 200, y: 20 },
  { x: 400, y: 20 },
];

describe("Undo and Redo, by the keys, what is done on the drawing (US-074)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    drawShapes();
  });

  it.each([
    { name: "Ctrl + Z then Ctrl + Shift + Z", redo: REDO, undo: UNDO },
    { name: "Ctrl + Z then Ctrl + Y", redo: { ctrlKey: true, key: "y" }, undo: UNDO },
    // macOS sends Cmd + Shift + Z as a small z.
    {
      name: "Cmd + Z then Cmd + Shift + Z",
      redo: { key: "z", metaKey: true, shiftKey: true },
      undo: { key: "z", metaKey: true },
    },
    {
      name: "Cmd + Z then Cmd + Y",
      redo: { key: "y", metaKey: true },
      undo: { key: "z", metaKey: true },
    },
  ])("[F40.AC1] undoes and redoes a shape placed by a click, by $name", (each) => {
    clickElement("tool-rectangle");
    clickAt({ x: 250, y: 140 }, NO_KEY);

    // 100 × 100 centred on (250, 140): its top-left at (200, 90).
    expectPlaces([...DRAWN, { x: 200, y: 90 }]);

    const undone = press("body", each.undo);

    expectPlaces(DRAWN);
    expect(undone.defaultPrevented).toBe(true);

    press("body", each.redo);

    expectPlaces([...DRAWN, { x: 200, y: 90 }]);
    expect(readShape(3)).toMatchObject({ height: 100, width: 100 });
  });

  it("[F40.AC1] undoes and redoes a shape drawn by a drag from (10, 10) to (130, 90)", () => {
    clickElement("tool-rectangle");
    dragBy({ x: 10, y: 10 }, { x: 120, y: 80 });

    // The box from (10, 10) to (130, 90): 120 × 80 at (10, 10).
    expect(readShape(3)).toMatchObject({ height: 80, width: 120, x: 10, y: 10 });

    press("body", UNDO);

    expectPlaces(DRAWN);

    press("body", REDO);

    expect(readShape(3)).toMatchObject({ height: 80, width: 120, x: 10, y: 10 });
  });

  it("[F40.AC1] gives back A dragged to (41, 39) as it was, selected, then lands it again", () => {
    clickAt(IN_A, NO_KEY);

    const before = readKeptDrawing(document);

    dragBy(IN_A, { x: 30.6, y: 19.4 });

    // 10 + 30.6 = 40.6 lands on 41, 20 + 19.4 = 39.4 on 39.
    expect(readShape(0)).toMatchObject({ x: 41, y: 39 });

    press("body", UNDO);

    expect(readKeptDrawing(document)).toEqual(before);
    expect(readSelectedShapes(document)).toEqual([0]);
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["10", "20"]);

    press("body", REDO);

    expect(readShape(0)).toMatchObject({ x: 41, y: 39 });
    expect(readSelectedShapes(document)).toEqual([0]);
  });

  it("[F40.AC1] takes no step for a drag the browser cancels", () => {
    clickAt(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    pressAndMove(IN_B, { x: 40, y: 40 });
    dispatchDrawingPointer("pointercancel", IN_B, NO_KEY);
    press("body", UNDO);

    // The move of A undone, B never moved.
    expectPlaces(DRAWN);
  });

  it("[F40.AC1] gives back, undone, the selection before the press of a drag on A not selected", () => {
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    press("body", UNDO);

    expectPlaces(DRAWN);
    expect(readSelectedShapes(document)).toEqual([]);
  });

  it("[F40.AC1] gives back B and C deleted, selected, in the order drawn, and removes them again", () => {
    clickAt(IN_B, NO_KEY);
    clickAt(IN_C, SHIFT);
    press("body", { key: "Delete" });

    expectPlaces([{ x: 10, y: 20 }]);

    press("body", UNDO);

    expectPlaces(DRAWN);
    expect(readSelectedShapes(document)).toEqual([1, 2]);

    press("body", REDO);

    expectPlaces([{ x: 10, y: 20 }]);
    expect(readSelectedShapes(document)).toEqual([]);
  });

  it.each([
    {
      action: (): void => {
        clickAt(IN_B, NO_KEY);
      },
      name: "B clicked",
    },
    {
      action: (): void => {
        clickAt(IN_C, SHIFT);
      },
      name: "Shift + click on C",
    },
    // From (190, 100) to (290, 50): the rectangle touches B alone.
    {
      action: (): void => {
        dragBy({ x: 190, y: 100 }, { x: 100, y: -50 });
      },
      name: "a rectangle",
    },
    {
      action: (): void => {
        press("body", { ctrlKey: true, key: "a" });
      },
      name: "Ctrl + A",
    },
    {
      action: (): void => {
        press("body", { key: "Escape" });
      },
      name: "Escape",
    },
  ])("[F40.AC3] undoes the move of A, then redoes it, after $name, no step", (each) => {
    clickAt(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    each.action();
    press("body", UNDO);

    expectPlaces(DRAWN);

    press("body", REDO);

    expect(readShape(0)).toMatchObject({ x: 41, y: 39 });
  });

  it("[F40.AC1] undoes Right held as one step, then a nudge alone as another", () => {
    clickAt(IN_A, NO_KEY);
    holdRight(30);

    // 10 + 30 × 1.
    expect(readShape(0)).toMatchObject({ x: 40, y: 20 });

    holdRight(1);

    expect(readShape(0)).toMatchObject({ x: 41, y: 20 });

    press("body", UNDO);

    expect(readShape(0)).toMatchObject({ x: 40, y: 20 });

    press("body", UNDO);

    expect(readShape(0)).toMatchObject({ x: 10, y: 20 });
  });

  it("[F40.AC1] redoes nothing once a step follows an undo", () => {
    clickAt(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    press("body", UNDO);
    dragBy(IN_B, { x: 10, y: 0 });

    // B from (200, 20) to (210, 20).
    expectPlaces([
      { x: 10, y: 20 },
      { x: 210, y: 20 },
      { x: 400, y: 20 },
    ]);

    press("body", REDO);

    expectPlaces([
      { x: 10, y: 20 },
      { x: 210, y: 20 },
      { x: 400, y: 20 },
    ]);
  });

  it("[F40.AC1] changes nothing while the Selector's press moves A, until the release", () => {
    clickAt(IN_A, NO_KEY);

    const to = pressAndMove(IN_A, { x: 30.6, y: 19.4 });

    press("body", UNDO);

    // (10, 20) + (30.6, 19.4) = (40.6, 39.4), not landed yet, nothing undone.
    expect(readShape(0).x).toBeCloseTo(40.6, 9);
    expect(readShape(0).y).toBeCloseTo(39.4, 9);

    dispatchDrawingPointer("pointerup", to, NO_KEY);
    press("body", UNDO);

    expectPlaces(DRAWN);
  });

  it("[F40.AC1] changes nothing while a tool's drag draws a shape, until the release", () => {
    clickElement("tool-rectangle");

    const to = pressAndMove({ x: 10, y: 10 }, { x: 120, y: 80 });

    press("body", UNDO);

    expect(readKeptDrawing(document)).toHaveLength(4);

    dispatchDrawingPointer("pointerup", to, NO_KEY);
    press("body", UNDO);

    expectPlaces(DRAWN);
  });

  it("[F40.AC1] keeps a tool armed after an undo", () => {
    clickAt(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    clickElement("tool-polygon");
    press("body", UNDO);

    expectPlaces(DRAWN);
    expect(readPressedState("tool-polygon")).toBe("true");
  });

  it("[F40.AC1] undoes the drawing from W once its value is applied, and leaves it to W typed in", () => {
    clickAt(IN_A, NO_KEY);
    typeValue("width", "150");
    commitField("width");

    const applied = press("#width", UNDO);

    expect(applied.defaultPrevented).toBe(true);
    expect(readValue("width")).toBe("100");
    expect(readShape(0)).toMatchObject({ width: 100 });

    typeValue("width", "1");

    const typed = press("#width", UNDO);

    expect(typed.defaultPrevented).toBe(false);
    expect(readShapeModel()).toMatchObject({ width: 1 });
  });

  it("[F40.AC1] takes no step at a key released in a field: 150 typed in W undone at once", () => {
    clickAt(IN_A, NO_KEY);

    for (const typed of ["1", "15", "150"]) {
      typeValue("width", typed);
      selectPageElement("#width").dispatchEvent(new KeyboardEvent("keyup", { bubbles: true }));
    }

    commitField("width");
    press("body", UNDO);

    expect(readShape(0)).toMatchObject({ width: 100 });
  });

  it("[F40.AC1] leaves Ctrl + Z to the drawing once a field typed in is left", () => {
    clickAt(IN_A, NO_KEY);
    typeValue("width", "100");
    selectPageElement("#width").dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
    dragBy(IN_A, { x: 30.6, y: 19.4 });

    const event = press("#width", UNDO);

    expect(event.defaultPrevented).toBe(true);
    expectPlaces(DRAWN);
  });

  it("[F40.AC1] leaves Ctrl + Z to a slider of the color picker, never typed in, to the drawing", () => {
    clickAt(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    press("#color-hue", UNDO);

    expectPlaces(DRAWN);
  });

  it.each([
    { init: { altKey: true, ctrlKey: true, key: "z" }, name: "Ctrl + Alt + Z" },
    { init: { key: "z" }, name: "Z alone" },
    { init: { ctrlKey: true, key: "x" }, name: "Ctrl + X" },
  ])("[F40.AC1] leaves $name to the browser, nothing undone", ({ init }) => {
    clickAt(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });

    const event = press("body", init);

    expect(event.defaultPrevented).toBe(false);
    expect(readShape(0)).toMatchObject({ x: 41, y: 39 });
  });
});

describe("the Undo and Redo buttons and the history's size (US-074)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F40.AC1] opens with nothing to undo nor to redo, the titles naming the keys", () => {
    expect(readDisabledState("history-undo")).toBe("true");
    expect(readDisabledState("history-redo")).toBe("true");
    expect(selectPageElement("#history-undo").title).toContain("Ctrl + Z");
    expect(selectPageElement("#history-redo").title).toContain("Ctrl + Shift + Z");
    expect(selectPageElement("#history-redo").title).toContain("Ctrl + Y");
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");
  });

  it("[F40.AC1] changes nothing on Ctrl + Z with nothing to undo", () => {
    const before = readKeptDrawing(document);

    const event = press("body", UNDO);

    expect(readKeptDrawing(document)).toEqual(before);
    expect(event.defaultPrevented).toBe(true);
  });

  it("[F40.AC1] empties the history at Reset during a press, started again at the release", () => {
    holdRight(1);

    const to = pressAndMove({ x: 60, y: 40 }, { x: 10, y: 0 });

    clickElement("tool-reset");

    expect(readDisabledState("history-undo")).toBe("true");
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");

    dispatchDrawingPointer("pointerup", to, NO_KEY);

    expect(readDisabledState("history-undo")).toBe("true");
  });

  it("[F40.AC1] shows the size of a nudge, and undoes and redoes it by the buttons", () => {
    holdRight(1);

    // The rectangle the page opens with, 616 characters, held by the past alone: 616 × 2 =
    // 1 232 bytes, its reference 8 and its index 8: 1 248 bytes, 1.2 kB.
    expect(readText("#history-size")).toBe("History 1.2 kB of 100 MB");
    expect(readDisabledState("history-undo")).toBe("false");
    expect(readDisabledState("history-redo")).toBe("true");

    clickElement("history-undo");

    expect(readShape(0)).toMatchObject({ x: 0, y: 0 });
    expect(readDisabledState("history-undo")).toBe("true");
    expect(readDisabledState("history-redo")).toBe("false");

    clickElement("history-redo");

    expect(readShape(0)).toMatchObject({ x: 1, y: 0 });
    expect(readDisabledState("history-redo")).toBe("true");
  });

  it("[F40.AC1] empties the history at Reset, the page as it opens", () => {
    drawShapes();
    clickElement("tool-reset");

    expect(readKeptDrawing(document)).toHaveLength(1);
    expect(readDisabledState("history-undo")).toBe("true");
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");
  });

  it("[F40.AC1] empties the history at a step of the guided test", () => {
    openServedPage("?guide=epic");
    holdRight(1);
    clickElement("guide-next");

    expect(readDisabledState("history-undo")).toBe("true");
    expect(readDisabledState("history-redo")).toBe("true");
  });
});
