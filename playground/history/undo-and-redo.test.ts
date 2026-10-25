// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { historySize, isDrawnText } from "../../src";
import { readDrawnTexts, readKeptDrawing, readSelectedShapes } from "../kept-shapes";
import {
  chooseOption,
  chooseRadio,
  chooseShapeKind,
  clickElement,
  clickPoint,
  commitField,
  dispatchDrawingPointer,
  openPage,
  pressPointer,
  readDisabledState,
  readModel,
  readPressedState,
  readShapeModel,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeTextContent,
  typeValue,
} from "../user-actions";

import { PAGE_HISTORIES } from "./page-histories";

import type { Point, Shape, Text } from "../../src";
import type { KeptShape } from "../kept-shapes";
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
function clickDrawing(at: Point, keys: PointerKeys): void {
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
  typeAndEnter("angle", "0");
  typeAndEnter("radius", "0");
  typeAndEnter("width", "100");
  typeAndEnter("height", "50");
  typeAndEnter("shape-x", x);
  typeAndEnter("shape-y", "20");
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

describe("Undo and Redo, by the keys, what is done on the drawing", () => {
  beforeEach(() => {
    openPage("");
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
    clickDrawing({ x: 250, y: 140 }, NO_KEY);

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
    clickDrawing(IN_A, NO_KEY);

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
    clickDrawing(IN_A, NO_KEY);
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
    clickDrawing(IN_B, NO_KEY);
    clickDrawing(IN_C, SHIFT);
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
        clickDrawing(IN_B, NO_KEY);
      },
      name: "B clicked",
    },
    {
      action: (): void => {
        clickDrawing(IN_C, SHIFT);
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
    clickDrawing(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    each.action();
    press("body", UNDO);

    expectPlaces(DRAWN);

    press("body", REDO);

    expect(readShape(0)).toMatchObject({ x: 41, y: 39 });
  });

  it("[F40.AC1] undoes Right held as one step, then a nudge alone as another", () => {
    clickDrawing(IN_A, NO_KEY);
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
    clickDrawing(IN_A, NO_KEY);
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
    clickDrawing(IN_A, NO_KEY);

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
    clickDrawing(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    clickElement("tool-polygon");
    press("body", UNDO);

    expectPlaces(DRAWN);
    expect(readPressedState("tool-polygon")).toBe("true");
  });

  it("[F40.AC1] undoes the drawing from W once its value is applied, and leaves it to W typed in", () => {
    clickDrawing(IN_A, NO_KEY);
    typeAndEnter("width", "150");
    commitField("width");

    const applied = press("#width", UNDO);

    expect(applied.defaultPrevented).toBe(true);
    expect(readValue("width")).toBe("100");
    expect(readShape(0)).toMatchObject({ width: 100 });

    // Typed, not applied: W keeps Ctrl + Z, the shape still 100 wide.
    typeValue("width", "1");

    const typed = press("#width", UNDO);

    expect(typed.defaultPrevented).toBe(false);
    expect(readShapeModel()).toMatchObject({ width: 100 });
  });

  it("[F40.AC1] takes no step at a key released in a field: 150 typed in W undone at once", () => {
    clickDrawing(IN_A, NO_KEY);

    for (const typed of ["1", "15", "150"]) {
      typeValue("width", typed);
      selectPageElement("#width").dispatchEvent(new KeyboardEvent("keyup", { bubbles: true }));
    }

    commitField("width");
    press("body", UNDO);

    expect(readShape(0)).toMatchObject({ width: 100 });
  });

  it("[F40.AC1] leaves Ctrl + Z to the drawing once a field typed in is left", () => {
    clickDrawing(IN_A, NO_KEY);
    typeAndEnter("width", "100");
    selectPageElement("#width").dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
    dragBy(IN_A, { x: 30.6, y: 19.4 });

    const event = press("#width", UNDO);

    expect(event.defaultPrevented).toBe(true);
    expectPlaces(DRAWN);
  });

  it("[F40.AC1] leaves Ctrl + Z to a slider of the color picker, never typed in, to the drawing", () => {
    clickDrawing(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    press("#color-hue", UNDO);

    expectPlaces(DRAWN);
  });

  it.each([
    { init: { altKey: true, ctrlKey: true, key: "z" }, name: "Ctrl + Alt + Z" },
    { init: { key: "z" }, name: "Z alone" },
    { init: { ctrlKey: true, key: "x" }, name: "Ctrl + X" },
  ])("[F40.AC1] leaves $name to the browser, nothing undone", ({ init }) => {
    clickDrawing(IN_A, NO_KEY);
    dragBy(IN_A, { x: 30.6, y: 19.4 });

    const event = press("body", init);

    expect(event.defaultPrevented).toBe(false);
    expect(readShape(0)).toMatchObject({ x: 41, y: 39 });
  });
});

describe("the Undo and Redo buttons and the history's size", () => {
  beforeEach(() => {
    openPage("");
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
    // 1 232 bytes, its reference 8 and its index 8: 1 248 bytes, 1.2 kB; not the 1 244 of A, a
    // rectangle 100 × 50 of 614 characters, that the moves held count.
    expect(readHistorySize()).toBe(1248);
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

  it("[F41.AC2] keeps the size shown equal to the size counted again through steps, undos and redos", () => {
    /**
     * Three nudges, the rectangle at x 0, 1 and 2 past, 616 characters each: 3 × (1 232 + 16) =
     * 3 744 bytes; undone twice, x 0 past and x 2, x 3 to come, the same; redone once, x 0 and 1
     * past, x 3 to come; a nudge then drops x 3: x 0, 1 and 2 past, the same 3 744 bytes.
     */
    const sizes = [
      (): void => {
        holdRight(1);
        holdRight(1);
        holdRight(1);
      },
      (): void => {
        clickElement("history-undo");
        clickElement("history-undo");
      },
      (): void => {
        clickElement("history-redo");
      },
      (): void => {
        holdRight(1);
      },
    ].map((action) => {
      action();

      return [PAGE_HISTORIES.get(document)?.size, readHistorySize()];
    });

    expect(sizes).toEqual([
      [3744, 3744],
      [3744, 3744],
      [3744, 3744],
      [3744, 3744],
    ]);

    expect(readText("#history-size")).toBe("History 3.7 kB of 100 MB");
  });

  it("[F40.AC1] empties the history at Reset, the page as it opens", () => {
    drawShapes();
    clickElement("tool-reset");

    expect(readKeptDrawing(document)).toHaveLength(1);
    expect(readDisabledState("history-undo")).toBe("true");
    expect(readText("#history-size")).toBe("History 0 B of 100 MB");
  });
});

/**
 * Types a value in a field, then leaves it, as Tab does: its `input`, its `change`, then its
 * `focusout`.
 *
 * @param id - the field
 * @param value - what is typed
 */
function typeAndLeave(id: string, value: string): void {
  const field = selectPageElement(`#${id}`);

  // An input or the Content text area alike.
  Object.assign(field, { value });
  field.dispatchEvent(new Event("input", { bubbles: true }));
  field.dispatchEvent(new Event("change", { bubbles: true }));
  field.dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
}

/**
 * Reads the drawing of the history's present, what the last step recorded.
 *
 * @returns the shapes the present holds; none without a history
 */
function readPresentDrawing(): readonly KeptShape[] {
  return PAGE_HISTORIES.get(document)?.present.drawing ?? [];
}

/**
 * Reads the size of the page's history, counted again (`historySize`).
 *
 * @returns its bytes; 0 without a history
 */
function readHistorySize(): number {
  const history = PAGE_HISTORIES.get(document);

  return history === undefined ? 0 : historySize(history);
}

/**
 * Asserts that an action is one step, taken at the action itself: the history's present the
 * drawing it leaves, Ctrl + Z giving back the drawing before it and Ctrl + Shift + Z the one after.
 *
 * @param action - the action, the drawing changed by it
 */
function expectOneStep(action: () => void): void {
  const before = readKeptDrawing(document);

  action();

  const after = readKeptDrawing(document);

  expect(after).not.toEqual(before);
  expect(readPresentDrawing()).toEqual(after);

  press("body", UNDO);

  expect(readKeptDrawing(document)).toEqual(before);

  press("body", REDO);

  expect(readKeptDrawing(document)).toEqual(after);
}

/**
 * Places a text by a click of the Text tool, its weight the page's, then types `Pump` in it and
 * leaves the Content field.
 *
 * @param at - the point clicked
 */
function placePump(at: Point): void {
  // happy-dom does not select the option marked `selected`: the page's weight, 400, chosen again.
  chooseOption("text-weight", "400");
  clickElement("tool-text");
  clickDrawing(at, NO_KEY);
  typeTextContent("Pump");
}

/**
 * Reads the text the panel edits.
 *
 * @returns its model, as the page shows it
 * @throws {TypeError} when the panel edits no text
 */
function readEditedText(): Text {
  const model = readModel();

  if (!isDrawnText(model)) {
    throw new TypeError("The panel edits no text.");
  }

  return model;
}

describe("Undo and Redo what is typed and chosen in the panel", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    drawShapes();
    clickDrawing(IN_A, NO_KEY);
  });

  it("[F40.AC2] undoes 150 typed in W and applied by Enter in one step, then 70 left in H", () => {
    for (const typed of ["1", "15", "150"]) {
      typeValue("width", typed);

      // Typed, not applied: A still 100 wide, drawn 150 wide at Enter only.
      expect(readShapeModel()).toMatchObject({ width: 100 });
    }

    commitField("width");

    expect(readShapeModel()).toMatchObject({ width: 150 });
    typeAndLeave("height", "70");
    press("body", UNDO);

    // H undone alone: two values applied are two steps.
    expect(readShape(0)).toMatchObject({ height: 50, width: 150 });

    press("body", UNDO);

    expect(readShape(0)).toMatchObject({ height: 50, width: 100 });
  });

  it("[F40.AC2] takes no step for 100 typed back in W: Ctrl + Z undoes the move before it", () => {
    dragBy(IN_A, { x: 30.6, y: 19.4 });
    typeAndLeave("width", "100");

    expect(readPresentDrawing()).toEqual(readKeptDrawing(document));

    press("body", UNDO);

    expectPlaces(DRAWN);
    expect(readShape(0)).toMatchObject({ width: 100 });
  });

  it("[F40.AC2] takes one step from 6 to 8 corners, none while the polygon is refused", () => {
    chooseShapeKind("polygon");
    // Emptied by a script, which the field never read: no corners, the polygon refused.
    typeAndLeave("corners", "");

    // Refused: the drawing kept without A, no step.
    expect(readKeptDrawing(document)).toHaveLength(2);
    expect(readPresentDrawing()).toHaveLength(3);

    typeAndLeave("corners", "8");

    expect(readPresentDrawing()).toEqual(readKeptDrawing(document));

    press("body", UNDO);

    expect(readValue("corners")).toBe("6");
    expect(readKeptDrawing(document)).toHaveLength(3);
  });

  it("[F40.AC2] gives back a polygon refused, then dropped by a click on the empty drawing", () => {
    chooseShapeKind("polygon");
    typeAndLeave("corners", "");
    clickDrawing({ x: 600, y: 450 }, NO_KEY);

    expect(readKeptDrawing(document)).toHaveLength(2);

    press("body", UNDO);

    expect(readKeptDrawing(document)).toHaveLength(3);
    expect(readSelectedShapes(document)).toEqual([0]);
    expect(readValue("corners")).toBe("6");
  });

  it("[F40.AC2] gives back the last valid values of a polygon refused, the history unmoved", () => {
    chooseShapeKind("polygon");
    typeAndLeave("corners", "");
    press("body", UNDO);

    expect(readValue("corners")).toBe("6");
    expect(readKeptDrawing(document)).toHaveLength(3);
    expect(readDisabledState("history-redo")).toBe("true");

    press("body", UNDO);

    // The step before: A a rectangle again.
    expect(readValue("shape")).toBe("rectangle");
  });

  it("[F40.AC2] undoes +50 typed in W for A and B in one step", () => {
    clickDrawing(IN_B, SHIFT);
    typeAndEnter("several-width", "+50");

    expect([readShape(0).width, readShape(1).width]).toEqual([150, 150]);
    expect(readPresentDrawing()).toEqual(readKeptDrawing(document));

    press("body", UNDO);

    expect([readShape(0).width, readShape(1).width]).toEqual([100, 100]);
  });

  it("[F40.AC2] takes no step at a swatch opening the color picker", () => {
    placePump({ x: 400, y: 300 });

    const present = readPresentDrawing();

    clickElement("text-color-swatch");

    expect(readPresentDrawing()).toBe(present);
  });

  it("[F40.AC2] gives back Pump once Valve is typed over it and the field left", () => {
    placePump({ x: 400, y: 300 });
    typeTextContent("Valve");
    press("body", UNDO);

    expect(readEditedText().content).toBe("Pump");

    press("body", REDO);

    expect(readEditedText().content).toBe("Valve");
  });

  it.each([
    {
      action: (): void => {
        typeAndLeave("text-size", "40");
      },
      name: "its size typed",
    },
    {
      action: (): void => {
        clickElement("text-bold");
      },
      name: "Bold",
    },
    {
      action: (): void => {
        clickElement("text-italic");
      },
      name: "Italic",
    },
    {
      action: (): void => {
        chooseOption("text-weight", "700");
      },
      name: "its weight",
    },
    {
      action: (): void => {
        chooseOption("text-fit", "box");
      },
      name: "its box fitted",
    },
  ])("[F40.AC2] undoes and redoes a text's style changed in one step: $name", ({ action }) => {
    placePump({ x: 400, y: 300 });
    expectOneStep(action);
  });

  it("[F40.AC2] undoes and redoes a text's box reset to the size it fits in one step", () => {
    placePump({ x: 400, y: 300 });
    chooseOption("text-fit", "box");
    typeAndLeave("text-box-width", "200");

    expectOneStep(() => {
      clickElement("text-box-reset");
    });
  });

  it("[F40.AC2] takes a step at each key on the hue slider, the plane's drag and the color typed", () => {
    placePump({ x: 400, y: 300 });
    chooseOption("color-model", "hsb");
    typeAndLeave("text-color", "#ff8000");
    clickElement("text-color-swatch");
    setElementBox("#color-plane", { height: 100, left: 10, top: 20, width: 200 });

    const colors = [readValue("text-color")];

    const keys = Array.from({ length: 5 }, () => "ArrowRight");

    for (const key of keys) {
      press("#color-hue", { key });
      colors.push(readValue("text-color"));
    }

    // From (110, 70), the middle of the plane, to (150, 40): S 70, B 80.
    pressPointer("#color-plane", "pointerdown", { buttons: 1, clientX: 110, clientY: 70 });
    pressPointer("#color-plane", "pointermove", { buttons: 1, clientX: 150, clientY: 40 });
    pressPointer("#color-plane", "pointerup", { buttons: 0, clientX: 150, clientY: 40 });
    colors.push(readValue("text-color"));
    typeAndLeave("text-color", "#336699");

    // 5 keys, a drag and a color typed: 7 steps, each undone in turn.
    const distinct = new Set(colors);

    expect(distinct.size).toBe(7);

    for (const color of colors.toReversed()) {
      press("body", UNDO);

      expect(readEditedText().color).toBe(color);
    }
  });

  it.each([
    {
      action: (): void => {
        chooseRadio("anchor", "min min");
      },
      name: "the anchor",
    },
    {
      action: (): void => {
        clickElement("stretched");
      },
      name: "Stretched",
    },
  ])("[F40.AC2] undoes and redoes $name chosen for a polygon in one step", ({ action }) => {
    chooseShapeKind("polygon");
    typeAndLeave("height", "80");
    expectOneStep(action);
  });
});

describe("Undo and Redo the buttons of the panel", () => {
  beforeEach(() => {
    // A rectangle 120 × 80 of radius 10.
    openPage("");
  });

  it.each([
    {
      action: (): void => {
        clickElement("turn-right");
      },
      name: "a turn",
      prepare: (): void => {
        // Edit the shape, as the page opens.
        clickElement("mode-shape");
      },
    },
    {
      action: (): void => {
        clickElement("convert");
      },
      name: "Convert to free shape",
      prepare: (): void => {
        // A rectangle is kept as a free shape already: a hexagon converted.
        typeAndEnter("corners", "6");
        chooseOption("shape", "polygon");
      },
    },
    {
      action: (): void => {
        clickElement("add-midpoint");
      },
      name: "Add a point in the middle",
      prepare: (): void => {
        clickElement("mode-points");
        clickPoint(1, false);
        clickPoint(2, true);
      },
    },
    {
      action: (): void => {
        clickElement("delete-points");
      },
      name: "Delete points",
      prepare: (): void => {
        clickElement("mode-points");
        clickPoint(2, false);
      },
    },
    {
      action: (): void => {
        clickElement("handles-switch");
      },
      name: "the handles switched on",
      prepare: (): void => {
        clickElement("mode-points");
        clickPoint(1, false);
      },
    },
    {
      action: (): void => {
        chooseOption("handle-mode", "none");
      },
      name: "a handle's mode",
      prepare: (): void => {
        clickElement("mode-points");
        clickPoint(1, false);
        clickElement("handles-switch");
      },
    },
  ])("[F40.AC2] undoes and redoes $name in one step", ({ action, prepare }) => {
    prepare();
    expectOneStep(action);
  });
});

describe("Undo and Redo a text placed and typed", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    // A step before the text: the page's rectangle nudged from (0, 0) to (1, 0).
    holdRight(1);
    chooseOption("text-weight", "400");
    clickElement("tool-text");
  });

  it("[F40.AC2] places a text with an empty content, its frame 1 wide and one line tall", () => {
    clickDrawing({ x: 250, y: 140 }, NO_KEY);

    expect(readEditedText().content).toBe("");
    expect(selectPageElement("#text-content")).toBe(document.activeElement);
    // Inter Regular 32, a line (1984 + 494) × 32 / 2048 = 38.71875 tall: the frame 1 × 38.71875
    // centred on (250, 140), its top-left (249.5, 120.640625), rounded to (250, 121).
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["250", "121"]);
    expect(document.querySelectorAll("#canvas .text-frame path")).toHaveLength(1);
  });

  it("[F40.AC2] undoes a text placed, its size and Valve typed, in one step", () => {
    clickDrawing({ x: 250, y: 140 }, NO_KEY);
    typeAndLeave("text-size", "40");
    typeTextContent("Valve");

    // The step taken as the Content field is left.
    expect(readKeptDrawing(document)).toHaveLength(2);
    expect(readPresentDrawing()).toEqual(readKeptDrawing(document));

    press("body", UNDO);

    // The text gone, the rectangle still nudged.
    expect(readKeptDrawing(document)).toHaveLength(1);
    expect(readShape(0)).toMatchObject({ x: 1, y: 0 });

    press("body", REDO);

    expect(readEditedText()).toMatchObject({ content: "Valve", size: 40 });
  });

  it.each([
    {
      action: (): void => {
        clickDrawing({ x: 600, y: 450 }, NO_KEY);
      },
      length: 1,
      name: "a click on the empty drawing",
    },
    {
      action: (): void => {
        press("body", { key: "Escape" });
      },
      length: 1,
      name: "Escape",
    },
    {
      action: (): void => {
        clickElement("tool-rectangle");
        clickDrawing({ x: 600, y: 450 }, NO_KEY);
      },
      length: 2,
      name: "another shape placed",
    },
  ])("[F40.AC2] removes a text left empty by $name, no step taken", ({ action, length }) => {
    clickDrawing({ x: 250, y: 140 }, NO_KEY);
    selectPageElement("#text-content").dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
    action();

    expect(readKeptDrawing(document)).toHaveLength(length);
    expect(readDrawnTexts(document)).toEqual([]);

    // Undone: the shape placed after it, if any, then the nudge.
    press("body", UNDO);

    if (length === 2) {
      press("body", UNDO);
    }

    expect(readKeptDrawing(document)).toHaveLength(1);
    expect(readShape(0)).toMatchObject({ x: 0, y: 0 });
  });

  it("[F40.AC2] removes a text still empty at Ctrl + Z, the history unmoved", () => {
    clickDrawing({ x: 250, y: 140 }, NO_KEY);

    const event = press("#text-content", UNDO);

    expect(event.defaultPrevented).toBe(true);
    expect(readKeptDrawing(document)).toHaveLength(1);
    expect(readShape(0)).toMatchObject({ x: 1, y: 0 });
    // Nothing to redo: the text was never a step.
    expect(readDisabledState("history-redo")).toBe("true");
  });
});
