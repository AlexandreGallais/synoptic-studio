// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readShapeModel,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeValue,
} from "../user-actions";

import { PAGE_VIEWS } from "./page-views";

import type { Point, View } from "../../src";
import type { PointerKeys } from "../user-actions";

// The canvas sits at (100, 50) in the window, 1 200 × 800: the offset (x, y) on the canvas is the
// window's (x + 100, y + 50). At the opening view, 100 % from (−40, −40), the drawing's point
// (x, y) is the offset (x + 40, y + 40).

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** The view of the examples at 200 %, (180, 130) at the canvas's top-left. */
const ZOOMED: View = { origin: { x: 180, y: 130 }, scale: 2 };

/** The pointer of the examples, (440, 340) on the canvas, in the window. */
const POINTER: Point = { x: 540, y: 390 };

/** What a wheel event of the examples sets: its deltas, their unit and the keys held. */
type Wheel = {
  /** Its horizontal delta. */
  readonly deltaX: number;
  /** Its vertical delta. */
  readonly deltaY: number;
  /** Its unit: 0 pixels, 1 lines, 2 pages. */
  readonly deltaMode: number;
  /** Whether Ctrl is held, or a pinch. */
  readonly ctrlKey: boolean;
  /** Whether Cmd is held. */
  readonly metaKey: boolean;
  /** Whether Shift is held. */
  readonly shiftKey: boolean;
};

/** A wheel turned alone toward the user by 100 pixels. */
const DOWN: Wheel = {
  ctrlKey: false,
  deltaMode: 0,
  deltaX: 0,
  deltaY: 100,
  metaKey: false,
  shiftKey: false,
};

/** A wheel turned away from the user by 100 pixels with Ctrl held. */
const ZOOM_IN: Wheel = { ...DOWN, ctrlKey: true, deltaY: -100 };

/**
 * Turns the wheel over the canvas, as the browser sends it: bubbling, cancelable.
 *
 * @param wheel - its deltas and keys
 * @param client - where the pointer is, in the window
 * @returns the event, telling whether its default was prevented
 */
function turnWheel(wheel: Wheel, client: Point): WheelEvent {
  const event = new WheelEvent("wheel", { ...wheel, bubbles: true, cancelable: true });

  // happy-dom's WheelEvent leaves out a mouse event's keys and place: set as a browser sets them.
  const mouse = {
    clientX: client.x,
    clientY: client.y,
    ctrlKey: wheel.ctrlKey,
    metaKey: wheel.metaKey,
    shiftKey: wheel.shiftKey,
  };

  for (const [key, value] of Object.entries(mouse)) {
    Object.defineProperty(event, key, { value });
  }

  selectPageElement("#canvas").dispatchEvent(event);

  return event;
}

/**
 * Shows the view at 200 %, (180, 130) at the canvas's top-left, as the page would after a zoom:
 * kept, then the page redrawn as the window resized.
 */
function showZoomed(): void {
  PAGE_VIEWS.set(document, ZOOMED);
  globalThis.dispatchEvent(new Event("resize"));
}

/**
 * Reads the `viewBox` of an SVG of the page.
 *
 * @param selector - `#canvas svg`, the drawing, or `#overlay svg`, the layer above it
 * @returns its `viewBox` attribute
 */
function readViewBox(selector: string): string | null | undefined {
  return document.querySelector(selector)?.getAttribute("viewBox");
}

/**
 * Asserts the `viewBox` both SVGs show, the drawing and the layer above it.
 *
 * @param expected - the `viewBox` computed by hand
 */
function expectViewBox(expected: string): void {
  expect(readViewBox("#canvas svg")).toBe(expected);
  expect(readViewBox("#overlay svg")).toBe(expected);
}

/**
 * Presses a pointer button on the canvas at the pointer of the examples, drags it by (100, 50) and
 * releases it, as the browser sends them: a click after the main button's release.
 *
 * @param button - 0 the main button, 1 the middle one
 */
function dragByPanExample(button: number): void {
  const canvas = selectPageElement("#canvas");
  const buttons = button === 0 ? 1 : 4;
  const to = { x: POINTER.x + 100, y: POINTER.y + 50 };

  for (const [type, place, held] of [
    ["pointerdown", POINTER, buttons],
    ["pointermove", to, buttons],
    ["pointerup", to, 0],
  ] as const) {
    canvas.dispatchEvent(
      new PointerEvent(type, {
        bubbles: true,
        button,
        buttons: held,
        clientX: place.x,
        clientY: place.y,
        pointerId: 1,
      }),
    );
  }

  if (button === 0) {
    canvas.dispatchEvent(new MouseEvent("click", { bubbles: true, clientX: to.x, clientY: to.y }));
  }
}

/**
 * Presses or releases Space on an element of the page, as the browser sends it.
 *
 * @param type - `keydown` or `keyup`
 * @param selector - CSS selector of the element the focus is on, `body` for none
 * @returns the event, telling whether its default was prevented
 */
function pressSpace(type: string, selector: string): KeyboardEvent {
  const event = new KeyboardEvent(type, {
    bubbles: true,
    cancelable: true,
    code: "Space",
    key: " ",
  });

  selectPageElement(selector).dispatchEvent(event);

  return event;
}

/**
 * Reads the outline of the shape selected, in the layer above the drawing.
 *
 * @returns its path data; undefined while none is selected
 */
function readSelectionOutline(): string | undefined {
  return document.querySelector("#overlay svg path.selection")?.getAttribute("d") ?? undefined;
}

/**
 * Reads the outline of the shape under the pointer, in the layer above the drawing.
 *
 * @returns its path data; undefined for none
 */
function readHoverOutline(): string | undefined {
  return document.querySelector("#overlay svg path.hover")?.getAttribute("d") ?? undefined;
}

/**
 * Reads the path of the shape drawn, the editor's frames, boxes and handles left out.
 *
 * @returns its `d`
 */
function readShapePath(): string | null | undefined {
  return document.querySelector("#canvas svg > path:not([class])")?.getAttribute("d");
}

describe("zoom and pan by the wheel, a pinch and a drag (US-078)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    // A, 100 × 50 at (10, 20), sharp, selected.
    typeValue("radius", "0");
    typeValue("width", "100");
    typeValue("height", "50");
    typeValue("shape-x", "10");
    typeValue("shape-y", "20");
  });

  it("[F42.AC1] opens at 100 %, (−40, −40) at the canvas's top-left", () => {
    expectViewBox("-40 -40 1200 800");
  });

  it("[F42.AC1] zooms to 157 % about the pointer by Ctrl and the wheel, the page not zoomed", () => {
    const event = turnWheel(ZOOM_IN, POINTER);

    // × (1 + 0.0057 · 100) = 1.57; o′ = (400 − 440 / 1.57, 300 − 340 / 1.57)
    // = (119.74522, 83.43949); 1 200 / 1.57 = 764.33121, 800 / 1.57 = 509.55414.
    expect(event.defaultPrevented).toBe(true);
    expectViewBox("119.74522 83.43949 764.33121 509.55414");

    // The pointer still over (400, 300): a rectangle placed there is centred on it.
    clickElement("tool-rectangle");

    selectPageElement("#canvas").dispatchEvent(
      new MouseEvent("click", { bubbles: true, clientX: POINTER.x, clientY: POINTER.y }),
    );

    expect(readShapeModel()).toMatchObject({ height: 100, width: 100, x: 350, y: 250 });
  });

  it.each([
    { name: "a pinch, a wheel with ctrlKey", wheel: ZOOM_IN },
    { name: "Cmd and the wheel", wheel: { ...ZOOM_IN, ctrlKey: false, metaKey: true } },
    { name: "2.5 lines of 40 pixels", wheel: { ...ZOOM_IN, deltaMode: 1, deltaY: -2.5 } },
    { name: "0.125 page of 800 pixels", wheel: { ...ZOOM_IN, deltaMode: 2, deltaY: -0.125 } },
  ])("[F42.AC1] zooms the same by $name", ({ wheel }) => {
    turnWheel(wheel, POINTER);

    expectViewBox("119.74522 83.43949 764.33121 509.55414");
  });

  it("[F42.AC1] zooms once a frame by the deltas' sum: two of −50 give 157 %", () => {
    const frames: FrameRequestCallback[] = [];

    Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
      configurable: true,
      value: (callback: FrameRequestCallback): number => {
        frames.push(callback);

        return frames.length;
      },
    });

    turnWheel({ ...ZOOM_IN, deltaY: -50 }, POINTER);
    turnWheel({ ...ZOOM_IN, deltaY: -50 }, POINTER);

    // One frame asked, the view unchanged until it runs.
    expect(frames).toHaveLength(1);
    expectViewBox("-40 -40 1200 800");

    for (const frame of frames.splice(0)) {
      frame(0);
    }

    // × (1 + 0.0057 · 100) = 1.57, not (1 + 0.0057 · 50)² = 1.65102.
    expectViewBox("119.74522 83.43949 764.33121 509.55414");
  });

  it("[F42.AC1] zooms out from 200 % by a deltaY of 100, about the pointer", () => {
    showZoomed();
    turnWheel({ ...ZOOM_IN, deltaY: 100 }, POINTER);

    // 2 / 1.57 = 1.27389; o′ = (400 − 440 / 1.27389, 300 − 340 / 1.27389) = (54.6, 33.1);
    // 1 200 / 1.27389 = 942, 800 / 1.27389 = 628.
    expectViewBox("54.6 33.1 942 628");
  });

  it("[F42.AC2] pans 50 units down by the wheel alone at 200 %, across with Shift", () => {
    showZoomed();

    const event = turnWheel(DOWN, POINTER);

    // o′ = o + δ / s = (180, 130 + 100 / 2).
    expect(event.defaultPrevented).toBe(true);
    expectViewBox("180 180 600 400");

    showZoomed();
    turnWheel({ ...DOWN, shiftKey: true }, POINTER);
    // With Shift, the vertical delta across: (180 + 100 / 2, 130).
    expectViewBox("230 130 600 400");

    showZoomed();
    turnWheel({ ...DOWN, deltaX: 100, deltaY: 0, shiftKey: true }, POINTER);
    // macOS sends it already across, as deltaX: the same.
    expectViewBox("230 130 600 400");

    showZoomed();
    turnWheel({ ...DOWN, deltaMode: 1, deltaY: 2.5 }, POINTER);
    // 2.5 lines of 40 pixels: 100 pixels, 50 units.
    expectViewBox("180 180 600 400");
  });

  it("[F42.AC2] pans by a drag with Space held, nothing selected nor moved", () => {
    showZoomed();

    const down = pressSpace("keydown", "body");

    dragByPanExample(0);

    const up = pressSpace("keyup", "body");

    // o′ = o − d / s = (180 − 100 / 2, 130 − 50 / 2) = (130, 105).
    expect(down.defaultPrevented).toBe(true);
    expect(up.defaultPrevented).toBe(true);
    expectViewBox("130 105 600 400");
    expect(readSelectionOutline()).toBeDefined();
    expect(readShapeModel()).toMatchObject({ height: 50, width: 100, x: 10, y: 20 });

    // Space released: the same drag selects by a rectangle, the empty place leaving none selected.
    dragByPanExample(0);
    expectViewBox("130 105 600 400");
    expect(readSelectionOutline()).toBeUndefined();
  });

  it("[F42.AC2] pans by a drag of the middle button, nothing selected", () => {
    showZoomed();
    dragByPanExample(1);

    expectViewBox("130 105 600 400");
    expect(readSelectionOutline()).toBeDefined();
  });

  it("[F42.AC2] leaves Space typed in a field to the field, a drag then no pan", () => {
    showZoomed();

    const down = pressSpace("keydown", "#width");

    dragByPanExample(0);

    expect(down.defaultPrevented).toBe(false);
    expectViewBox("180 130 600 400");
  });

  it("[F42.AC4] changes no number of A, takes no step, and Ctrl + Z leaves the view", () => {
    // A key released outside the fields records A as typed: the history's last step.
    document.body.dispatchEvent(new KeyboardEvent("keyup", { bubbles: true, key: "Shift" }));

    const path = readShapePath();
    const steps = readText("#history-list");

    turnWheel(ZOOM_IN, POINTER);
    turnWheel(DOWN, POINTER);
    pressSpace("keydown", "body");
    dragByPanExample(0);
    pressSpace("keyup", "body");

    expect(readShapePath()).toBe(path);

    expect(["shape-x", "shape-y", "width", "height"].map((id) => readValue(id))).toEqual([
      "10",
      "20",
      "100",
      "50",
    ]);

    expect(readText("#history-list")).toBe(steps);

    // A nudged right, a step; Ctrl + Z gives it back, the view as it was.
    const viewBox = readViewBox("#canvas svg");

    for (const type of ["keydown", "keyup"]) {
      document.body.dispatchEvent(
        new KeyboardEvent(type, { bubbles: true, cancelable: true, key: "ArrowRight" }),
      );
    }

    expect(readShapeModel().x).toBe(11);

    document.body.dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ctrlKey: true, key: "z" }),
    );

    expect(readShapeModel().x).toBe(10);
    expectViewBox(viewBox ?? "");
  });

  it("[F42.AC1] keeps A under the pointer while it is moved and the wheel zooms", () => {
    // Pressed in A at (50, 40), moved to (60, 40): A at (20, 20), not landed yet.
    dispatchDrawingPointer("pointerdown", { x: 50, y: 40 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 60, y: 40 }, NO_KEY);
    expect(readShapeModel()).toMatchObject({ x: 20, y: 20 });

    // Zoomed to 157 % about the pointer, the offset (100, 80): (60, 40) still under it.
    turnWheel(ZOOM_IN, { x: 200, y: 130 });
    expect(readShapeModel()).toMatchObject({ x: 20, y: 20 });

    // 15.7 pixels right is 15.7 / 1.57 = 10 units: A lands at (30, 20).
    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointermove", { bubbles: true, buttons: 1, clientX: 215.7, clientY: 130 }),
    );

    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointerup", { bubbles: true, clientX: 215.7, clientY: 130 }),
    );

    expect(readShapeModel()).toMatchObject({ x: 30, y: 20 });
  });

  it("outlines the shape under the pointer in the new view, the pointer still", () => {
    // Over A's (50, 40), the offset (90, 80): A outlined.
    dispatchDrawingPointer("pointermove", { x: 50, y: 40 }, NO_KEY);
    expect(readHoverOutline()).toBeDefined();

    // The view panned 100 down: (50, 140) under the pointer, empty.
    turnWheel(DOWN, { x: 190, y: 130 });
    expect(readHoverOutline()).toBeUndefined();

    // Back up: A under it again.
    turnWheel({ ...DOWN, deltaY: -100 }, { x: 190, y: 130 });
    expect(readHoverOutline()).toBeDefined();
  });

  it.each(["tool-reset", "guide-next"])("brings back the opening view by #%s", (id) => {
    turnWheel(ZOOM_IN, POINTER);
    clickElement(id);

    expectViewBox("-40 -40 1200 800");
  });
});
