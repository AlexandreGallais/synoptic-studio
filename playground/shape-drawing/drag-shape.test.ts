// @vitest-environment happy-dom
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  chooseOption,
  clickElement,
  dispatchDrawingPointer,
  openPage,
  readModel,
  readPressedState,
  readShapeModel,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
} from "../user-actions";

import type { Point } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

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
  const keys = { altKey: false, shiftKey: drag.isShift };

  clickElement(`tool-${tool}`);
  dispatchDrawingPointer("pointerdown", drag.from, keys);
  dispatchDrawingPointer("pointermove", drag.to, keys);
  dispatchDrawingPointer("pointerup", drag.to, keys);
  dispatchDrawingPointer("click", drag.to, keys);
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

describe("a drag with a tool draws the shape's box", () => {
  beforeEach(() => {
    // The page as it opens, a rectangle 120 × 80 at (0, 0).
    openPage("");
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
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);

    // Pressed only: nothing drawn yet.
    expect(countShapes()).toBe(1);

    dispatchDrawingPointer("pointermove", { x: 70, y: 50 }, NO_KEY);

    // 70 − 10 = 60, 50 − 10 = 40; the tool still armed until the release.
    expect([...readBox(), countShapes()]).toEqual([10, 10, 60, 40, 2]);
    expect(readPressedState("tool-rectangle")).toBe("true");

    dispatchDrawingPointer("pointermove", { x: 130, y: 90 }, NO_KEY);
    expect(readBox()).toEqual([10, 10, 120, 80]);
  });

  it("[F36.AC1] draws the box to the release, past the last move", () => {
    clickElement("tool-rectangle");
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 70, y: 50 }, NO_KEY);
    dispatchDrawingPointer("pointerup", { x: 130, y: 90 }, NO_KEY);

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
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 100, y: 100 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 10.3, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointerup", { x: 10.3, y: 10 }, NO_KEY);
    dispatchDrawingPointer("click", { x: 10.3, y: 10 }, NO_KEY);

    // Centred on the point pressed: (10 − 50, 10 − 50) = (−40, −40); one shape added.
    expect(readBox()).toEqual([-40, -40, 100, 100]);
    expect(countShapes()).toBe(2);
    expect(readPressedState("tool-selector")).toBe("true");
  });

  it("[F36.AC1] adds a text in a box 200 × 60 dragged from (10, 10) to (210, 70), its content focused", () => {
    chooseOption("text-weight", "400");

    const focus = vi.spyOn(HTMLTextAreaElement.prototype, "focus");

    dragDrawing("text", { from: { x: 10, y: 10 }, isShift: false, to: { x: 210, y: 70 } });

    expect(readModel()).toMatchObject({
      boxHeight: 60,
      boxWidth: 200,
      content: "",
      isInBox: true,
      x: 10,
      y: 10,
    });

    expect(readValue("text-fit")).toBe("box");

    expect(focus.mock.contexts).toEqual([selectPageElement("#text-content")]);
    expect(readPressedState("tool-selector")).toBe("true");

    focus.mockRestore();
  });

  it("[F43.AC4] squares a text's box with Shift, 120 × 120 at (10, 10) from (10, 10) to (130, 90)", () => {
    // 120 and 80 dragged: the side is the larger, 120, laid from the press toward the pointer.
    dragDrawing("text", { from: { x: 10, y: 10 }, isShift: true, to: { x: 130, y: 90 } });

    expect(readModel()).toMatchObject({
      boxHeight: 120,
      boxWidth: 120,
      isInBox: true,
      x: 10,
      y: 10,
    });

    expect([readValue("text-box-width"), readValue("text-box-height")]).toEqual(["120", "120"]);
  });

  it("[F43.AC4] lays a text's square up from the press, 120 × 120 at (10, −30) from (130, 90) to (10, 10)", () => {
    // The side 120 laid left and up from (130, 90): x 130 − 120 = 10, y 90 − 120 = −30.
    dragDrawing("text", { from: { x: 130, y: 90 }, isShift: true, to: { x: 10, y: 10 } });

    expect(readModel()).toMatchObject({ boxHeight: 120, boxWidth: 120, x: 10, y: -30 });
  });

  it("[F43.AC4] changes a text's box at once when Shift is pressed or released during the drag", () => {
    clickElement("tool-text");
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 130, y: 90 }, NO_KEY);
    expect(readModel()).toMatchObject({ boxHeight: 80, boxWidth: 120 });

    fireShift("keydown");
    expect(readModel()).toMatchObject({ boxHeight: 120, boxWidth: 120 });

    fireShift("keyup");
    expect(readModel()).toMatchObject({ boxHeight: 80, boxWidth: 120 });

    dispatchDrawingPointer("pointerup", { x: 130, y: 90 }, NO_KEY);
    expect(readModel()).toMatchObject({ boxHeight: 80, boxWidth: 120, x: 10, y: 10 });
  });

  it("[F36.AC1] adds a circle 120 × 120 with the round shape tool and Shift held", () => {
    // 120 and 80 dragged: the side is the larger, 120.
    dragDrawing("round", { from: { x: 10, y: 10 }, isShift: true, to: { x: 130, y: 90 } });

    expect(readBox()).toEqual([10, 10, 120, 120]);
    expect(readText("#shape-title")).toBe("Round shape");
  });

  it("[F36.AC1] changes the shape at once when Shift is pressed or released during the drag", () => {
    clickElement("tool-polygon");
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 130, y: 90 }, NO_KEY);
    expect(readBox()).toEqual([10, 10, 120, 80]);

    fireShift("keydown");
    expect(readBox()).toEqual([10, 10, 120, 120]);

    fireShift("keyup");
    expect(readBox()).toEqual([10, 10, 120, 80]);

    dispatchDrawingPointer("pointerup", { x: 130, y: 90 }, NO_KEY);
    expect(readBox()).toEqual([10, 10, 120, 80]);
    expect(readText("#shape-title")).toBe("Polygon");
  });

  it("[F36.AC1] adds a shape of width 0 when dragged straight down", () => {
    // (10, 10) to (10.3, 90): both x round to 10, 0 × 80.
    dragDrawing("rectangle", { from: { x: 10, y: 10 }, isShift: false, to: { x: 10.3, y: 90 } });

    expect(readBox()).toEqual([10, 10, 0, 80]);
    expect(countShapes()).toBe(2);
  });

  it("[F36.AC1] draws nothing with the selector armed, the drag moving the shape pressed", () => {
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 130, y: 90 }, NO_KEY);
    fireShift("keydown");
    dispatchDrawingPointer("pointerup", { x: 130, y: 90 }, NO_KEY);

    // No shape drawn: the press on the rectangle 120 × 80 at (0, 0) moves it by (120, 80).
    expect([...readBox(), countShapes()]).toEqual([120, 80, 120, 80, 1]);
  });

  it("[F36.AC1] ignores a press of another button than the main one", () => {
    clickElement("tool-rectangle");

    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointerdown", { bubbles: true, button: 2, clientX: 150, clientY: 100 }),
    );

    dispatchDrawingPointer("pointermove", { x: 130, y: 90 }, NO_KEY);

    expect(countShapes()).toBe(1);
    expect(readPressedState("tool-rectangle")).toBe("true");
  });

  it("[F36.AC1] keeps the shape drawn when the drag is cancelled, the selector armed again", () => {
    clickElement("tool-rectangle");
    dispatchDrawingPointer("pointerdown", { x: 10, y: 10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 130, y: 90 }, NO_KEY);
    dispatchDrawingPointer("pointercancel", { x: 130, y: 90 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 300, y: 300 }, NO_KEY);

    expect([...readBox(), countShapes()]).toEqual([10, 10, 120, 80, 2]);
    expect(readPressedState("tool-selector")).toBe("true");
  });

  it("[F36.AC1] keeps each shape dragged, the last on top", () => {
    dragDrawing("rectangle", { from: { x: 10, y: 10 }, isShift: false, to: { x: 130, y: 90 } });
    dragDrawing("round", { from: { x: 200, y: 10 }, isShift: false, to: { x: 300, y: 90 } });

    expect([...readBox(), countShapes()]).toEqual([200, 10, 100, 80, 3]);
  });
});
