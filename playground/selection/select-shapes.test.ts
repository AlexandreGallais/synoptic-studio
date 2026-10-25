// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourPiecesToPathData, shapePickArea } from "../../src";
import { readExportedSvg } from "../document";
import { readDrawnTexts } from "../kept-shapes";
import { readSelection } from "../page";
import {
  clickAt,
  clickElement,
  openPage,
  pressAndClick,
  pressKey,
  pressPointer,
  readDisabledState,
  readDisplay,
  readPointRows,
  readPressedState,
  readShapeModel,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
} from "../user-actions";

import type { Point, Shape } from "../../src";
import type { Keys } from "../user-actions";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner at the opening view (`OPENING_VIEW`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/** A point of the empty drawing, under none of A, B and C. */
const EMPTY = { x: 200, y: 300 };

/**
 * The window's point of a point of the drawing.
 *
 * @param point - the point of the drawing
 * @returns its `clientX` and `clientY`
 */
function toWindow(point: Point): { readonly clientX: number; readonly clientY: number } {
  return { clientX: point.x + ORIGIN_LEFT, clientY: point.y + ORIGIN_TOP };
}

/**
 * Places a shape: its tool chosen, then a click on the drawing (F35).
 *
 * @param tool - `rectangle`, `polygon`, `round` or `text`
 * @param point - the point of the drawing clicked
 */
function placeShape(tool: string, point: Point): void {
  clickElement(`tool-${tool}`);
  clickAt("#canvas", toWindow(point));
}

/**
 * Clicks an element of the drawing with the selector armed and keys held, as a mouse does:
 * pressed, released, then clicked at a point of the drawing.
 *
 * @param point - the point of the drawing clicked
 * @param keys - the keys held
 * @param selector - the element under the pointer, `#canvas` or one of its handles
 */
function pickWithKeys(point: Point, keys: Keys, selector: string): void {
  const at = toWindow(point);

  pressPointer(selector, "pointerdown", { ...at, buttons: 1 });
  pressPointer(selector, "pointerup", { ...at, buttons: 0 });

  selectPageElement(selector).dispatchEvent(
    new MouseEvent("click", { ...at, ...keys, bubbles: true }),
  );
}

/**
 * Clicks the drawing with Shift held.
 *
 * @param point - the point of the drawing clicked
 */
function shiftPick(point: Point): void {
  pickWithKeys(point, { shiftKey: true }, "#canvas");
}

/**
 * Presses A with Ctrl, or Cmd, held on an element of the page, as the browser fires it: its key
 * `A` with Shift held too, `a` otherwise.
 *
 * @param selector - the element having the focus, `body` for none
 * @param keys - Ctrl or Cmd held, and Shift
 * @returns whether the page left the key to the browser, its default not prevented
 */
function pressSelectAll(selector: string, keys: Keys): boolean {
  const key = keys.shiftKey === true ? "A" : "a";

  return selectPageElement(selector).dispatchEvent(
    new KeyboardEvent("keydown", { ...keys, bubbles: true, cancelable: true, key }),
  );
}

/**
 * Reads the path data of the shapes drawn, the editor's frames, boxes and handles left out.
 *
 * @returns each shape's `d`, the first drawn first
 */
function readShapePaths(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas svg > path:not([class])")].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Reads the outlines of the shapes selected in the layer above the drawing.
 *
 * @returns their path data, the first drawn first
 */
function readSelectionOutlines(): readonly (string | null)[] {
  return [...document.querySelectorAll("#overlay svg path.selection")].map((path) =>
    path.getAttribute("d"),
  );
}

/**
 * Reads the dashed box around the shapes selected, in the layer above the drawing.
 *
 * @returns its path data; undefined for none
 */
function readSelectionBox(): string | undefined {
  return document.querySelector("#overlay svg path.selection-box")?.getAttribute("d") ?? undefined;
}

/**
 * The outline expected of a shape: the path of its contour as drawn, stroked as the hover's.
 *
 * @param shape - the shape outlined
 * @returns the path data of its contour
 */
function outlineOf(shape: Shape): string {
  return contourPiecesToPathData(shapePickArea(shape).pieces);
}

/**
 * Draws A, B and C, rectangles 100 × 100 of radius 10, in that order, C edited: A the rectangle
 * the page opens with, typed 100 × 100 at (0, 0); B centred on (200, 50), at (150, 0); C centred
 * on (350, 50), at (300, 0). (50, 50), (200, 50) and (350, 50) are inside A, B and C.
 *
 * @returns their models, and their paths in the order drawn
 */
function drawThreeRectangles(): {
  readonly a: Shape;
  readonly b: Shape;
  readonly c: Shape;
  readonly paths: readonly string[];
} {
  typeAndEnter("width", "100");
  typeAndEnter("height", "100");

  const a = { model: readShapeModel(), path: readText("#path-data") };

  placeShape("rectangle", { x: 200, y: 50 });

  const b = { model: readShapeModel(), path: readText("#path-data") };

  placeShape("rectangle", { x: 350, y: 50 });

  const c = { model: readShapeModel(), path: readText("#path-data") };

  return { a: a.model, b: b.model, c: c.model, paths: [a.path, b.path, c.path] };
}

describe("Shift + click adds or removes a shape", () => {
  beforeEach(() => {
    // The page as it opens, a rectangle 120 × 80 at (0, 0), radius 10.
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F37.AC2] selects A and B, each outlined, a dashed box around both, the panel saying 2", () => {
    const { a, b, paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 200, y: 50 });

    expect(readSelectionOutlines()).toEqual([outlineOf(a), outlineOf(b)]);

    // The box from (0, 0) to (250, 100), its dashed line 1 pixel wide half a pixel inside it.
    expect(readSelectionBox()).toBe("M0.5 0.5H249.5V99.5H0.5Z");
    expect(readText("#shape-title")).toBe("2 shapes selected");
    expect(readDisplay("shape-panel")).toBe("none");
    expect(readShapePaths()).toEqual(paths);
  });

  it("[F37.AC2] takes B out with Shift, A alone loaded into the panel, no dashed box", () => {
    const { a } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 200, y: 50 });
    shiftPick({ x: 200, y: 50 });

    expect(readShapeModel()).toEqual(a);
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readDisplay("shape-panel")).not.toBe("none");
    expect(readSelectionOutlines()).toEqual([outlineOf(a)]);
    expect(readSelectionBox()).toBeUndefined();
  });

  it("[F37.AC2] selects C alone on Ctrl + click or Cmd + click, kept for E13's groups", () => {
    const { c } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    pickWithKeys({ x: 350, y: 50 }, { ctrlKey: true }, "#canvas");

    expect(readShapeModel()).toEqual(c);
    expect(readSelectionOutlines()).toEqual([outlineOf(c)]);

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    pickWithKeys({ x: 350, y: 50 }, { metaKey: true }, "#canvas");

    expect(readSelectionOutlines()).toEqual([outlineOf(c)]);
  });

  it("[F37.AC2] keeps A and B on Shift + click on the empty drawing; a plain click selects none", () => {
    const { a, b } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 200, y: 50 });
    shiftPick(EMPTY);

    expect(readSelectionOutlines()).toEqual([outlineOf(a), outlineOf(b)]);
    expect(readText("#shape-title")).toBe("2 shapes selected");

    pressAndClick("#canvas", toWindow(EMPTY));

    expect(readSelectionOutlines()).toEqual([]);
    expect(readText("#shape-title")).toBe("No shape selected");
  });

  it("[F37.AC2] selects C alone on a plain click, from A and B", () => {
    const { c, paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 200, y: 50 });
    pressAndClick("#canvas", toWindow({ x: 350, y: 50 }));

    expect(readShapeModel()).toEqual(c);
    expect(readSelectionOutlines()).toEqual([outlineOf(c)]);
    expect(readSelectionBox()).toBeUndefined();
    expect(readShapePaths()).toEqual(paths);
  });

  it("[F37.AC2] keeps A's radius typed 25 when B is added, for the next rectangle placed", () => {
    drawThreeRectangles();
    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    typeAndEnter("radius", "25");

    const a = readShapeModel();

    shiftPick({ x: 200, y: 50 });
    shiftPick({ x: 200, y: 50 });

    expect(readShapeModel()).toEqual(a);

    shiftPick({ x: 200, y: 50 });
    placeShape("rectangle", { x: 550, y: 250 });

    expect(readValue("radius")).toBe("25");
  });

  it("[F37.AC2] places a rectangle on top of all, selected alone, A and B unchanged", () => {
    const { paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 200, y: 50 });
    // Centred on (550, 100): 100 × 100 at (500, 50).
    placeShape("rectangle", { x: 550, y: 100 });

    expect(readShapeModel()).toMatchObject({ x: 500, y: 50 });
    expect(readText("#shape-title")).toBe("Rectangle");
    expect(readShapePaths()).toEqual([...paths, readText("#path-data")]);
    expect(readSelectionOutlines()).toHaveLength(1);
    expect(readSelectionBox()).toBeUndefined();
  });

  it("[F37.AC2] exports A, B and C, no outline nor dashed box, whichever are selected", () => {
    drawThreeRectangles();

    const exported = readExportedSvg(document, readDrawnTexts(document));

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 200, y: 50 });

    expect(readSelectionBox()).toBeDefined();
    expect(selectPageElement("#canvas").querySelector(".selection, .selection-box")).toBeNull();
    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);
  });
});

describe("Ctrl + A selects every shape and Escape none", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F37.AC2] selects A, B and C on Ctrl + A or Cmd + A outside the fields", () => {
    const { a, b, c } = drawThreeRectangles();

    expect(pressSelectAll("body", { ctrlKey: true })).toBe(false);
    expect(readSelectionOutlines()).toEqual([outlineOf(a), outlineOf(b), outlineOf(c)]);
    expect(readText("#shape-title")).toBe("3 shapes selected");

    pressKey("body", "Escape");
    pressSelectAll("body", { metaKey: true });

    expect(readText("#shape-title")).toBe("3 shapes selected");

    pressKey("body", "Escape");
    pressSelectAll("body", { ctrlKey: true, shiftKey: true });

    expect(readText("#shape-title")).toBe("3 shapes selected");
  });

  it("[F37.AC2] holds a shape 0 × 0 in the dashed box, at the point it lies on", () => {
    drawThreeRectangles();

    // C, edited, made 0 × 0 at (400, 200): its contour has no piece, its corners lie on that point.
    typeAndEnter("radius", "0");
    typeAndEnter("width", "0");
    typeAndEnter("height", "0");
    typeAndEnter("shape-x", "400");
    typeAndEnter("shape-y", "200");
    pressSelectAll("body", { ctrlKey: true });

    // The box from (0, 0) to (400, 200), its line half a pixel inside it.
    expect(readSelectionBox()).toBe("M0.5 0.5H399.5V199.5H0.5Z");
  });

  it("[F37.AC2] leaves Ctrl + A to a field having the focus, the shapes' selection unchanged", () => {
    drawThreeRectangles();
    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));

    expect(pressSelectAll("#width", { ctrlKey: true })).toBe(true);
    expect(pressSelectAll("#text-content", { ctrlKey: true })).toBe(true);
    // A slider of the color picker counts as a field.
    expect(pressSelectAll("#color-hue", { ctrlKey: true })).toBe(true);
    expect(readSelectionOutlines()).toHaveLength(1);
    expect(readText("#shape-title")).toBe("Rectangle");
  });

  it("[F37.AC2] selects none on Escape, the panel saying so, and nothing in a field", () => {
    drawThreeRectangles();
    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    pressKey("#width", "Escape");

    expect(readText("#shape-title")).toBe("Rectangle");

    shiftPick({ x: 200, y: 50 });
    pressKey("body", "Escape");

    expect(readSelectionOutlines()).toEqual([]);
    expect(readSelectionBox()).toBeUndefined();
    expect(readText("#shape-title")).toBe("No shape selected");
  });

  it("[F37.AC2] closes the color picker only on Escape, the text kept selected", () => {
    placeShape("text", { x: 200, y: 300 });
    clickElement("text-color-swatch");

    expect(selectPageElement("#text-color-swatch").getAttribute("aria-expanded")).toBe("true");

    pressKey("#text-color-swatch", "Escape");

    expect(selectPageElement("#text-color-swatch").getAttribute("aria-expanded")).toBe("false");
    expect(readText("#shape-title")).toBe("Text");
    expect(readSelectionOutlines()).toHaveLength(1);
  });
});

describe("the shapes' selection in Points mode", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F37.AC2] keeps A selected on Shift + click on B or Ctrl + A in Points mode", () => {
    const { a } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    clickElement("mode-points");
    shiftPick({ x: 200, y: 50 });
    pressSelectAll("body", { ctrlKey: true });

    expect(readShapeModel()).toEqual(a);
    expect(readSelectionOutlines()).toEqual([outlineOf(a)]);
  });

  it("[F37.AC2] selects points by their handles, plain or with Shift, A kept selected", () => {
    const { a } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    clickElement("mode-points");
    // Each vertex of A is a corner of its box, cut by its radius of 10: outside its painted area.
    pickWithKeys({ x: 0, y: 0 }, {}, '#canvas path.handle[data-vertex="1"]');
    pickWithKeys({ x: 100, y: 100 }, { shiftKey: true }, '#canvas path.handle[data-vertex="3"]');

    expect(readSelection(document).selected).toEqual([0, 2]);
    expect(readShapeModel()).toEqual(a);
  });

  it("[F37.AC2] disables the Points button while several shapes are selected", () => {
    drawThreeRectangles();
    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));

    expect(readDisabledState("mode-points")).toBe("false");

    shiftPick({ x: 200, y: 50 });

    expect(readDisabledState("mode-points")).toBe("true");

    shiftPick({ x: 200, y: 50 });

    expect(readDisabledState("mode-points")).toBe("false");
  });
});

describe("Delete removes the selected shapes", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it.each(["Delete", "Backspace"])(
    "[F37.AC1] removes B on %s, A and C unchanged and in order, none selected",
    (key) => {
      const { paths } = drawThreeRectangles();

      pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));
      pressKey("body", key);

      expect(readShapePaths()).toEqual([paths[0], paths[2]]);
      expect(readSelectionOutlines()).toEqual([]);
      expect(readText("#shape-title")).toBe("No shape selected");
      expect(readDisplay("shape-panel")).toBe("none");
    },
  );

  it("[F37.AC1] removes A and C selected together, B alone left, unchanged", () => {
    const { b, paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 50, y: 50 }));
    shiftPick({ x: 350, y: 50 });
    pressKey("body", "Delete");

    expect(readShapePaths()).toEqual([paths[1]]);
    expect(readText("#shape-title")).toBe("No shape selected");
    expect(readSelectionBox()).toBeUndefined();

    // B, now first, is picked as it was drawn.
    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));

    expect(readShapeModel()).toEqual(b);
  });

  it("[F37.AC1] removes every shape after Ctrl + A, none left", () => {
    drawThreeRectangles();
    pressSelectAll("body", { ctrlKey: true });
    pressKey("body", "Backspace");

    expect(readShapePaths()).toEqual([]);
    expect(readText("#shape-title")).toBe("No shape selected");
  });

  it.each([
    { field: "#width", key: "Delete" },
    { field: "#width", key: "Backspace" },
    { field: "#text-content", key: "Delete" },
    { field: "#text-content", key: "Backspace" },
    // A slider of the color picker counts as a field.
    { field: "#color-hue", key: "Delete" },
    { field: "#color-plane", key: "Backspace" },
  ])("[F37.AC1] leaves $key in $field to the field, no shape removed", ({ field, key }) => {
    const { b, paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));

    expect(
      selectPageElement(field).dispatchEvent(
        new KeyboardEvent("keydown", { bubbles: true, cancelable: true, key }),
      ),
    ).toBe(true);

    expect(readShapePaths()).toEqual(paths);
    expect(readShapeModel()).toEqual(b);
  });

  it("[F37.AC1] changes nothing on Delete with none selected", () => {
    const { paths } = drawThreeRectangles();

    pressKey("body", "Escape");
    pressKey("body", "Delete");

    expect(readShapePaths()).toEqual(paths);
    expect(readText("#shape-title")).toBe("No shape selected");
  });

  it("[F37.AC1] removes B while the Rectangle tool is armed, the tool kept armed", () => {
    const { paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));
    clickElement("tool-rectangle");
    pressKey("body", "Delete");

    expect(readShapePaths()).toEqual([paths[0], paths[2]]);
    expect(readPressedState("tool-rectangle")).toBe("true");
    expect(readSelectionOutlines()).toEqual([]);
  });

  it("[F37.AC1] deletes B's vertices in Points mode, never B; none selected, nothing changes", () => {
    const { b, paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));
    clickElement("mode-points");
    pressKey("body", "Delete");

    expect(readShapePaths()).toEqual(paths);
    expect(readShapeModel()).toEqual(b);

    // B's vertex 2, its top-right corner (250, 0), cut by its radius of 10.
    pickWithKeys({ x: 250, y: 0 }, {}, '#canvas path.handle[data-vertex="2"]');
    pressKey("#text-content", "Delete");

    expect(readPointRows()).toHaveLength(4);

    pressKey("body", "Delete");

    expect(readPointRows()).toHaveLength(3);
    expect(readShapePaths()).toHaveLength(3);
    expect(readText("#shape-title")).not.toBe("No shape selected");
  });

  it("[F37.AC1] keeps B's radius typed 25 for the next rectangle placed once B is removed", () => {
    drawThreeRectangles();
    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));
    typeAndEnter("radius", "25");
    pressKey("body", "Delete");
    placeShape("rectangle", { x: 550, y: 250 });

    expect(readValue("radius")).toBe("25");
  });

  it("[F37.AC1] drops a shape refused in the panel, as a click on the empty drawing does", () => {
    const { paths } = drawThreeRectangles();

    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));
    typeAndEnter("angle", "");

    // Refused, B is not outlined.
    expect(readSelectionOutlines()).toEqual([]);

    pressKey("body", "Delete");

    expect(readShapePaths()).toEqual([paths[0], paths[2]]);
    expect(readText("#shape-title")).toBe("No shape selected");
  });

  it("[F37.AC1] exports A and C only once B is removed", () => {
    const [pathA, pathB, pathC] = drawThreeRectangles().paths.map((path) => `d="${path}"`);

    pressAndClick("#canvas", toWindow({ x: 200, y: 50 }));
    pressKey("body", "Delete");

    const exported = readExportedSvg(document, readDrawnTexts(document));

    expect(exported).toContain(pathA);
    expect(exported).toContain(pathC);
    expect(exported).not.toContain(pathB);
    expect(exported).not.toContain("selection");
  });
});
