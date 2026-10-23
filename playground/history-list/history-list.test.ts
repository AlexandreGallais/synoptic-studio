// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { readKeptDrawing } from "../kept-shapes";
import {
  chooseOption,
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readDisabledState,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeValue,
} from "../user-actions";

import type { Point } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** The marks of a row's place: none for a past step and the present, `>` for a step to redo. */
const PLACE_MARKS = new Map([
  ["future", " >"],
  ["past", ""],
  ["present", ""],
]);

/** Inside A, the rectangle the page opens with, 120 × 80 at (0, 0). */
const IN_A: Point = { x: 60, y: 40 };

/** The middle of B, 100 × 100 placed by a click at (250, 140): its box from (200, 90). */
const IN_B: Point = { x: 250, y: 140 };

/**
 * Presses a key with the focus outside the fields, then releases it.
 *
 * @param key - the key, as `KeyboardEvent.key` names it
 */
function press(key: string): void {
  const body = selectPageElement("body");

  body.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key }));
  body.dispatchEvent(new KeyboardEvent("keyup", { bubbles: true, cancelable: true, key }));
}

/**
 * Clicks the drawing as a mouse does: pressed, released, then the `click`.
 *
 * @param at - the point clicked
 */
function clickDrawing(at: Point): void {
  dispatchDrawingPointer("pointerdown", at, NO_KEY);
  dispatchDrawingPointer("pointerup", at, NO_KEY);
  dispatchDrawingPointer("click", at, NO_KEY);
}

/**
 * Reads the buttons of the list of steps, the oldest first.
 *
 * @returns each button
 */
function readStepButtons(): readonly HTMLButtonElement[] {
  return [...document.querySelectorAll<HTMLButtonElement>("#history-list button")];
}

/**
 * Reads the list of steps as the page shows it.
 *
 * @returns each row's text, a step's name followed by where it stands, `*` for the present and
 * `>` for a step to redo, and each line of the steps left out
 */
function readList(): readonly string[] {
  return [...document.querySelectorAll("#history-list li")].map((row) => {
    const button = row.querySelector("button");

    if (button === null) {
      return row.textContent;
    }

    const current = button.getAttribute("aria-current") === "step" ? " *" : "";

    return `${button.textContent}${current}${PLACE_MARKS.get(button.dataset["place"] ?? "") ?? ""}`;
  });
}

/**
 * Nudges the selection by Right several times, each key pressed then released.
 *
 * @param count - how many times
 */
function nudgeRight(count: number): void {
  if (!(count > 0)) {
    return;
  }

  press("ArrowRight");
  nudgeRight(count - 1);
}

/**
 * Clicks a button of the page several times.
 *
 * @param id - its id
 * @param count - how many times
 */
function clickTimes(id: string, count: number): void {
  if (!(count > 0)) {
    return;
  }

  clickElement(id);
  clickTimes(id, count - 1);
}

/**
 * Clicks the button of a step of the list.
 *
 * @param index - its index among the buttons, the oldest first
 */
function clickStep(index: number): void {
  readStepButtons()[index]?.click();
}

/**
 * Reads the x and the width of each shape of the drawing, in the order drawn.
 *
 * @returns each shape's x and width
 */
function readBoxes(): readonly object[] {
  return readKeptDrawing(document).map(({ shape }) =>
    "width" in shape ? { width: shape.width, x: shape.x } : {},
  );
}

describe("The list of the history's steps (US-077)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });

    // B placed by a click, then A nudged by Right, B deleted and 150 typed in A's W.
    clickElement("tool-rectangle");
    clickDrawing(IN_B);
    clickElement("tool-selector");
    clickDrawing(IN_A);
    press("ArrowRight");
    clickDrawing(IN_B);
    press("Delete");
    clickDrawing(IN_A);
    typeAndEnter("width", "150");
  });

  it("[F41.AC1] names A nudged, B deleted and 150 typed in W: Move, Delete, Width, Width marked", () => {
    expect(readList()).toEqual(["Start", "Add", "Move", "Delete", "Width *"]);
    expect(readBoxes()).toEqual([{ width: 150, x: 1 }]);
  });

  it("[F41.AC1] goes back to the state after Move by a click, Delete and Width kept to redo", () => {
    clickStep(2);

    // A nudged to x 1, 120 wide, and B back.
    expect(readBoxes()).toEqual([
      { width: 120, x: 1 },
      { width: 100, x: 200 },
    ]);

    expect(readList()).toEqual(["Start", "Add", "Move *", "Delete >", "Width >"]);
    expect(readDisabledState("history-redo")).toBe("false");

    clickElement("history-redo");
    clickElement("history-redo");

    expect(readBoxes()).toEqual([{ width: 150, x: 1 }]);
    expect(readList()).toEqual(["Start", "Add", "Move", "Delete", "Width *"]);
  });

  it("[F41.AC1] goes forward to Width by a click once back at Start", () => {
    clickStep(0);

    // The page as it opened: A at (0, 0), 120 wide.
    expect(readBoxes()).toEqual([{ width: 120, x: 0 }]);
    expect(readDisabledState("history-undo")).toBe("true");

    clickStep(4);

    expect(readBoxes()).toEqual([{ width: 150, x: 1 }]);
  });

  it("[F41.AC1] drops the steps to redo at a step taken after going back", () => {
    clickStep(2);
    clickDrawing(IN_A);
    press("ArrowRight");

    // A nudged again to x 2, B still there: Delete and Width dropped.
    expect(readBoxes()).toEqual([
      { width: 120, x: 2 },
      { width: 100, x: 200 },
    ]);

    expect(readList()).toEqual(["Start", "Add", "Move", "Move *"]);
    expect(readDisabledState("history-redo")).toBe("true");
  });

  it("[F41.AC1] keeps the history as it is at a click on the present", () => {
    clickStep(4);

    expect(readList()).toEqual(["Start", "Add", "Move", "Delete", "Width *"]);
    expect(readBoxes()).toEqual([{ width: 150, x: 1 }]);
  });

  it("[F41.AC1] names A made a hexagon, then converted to a free shape, 'Convert' each", () => {
    typeValue("corners", "6");
    chooseOption("shape", "polygon");
    clickElement("convert");

    // The kind the panel shows changed each time, with the shape's fields.
    expect(readList().slice(-3)).toEqual(["Width", "Convert", "Convert *"]);
  });

  it("[F41.AC1] lists Start alone after Reset", () => {
    clickElement("tool-reset");

    expect(readList()).toEqual(["Start *"]);
  });
});

describe("The list of a long history (US-077)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    clickDrawing(IN_A);

    nudgeRight(20);
  });

  it("[F41.AC1] shows the last 15 states of 21, the 6 older left out", () => {
    // Start and 20 moves: 14 rows before the present, 21 − 15 = 6 left out.
    expect(readList()).toEqual([
      "6 earlier steps",
      ...Array.from({ length: 14 }, () => "Move"),
      "Move *",
    ]);
  });

  it("[F41.AC1] goes back to the first row shown, x 6, then shows Start and 6 later steps", () => {
    clickStep(0);

    // The 7th state, after the 6th move: x 6. Then 6 past, 14 to come: 7 rows after it asked,
    // the 6 past leave 8; 14 − 8 = 6 left out after.
    expect(readBoxes()).toEqual([{ width: 120, x: 6 }]);

    expect(readList()).toEqual([
      "Start",
      ...Array.from({ length: 5 }, () => "Move"),
      "Move *",
      ...Array.from({ length: 8 }, () => "Move >"),
      "6 later steps",
    ]);
  });

  it("[F41.AC1] keeps 6 earlier steps after 6 undos, the 6 undone shown to redo", () => {
    clickTimes("history-undo", 6);

    // 14 past, 6 to come: 14 rows beside the present, at most 7 after it: the 6, then 8 before;
    // 14 − 8 = 6 earlier.
    expect(readList()).toEqual([
      "6 earlier steps",
      ...Array.from({ length: 8 }, () => "Move"),
      "Move *",
      ...Array.from({ length: 6 }, () => "Move >"),
    ]);
  });

  it("[F41.AC1] writes a single state left out as 1 earlier step", () => {
    clickElement("tool-reset");
    clickDrawing(IN_A);

    nudgeRight(15);

    // Start and 15 moves: 14 rows before the present, 16 − 15 = 1 left out.
    expect(readList()[0]).toBe("1 earlier step");
    expect(readList()).toHaveLength(16);
  });
});
