// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { isDrawnText } from "../../src";
import { PAGE_HISTORIES } from "../history";
import {
  chooseOption,
  clickAt,
  clickElement,
  openPage,
  readModel,
  readShapeModel,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeTextContent,
} from "../user-actions";

import type { DrawingShape } from "../../src";

/** The pause after an arrow's release within which another arrow joins its burst, in ms. */
const PAUSE = 500;

/**
 * Sends a key to a field of the panel as the browser sends it: bubbling, cancelable.
 *
 * @param id - the field, without `#`
 * @param type - `keydown` or `keyup`
 * @param init - the key, whether it repeats and the modifiers held
 */
function sendKey(id: string, type: string, init: KeyboardEventInit): void {
  selectPageElement(`#${id}`).dispatchEvent(
    new KeyboardEvent(type, { ...init, bubbles: true, cancelable: true }),
  );
}

/**
 * Holds an arrow in a field, its repeats included, then releases it: the page steps the field at
 * each key, the browser stepping nothing in a text field.
 *
 * @param id - the field, without `#`
 * @param init - the key and the modifiers held
 * @param count - how many keys the browser sends, the first and its repeats
 */
function holdKey(id: string, init: KeyboardEventInit, count: number): void {
  const sent = Array.from({ length: count }).keys();

  for (const index of sent) {
    sendKey(id, "keydown", { ...init, repeat: index > 0 });
  }

  sendKey(id, "keyup", init);
}

/** Presses Ctrl + Z, then releases it, outside the fields. */
function undo(): void {
  const init = { bubbles: true, cancelable: true, ctrlKey: true, key: "z" };
  const body = selectPageElement("body");

  body.dispatchEvent(new KeyboardEvent("keydown", init));
  body.dispatchEvent(new KeyboardEvent("keyup", init));
}

/**
 * Reads the shapes the history's present holds, what the last step recorded.
 *
 * @returns them; none without a history
 */
function readPresent(): readonly DrawingShape[] {
  return (PAGE_HISTORIES.get(document)?.present.drawing ?? []).map(({ shape }) => shape);
}

describe("a burst of arrows in one shape's fields, one step", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    // The page as it opens: the rectangle 120 × 80 at (0, 0), selected alone.
    openPage("");
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("[F44.AC2] makes the rectangle 150 wide for Up held 30 keys in Width, one Ctrl + Z giving 120", () => {
    holdKey("width", { key: "ArrowUp" }, 30);

    // 120 + 30 × 1, the step taken 500 ms after the release.
    expect(readShapeModel().width).toBe(150);
    expect(readPresent()).toMatchObject([{ width: 120 }]);

    vi.advanceTimersByTime(PAUSE);

    expect(readPresent()).toMatchObject([{ width: 150 }]);

    undo();

    expect(readShapeModel().width).toBe(120);
    expect(readValue("width")).toBe("120");
  });

  it("[F44.AC2] gives the rectangle back its height of 80 for Ctrl + Z within the pause", () => {
    holdKey("height", { key: "ArrowUp" }, 20);
    vi.advanceTimersByTime(PAUSE - 1);

    // 80 + 20, the burst open: Ctrl + Z records it first, then undoes it.
    expect(readShapeModel().height).toBe(100);

    undo();

    expect(readShapeModel().height).toBe(80);
    expect(readValue("height")).toBe("80");
  });

  it("[F44.AC2] takes one step for Shift + Up pressed twice in X, 100 ms apart", () => {
    holdKey("shape-x", { key: "ArrowUp", shiftKey: true }, 1);
    vi.advanceTimersByTime(100);
    holdKey("shape-x", { key: "ArrowUp", shiftKey: true }, 1);
    vi.advanceTimersByTime(PAUSE);

    // 0 + 10 + 10: one step, which one Ctrl + Z undoes.
    expect(readPresent()).toMatchObject([{ x: 20 }]);

    undo();

    expect(readShapeModel().x).toBe(0);
  });

  it("[F44.AC2] takes two steps for Up released in Width, then 150 typed and Enter", () => {
    holdKey("width", { key: "ArrowUp" }, 1);
    vi.advanceTimersByTime(100);
    // The first digit typed ends the burst, its step taken.
    sendKey("width", "keydown", { key: "1" });
    typeAndEnter("width", "150");
    vi.advanceTimersByTime(PAUSE);
    undo();

    expect(readShapeModel().width).toBe(121);

    undo();

    expect(readShapeModel().width).toBe(120);
  });
});

describe("a burst of arrows in a text's box, one step", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    clickElement("tool-text");
    clickAt("#canvas", { clientX: 400, clientY: 300 });
    typeTextContent("Text");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "200");
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("[F44.AC2] makes its box 205 wide for Up held 5 keys, one Ctrl + Z giving 200", () => {
    holdKey("text-box-width", { key: "ArrowUp" }, 5);
    vi.advanceTimersByTime(PAUSE);

    const model = readModel();

    expect(isDrawnText(model) ? model.boxWidth : NaN).toBe(205);

    undo();

    expect(readValue("text-box-width")).toBe("200");
  });
});
