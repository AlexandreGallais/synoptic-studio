// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import {
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readShapeModel,
  selectPageElement,
  setElementBox,
} from "../user-actions";
import { PAN_DRAGS } from "../view";

import type { Point } from "../../src";
import type { PointerKeys } from "../user-actions";

// The canvas sits at (100, 50) in the window, 800 × 600, the page at the opening view, 100 % from
// (−40, −40): the drawing's point (x, y) is the window's (x + 140, y + 90). The epic's step 1
// holds the rectangle 120 × 80 at (0, 0) alone: (300, 300) to (400, 400) is empty.

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** The press of the examples on the empty drawing. */
const PRESS: Point = { x: 300, y: 300 };

/** Where the examples move the pointer, the button held. */
const POINTER: Point = { x: 400, y: 400 };

/**
 * Presses the drawing with the main button and moves the pointer, not released.
 *
 * @param from - the point pressed
 * @param to - the point the pointer is moved to
 */
function pressAndMove(from: Point, to: Point): void {
  dispatchDrawingPointer("pointerdown", from, NO_KEY);
  dispatchDrawingPointer("pointermove", to, NO_KEY);
}

/**
 * Sends a key's press or release to the page, the focus on none of its fields.
 *
 * @param type - `keydown` or `keyup`
 * @param key - the key, `" "` for Space, `Meta` for Cmd
 * @param init - the event's other fields: `metaKey` while Cmd is held, `repeat` for a key repeated
 */
function sendKey(type: string, key: string, init: KeyboardEventInit): void {
  selectPageElement("body").dispatchEvent(
    new KeyboardEvent(type, { ...init, bubbles: true, cancelable: true, key }),
  );
}

/**
 * Presses Cmd, as the browser sends it: its `keydown` with `metaKey` set, a modifier's state
 * changing as it is pressed down (UI Events 4.3.1).
 */
function pressCommand(): void {
  sendKey("keydown", "Meta", { metaKey: true });
}

/**
 * Sends a pointer event of the main button to the canvas, Cmd held, at the window's (x, y).
 *
 * @param type - `pointermove` or `pointerdown`
 * @param client - the point in the window
 */
function sendPointerWithCommand(type: string, client: Point): void {
  selectPageElement("#canvas").dispatchEvent(
    new PointerEvent(type, {
      bubbles: true,
      button: 0,
      buttons: type === "pointerdown" ? 1 : 0,
      cancelable: true,
      clientX: client.x,
      clientY: client.y,
      metaKey: true,
      pointerId: 1,
    }),
  );
}

/**
 * Reads the selection rectangle's line in the layer above the drawing.
 *
 * @returns its path data; undefined when none is drawn
 */
function readRectangleLine(): string | undefined {
  return (
    document.querySelector("#overlay svg path.selection-rectangle")?.getAttribute("d") ?? undefined
  );
}

/**
 * Presses the empty drawing at (300, 300) and moves to (400, 400), then reads the rectangle drawn.
 *
 * @returns its line's path data, the box from the press to the pointer moved half a pixel inward
 */
function dragOnEmptyDrawing(): string | undefined {
  pressAndMove(PRESS, POINTER);

  return readRectangleLine();
}

/** The rectangle from (300, 300) to (400, 400), its line 1 pixel wide half a pixel inward. */
const RECTANGLE = "M300.5 300.5H399.5V399.5H300.5Z";

describe("a press left open, then the next press on the empty drawing", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  afterEach(() => {
    // The drag each example leaves open ended, a pan by its click, the others by a release, and
    // Space let go, so that the next example, on the same document, starts with none.
    dispatchDrawingPointer("click", POINTER, NO_KEY);
    dispatchDrawingPointer("pointerup", POINTER, NO_KEY);
    globalThis.dispatchEvent(new Event("blur"));
  });

  it("[F46.AC2] draws the rectangle on a plain drag, the Selector armed", () => {
    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F46.AC2] draws the rectangle after Space released while Cmd was held, no keyup sent", () => {
    // macOS sends no keyup for a key released while Cmd is held (REF-WEBKIT-CMD-KEYUP): Space
    // pressed, Cmd pressed, Space released, then Cmd released; only Cmd's keyup reaches the page.
    sendKey("keydown", " ", {});
    pressCommand();
    sendKey("keyup", "Meta", {});

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F47.AC1] draws the rectangle after Space then Cmd pressed, neither keyup heard", () => {
    // Cmd + Space is a system shortcut (REF-APPLE-SHORTCUTS): Space's and Cmd's keyups may never
    // reach the page; Cmd's keydown comes first, as a modifier's state changes as it is pressed.
    sendKey("keydown", " ", {});
    pressCommand();

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F47.AC1] draws the rectangle after Space repeated with Cmd held, no other key heard", () => {
    // Cmd's own keydown lost: Space's next repeat carries Cmd's state, and holds Space no more.
    sendKey("keydown", " ", {});
    sendKey("keydown", " ", { metaKey: true, repeat: true });

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F47.AC1] draws the rectangle after the pointer moved with Cmd held, no key heard", () => {
    // No key event at all after Space's press: the pointer's move carries Cmd's state.
    sendKey("keydown", " ", {});
    sendPointerWithCommand("pointermove", { x: 600, y: 200 });

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F47.AC1] pans no view at a press with Cmd held, Space's keydown the last key heard", () => {
    // The press itself carries Cmd: Space may have been let go under it, never heard.
    sendKey("keydown", " ", {});
    sendPointerWithCommand("pointerdown", PRESS);

    expect(PAN_DRAGS.has(document)).toBe(false);
  });

  it("[F47.AC1] holds Space again at its next repeat once Cmd is released", () => {
    // Space still down as Cmd is released: its next keydown, Cmd's state gone, holds it again.
    sendKey("keydown", " ", {});
    pressCommand();
    sendKey("keyup", "Meta", {});
    sendKey("keydown", " ", { repeat: true });
    dispatchDrawingPointer("pointerdown", PRESS, NO_KEY);

    expect(PAN_DRAGS.has(document)).toBe(true);
  });

  it("[F47.AC1] pans by a drag with Space really held, its repeats heard", () => {
    // Space held, repeated, then the press: a pan, no rectangle drawn.
    sendKey("keydown", " ", {});
    sendKey("keydown", " ", { repeat: true });

    expect(dragOnEmptyDrawing()).toBeUndefined();
    expect(PAN_DRAGS.has(document)).toBe(true);
  });

  it("[F46.AC2] draws the rectangle after Space held as the window lost the focus", () => {
    sendKey("keydown", " ", {});
    globalThis.dispatchEvent(new Event("blur"));

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F46.AC2] draws the rectangle after a drag whose release outside the window was lost", () => {
    // From (500, 100) to (600, 200), the window then losing the focus, no release heard.
    pressAndMove({ x: 500, y: 100 }, { x: 600, y: 200 });
    globalThis.dispatchEvent(new Event("blur"));

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F46.AC2] draws the rectangle after a pan of the middle button never released", () => {
    // The middle button pressed at the window's (640, 390), over (500, 300), and moved: a pan.
    for (const type of ["pointerdown", "pointermove"]) {
      selectPageElement("#canvas").dispatchEvent(
        new PointerEvent(type, {
          bubbles: true,
          button: 1,
          buttons: 4,
          clientX: 640,
          clientY: 390,
        }),
      );
    }

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F46.AC2] draws the rectangle after a drag cancelled", () => {
    pressAndMove({ x: 500, y: 100 }, { x: 600, y: 200 });
    dispatchDrawingPointer("pointercancel", { x: 600, y: 200 }, NO_KEY);

    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
  });

  it("[F46.AC2] leaves a shape drawn by a tool, its release lost, as it was at the next press", () => {
    // The rectangle tool dragged from (500, 100) to (600, 200), no release heard: a rectangle
    // 100 × 100 at (500, 100) drawn; then the Selector armed by its button.
    clickElement("tool-rectangle");
    pressAndMove({ x: 500, y: 100 }, { x: 600, y: 200 });
    clickElement("tool-selector");

    const drawn = readShapeModel();

    expect(drawn).toMatchObject({ height: 100, width: 100, x: 500, y: 100 });
    expect(dragOnEmptyDrawing()).toBe(RECTANGLE);
    // The drag left open dropped at the press: its shape does not follow the Selector's pointer.
    expect(readShapeModel()).toEqual(drawn);
  });
});
