// @vitest-environment happy-dom
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  chooseOption,
  clickElement,
  openServedPage,
  readModel,
  readPressedState,
  readShapeModel,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
} from "../user-actions";

import type { Point } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner (`DRAWING_MARGIN`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/**
 * Fires a pointer event on the canvas at a point of the drawing, as the browser does.
 *
 * @param type - `pointerdown`, `pointermove`, `pointerup`, `pointercancel` or `click`
 * @param point - the point of the drawing, possibly fractional
 * @param isShift - whether Shift is held
 */
function firePointer(type: string, point: Point, isShift: boolean): void {
  const init = {
    bubbles: true,
    buttons: type === "pointerdown" || type === "pointermove" ? 1 : 0,
    clientX: point.x + ORIGIN_LEFT,
    clientY: point.y + ORIGIN_TOP,
    pointerId: 1,
    shiftKey: isShift,
  };
  const event = type === "click" ? new MouseEvent(type, init) : new PointerEvent(type, init);

  selectPageElement("#canvas").dispatchEvent(event);
}

/**
 * Presses or releases Shift on the page, as the browser fires it to the focused element.
 *
 * @param type - `keydown` or `keyup`
 */
function fireShift(type: string): void {
  document.body.dispatchEvent(new KeyboardEvent(type, { bubbles: true, key: "Shift" }));
}

/**
 * Drags with a tool from one point of the drawing to another, then releases: the browser's
 * `pointerdown`, `pointermove`, `pointerup`, then the `click` it fires on the same element.
 *
 * @param tool - `rectangle`, `polygon`, `round` or `text`
 * @param drag - the point pressed, the point released, and whether Shift is held
 * @param drag.from - the point pressed
 * @param drag.to - the point released
 * @param drag.isShift - whether Shift is held all along
 */
function dragDrawing(
  tool: string,
  drag: { readonly from: Point; readonly isShift: boolean; readonly to: Point },
): void {
  clickElement(`tool-${tool}`);
  firePointer("pointerdown", drag.from, drag.isShift);
  firePointer("pointermove", drag.to, drag.isShift);
  firePointer("pointerup", drag.to, drag.isShift);
  firePointer("click", drag.to, drag.isShift);
}

/**
 * Reads the box of the shape the panel edits, from its model: the top-left of its box before it
 * turns (ADR-0035), its width and its height.
 *
 * @returns x, y, width and height, in that order
 */
function readBox(): readonly unknown[] {
  const model = readShapeModel();

  return [model.x, model.y, model.width, model.height];
}

/**
 * Counts the shapes drawn, the editor's frames, boxes and handles left out.
 *
 * @returns how many paths of shapes the drawing holds
 */
function countShapes(): number {
  return document.querySelectorAll("#canvas svg > path:not([class])").length;
}

describe("a drag with a tool draws the shape's box (US-064)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), whatever feature is being validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F36.AC1] adds a rectangle 120 × 80 at (10, 10) dragged from (10, 10) to (130, 90)", () => {
    const first = readText("#path-data");

    dragDrawing("rectangle", { from: { x: 10, y: 10 }, isShift: false, to: { x: 130, y: 90 } });

    // 130 − 10 = 120 wide, 90 − 10 = 80 high; the click after the release places nothing more.
    expect(readBox()).toEqual([10, 10, 120, 80]);
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(countShapes()).toBe(2);

    expect(document.querySelector("#canvas svg > path:not([class])")?.getAttribute("d")).toBe(
      first,
    );

    expect(readPressedState("tool-selector")).toBe("true");
  });

  it("[F36.AC1] draws the shape following the pointer while dragging", () => {
    clickElement("tool-rectangle");
    firePointer("pointerdown", { x: 10, y: 10 }, false);

    // Pressed only: nothing drawn yet.
    expect(countShapes()).toBe(1);

    firePointer("pointermove", { x: 70, y: 50 }, false);

    // 70 − 10 = 60, 50 − 10 = 40; the tool still armed until the release.
    expect([...readBox(), countShapes()]).toEqual([10, 10, 60, 40, 2]);
    expect(readPressedState("tool-rectangle")).toBe("true");

    firePointer("pointermove", { x: 130, y: 90 }, false);
    expect(readBox()).toEqual([10, 10, 120, 80]);
  });

  it("[F36.AC1] draws the box to the release, past the last move", () => {
    clickElement("tool-rectangle");
    firePointer("pointerdown", { x: 10, y: 10 }, false);
    firePointer("pointermove", { x: 70, y: 50 }, false);
    firePointer("pointerup", { x: 130, y: 90 }, false);

    expect(readBox()).toEqual([10, 10, 120, 80]);
  });

  it("[F36.AC1] adds the same rectangle dragged up and to the left, from (130, 90) to (10, 10)", () => {
    dragDrawing("rectangle", { from: { x: 130, y: 90 }, isShift: false, to: { x: 10, y: 10 } });

    expect(readBox()).toEqual([10, 10, 120, 80]);
  });

  it("[F36.AC1] puts a box dragged between integers on integers", () => {
    // (10.4, 9.6) rounds to (10, 10), (130.5, 90.49) to (131, 90): 121 × 80.
    dragDrawing("rectangle", {
      from: { x: 10.4, y: 9.6 },
      isShift: false,
      to: { x: 130.5, y: 90.49 },
    });

    expect(readBox()).toEqual([10, 10, 121, 80]);
  });

  it("[F36.AC1] counts a release on the press's integer point as a click", () => {
    // (10.2, 10.3) and (9.8, 10.4) both round to (10, 10): a click, a box 100 × 100 centred on
    // the point pressed, (10.2 − 50, 10.3 − 50) = (−39.8, −39.7) rounded to (−40, −40).
    dragDrawing("rectangle", {
      from: { x: 10.2, y: 10.3 },
      isShift: false,
      to: { x: 9.8, y: 10.4 },
    });

    expect(readBox()).toEqual([-40, -40, 100, 100]);
    expect(countShapes()).toBe(2);
  });

  it("[F36.AC1] counts as a click a pointer dragged away and released back on the press", () => {
    clickElement("tool-rectangle");
    firePointer("pointerdown", { x: 10, y: 10 }, false);
    firePointer("pointermove", { x: 100, y: 100 }, false);
    firePointer("pointermove", { x: 10.3, y: 10 }, false);
    firePointer("pointerup", { x: 10.3, y: 10 }, false);
    firePointer("click", { x: 10.3, y: 10 }, false);

    // Centred on the point pressed: (10 − 50, 10 − 50) = (−40, −40); one shape added.
    expect(readBox()).toEqual([-40, -40, 100, 100]);
    expect(countShapes()).toBe(2);
    expect(readPressedState("tool-selector")).toBe("true");
  });

  it("[F36.AC1] adds a text in a box 200 × 60 dragged from (10, 10) to (210, 70), its content selected", () => {
    chooseOption("text-weight", "400");

    const select = vi.spyOn(HTMLTextAreaElement.prototype, "select");

    dragDrawing("text", { from: { x: 10, y: 10 }, isShift: false, to: { x: 210, y: 70 } });

    expect(readModel()).toMatchObject({
      boxHeight: 60,
      boxWidth: 200,
      content: "Text",
      isInBox: true,
      x: 10,
      y: 10,
    });

    expect(readValue("text-fit")).toBe("box");

    expect(select.mock.contexts).toEqual([selectPageElement("#text-content")]);
    expect(readPressedState("tool-selector")).toBe("true");

    select.mockRestore();
  });

  it("[F36.AC1] never squares a text with Shift", () => {
    dragDrawing("text", { from: { x: 10, y: 10 }, isShift: true, to: { x: 210, y: 70 } });

    expect([readValue("text-box-width"), readValue("text-box-height")]).toEqual(["200", "60"]);
  });

  it("[F36.AC1] adds a circle 120 × 120 with the round shape tool and Shift held", () => {
    // 120 and 80 dragged: the side is the larger, 120.
    dragDrawing("round", { from: { x: 10, y: 10 }, isShift: true, to: { x: 130, y: 90 } });

    expect(readBox()).toEqual([10, 10, 120, 120]);
    expect(readText("#shape-title")).toBe("Round shape");
  });

  it("[F36.AC1] changes the shape at once when Shift is pressed or released during the drag", () => {
    clickElement("tool-polygon");
    firePointer("pointerdown", { x: 10, y: 10 }, false);
    firePointer("pointermove", { x: 130, y: 90 }, false);
    expect(readBox()).toEqual([10, 10, 120, 80]);

    fireShift("keydown");
    expect(readBox()).toEqual([10, 10, 120, 120]);

    fireShift("keyup");
    expect(readBox()).toEqual([10, 10, 120, 80]);

    firePointer("pointerup", { x: 130, y: 90 }, false);
    expect(readBox()).toEqual([10, 10, 120, 80]);
    expect(readText("#shape-title")).toBe("Polygon");
  });

  it("[F36.AC1] adds a shape of width 0 when dragged straight down", () => {
    // (10, 10) to (10.3, 90): both x round to 10, 0 × 80.
    dragDrawing("rectangle", { from: { x: 10, y: 10 }, isShift: false, to: { x: 10.3, y: 90 } });

    expect(readBox()).toEqual([10, 10, 0, 80]);
    expect(countShapes()).toBe(2);
  });

  it("[F36.AC1] draws nothing with the selector armed", () => {
    firePointer("pointerdown", { x: 10, y: 10 }, false);
    firePointer("pointermove", { x: 130, y: 90 }, false);
    fireShift("keydown");
    firePointer("pointerup", { x: 130, y: 90 }, false);

    expect([...readBox(), countShapes()]).toEqual([0, 0, 120, 80, 1]);
  });

  it("[F36.AC1] ignores a press of another button than the main one", () => {
    clickElement("tool-rectangle");

    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, button: 2, clientX: 150, clientY: 100 }),
    );

    firePointer("pointermove", { x: 130, y: 90 }, false);

    expect(countShapes()).toBe(1);
    expect(readPressedState("tool-rectangle")).toBe("true");
  });

  it("[F36.AC1] keeps the shape drawn when the drag is cancelled, the selector armed again", () => {
    clickElement("tool-rectangle");
    firePointer("pointerdown", { x: 10, y: 10 }, false);
    firePointer("pointermove", { x: 130, y: 90 }, false);
    firePointer("pointercancel", { x: 130, y: 90 }, false);
    firePointer("pointermove", { x: 300, y: 300 }, false);

    expect([...readBox(), countShapes()]).toEqual([10, 10, 120, 80, 2]);
    expect(readPressedState("tool-selector")).toBe("true");
  });

  it("[F36.AC1] keeps each shape dragged, the last on top", () => {
    dragDrawing("rectangle", { from: { x: 10, y: 10 }, isShift: false, to: { x: 130, y: 90 } });
    dragDrawing("round", { from: { x: 200, y: 10 }, isShift: false, to: { x: 300, y: 90 } });

    expect([...readBox(), countShapes()]).toEqual([200, 10, 100, 80, 3]);
  });
});
