// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { readDrawnTexts, readSelectedShapes } from "../kept-shapes";
import {
  clickAt,
  clickElement,
  dispatchDrawingPointer,
  openServedPage,
  readText,
  selectPageElement,
  setElementBox,
  typeTextContent,
} from "../user-actions";

import type { Point } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/**
 * Reads the property the page's style gives an element to let its text be selected or not.
 *
 * @param selector - the element
 * @returns its computed `user-select`
 */
function readUserSelect(selector: string): string {
  return getComputedStyle(selectPageElement(selector)).getPropertyValue("user-select");
}

/**
 * Double-clicks the drawing at a point, as the browser fires it: two presses and releases, each
 * followed by its `click`, then the `dblclick`, which the page does not listen to.
 *
 * @param point - the point of the drawing double-clicked
 */
function doubleClick(point: Point): void {
  for (const type of ["pointerdown", "pointerup", "click", "pointerdown", "pointerup", "click"]) {
    dispatchDrawingPointer(type, point, NO_KEY);
  }

  selectPageElement("#canvas").dispatchEvent(
    new MouseEvent("dblclick", { bubbles: true, cancelable: true, detail: 2 }),
  );
}

describe("the drawing's texts never selected as page text (US-080)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80 at (0, 0), whatever feature is validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F43.AC2] lets no selection start on the canvas nor on the layer above it", () => {
    expect(readUserSelect("#canvas")).toBe("none");
    expect(readUserSelect("#overlay")).toBe("none");
  });

  it("[F43.AC2] keeps the panel's fields free to select their text", () => {
    expect(readUserSelect("#width")).not.toBe("none");
    expect(readUserSelect("#text-content")).not.toBe("none");
  });

  it("[F43.AC2] picks a text double-clicked with the Selector as a shape, no page text selected", () => {
    // A text placed at (400, 300), then the empty drawing clicked, so that none is selected.
    clickElement("tool-text");
    clickAt("#canvas", { clientX: 540, clientY: 390 });
    typeTextContent("Pump");
    dispatchDrawingPointer("pointerdown", { x: 200, y: 500 }, NO_KEY);
    dispatchDrawingPointer("pointerup", { x: 200, y: 500 }, NO_KEY);
    dispatchDrawingPointer("click", { x: 200, y: 500 }, NO_KEY);
    expect(readSelectedShapes(document)).toEqual([]);

    // (410, 295) lies in its block, which starts at (400, 281) (US-075).
    doubleClick({ x: 410, y: 295 });

    expect(readSelectedShapes(document)).toEqual([1]);
    expect(readDrawnTexts(document)).toHaveLength(1);
    expect(readText("#shape-title")).toMatch(/text/iu);
    expect(document.getSelection()?.toString() ?? "").toBe("");
  });
});
