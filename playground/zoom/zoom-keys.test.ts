// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  clickAt,
  clickElement,
  openServedPage,
  readText,
  readValue,
  runResizeObservers,
  selectPageElement,
  setElementBox,
  typeAndEnter,
} from "../user-actions";
import { PAGE_VIEWS } from "../view";

import type { Point, View } from "../../src";

// The canvas sits at (100, 50) in the window, 1 200 × 800: the offset (x, y) on the canvas is the
// window's (x + 100, y + 50). At the opening view, 100 % from (−40, −40), the drawing's point
// (x, y) is the window's (x + 140, y + 90). Its centre, the offset (600, 400), is over (560, 360).

/** The view of the examples at 200 %, (180, 130) at the canvas's top-left. */
const ZOOMED: View = { origin: { x: 180, y: 130 }, scale: 2 };

/** The pointer of the examples, (440, 340) on the canvas, in the window: over (400, 300). */
const POINTER: Point = { x: 540, y: 390 };

/** The opening view's `viewBox`. */
const OPENING = "-40 -40 1200 800";

/** The keys of the examples, as the browser sends them. */
const KEYS = {
  actual: { code: "Digit0", key: ")", shiftKey: true },
  drawing: { code: "Digit1", key: "!", shiftKey: true },
  in: { code: "Equal", key: "+", shiftKey: true },
  out: { code: "Minus", key: "-", shiftKey: false },
  selection: { code: "Digit2", key: "@", shiftKey: true },
} as const;

/**
 * Presses a key on an element of the page, as the browser sends it: bubbling, cancelable.
 *
 * @param selector - the element the focus is on, `body` for none
 * @param init - its key, its code and the keys held
 * @returns the event, telling whether its default was prevented
 */
function pressZoomKey(selector: string, init: KeyboardEventInit): KeyboardEvent {
  const event = new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true });

  selectPageElement(selector).dispatchEvent(event);

  return event;
}

/**
 * Shows a view, as the page would after a zoom: kept, then the page redrawn as the canvas resized.
 *
 * @param view - the view shown
 */
function showView(view: View): void {
  PAGE_VIEWS.set(document, view);
  runResizeObservers("#canvas");
}

/**
 * Moves the pointer over the canvas, at the pointer of the examples.
 */
function pointAtExample(): void {
  selectPageElement("#canvas").dispatchEvent(
    new PointerEvent("pointermove", { bubbles: true, clientX: POINTER.x, clientY: POINTER.y }),
  );
}

/**
 * Asserts the `viewBox` both SVGs show, the drawing and the layer above it.
 *
 * @param expected - the `viewBox` computed by hand
 */
function expectViewBox(expected: string): void {
  for (const selector of ["#canvas svg", "#overlay svg"]) {
    expect(document.querySelector(selector)?.getAttribute("viewBox")).toBe(expected);
  }
}

/**
 * Asserts the zoom the toolbar shows, on the menu's button and in its field.
 *
 * @param percentage - the percentage, rounded by hand
 */
function expectZoomShown(percentage: number): void {
  expect(readText("#zoom-percentage")).toBe(`${String(percentage)} %`);
  expect(readValue("zoom-field")).toBe(String(percentage));
}

/**
 * Places a rectangle by a click on an empty place of the drawing at the opening view, then types
 * its size and place: selected alone, as a shape placed is.
 *
 * @param at - its top-left, 100 × 50 typed
 */
function placeRectangle(at: Point): void {
  clickElement("tool-rectangle");
  // (800, 600) of the drawing, under no shape of the examples.
  clickAt("#canvas", { clientX: 940, clientY: 690 });
  typeAndEnter("width", "100");
  typeAndEnter("height", "50");
  typeAndEnter("shape-x", String(at.x));
  typeAndEnter("shape-y", String(at.y));
}

describe("the zoom's keys, the fits and the percentage shown (US-079)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    // A, 100 × 50 at (10, 20), sharp, selected.
    typeAndEnter("radius", "0");
    typeAndEnter("width", "100");
    typeAndEnter("height", "50");
    typeAndEnter("shape-x", "10");
    typeAndEnter("shape-y", "20");
  });

  it("[F42.AC3] zooms in by a step about the pointer over the drawing by +, 141 % shown", () => {
    pointAtExample();

    const event = pressZoomKey("body", KEYS.in);

    // s′ = √2; o′ = (400 − 440 / √2, 300 − 340 / √2) = (88.87302, 59.58369);
    // 1 200 / √2 = 848.52814, 800 / √2 = 565.68542; 100 · √2 = 141.42, 141 %.
    expect(event.defaultPrevented).toBe(true);
    expectViewBox("88.87302 59.58369 848.52814 565.68542");
    expectZoomShown(141);
  });

  it("[F42.AC3] zooms in by =, as Inkscape, the same step", () => {
    pointAtExample();
    pressZoomKey("body", { code: "Equal", key: "=", shiftKey: false });

    expectViewBox("88.87302 59.58369 848.52814 565.68542");
  });

  it("[F42.AC3] zooms out by a step about the pointer by -, 71 % shown", () => {
    pointAtExample();
    pressZoomKey("body", KEYS.out);

    // s′ = 1 / √2; o′ = (400 − 440 · √2, 300 − 340 · √2) = (−222.25397, −180.83261);
    // 1 200 · √2 = 1 697.05627, 800 · √2 = 1 131.37085; 100 / √2 = 70.71, 71 %.
    expectViewBox("-222.25397 -180.83261 1697.05627 1131.37085");
    expectZoomShown(71);
  });

  it("[F42.AC3] zooms about the canvas's centre once the pointer has left the drawing", () => {
    pointAtExample();
    selectPageElement("#canvas").dispatchEvent(new PointerEvent("pointerleave"));
    pressZoomKey("body", KEYS.in);

    // The offset (600, 400) over (560, 360): o′ = (560 − 600 / √2, 360 − 400 / √2)
    // = (135.73593, 77.15729).
    expectViewBox("135.73593 77.15729 848.52814 565.68542");
  });

  it("[F42.AC3] changes nothing by + at the upper bound, 25 600 %", () => {
    showView({ origin: { x: 0, y: 0 }, scale: 256 });
    pointAtExample();
    pressZoomKey("body", KEYS.in);

    // 1 200 / 256 = 4.6875, 800 / 256 = 3.125: the view as it was.
    expectViewBox("0 0 4.6875 3.125");
    expectZoomShown(25_600);
  });

  it("[F42.AC3] gives 100 % about the canvas's centre by Shift + 0, whatever the pointer", () => {
    showView(ZOOMED);
    pointAtExample();

    const event = pressZoomKey("body", KEYS.actual);

    // The offset (600, 400) over (180 + 600 / 2, 130 + 400 / 2) = (480, 330): o′ = (−120, −70).
    expect(event.defaultPrevented).toBe(true);
    expectViewBox("-120 -70 1200 800");
    expectZoomShown(100);
  });

  it("[F42.AC3] fits A, B and C by Shift + 1, 40 pixels on each side, 280 %", () => {
    placeRectangle({ x: 310, y: 170 });
    placeRectangle({ x: 200, y: 20 });
    pressZoomKey("body", KEYS.drawing);

    // The box from (10, 20) to (410, 220): s = min(1 120 / 400, 720 / 200) = 2.8;
    // o = (210 − 600 / 2.8, 120 − 400 / 2.8) = (−4.28571, −22.85714);
    // 1 200 / 2.8 = 428.57143, 800 / 2.8 = 285.71429.
    expectViewBox("-4.28571 -22.85714 428.57143 285.71429");
    expectZoomShown(280);
  });

  it("[F42.AC3] fits B selected alone by Shift + 2, 1 120 %", () => {
    placeRectangle({ x: 310, y: 170 });
    placeRectangle({ x: 200, y: 20 });
    pressZoomKey("body", KEYS.selection);

    // B from (200, 20) to (300, 70): s = min(1 120 / 100, 720 / 50) = 11.2;
    // o = (250 − 600 / 11.2, 45 − 400 / 11.2) = (196.42857, 9.28571);
    // 1 200 / 11.2 = 107.14286, 800 / 11.2 = 71.42857.
    expectViewBox("196.42857 9.28571 107.14286 71.42857");
    expectZoomShown(1120);
  });

  it("[F42.AC3] changes nothing by Shift + 1 on an empty drawing", () => {
    // A selected, deleted: no shape left.
    pressZoomKey("body", { key: "Delete" });
    pressZoomKey("body", KEYS.drawing);

    expectViewBox(OPENING);
    expectZoomShown(100);
  });

  it("[F42.AC3] changes nothing by Shift + 2 with nothing selected", () => {
    pressZoomKey("body", { key: "Escape" });
    pressZoomKey("body", KEYS.selection);

    expectViewBox(OPENING);
  });

  it.each([
    { name: "+", selector: "#width", init: KEYS.in },
    { name: "-", selector: "#width", init: KEYS.out },
    { name: "Shift + 0", selector: "#width", init: KEYS.actual },
    { name: "Shift + 1", selector: "#shape-x", init: KEYS.drawing },
    { name: "+ in the zoom's field", selector: "#zoom-field", init: KEYS.in },
  ])("[F42.AC3] leaves $name typed in a field to the field", ({ init, selector }) => {
    showView(ZOOMED);
    pointAtExample();

    const event = pressZoomKey(selector, init);

    expect(event.defaultPrevented).toBe(false);
    expectViewBox("180 130 600 400");
  });

  it.each([
    { name: "Ctrl", init: { ...KEYS.in, ctrlKey: true } },
    { name: "Cmd", init: { ...KEYS.out, metaKey: true } },
    { name: "Alt", init: { ...KEYS.drawing, altKey: true } },
  ])("[F42.AC3] leaves a key with $name held to the browser", ({ init }) => {
    pointAtExample();

    const event = pressZoomKey("body", init);

    expect(event.defaultPrevented).toBe(false);
    expectViewBox(OPENING);
  });

  it("leaves a key a listener took before to it", () => {
    pointAtExample();

    const event = new KeyboardEvent("keydown", { ...KEYS.in, bubbles: true, cancelable: true });

    event.preventDefault();
    document.body.dispatchEvent(event);

    expectViewBox(OPENING);
  });

  it("opens showing 100 %, and Reset brings it back", () => {
    expectZoomShown(100);

    pointAtExample();
    pressZoomKey("body", KEYS.in);
    clickElement("tool-reset");

    expectViewBox(OPENING);
    expectZoomShown(100);
  });

  it("shows each item of the menu with its key", () => {
    const ids = ["zoom-in", "zoom-out", "zoom-actual", "zoom-fit-drawing", "zoom-fit-selection"];

    // Each its label and its key read, and its key given to assistive technologies.
    const items = ids.map((id) => {
      const button = selectPageElement(`#${id}`);
      const label = button.textContent.replaceAll(/\s+/gu, " ").trim();

      return [id, label, button.getAttribute("aria-keyshortcuts")];
    });

    expect(items).toEqual([
      ["zoom-in", "Zoom in +", "+"],
      ["zoom-out", "Zoom out -", "-"],
      ["zoom-actual", "Zoom to 100 % Shift + 0", "Shift+0"],
      ["zoom-fit-drawing", "Fit the drawing Shift + 1", "Shift+1"],
      ["zoom-fit-selection", "Fit the selection Shift + 2", "Shift+2"],
    ]);
  });

  it.each([
    // About the centre, (600, 400) over (560, 360): (560 − 600 / √2, 360 − 400 / √2).
    { expected: "135.73593 77.15729 848.52814 565.68542", id: "zoom-in", percentage: 141 },
    // (560 − 600 · √2, 360 − 400 · √2) = (−288.52814, −205.68542).
    { expected: "-288.52814 -205.68542 1697.05627 1131.37085", id: "zoom-out", percentage: 71 },
  ])(
    "zooms by $id about the canvas's centre and closes the menu",
    ({ expected, id, percentage }) => {
      // The pointer went from the drawing to the menu.
      pointAtExample();
      selectPageElement("#canvas").dispatchEvent(new PointerEvent("pointerleave"));
      selectPageElement("#zoom-box").setAttribute("open", "");
      clickElement(id);

      expectViewBox(expected);
      expectZoomShown(percentage);
      expect(selectPageElement("#zoom-box").hasAttribute("open")).toBe(false);
    },
  );

  it("gives 100 %, and fits the drawing and the selection, by the menu", () => {
    showView(ZOOMED);
    clickElement("zoom-actual");
    expectViewBox("-120 -70 1200 800");

    placeRectangle({ x: 310, y: 170 });
    placeRectangle({ x: 200, y: 20 });
    clickElement("zoom-fit-drawing");
    expectViewBox("-4.28571 -22.85714 428.57143 285.71429");

    clickElement("zoom-fit-selection");
    expectViewBox("196.42857 9.28571 107.14286 71.42857");
  });

  it("zooms to a percentage typed in the menu's field, about the canvas's centre", () => {
    typeAndEnter("zoom-field", "200");

    // (600, 400) over (560, 360): o′ = (560 − 600 / 2, 360 − 400 / 2) = (260, 160).
    expectViewBox("260 160 600 400");
    expectZoomShown(200);

    // 30 000 % taken at the bound, 25 600 %: o′ = (560 − 600 / 256, 360 − 400 / 256)
    // = (557.65625, 358.4375), still over the centre.
    typeAndEnter("zoom-field", "30000");
    expectViewBox("557.65625 358.4375 4.6875 3.125");
    expectZoomShown(25_600);

    // 0.5 %, above 0, taken at the lower bound, 1 % (AUD-044): o′ = (560 − 600 / 0.01, 360 − 400 /
    // 0.01) = (−59 440, −39 640); 1 200 / 0.01 = 120 000, 800 / 0.01 = 80 000.
    typeAndEnter("zoom-field", "0.5");
    expectViewBox("-59440 -39640 120000 80000");
    expectZoomShown(1);
  });

  it.each([
    { name: "a text", typed: "abc" },
    { name: "nothing", typed: "" },
    { name: "0", typed: "0" },
    { name: "a number below 0", typed: "-50" },
  ])("changes nothing by $name typed in the menu's field, the zoom shown again", ({ typed }) => {
    showView(ZOOMED);
    typeAndEnter("zoom-field", typed);

    expectViewBox("180 130 600 400");
    expectZoomShown(200);
  });
});
