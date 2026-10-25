// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { DRAWING_KEYS, readStoredDrawing } from "../kept-shapes";
import {
  chooseShapeKind,
  clickAt,
  clickElement,
  commitField,
  openPage,
  pressPointer,
  readShapeModel,
  readText,
  setElementBox,
  typeAndEnter,
} from "../user-actions";

import { drawPendingRedraw } from "./draw-pending-redraw";
import { PENDING_REDRAWS } from "./pending-redraws";
import { scheduleRedraw } from "./schedule-redraw";

/** The animation frames asked and not run yet, as the browser holds them until its next repaint. */
const FRAMES: FrameRequestCallback[] = [];

/**
 * Runs the frames asked, as the browser does before its next repaint.
 */
function runFrames(): void {
  const asked = FRAMES.splice(0);

  for (const frame of asked) {
    frame(0);
  }
}

/**
 * Holds the animation frames the page asks from here on, as a browser holds them until its next
 * repaint, so that the test runs them (`runFrames`).
 */
function holdFrames(): void {
  Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
    configurable: true,
    value: (callback: FrameRequestCallback): number => {
      FRAMES.push(callback);

      return FRAMES.length;
    },
  });
}

/**
 * Reads the height of the measured block shown in the pipeline column.
 *
 * @returns its height
 */
function readBlockHeight(): number {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by showText from a TextBlock
  return (JSON.parse(readText("#contour")) as { readonly height: number }).height;
}

describe("A redraw per animation frame (REF-MDN-RAF)", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");

    // Two lines typed, as the page opens with them in a browser, which drops the line break
    // right after `<textarea>`; happy-dom keeps it.
    const area = document.querySelector("#text-content");

    if (area instanceof HTMLTextAreaElement) {
      area.value = "Pump P-101\nRunning";
    }

    typeAndEnter("text-size", "16");

    // From here, the frames wait for the test to run them, as in a browser.
    holdFrames();
  });

  it("[F17.AC5] gathers 30 sizes typed before a frame into one redraw, the last size drawn", () => {
    const before = readBlockHeight();

    const sizes = Array.from({ length: 30 }, (unused, index) => 17 + index);

    for (const size of sizes) {
      typeAndEnter("text-size", String(size));
    }

    // One frame asked for the 30 inputs, none drawn yet: no redraw waits behind another.
    expect(FRAMES).toHaveLength(1);
    expect(readBlockHeight()).toBe(before);

    runFrames();

    // Two lines of Inter, each 2478 × size / 2048 tall: 38.71875 at 16, 111.31640625 at 46.
    expect(before).toBe(38.71875);
    expect(readBlockHeight()).toBe(111.31640625);
    expect(FRAMES).toHaveLength(0);
  });

  it("[F17.AC5] asks a new frame for a value typed after the frame ran", () => {
    typeAndEnter("text-size", "20");
    runFrames();
    typeAndEnter("text-size", "32");

    expect(FRAMES).toHaveLength(1);
    runFrames();
    // Two lines of 2478 × 32 / 2048 = 38.71875: 77.4375.
    expect(readBlockHeight()).toBe(77.4375);
  });

  it("redraws at once a document without a window", () => {
    const calls: string[] = [];
    const detached = document.implementation.createHTMLDocument("");

    scheduleRedraw(detached, () => {
      calls.push("draw");
    });

    expect(detached.defaultView).toBeNull();
    expect(calls).toEqual(["draw"]);
  });
});

/**
 * Presses the drawing at a point of the window and releases it there, as a mouse does, the Y field
 * focused left as the button goes down: `pointerdown`, the field's `change`, `pointerup`, `click`.
 *
 * @param at - the window's point pressed
 * @param at.clientX - its x in the window
 * @param at.clientY - its y in the window
 */
function clickLeavingY(at: { readonly clientX: number; readonly clientY: number }): void {
  pressPointer("#canvas", "pointerdown", { ...at, buttons: 1 });
  commitField("shape-y");
  pressPointer("#canvas", "pointerup", { ...at, buttons: 0 });
  clickAt("#canvas", at);
}

/**
 * Reads the box of each shape kept, the first drawn first, its position and its own size.
 *
 * @returns x, y, width and height of each, every shape kept below as none is selected
 */
function readKeptBoxes(): readonly (readonly number[])[] {
  return readStoredDrawing(document, DRAWING_KEYS.below).map(({ shape }) =>
    "content" in shape ? [shape.x, shape.y] : [shape.x, shape.y, shape.width, shape.height],
  );
}

describe("a value typed then the drawing clicked within the same frame", () => {
  // The canvas sits at (100, 50) in the window and the drawing's origin 40 right of and below its
  // corner at the opening view: the drawing's point (x, y) is the window's (x + 140, y + 90).
  beforeEach(() => {
    // The page as it opens, a rectangle 120 × 80 at (0, 0), A; then B placed by a click at
    // (250, 45), centred: 100 × 100 at (250 − 50, 45 − 50) = (200, −5), edited.
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    clickElement("tool-rectangle");
    clickAt("#canvas", { clientX: 390, clientY: 135 });
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 200, y: -5 });

    holdFrames();
    typeAndEnter("shape-y", "20");
    expect(FRAMES).toHaveLength(1);
  });

  afterEach(() => {
    FRAMES.length = 0;
  });

  it("[F43.AC4] keeps B at y 20 when the empty drawing is clicked within the frame", () => {
    // (600, 400) of the drawing, the window's (740, 490): no shape there.
    clickLeavingY({ clientX: 740, clientY: 490 });

    // Drawn at the release, before the click selects none: B at y 20 before any frame runs.
    expect(readKeptBoxes()).toEqual([
      [0, 0, 120, 80],
      [200, 20, 100, 100],
    ]);

    runFrames();

    expect(readKeptBoxes()).toEqual([
      [0, 0, 120, 80],
      [200, 20, 100, 100],
    ]);

    expect(readText("#status")).toMatch(/^Click a shape/u);
  });

  it("[F43.AC4] keeps B at y 20 when A is clicked within the frame, A then edited", () => {
    // (60, 40) of the drawing, inside A, the window's (200, 130).
    clickLeavingY({ clientX: 200, clientY: 130 });
    runFrames();

    expect(readShapeModel()).toMatchObject({ height: 80, width: 120, x: 0, y: 0 });

    expect(readStoredDrawing(document, DRAWING_KEYS.above).map(({ shape }) => shape)).toEqual([
      expect.objectContaining({ height: 100, width: 100, x: 200, y: 20 }),
    ]);
  });

  it("[F43.AC4] keeps B at y 20 when a tool's click places a shape within the frame", () => {
    clickElement("tool-round");
    expect(FRAMES).toHaveLength(1);
    // (600, 400) of the drawing: a round shape 100 × 100 at (550, 350).
    clickLeavingY({ clientX: 740, clientY: 490 });
    runFrames();

    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 550, y: 350 });

    expect(readStoredDrawing(document, DRAWING_KEYS.below).map(({ shape }) => shape)).toEqual([
      expect.objectContaining({ x: 0, y: 0 }),
      expect.objectContaining({ height: 100, width: 100, x: 200, y: 20 }),
    ]);
  });

  it("[F43.AC4] keeps B at y 20 when a selection rectangle is released within the frame", () => {
    // From (−30, −30) of the drawing, empty, to (130, 90), over A alone: the window's (110, 60)
    // to (270, 180).
    pressPointer("#canvas", "pointerdown", { buttons: 1, clientX: 110, clientY: 60 });
    commitField("shape-y");
    pressPointer("#canvas", "pointermove", { buttons: 1, clientX: 270, clientY: 180 });
    pressPointer("#canvas", "pointerup", { buttons: 0, clientX: 270, clientY: 180 });
    runFrames();

    expect(readShapeModel()).toMatchObject({ height: 80, width: 120, x: 0, y: 0 });

    expect(readStoredDrawing(document, DRAWING_KEYS.above).map(({ shape }) => shape)).toEqual([
      expect.objectContaining({ height: 100, width: 100, x: 200, y: 20 }),
    ]);
  });

  it.each(["pointerup", "click"])(
    "[F43.AC4] draws the redraw at %s before any listener of the canvas or inside it acts",
    (type) => {
      // The drawing's layer, inside the canvas: a listener there hears the event before the
      // canvas's own listeners, after those the canvas holds in the capture phase.
      const inside = document.querySelector("#canvas > *");
      const pending: boolean[] = [];

      inside?.addEventListener(type, () => {
        pending.push(PENDING_REDRAWS.has(document));
      });

      inside?.dispatchEvent(new MouseEvent(type, { bubbles: true, clientX: 740, clientY: 490 }));

      expect(inside).not.toBeNull();
      expect(pending).toEqual([false]);
    },
  );

  it("[F43.AC4] forgets at a mount the redraw the mount before asked, a redraw then asked anew", () => {
    openPage("");
    holdFrames();

    // Drawn by the mount itself; the frame the mount before asked then finds nothing to draw.
    expect(PENDING_REDRAWS.has(document)).toBe(false);

    typeAndEnter("shape-x", "30");

    // A frame of its own: the redraw asked before the mount no longer swallows it.
    expect(PENDING_REDRAWS.has(document)).toBe(true);
    expect(FRAMES).toHaveLength(2);
  });

  it("[F43.AC4] draws a redraw asked once, at once, its frame then finding nothing to draw", () => {
    runFrames();

    const calls: string[] = [];

    scheduleRedraw(document, () => {
      calls.push("draw");
    });

    drawPendingRedraw(document);
    expect(calls).toEqual(["draw"]);

    runFrames();
    drawPendingRedraw(document);
    expect(calls).toEqual(["draw"]);
  });
});
