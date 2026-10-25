// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { freeShapeRadii, isDrawnText, isFreeShape } from "../../src";
import { PAGE_HISTORIES } from "../history";
import { readKeptDrawing } from "../kept-shapes";
import {
  clickElement,
  dispatchDrawingPointer,
  openPage,
  selectInput,
  selectPageElement,
  setElementBox,
  typeAndEnter,
} from "../user-actions";

import { ARROW_BURSTS } from "./arrow-bursts";

import type { KeptShape } from "../kept-shapes";

/** The pause after an arrow's release within which another arrow joins its burst, in ms. */
const PAUSE = 500;

/**
 * Sends a key to a field of the panel as the browser sends it: bubbling, cancelable.
 *
 * @param id - the field, without `#`
 * @param type - `keydown` or `keyup`
 * @param init - the key and whether it repeats
 */
function sendKey(id: string, type: string, init: KeyboardEventInit): void {
  selectPageElement(`#${id}`).dispatchEvent(
    new KeyboardEvent(type, { ...init, bubbles: true, cancelable: true }),
  );
}

/**
 * Sends the keys of Up held in a field, its repeats included, as Chromium sends them in a number
 * field: at each key, the value stepped by 1, then `input` and `change` (note 0032, §6); a field
 * acting as a number field, one shape's or several's, steps itself at the key.
 * The key is not released.
 *
 * @param id - the field, without `#`
 * @param count - how many keys the browser sends, the first and its repeats
 */
function pressUp(id: string, count: number): void {
  const field = selectInput(id);

  const sent = Array.from({ length: count }).keys();

  for (const index of sent) {
    sendKey(id, "keydown", { key: "ArrowUp", repeat: index > 0 });

    if (field.type === "number") {
      field.value = String(Number(field.value) + 1);
      field.dispatchEvent(new Event("input", { bubbles: true }));
      field.dispatchEvent(new Event("change", { bubbles: true }));
    }
  }
}

/**
 * Holds Up in a field, then releases it.
 *
 * @param id - the field, without `#`
 * @param count - how many keys the browser sends, the first and its repeats
 */
function holdUp(id: string, count: number): void {
  pressUp(id, count);
  sendKey(id, "keyup", { key: "ArrowUp" });
}

/** Presses Ctrl + Z, then releases it, outside the fields. */
function undo(): void {
  const init = { bubbles: true, cancelable: true, ctrlKey: true, key: "z" };
  const body = selectPageElement("body");

  body.dispatchEvent(new KeyboardEvent("keydown", init));
  body.dispatchEvent(new KeyboardEvent("keyup", init));
}

/**
 * Reads the width of each shape of the drawing, in the order drawn.
 *
 * @returns the widths, a text's left out
 */
function readWidths(): readonly number[] {
  return readKeptDrawing(document).flatMap(({ shape }) =>
    isDrawnText(shape) ? [] : [shape.width],
  );
}

/**
 * Reads the width of each shape the history's present holds, what the last step recorded.
 *
 * @returns the widths, a text's left out; none without a history
 */
function readPresentWidths(): readonly number[] {
  const present: readonly KeptShape[] = PAGE_HISTORIES.get(document)?.present.drawing ?? [];

  return present.flatMap(({ shape }) => (isDrawnText(shape) ? [] : [shape.width]));
}

/**
 * Reads the radius of the first corner of each rectangle of a drawing, stored as a free shape.
 *
 * @param drawing - the shapes, in the order drawn
 * @returns the radii, a text's left out
 */
function readRadii(drawing: readonly KeptShape[]): readonly number[] {
  return drawing.flatMap(({ shape }) =>
    !isDrawnText(shape) && isFreeShape(shape) ? freeShapeRadii(shape).slice(0, 1) : [],
  );
}

describe("a burst of arrows in a number field, one step", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    // The page opens with its rectangle, 120 × 80, selected: made 100 wide, a step of its own.
    openPage("");
    typeAndEnter("width", "100");
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("[F43.AC3] takes one step 500 ms after Up held is released, not 30", () => {
    holdUp("width", 30);

    // 100 + 30 × 1, no step taken yet.
    expect(readWidths()).toEqual([130]);
    expect(readPresentWidths()).toEqual([100]);

    vi.advanceTimersByTime(PAUSE - 1);

    expect(readPresentWidths()).toEqual([100]);

    vi.advanceTimersByTime(1);

    expect(readPresentWidths()).toEqual([130]);

    undo();

    expect(readWidths()).toEqual([100]);
  });

  it("[F43.AC3] counts the pause from the last arrow released, Up then Down held together", () => {
    pressUp("width", 30);
    sendKey("width", "keydown", { key: "ArrowDown" });
    sendKey("width", "keyup", { key: "ArrowUp" });
    vi.advanceTimersByTime(300);
    sendKey("width", "keyup", { key: "ArrowDown" });
    vi.advanceTimersByTime(PAUSE - 1);

    // 799 ms after Up's release, 499 after Down's: no step yet.
    expect(readPresentWidths()).toEqual([100]);

    vi.advanceTimersByTime(1);

    // 100 + 30 − 1: the page steps W at Down's key too.
    expect(readPresentWidths()).toEqual([129]);
  });

  it("[F43.AC3] takes no step while Up is held, however long", () => {
    pressUp("width", 30);
    vi.advanceTimersByTime(10 * PAUSE);

    expect(readPresentWidths()).toEqual([100]);
  });

  it("[F43.AC3] takes one step for Up pressed 10 times, 100 ms apart", () => {
    const delays = Array.from({ length: 10 }, () => 100);

    for (const delay of delays) {
      holdUp("width", 1);
      vi.advanceTimersByTime(delay);
    }

    // 100 + 10 × 1; the last release 100 ms ago.
    expect(readWidths()).toEqual([110]);
    expect(readPresentWidths()).toEqual([100]);

    vi.advanceTimersByTime(PAUSE);
    undo();

    expect(readWidths()).toEqual([100]);
  });

  it("[F43.AC3] joins a press 300 ms after the release to its burst, 131 undone to 100", () => {
    holdUp("width", 30);
    vi.advanceTimersByTime(300);
    holdUp("width", 1);
    vi.advanceTimersByTime(PAUSE);

    expect(readPresentWidths()).toEqual([131]);

    undo();

    expect(readWidths()).toEqual([100]);
  });

  it("[F43.AC3] makes a press 2 s after the release a step of its own, 131 undone to 130", () => {
    holdUp("width", 30);
    vi.advanceTimersByTime(2000);
    holdUp("width", 1);
    vi.advanceTimersByTime(PAUSE);
    undo();

    expect(readWidths()).toEqual([130]);

    undo();

    expect(readWidths()).toEqual([100]);
  });

  it("[F43.AC3] undoes a burst at Ctrl + Z within its pause, the pause's end taking no step", () => {
    holdUp("width", 30);
    vi.advanceTimersByTime(200);
    undo();

    expect(readWidths()).toEqual([100]);

    vi.advanceTimersByTime(PAUSE);

    // Nothing recorded at the pause's end: the burst stays to redo.
    expect(readPresentWidths()).toEqual([100]);
    expect(PAGE_HISTORIES.get(document)?.future).toHaveLength(1);
  });

  it("[F43.AC3] takes the step at once when the window loses the focus, the release never heard", () => {
    pressUp("width", 30);
    document.defaultView?.dispatchEvent(new FocusEvent("blur"));

    expect(readPresentWidths()).toEqual([130]);
  });

  it("[F43.AC3] takes the step of another field's change within the pause, the burst with it", () => {
    holdUp("width", 30);
    typeAndEnter("height", "70");

    expect(readPresentWidths()).toEqual([130]);

    vi.advanceTimersByTime(PAUSE);
    undo();

    expect(readWidths()).toEqual([100]);
  });

  it("[F43.AC3] forgets a burst whose pause runs when the page is mounted again", () => {
    holdUp("width", 3);
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    // No burst left open, its pause's end finding none (every field applies its value on Enter,
    // a step taken at once).
    expect(ARROW_BURSTS.has(document)).toBe(false);

    typeAndEnter("radius", "20");
    vi.advanceTimersByTime(PAUSE);

    expect(readRadii(PAGE_HISTORIES.get(document)?.present.drawing ?? [])).toEqual([20]);
  });

  it("[F43.AC3] opens no burst at Cmd + Up, whose keyup macOS never sends", () => {
    sendKey("width", "keydown", { key: "Meta", metaKey: true });
    sendKey("width", "keydown", { key: "ArrowUp", metaKey: true });
    sendKey("width", "keyup", { key: "Meta" });
    typeAndEnter("width", "90");

    // No burst open: the value applied takes its step at once.
    expect(readPresentWidths()).toEqual([90]);
  });

  it.each(["ctrlKey", "altKey"])("[F43.AC3] opens no burst at Up with %s", (modifier) => {
    sendKey("width", "keydown", { key: "ArrowUp", [modifier]: true });
    typeAndEnter("width", "90");

    expect(readPresentWidths()).toEqual([90]);
  });

  it("[F43.AC3] ends a burst at once, its step taken, when Cmd is pressed, no keyup sent", () => {
    pressUp("width", 3);
    // Cmd pressed while Up is held, then both released, Cmd's keyup taken by a shortcut: none sent.
    sendKey("width", "keydown", { key: "Meta", metaKey: true });

    // 100 + 3, recorded at Cmd's press, no pause waited for.
    expect(readPresentWidths()).toEqual([103]);

    vi.advanceTimersByTime(PAUSE);
    undo();

    expect(readWidths()).toEqual([100]);
  });

  it("[F43.AC3] ends a burst at once, its step taken, at a pointerdown carrying Cmd", () => {
    pressUp("width", 3);

    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, cancelable: true, metaKey: true }),
    );

    // 100 + 3.
    expect(readPresentWidths()).toEqual([103]);
  });

  it("[F43.AC3] ends a burst at once, its step taken, when an arrow is pressed in another field", () => {
    holdUp("width", 30);
    holdUp("height", 5);

    // W's burst, 100 to 130, one step; H's, 80 to 85, waiting for its pause.
    expect(readPresentWidths()).toEqual([130]);

    vi.advanceTimersByTime(PAUSE);
    undo();

    expect(readWidths()).toEqual([130]);

    undo();

    expect(readWidths()).toEqual([100]);
  });
});

describe("a burst of arrows in a field of several shapes, one step", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    // The page's rectangle, 120 wide, and B, placed by the Rectangle tool and typed 40 wide.
    clickElement("tool-rectangle");
    dispatchDrawingPointer("click", { x: 600, y: 450 }, { altKey: false, shiftKey: false });
    typeAndEnter("width", "40");

    selectPageElement("body").dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ctrlKey: true, key: "a" }),
    );
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("[F43.AC3] makes A 150 wide and B 70 for Up held 30 keys in W, one step at the pause's end", () => {
    const before = readPresentWidths();

    holdUp("several-width", 30);

    // 120 + 30 and 40 + 30.
    expect(readWidths()).toEqual([150, 70]);
    expect(readPresentWidths()).toEqual(before);

    vi.advanceTimersByTime(PAUSE);

    expect(readPresentWidths()).toEqual([150, 70]);

    undo();

    expect(readWidths()).toEqual([120, 40]);
  });

  it("[F43.AC3] takes two steps for Up released in W, then 150 typed and Enter within the pause", () => {
    holdUp("several-width", 1);
    vi.advanceTimersByTime(100);

    for (const key of ["1", "5", "0"]) {
      sendKey("several-width", "keydown", { key });
    }

    // The first digit ends the burst, 120 + 1 and 40 + 1, its step taken.
    expect(readPresentWidths()).toEqual([121, 41]);

    typeAndEnter("several-width", "150");

    expect(readPresentWidths()).toEqual([150, 150]);

    vi.advanceTimersByTime(PAUSE);
    // Ctrl + Z gives back the burst's widths first, then those before it.
    undo();

    expect(readWidths()).toEqual([121, 41]);

    undo();

    expect(readWidths()).toEqual([120, 40]);
  });

  it("[F43.AC3] keeps a burst open at Shift, Ctrl, Alt or Left pressed in its field", () => {
    holdUp("several-width", 1);

    for (const key of ["Shift", "Control", "Alt", "ArrowLeft"]) {
      sendKey("several-width", "keydown", { key });
    }

    expect(readPresentWidths()).toEqual([120, 40]);

    vi.advanceTimersByTime(PAUSE);

    expect(readPresentWidths()).toEqual([121, 41]);
  });

  it("[F43.AC3] keeps a burst open at a key typed in another field, as before", () => {
    holdUp("several-width", 1);
    sendKey("several-height", "keydown", { key: "5" });

    expect(readPresentWidths()).toEqual([120, 40]);
  });
});
