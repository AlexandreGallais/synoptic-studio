// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { isDrawnText, shapePanelValues } from "../../src";
import { HELLO_TEXT } from "../../tests";
import { readFontSession, writeFontSession } from "../fonts";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";
import {
  clickElement,
  dispatchDrawingPointer,
  openPage,
  selectPageElement,
  setElementBox,
  typeAndEnter,
} from "../user-actions";

import type { Point } from "../../src";

/** The pause after an arrow's release within which another arrow joins its burst, in ms. */
const PAUSE = 500;

/**
 * Sends a key to an element of the page as the browser sends it: bubbling, cancelable.
 *
 * @param selector - CSS selector of the element
 * @param type - `keydown` or `keyup`
 * @param init - the key, the modifiers held and whether it repeats
 */
function sendKey(selector: string, type: string, init: KeyboardEventInit): void {
  selectPageElement(selector).dispatchEvent(
    new KeyboardEvent(type, { ...init, bubbles: true, cancelable: true }),
  );
}

/**
 * Holds an arrow in a field of several shapes for keys the browser sends, then releases it.
 *
 * @param id - the field, without `#`
 * @param init - the arrow and the modifiers held
 * @param count - how many keys the browser sends, the first and its repeats
 */
function holdIn(id: string, init: KeyboardEventInit, count: number): void {
  const sent = Array.from({ length: count }).keys();

  for (const index of sent) {
    sendKey(`#${id}`, "keydown", { ...init, repeat: index > 0 });
  }

  sendKey(`#${id}`, "keyup", init);
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
 * Reads the top-left of each shape's straight box, as the panel shows it, rounded to 5 decimals.
 *
 * @returns the anchors, in the order drawn, a text's left out
 */
function readAnchors(): readonly Point[] {
  return readKeptDrawing(document).flatMap(({ shape }) => {
    if (isDrawnText(shape)) {
      return [];
    }

    const { x, y } = shapePanelValues(shape);

    return { x: Number(x.toFixed(5)) + 0, y: Number(y.toFixed(5)) + 0 };
  });
}

describe("an arrow in several shapes' X or Y, each landed on integers", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    // A′, a sharp 100 × 50 at (10, 20) turned by 30° about its middle (60, 45): its straight box
    // starts at (60 − 55.80127, 45 − 46.65064) = (4.19873, −1.65064). Then A, 100 × 50 at
    // (10, 20), upright, placed by the Rectangle tool; both selected by Ctrl + A.
    typeRectangle(["10", "20", "100", "50", "30"]);
    clickElement("tool-rectangle");
    dispatchDrawingPointer("click", { x: 400, y: 300 }, { altKey: false, shiftKey: false });
    typeRectangle(["10", "20", "100", "50", "0"]);
    sendKey("body", "keydown", { ctrlKey: true, key: "a" });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it.each([
    // As one shape's: A′ 4.19873 + 1 = 5.19873 rounds to 5, −1.65064 to −2; A 10 + 1 = 11. Their
    // dashed box, landed together, would take A to (10.80127, 19.65064) instead.
    {
      anchors: [
        { x: 5, y: -2 },
        { x: 11, y: 20 },
      ],
      field: "several-x",
      key: "ArrowUp",
      name: "Up in X",
    },
    // A′ 3.19873 rounds to 3; A 9.
    {
      anchors: [
        { x: 3, y: -2 },
        { x: 9, y: 20 },
      ],
      field: "several-x",
      key: "ArrowDown",
      name: "Down in X",
    },
    // As Down: A′ −1.65064 + 1 = −0.65064 rounds to −1, 4.19873 to 4; A 20 + 1 = 21.
    {
      anchors: [
        { x: 4, y: -1 },
        { x: 10, y: 21 },
      ],
      field: "several-y",
      key: "ArrowUp",
      name: "Up in Y",
    },
  ])("[F46.AC1] lands each shape's own straight box for $name, as one shape's", (each) => {
    expect(readAnchors()).toEqual([
      { x: 4.19873, y: -1.65064 },
      { x: 10, y: 20 },
    ]);

    holdIn(each.field, { key: each.key }, 1);

    expect(readAnchors()).toEqual(each.anchors);
  });

  it("[F46.AC1] moves both on by exactly 10 at Shift + Up in X once landed", () => {
    holdIn("several-x", { key: "ArrowUp" }, 1);
    holdIn("several-x", { key: "ArrowUp", shiftKey: true }, 1);

    // Landed on (5, −2) and (11, 20), then 10 across each.
    expect(readAnchors()).toEqual([
      { x: 15, y: -2 },
      { x: 21, y: 20 },
    ]);
  });

  it("[F46.AC1] keeps each fraction for +1 typed in X, a value typed and not an arrow", () => {
    typeAndEnter("several-x", "+1");

    // 4.19873 + 1 and 10 + 1, each Y kept (A value typed, Q27).
    expect(readAnchors()).toEqual([
      { x: 5.19873, y: -1.65064 },
      { x: 11, y: 20 },
    ]);
  });

  it("[F46.AC1] moves a text never measured by the step as typed, the others landed", () => {
    const never = { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn", x: 5, y: -50 } };

    writeFontSession(document, { ...readFontSession(document), kept: [] });

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      ...readStoredDrawing(document, DRAWING_KEYS.below),
      never,
    ]);

    sendKey("body", "keydown", { ctrlKey: true, key: "a" });

    holdIn("several-x", { key: "ArrowUp" }, 1);

    // A′ and A landed as above; the text, which has no straight box, given `+1` in X, which a text
    // never measured leaves as it is (A text, `readEditedKeptShape`).
    expect(readAnchors()).toEqual([
      { x: 5, y: -2 },
      { x: 11, y: 20 },
    ]);

    expect(readKeptDrawing(document)[2]?.shape).toEqual(never.shape);
  });

  it("[F46.AC1] takes Up held 30 keys in X as one step of the history, Ctrl + Z giving both back", () => {
    holdIn("several-x", { key: "ArrowUp" }, 30);

    // Landed on (5, −2) and (11, 20) at the first key, then 29 × 1 across.
    expect(readAnchors()).toEqual([
      { x: 34, y: -2 },
      { x: 40, y: 20 },
    ]);

    vi.advanceTimersByTime(PAUSE);
    sendKey("body", "keydown", { ctrlKey: true, key: "z" });
    sendKey("body", "keyup", { ctrlKey: true, key: "z" });

    expect(readAnchors()).toEqual([
      { x: 4.19873, y: -1.65064 },
      { x: 10, y: 20 },
    ]);
  });
});
