// @vitest-environment happy-dom
import { beforeEach, describe, expect, it, vi } from "vitest";

import { mountPlayground } from "../mount";
import {
  WATCHED_ELEMENTS,
  clickElement,
  openPage,
  pressAndClick,
  pressPointer,
  readShapeModel,
  runResizeObservers,
  selectPageElement,
  setElementBox,
} from "../user-actions";

import { listenToCanvasSize } from "./listen-to-canvas-size";
import { PAGE_VIEWS } from "./page-views";

// A window of 2 560 × 1 000, the page as it opens: the canvas at the window's top-left, 1 960 ×
// 1 000, the panel and the pipeline column 300 pixels each beside it; the column hidden gives its
// 300 to the canvas, 2 260 wide (note 0033, §2).
// The page opens at the view 100 %, origin (−40, −40), its rectangle 120 × 80 at (0, 0) selected.

/** The animation frames asked and not run yet, as the browser holds them until its next repaint. */
const FRAMES: FrameRequestCallback[] = [];

/**
 * Sets the canvas's size, as the browser lays it out, then tells its observers, as the browser
 * does before the next paint.
 *
 * @param width - its width, in CSS pixels
 * @param height - its height, in CSS pixels
 */
function resizeCanvas(width: number, height: number): void {
  setElementBox("#canvas", { height, left: 0, top: 0, width });
  runResizeObservers("#canvas");
}

/**
 * Reads the `viewBox` of both SVGs of the page, the drawing and the layer above it.
 *
 * @returns the drawing's, then the layer above's
 */
function readViewBoxes(): readonly (string | null | undefined)[] {
  return ["#canvas svg", "#overlay svg"].map((selector) =>
    document.querySelector(selector)?.getAttribute("viewBox"),
  );
}

/**
 * Reads the paths of the shapes drawn, the editor's frames, boxes and handles left out.
 *
 * @returns their `d`, in the order drawn
 */
function readShapePaths(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas svg > path:not([class])")].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Holds the animation frames the page asks from here on, as a browser holds them until its next
 * repaint, so that the test runs them.
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

describe("the view follows the canvas's size (REF-RESIZE-OBSERVER-1)", () => {
  beforeEach(() => {
    openPage("");
    resizeCanvas(1960, 1000);
  });

  it("[F44.AC1] gives both SVGs the canvas's new size when Pipeline hides its column, then shows it", () => {
    // Opened: the canvas 1 960 × 1 000 at scale 1 from (−40, −40).
    expect(readViewBoxes()).toEqual(["-40 -40 1960 1000", "-40 -40 1960 1000"]);

    clickElement("tool-output");
    resizeCanvas(2260, 1000);

    // The canvas's aspect, one scale and nothing centred: (0, 0) at (0 + 40, 0 + 40) = (40, 40)
    // on screen and (120, 80) at (160, 120), where the drawing kept at 1 960 was centred
    // (2 260 − 1 960) / 2 = 150 pixels right, at (190, 40).
    expect(readViewBoxes()).toEqual(["-40 -40 2260 1000", "-40 -40 2260 1000"]);

    // The pointer over (100, 80) on screen, over (60, 40) inside the rectangle: its hover drawn.
    pressPointer("#canvas", "pointermove", { buttons: 0, clientX: 100, clientY: 80 });
    expect(document.querySelector("#overlay svg path.hover")).not.toBeNull();

    clickElement("tool-output");
    resizeCanvas(1960, 1000);
    expect(readViewBoxes()).toEqual(["-40 -40 1960 1000", "-40 -40 1960 1000"]);
  });

  it("keeps the view's scale and origin, the canvas showing more to the right", () => {
    PAGE_VIEWS.set(document, { origin: { x: 180, y: 130 }, scale: 2 });
    resizeCanvas(1960, 1000);
    // (180, 130, 1 960 / 2, 1 000 / 2).
    expect(readViewBoxes()).toEqual(["180 130 980 500", "180 130 980 500"]);

    clickElement("tool-output");
    resizeCanvas(2260, 1000);

    // (180, 130, 2 260 / 2, 1 000 / 2): 1 130 − 980 = 150 units more to the right.
    expect(readViewBoxes()).toEqual(["180 130 1130 500", "180 130 1130 500"]);
  });

  it("[F44.AC1] places a shape under the pointer and picks one, the column hidden", () => {
    clickElement("tool-output");
    resizeCanvas(2260, 1000);
    expect(readViewBoxes()).toEqual(["-40 -40 2260 1000", "-40 -40 2260 1000"]);

    const [first] = readShapePaths();

    clickElement("tool-rectangle");
    pressAndClick("#canvas", { clientX: 600, clientY: 600 });

    // Over (−40 + 600, −40 + 600) = (560, 560), a 100 × 100 centred on it at (510, 510).
    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 510, y: 510 });
    // The first rectangle not moved, the new one drawn after it.
    expect(readShapePaths()).toHaveLength(2);
    expect(readShapePaths()[0]).toBe(first);

    // The Selector armed again: (100, 80) on screen over (60, 40), inside the first rectangle.
    pressAndClick("#canvas", { clientX: 100, clientY: 80 });
    expect(readShapeModel()).toMatchObject({ height: 80, width: 120, x: 0, y: 0 });
  });

  it("gives both SVGs the canvas 1 700 × 800 when the window is resized to 2 300 × 800", () => {
    // 2 300 − 600 = 1 700 wide, 800 high.
    resizeCanvas(1700, 800);
    expect(readViewBoxes()).toEqual(["-40 -40 1700 800", "-40 -40 1700 800"]);
  });

  it("[F46.AC3] gives both SVGs the new size in the observer's callback, before any frame", () => {
    holdFrames();
    FRAMES.length = 0;

    // The column hidden, the canvas 2 260 wide: the callback runs before the browser paints
    // (REF-RESIZE-OBSERVER-1, §3.6.1), so the new `viewBox` is painted with the new box.
    clickElement("tool-output");
    resizeCanvas(2260, 1000);
    expect(readViewBoxes()).toEqual(["-40 -40 2260 1000", "-40 -40 2260 1000"]);

    clickElement("tool-output");
    resizeCanvas(1960, 1000);
    expect(readViewBoxes()).toEqual(["-40 -40 1960 1000", "-40 -40 1960 1000"]);

    // The window resized to 2 300 × 800: the canvas 1 700 × 800, no column toggled.
    resizeCanvas(1700, 800);
    expect(readViewBoxes()).toEqual(["-40 -40 1700 800", "-40 -40 1700 800"]);
  });

  it("[F46.AC3] leaves the frames asked before nothing to draw once the callback drew the page", () => {
    holdFrames();
    FRAMES.length = 0;

    clickElement("tool-output");
    resizeCanvas(2260, 1000);

    // The toggle's frame was asked; the callback drew it at once, so the frame draws nothing.
    expect(FRAMES).toHaveLength(1);
    selectPageElement("#canvas svg").setAttribute("viewBox", "0 0 1 1");

    for (const frame of FRAMES.splice(0)) {
      frame(0);
    }

    expect(readViewBoxes()).toEqual(["0 0 1 1", "-40 -40 2260 1000"]);
  });

  it("clips the stage, which no focus scrolls (REF-CSS-OVERFLOW-3)", () => {
    // `clip` is not a scroll container; `hidden` is, and a focus outside the stage's box scrolled
    // it by 266 pixels in Chromium (note 0033, §2).
    expect(getComputedStyle(selectPageElement("#stage")).overflow).toBe("clip");
  });
});

describe("the canvas's observer, mounted again", () => {
  it("[F44.AC1] redraws the page once for the canvas resized after 2 mounts of the same page", () => {
    openPage("");
    mountPlayground(document);
    setElementBox("#canvas", { height: 1000, left: 0, top: 0, width: 1960 });
    holdFrames();
    FRAMES.length = 0;

    // The first mount's observer disconnected by the second: the canvas watched once.
    expect(
      [...WATCHED_ELEMENTS].filter((watched) => watched.target === selectPageElement("#canvas")),
    ).toHaveLength(1);

    runResizeObservers("#canvas");
    expect(FRAMES).toHaveLength(1);
  });

  it("watches nothing in a document without a window", () => {
    const own = document.implementation.createHTMLDocument("");
    const before = WATCHED_ELEMENTS.size;
    const update = vi.fn<() => void>();

    listenToCanvasSize(own, update);

    expect(WATCHED_ELEMENTS.size).toBe(before);
    expect(update).not.toHaveBeenCalled();
  });
});
