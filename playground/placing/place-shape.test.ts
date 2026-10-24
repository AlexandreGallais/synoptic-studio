// @vitest-environment happy-dom
import { beforeEach, describe, expect, it, vi } from "vitest";

import { HELLO_TEXT } from "../../tests";
import { readExportedSvg } from "../document";
import { readFontSession, writeFontSession } from "../fonts";
import { DRAWING_KEYS, readDrawnTexts, writeStoredDrawing } from "../kept-shapes";
import {
  chooseOption,
  chooseRadio,
  clickAt,
  clickElement,
  clickPoint,
  openServedPage,
  readCheckedRadio,
  readDisplay,
  readPressedState,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeTextContent,
} from "../user-actions";

import type { Point } from "../../src";

// The canvas sits at (100, 50) in the window, and the drawing's origin 40 right of and below its
// corner at the opening view (`OPENING_VIEW`): the drawing's point (x, y) is the window's (x + 140, y + 90).

/** Left of the drawing's origin in the window: the canvas's 100 and the margin's 40. */
const ORIGIN_LEFT = 140;

/** Top of the drawing's origin in the window: the canvas's 50 and the margin's 40. */
const ORIGIN_TOP = 90;

/**
 * Clicks the drawing at a point of its own coordinates, as the designer would.
 *
 * @param point - the point of the drawing, possibly fractional
 */
function clickDrawing(point: Point): void {
  clickAt("#canvas", { clientX: point.x + ORIGIN_LEFT, clientY: point.y + ORIGIN_TOP });
}

/**
 * Places a shape: its tool chosen, then a click on the drawing.
 *
 * @param tool - `rectangle`, `polygon`, `round` or `text`
 * @param point - the point of the drawing clicked
 */
function placeShape(tool: string, point: Point): void {
  clickElement(`tool-${tool}`);
  clickDrawing(point);
}

/**
 * Places a text by a click, types its content and leaves its Content field (US-075).
 *
 * @param point - the point of the drawing clicked
 * @param content - what is typed in it
 */
function placeText(point: Point, content: string): void {
  placeShape("text", point);
  typeTextContent(content);
}

/**
 * Reads the box of the shape the panel edits, from its model: the top-left of its box before it
 * turns (ADR-0035), its width and its height. The X and Y fields show the straight box of the
 * shape drawn instead, a hexagon's narrower than its box (F04).
 *
 * @returns x, y, width and height, in that order
 */
function readBox(): readonly unknown[] {
  // oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- the model shown, a shape with a box
  const model = JSON.parse(readText("#model")) as Record<string, unknown>;

  return [model["x"], model["y"], model["width"], model["height"]];
}

/**
 * Reads the content of the Text tool's text area.
 *
 * @returns its text
 */
function readContent(): string {
  const area = selectPageElement("#text-content");

  return area instanceof HTMLTextAreaElement ? area.value : "";
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
 * Reads the tool armed: the tool whose button is pressed.
 *
 * @returns `selector`, `rectangle`, `polygon`, `round` or `text`
 */
function readArmedTool(): string {
  const tools = ["selector", "rectangle", "polygon", "round", "text"];

  return tools.find((tool) => readPressedState(`tool-${tool}`) === "true") ?? "";
}

describe("a click with a tool adds a shape where it is clicked (US-063)", () => {
  beforeEach(() => {
    // The epic's step 1, a rectangle 120 × 80, radius 10, whatever feature is being validated.
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F35.AC1] adds a rectangle 100 × 100 at (200, 90) above the first, selected", () => {
    const first = readText("#path-data");

    placeShape("rectangle", { x: 250, y: 140 });

    // Centred on (250, 140): (250 − 100 / 2, 140 − 100 / 2) = (200, 90).
    expect(readBox()).toEqual([200, 90, 100, 100]);
    expect(readText("#shape-title")).toBe("Rectangle");
    // The first rectangle kept below, unchanged; the new one drawn last, on top, edited.
    expect(readShapePaths()).toEqual([first, readText("#path-data")]);
    expect(readArmedTool()).toBe("selector");
  });

  it("[F35.AC1] arms a tool without changing the shape edited, until the drawing is clicked", () => {
    const first = readText("#path-data");

    clickElement("tool-polygon");

    expect(readArmedTool()).toBe("polygon");
    expect([readText("#shape-title"), readText("#path-data")]).toEqual(["Rectangle", first]);
    expect(readShapePaths()).toEqual([first]);
  });

  it("[F35.AC1] does nothing on a click with the selector armed", () => {
    const first = readText("#path-data");

    clickElement("tool-rectangle");
    clickElement("tool-selector");
    clickDrawing({ x: 250, y: 140 });

    expect(readShapePaths()).toEqual([first]);
    expect(readBox()).toEqual([0, 0, 120, 80]);
  });

  it.each([
    { title: "Polygon", tool: "polygon" },
    { title: "Round shape", tool: "round" },
  ])("[F35.AC1] adds a $tool in a box 100 × 100 the same way", ({ title, tool }) => {
    placeShape(tool, { x: 250, y: 140 });

    expect(readBox()).toEqual([200, 90, 100, 100]);
    expect(readText("#shape-title")).toBe(title);
    expect(readShapePaths()).toHaveLength(2);
    expect(readArmedTool()).toBe("selector");
  });

  it("[F35.AC1] places on integers a point clicked between two, a tie rounded up", () => {
    // (250.5 − 50, 140.4 − 50) = (200.5, 90.4), rounded to (201, 90): a tie toward +∞.
    placeShape("rectangle", { x: 250.5, y: 140.4 });
    expect(readBox()).toEqual([201, 90, 100, 100]);

    // (99.5 − 50, 60.6 − 50) = (49.5, 10.6), rounded to (50, 11).
    placeShape("rectangle", { x: 99.5, y: 60.6 });
    expect(readBox()).toEqual([50, 11, 100, 100]);
  });

  it("[F35.AC1] adds a text with no content of its own, centred on the point, its content focused", () => {
    // happy-dom does not select the option marked `selected`: the page's weight, 400, chosen again.
    chooseOption("text-weight", "400");

    // happy-dom focuses a text area hidden, which a browser does not: whether the fields show at
    // the focus is read.
    const displays: string[] = [];
    const focus = vi.spyOn(HTMLTextAreaElement.prototype, "focus").mockImplementation(() => {
      displays.push(readDisplay("text-options"));
    });

    placeShape("text", { x: 250, y: 140 });

    // Empty, shown 1 wide and one line of Inter Regular 32 tall, (1984 + 494) × 32 / 2048 =
    // 38.71875 (US-075): its top-left (250 − 0.5, 140 − 19.359375) = (249.5, 120.640625),
    // rounded to (250, 121), a tie rounded up.
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["250", "121"]);
    expect(readText("#shape-title")).toBe("Text");

    const content = selectPageElement("#text-content");

    expect(content).toBeInstanceOf(HTMLTextAreaElement);
    expect(focus.mock.contexts).toEqual([content]);
    expect(displays).not.toContain("none");
    expect(readContent()).toBe("");
    expect(document.querySelectorAll("#canvas .text-frame")).toHaveLength(1);

    focus.mockRestore();

    // The rectangle kept below the text.
    expect(readShapePaths()).toHaveLength(1);
  });

  it("[F35.AC1] keeps a text typed once placed, growing from its frame's left", () => {
    chooseOption("text-weight", "400");
    placeText({ x: 250, y: 140 }, "Text");

    // "Text" in Inter Regular 32: advances T 1322, e 1194, x 1118, t 670, 4304 units, kerning
    // T-e −160, e-x −46, x-t +40, −166: 4138 × 32 / 2048 = 64.65625 wide, from (250, 121).
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["250", "121"]);
    expect(readContent()).toBe("Text");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
  });

  it("[F35.AC1] keeps a text below the shape placed after it", () => {
    chooseOption("text-weight", "400");
    placeText({ x: 250, y: 140 }, "Text");
    placeShape("rectangle", { x: 400, y: 300 });

    expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
    expect(readShapePaths()).toHaveLength(2);
    expect(readDrawnTexts(document).map((text) => text.content)).toEqual(["Text"]);
  });

  it("[F35.AC1] places a text whose font has no metrics with its top-left on the point", () => {
    // No font available yet (US-056): the text's size is unknown, its box taken as 0 × 0.
    writeFontSession(document, { ...readFontSession(document), kept: [] });
    placeShape("text", { x: 250.5, y: 140.6 });

    // (250.5 − 0 / 2, 140.6 − 0 / 2), rounded to (251, 141): a tie rounded up.
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["251", "141"]);
  });

  it("[F35.AC1] keeps a text whose font's metrics are no longer read, drawn and exported as it was", () => {
    chooseOption("text-weight", "400");
    placeText({ x: 250, y: 140 }, "Text");
    placeShape("rectangle", { x: 400, y: 300 });

    const drawn = document.querySelector("#canvas text")?.outerHTML;

    // A family read ahead whose files could not be read again leaves the session (US-061), Inter
    // not drawn on this computer (US-056): no family measures the kept text any more.
    writeFontSession(document, { ...readFontSession(document), kept: [] });
    placeShape("round", { x: 500, y: 300 });

    expect(document.querySelector("#canvas text")?.outerHTML).toBe(drawn);
    expect(readExportedSvg(document, readDrawnTexts(document))).toContain("<text");
  });

  it("[F35.AC1] draws no kept text that no metrics ever measured, the other shapes drawn", () => {
    const first = readText("#path-data");

    // A text kept by hand, never drawn: no family measures it, and none did before.
    writeFontSession(document, { ...readFontSession(document), kept: [] });

    writeStoredDrawing(document, DRAWING_KEYS.below, [
      { kind: "text", shape: { ...HELLO_TEXT, content: "Never drawn" } },
    ]);

    placeShape("round", { x: 500, y: 300 });

    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(readShapePaths()).toEqual([first, readText("#path-data")]);
    // Nothing in its place either, not even a stray word.
    expect(document.querySelector("#canvas svg")?.textContent).not.toContain("undefined");
  });

  it("[F35.AC1] keeps no refused shape: a rectangle without angle is not drawn below", () => {
    typeAndEnter("angle", "");
    placeShape("rectangle", { x: 250, y: 140 });

    expect(readShapePaths()).toHaveLength(1);
    expect(readBox()).toEqual([200, 90, 100, 100]);
  });

  it("[F35.AC1] keeps no refused text: a text of size 0 is not drawn below", () => {
    chooseOption("text-weight", "400");
    placeShape("text", { x: 250, y: 140 });
    typeAndEnter("text-size", "0");

    expect(readText("#status")).not.toBe("");

    placeShape("rectangle", { x: 400, y: 300 });

    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(readShapePaths()).toHaveLength(2);
  });

  it("[F35.AC1] exports the three shapes drawn, in the order drawn, each unchanged", () => {
    const first = readText("#path-data");

    placeShape("polygon", { x: 250, y: 140 });

    const second = readText("#path-data");

    placeShape("round", { x: 400, y: 300 });

    const parser = new DOMParser();
    const exported = parser.parseFromString(
      readExportedSvg(document, readDrawnTexts(document)),
      "image/svg+xml",
    );

    expect([...exported.querySelectorAll("path")].map((path) => path.getAttribute("d"))).toEqual([
      first,
      second,
      readText("#path-data"),
    ]);
  });

  it("[F35.AC1] forgets the shapes placed on Reset and at the next guided step", () => {
    placeShape("rectangle", { x: 250, y: 140 });
    clickElement("tool-reset");

    expect(readShapePaths()).toHaveLength(1);
    expect(readArmedTool()).toBe("selector");

    placeShape("rectangle", { x: 250, y: 140 });
    clickElement("guide-next");

    expect(readShapePaths()).toHaveLength(1);
  });
});

describe("what a new shape starts with (SP-026)", () => {
  beforeEach(() => {
    openServedPage("?guide=epic");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F35.AC1] gives a polygon the last polygon's settings, a rectangle the last rectangle's", () => {
    placeShape("polygon", { x: 100, y: 100 });
    typeAndEnter("corners", "3");
    typeAndEnter("radius", "4");
    chooseRadio("anchor", "mid max");
    typeAndEnter("angle", "30");
    placeShape("rectangle", { x: 400, y: 100 });
    typeAndEnter("radius", "0");

    placeShape("polygon", { x: 250, y: 140 });

    // A triangle of radius 4 anchored at the bottom center, 100 × 100 at (200, 90), straight.
    expect([readValue("corners"), readValue("radius"), readCheckedRadio("anchor")]).toEqual([
      "3",
      "4",
      "mid max",
    ]);

    expect([...readBox(), readValue("angle")]).toEqual([200, 90, 100, 100, "0"]);

    placeShape("rectangle", { x: 400, y: 300 });
    expect(readValue("radius")).toBe("0");
  });

  it("[F35.AC1] gives the first polygon the page's 6 corners and radius 10, whatever the rectangle's", () => {
    typeAndEnter("radius", "4");
    placeShape("polygon", { x: 250, y: 140 });

    expect([readValue("corners"), readValue("radius"), readCheckedRadio("anchor")]).toEqual([
      "6",
      "10",
      "mid mid",
    ]);

    // The page's rectangle was the last rectangle shown: its radius 4 goes to the next one.
    placeShape("rectangle", { x: 400, y: 300 });
    expect(readValue("radius")).toBe("4");
  });

  it("[F35.AC1] gives a polygon the settings of the last one, converted to a free shape since", () => {
    placeShape("polygon", { x: 250, y: 140 });
    typeAndEnter("corners", "3");
    clickElement("mode-points");
    clickElement("convert");

    expect(readText("#shape-title")).toBe("Free shape");

    clickElement("mode-shape");
    placeShape("polygon", { x: 400, y: 300 });

    expect([readText("#shape-title"), readValue("corners")]).toEqual(["Polygon", "3"]);
  });

  it("[F35.AC1] gives a rectangle the last rectangle's radius of each corner", () => {
    clickElement("mode-points");
    clickPoint(1, false);
    typeAndEnter("point-radius", "20");
    clickElement("mode-shape");
    placeShape("polygon", { x: 250, y: 140 });

    // The polygon has the page's one radius, 10, for each of its 6 corners.
    expect(selectPageElement("#radius").dataset["radii"]).toBe("10 10 10 10 10 10");
    expect(readValue("radius")).toBe("10");

    placeShape("rectangle", { x: 400, y: 300 });

    // The first corner 20, the three others the page's rectangle's 10.
    expect(selectPageElement("#radius").dataset["radii"]).toBe("20 10 10 10");
    expect(readText("#shape-title")).toBe("Rectangle");
  });

  it("[F35.AC1] gives a new text the last text's options and no content of its own", () => {
    chooseOption("text-weight", "400");
    placeText({ x: 250, y: 140 }, "Pump");
    typeAndEnter("text-size", "20");
    typeAndEnter("angle", "45");
    placeShape("rectangle", { x: 400, y: 300 });
    placeShape("text", { x: 250, y: 300 });

    expect([readValue("text-size"), readContent(), readValue("angle")]).toEqual(["20", "", "0"]);
  });

  it("[F35.AC1] measures a new text's box again when Box is chosen, not the last text's typed", () => {
    chooseOption("text-weight", "400");
    placeText({ x: 250, y: 140 }, "Pump");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "300");
    placeShape("rectangle", { x: 400, y: 300 });
    placeText({ x: 250, y: 300 }, "Text");
    chooseOption("text-fit", "box");

    // "Text" 64.65625 wide, its box the block rounded up (US-055): 65.
    expect(readValue("text-box-width")).toBe("65");
  });

  it("[F35.AC1] forgets the settings kept on Reset: the next polygon has the page's", () => {
    placeShape("polygon", { x: 250, y: 140 });
    typeAndEnter("corners", "3");
    placeShape("rectangle", { x: 400, y: 300 });
    clickElement("tool-reset");
    placeShape("polygon", { x: 250, y: 140 });

    expect(readValue("corners")).toBe("6");
  });

  it("[F35.AC1] forgets the last text's options on Reset: the next text is Inter 32 Regular", () => {
    placeShape("text", { x: 250, y: 140 });
    typeAndEnter("text-size", "20");
    chooseOption("text-weight", "700");
    clickElement("tool-reset");
    placeShape("text", { x: 250, y: 140 });

    expect([readValue("text-font"), readValue("text-size"), readValue("text-weight")]).toEqual([
      "Inter",
      "32",
      "400",
    ]);

    // The epic's step 2 holds no text: the next text is the page's too.
    typeAndEnter("text-size", "20");
    clickElement("guide-next");
    placeShape("text", { x: 250, y: 140 });

    expect(readValue("text-size")).toBe("32");
  });
});
