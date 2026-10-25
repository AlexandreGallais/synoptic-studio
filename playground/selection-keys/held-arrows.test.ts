// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { isDrawnText } from "../../src";
import { PAGE_HISTORIES } from "../history";
import { readKeptDrawing } from "../kept-shapes";
import { openServedPage, selectPageElement, setElementBox, typeAndEnter } from "../user-actions";

import type { Point } from "../../src";

/**
 * Sends a key with the focus outside the fields, as the browser sends it: bubbling, cancelable.
 *
 * @param type - `keydown` or `keyup`
 * @param key - the key, as `KeyboardEvent.key` names it
 * @param init - the modifiers held and whether the key repeats
 */
function sendKey(type: string, key: string, init: KeyboardEventInit): void {
  selectPageElement("body").dispatchEvent(
    new KeyboardEvent(type, { ...init, bubbles: true, cancelable: true, key }),
  );
}

/**
 * Sends the keys of an arrow held, as the browser sends them: the first, then its repeats.
 *
 * @param key - the arrow, `ArrowRight`
 * @param count - how many keys, >= 1, the first and its repeats
 * @param isShift - whether Shift is held
 */
function holdArrow(key: string, count: number, isShift: boolean): void {
  const sent = Array.from({ length: count }).keys();

  for (const index of sent) {
    sendKey("keydown", key, { repeat: index > 0, shiftKey: isShift });
  }
}

/** Sends 5 repeats of Up already held, as the browser sends them once Right is released. */
function repeatUp(): void {
  const repeats = Array.from({ length: 5 }, () => true);

  for (const repeat of repeats) {
    sendKey("keydown", "ArrowUp", { repeat });
  }
}

/**
 * Reads where A, the shape the page opens with, is.
 *
 * @returns its x and y
 */
function readA(): Point {
  const shape = readKeptDrawing(document)[0]?.shape;

  return shape === undefined || isDrawnText(shape)
    ? { x: NaN, y: NaN }
    : { x: shape.x, y: shape.y };
}

/**
 * Reads where the history's present, what the last step recorded, has A.
 *
 * @returns its x and y
 */
function readPresentA(): Point {
  const shape = PAGE_HISTORIES.get(document)?.present.drawing[0]?.shape;

  return shape === undefined || isDrawnText(shape)
    ? { x: NaN, y: NaN }
    : { x: shape.x, y: shape.y };
}

/**
 * Opens the page, its rectangle selected and typed as A, sharp, 100 × 50 at (10, 20); then released,
 * as typed values are recorded by a key released outside the fields.
 */
function openWithA(): void {
  openServedPage("");
  setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  typeAndEnter("radius", "0");
  typeAndEnter("width", "100");
  typeAndEnter("height", "50");
  typeAndEnter("shape-x", "10");
  typeAndEnter("shape-y", "20");
  sendKey("keyup", "Escape", {});
}

describe("two arrows held move the selection diagonally", () => {
  beforeEach(() => {
    openWithA();
  });

  it("[F43.AC3] moves A by (+1, −1) at each key from Right's first, Up held: (39, −10)", () => {
    holdArrow("ArrowUp", 1, false);

    // Up's key alone: (10, 20 − 1).
    expect(readA()).toEqual({ x: 10, y: 19 });

    // The browser repeats Right's key alone, 29 keys: (10 + 29, 19 − 29).
    holdArrow("ArrowRight", 29, false);

    expect(readA()).toEqual({ x: 39, y: -10 });
  });

  it("[F43.AC3] moves A up only once Right is released, Up still held", () => {
    holdArrow("ArrowUp", 1, false);
    holdArrow("ArrowRight", 29, false);
    sendKey("keyup", "ArrowRight", {});
    repeatUp();

    // (39, −10), then 5 keys of Up alone: (39, −15).
    expect(readA()).toEqual({ x: 39, y: -15 });
  });

  it("[F43.AC3] cancels Left and Right held together", () => {
    holdArrow("ArrowLeft", 1, false);
    holdArrow("ArrowRight", 10, false);

    // Left's key alone, then (−1 + 1, 0) at each of Right's.
    expect(readA()).toEqual({ x: 9, y: 20 });
  });

  it("[F43.AC3] moves A by 10 on both axes with Shift", () => {
    holdArrow("ArrowUp", 1, true);
    holdArrow("ArrowRight", 3, true);

    // (10, 20 − 10), then 3 keys of (+10, −10): (40, −20).
    expect(readA()).toEqual({ x: 40, y: -20 });
  });

  it("[F43.AC3] holds no arrow once the window loses the focus", () => {
    holdArrow("ArrowUp", 1, false);
    document.defaultView?.dispatchEvent(new FocusEvent("blur"));
    holdArrow("ArrowRight", 1, false);

    // Up's key, then Right alone: Up's release, in another window, never heard.
    expect(readA()).toEqual({ x: 11, y: 19 });
  });

  it("[F43.AC3] holds no arrow pressed in a field, which keeps it", () => {
    selectPageElement("#width").dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key: "ArrowUp" }),
    );

    holdArrow("ArrowRight", 1, false);

    expect(readA()).toEqual({ x: 11, y: 20 });
  });

  it("[F43.AC3] holds no arrow left by the page mounted before", () => {
    holdArrow("ArrowUp", 1, false);
    openWithA();
    holdArrow("ArrowRight", 1, false);

    expect(readA()).toEqual({ x: 11, y: 20 });
  });

  it("[F43.AC3] takes one step from the first press to the last release", () => {
    holdArrow("ArrowUp", 1, false);
    holdArrow("ArrowRight", 29, false);
    sendKey("keyup", "ArrowRight", {});
    repeatUp();

    // Right released, Up still held: no step yet.
    expect(readPresentA()).toEqual({ x: 10, y: 20 });

    sendKey("keyup", "ArrowUp", {});

    expect(readPresentA()).toEqual({ x: 39, y: -15 });

    sendKey("keydown", "z", { ctrlKey: true });

    expect(readA()).toEqual({ x: 10, y: 20 });
  });

  it("[F43.AC3] holds no arrow pressed with Cmd, whose keyup macOS never sends", () => {
    // Cmd + Up, as macOS sends it: Cmd's key, Up's with Cmd held, Cmd released, Up's never.
    sendKey("keydown", "Meta", { metaKey: true });
    sendKey("keydown", "ArrowUp", { metaKey: true });
    holdArrow("ArrowRight", 1, false);
    sendKey("keyup", "ArrowRight", {});

    // Right alone, (10 + 1, 20), Up never held; its release taking the step.
    expect(readA()).toEqual({ x: 11, y: 20 });
    expect(readPresentA()).toEqual({ x: 11, y: 20 });
  });

  it.each(["ctrlKey", "altKey"])("[F43.AC3] holds no arrow pressed with %s", (modifier) => {
    sendKey("keydown", "ArrowUp", { [modifier]: true });
    holdArrow("ArrowRight", 1, false);

    expect(readA()).toEqual({ x: 11, y: 20 });
  });

  it("[F43.AC3] releases every arrow at Cmd pressed, their keyup never sent, the step taken", () => {
    holdArrow("ArrowUp", 1, false);
    holdArrow("ArrowRight", 2, false);
    // Cmd pressed while both are held, then both released and Cmd last: macOS sends Cmd's keyup.
    sendKey("keydown", "Meta", { metaKey: true });
    sendKey("keyup", "Meta", {});

    // Up's key, then 2 of both: (10 + 2, 20 − 1 − 2), recorded at Cmd's release.
    expect(readPresentA()).toEqual({ x: 12, y: 17 });

    holdArrow("ArrowLeft", 1, false);

    // Left alone: (11, 17).
    expect(readA()).toEqual({ x: 11, y: 17 });
  });

  it("[F43.AC3] releases every arrow at Cmd's keydown, its keyup taken by the system", () => {
    holdArrow("ArrowUp", 1, false);
    // Up released under Cmd, and Cmd's keyup taken by a shortcut: no keyup at all.
    sendKey("keydown", "Meta", { metaKey: true });
    holdArrow("ArrowRight", 1, false);

    // Up's key (10, 19), then Right alone: (10 + 1, 19), never (+1, −1) to (11, 18).
    expect(readA()).toEqual({ x: 11, y: 19 });
  });

  it.each(["pointerdown", "pointermove"])(
    "[F43.AC3] releases every arrow at a %s carrying Cmd, no keyup sent",
    (type) => {
      holdArrow("ArrowUp", 1, false);

      selectPageElement("#canvas").dispatchEvent(
        new PointerEvent(type, { bubbles: true, cancelable: true, metaKey: true, pointerId: 1 }),
      );

      holdArrow("ArrowRight", 1, false);

      // Up's key (10, 19), then Right alone: (11, 19).
      expect(readA()).toEqual({ x: 11, y: 19 });
    },
  );

  it("[F43.AC3] holds an arrow again at its repeat once Cmd is gone from it", () => {
    holdArrow("ArrowUp", 1, false);
    sendKey("keydown", "Meta", { metaKey: true });
    // Cmd released, its keyup lost, Up still down: its repeat comes without Cmd.
    sendKey("keydown", "ArrowUp", { repeat: true });
    holdArrow("ArrowRight", 1, false);

    // Up's two keys (10, 18), then Right with Up held: (10 + 1, 18 − 1).
    expect(readA()).toEqual({ x: 11, y: 17 });
  });

  it("[F43.AC3] takes the step at once when the window loses the focus, arrows held", () => {
    holdArrow("ArrowUp", 1, false);
    holdArrow("ArrowRight", 4, false);
    document.defaultView?.dispatchEvent(new FocusEvent("blur"));

    // (10 + 4, 20 − 1 − 4).
    expect(readPresentA()).toEqual({ x: 14, y: 15 });
  });
});
