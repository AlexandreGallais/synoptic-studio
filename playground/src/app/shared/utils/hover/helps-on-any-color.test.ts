import { contourPiecesToPathData, shapePickArea } from "editor";
// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  clickAt,
  clickElement,
  dispatchDrawingPointer,
  openPage,
  readShapeModel,
  runResizeObservers,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeTextContent,
} from "../../../../../testing";
import { readExportedSvg } from "../document";
import { readDrawnTexts } from "../kept-shapes";
import { readPickAreas } from "../picking";
import { PAGE_VIEWS } from "../view";

import type { PointerKeys } from "../../../../../testing";
import type { Point, Shape } from "editor";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** Shift held. */
const SHIFT: PointerKeys = { altKey: false, shiftKey: true };

/** What a path of the layer above the drawing is drawn with. */
type OverlayPath = {
  /** Its class. */
  readonly name: string;
  /** Its path data. */
  readonly d: string | undefined;
  /** Its `stroke-width` attribute; undefined for none. */
  readonly width: string | undefined;
  /** Its `stroke-dasharray` attribute; undefined for none. */
  readonly dashes: string | undefined;
};

/** A help's width and dash length, as written in its attributes. */
type LineSize = {
  /** Its `stroke-width`, in user units. */
  readonly width: string;
  /** The length of a dash and of a gap, in user units. */
  readonly dash: string;
};

/**
 * Presses the drawing, moves the pointer, releases it, then the `click` the browser fires.
 *
 * @param from - the point pressed
 * @param to - the point released
 * @param keys - the keys held
 */
function dragFrom(from: Point, to: Point, keys: PointerKeys): void {
  dispatchDrawingPointer("pointerdown", from, keys);
  dispatchDrawingPointer("pointermove", to, keys);
  dispatchDrawingPointer("pointerup", to, keys);
  dispatchDrawingPointer("click", to, keys);
}

/**
 * Places a text `Pump` with the Text tool at a point of the opening view, in a color, then clicks
 * the empty drawing so that none is selected.
 *
 * @param point - the point clicked, in the drawing's units
 * @param color - its color
 */
function placeText(point: Point, color: string): void {
  // The canvas at (100, 50) in the window, the opening view's (−40, −40) at its top-left.
  clickElement("tool-text");
  clickAt("#canvas", { clientX: point.x + 140, clientY: point.y + 90 });
  typeTextContent("Pump");
  typeAndEnter("text-color", color);
  dragFrom({ x: 600, y: 500 }, { x: 600, y: 500 }, NO_KEY);
}

/**
 * Draws A and B, rectangles 100 × 100 at (0, 0) and (150, 0) in the page's color, A the rectangle
 * the page opens with typed 100 × 100, its corners sharp, B dragged with the Rectangle tool; then
 * the texts `Pump` at (0, 150), white, and at (150, 150), black.
 *
 * @returns the models of A and B
 */
function drawShapesAndTexts(): { readonly a: Shape; readonly b: Shape } {
  typeAndEnter("radius", "0");
  typeAndEnter("width", "100");
  typeAndEnter("height", "100");

  const a = readShapeModel();

  clickElement("tool-rectangle");
  dragFrom({ x: 150, y: 0 }, { x: 250, y: 100 }, NO_KEY);

  const b = readShapeModel();

  placeText({ x: 0, y: 150 }, "#ffffff");
  placeText({ x: 150, y: 150 }, "#000000");

  return { a, b };
}

/**
 * Reads the paths of the layer above the drawing, in the order drawn.
 *
 * @returns each one's class, path data, width and dashes
 */
function readOverlayPaths(): readonly OverlayPath[] {
  return [...document.querySelectorAll("#overlay svg path")].map((path) => ({
    d: path.getAttribute("d") ?? undefined,
    dashes: path.getAttribute("stroke-dasharray") ?? undefined,
    name: path.getAttribute("class") ?? "",
    width: path.getAttribute("stroke-width") ?? undefined,
  }));
}

/**
 * Reads the paths of a help of the layer above the drawing, its white lines and its dashes.
 *
 * @param name - the dashes' class, `<name>-light` the white lines'
 * @returns each one's class, path data, width and dashes
 */
function readHelps(name: string): readonly OverlayPath[] {
  return readOverlayPaths().filter((path) => [name, `${name}-light`].includes(path.name));
}

/**
 * The white line and the black dashes of a help, as expected.
 *
 * @param name - their class's root
 * @param d - the path data they follow
 * @param size - their width and dash length, in user units
 * @returns the white line, then the dashes
 */
function dashedLine(name: string, d: string, size: LineSize): readonly OverlayPath[] {
  return [
    { d, dashes: undefined, name: `${name}-light`, width: size.width },
    { d, dashes: `${size.dash} ${size.dash}`, name, width: size.width },
  ];
}

/**
 * The path data of a shape's contour as drawn.
 *
 * @param shape - the shape outlined
 * @returns its path data
 */
function contourOf(shape: Shape): string {
  return contourPiecesToPathData(shapePickArea(shape).pieces);
}

/**
 * The path data of the frame of a drawn shape, as the pointer finds it.
 *
 * @param index - its index in the drawing
 * @returns its path data
 */
function frameOf(index: number): string {
  return contourPiecesToPathData(readPickAreas(document)[index]?.pieces ?? []);
}

/**
 * Reads a property the page's style gives a path of the layer above the drawing.
 *
 * @param name - its class
 * @param property - the CSS property read
 * @returns its computed value
 */
function readStyle(name: string, property: string): string {
  return getComputedStyle(selectPageElement(`#overlay svg path.${name}`)).getPropertyValue(
    property,
  );
}

describe("the helps as one dashed line of white and black", () => {
  beforeEach(() => {
    // The page as it opens, a rectangle 120 × 80 at (0, 0), radius 10.
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F44.AC3] outlines A under the pointer by a white line 2 pixels wide, black dashes 4 4 over it", () => {
    const { a } = drawShapesAndTexts();

    dispatchDrawingPointer("pointermove", { x: 50, y: 50 }, NO_KEY);

    // At 100 %: 2 / 1 = 2 units wide, dashes 4 / 1 = 4; A's contour from its top-left corner
    // clockwise, the dashes from it, no offset: black from 0 to 4 along the top side.
    expect(readHelps("hover")).toEqual(
      dashedLine("hover", contourOf(a), { dash: "4", width: "2" }),
    );

    expect(contourOf(a)).toBe("M0 0 L100 0 L100 100 L0 100 Z");

    expect(
      selectPageElement("#overlay svg path.hover").getAttribute("stroke-dashoffset"),
    ).toBeNull();

    expect(readStyle("hover-light", "stroke")).toBe("#fff");
    expect(readStyle("hover", "stroke")).toBe("#000");
    expect(readStyle("hover-light", "fill")).toBe("none");
    expect(readStyle("hover", "fill")).toBe("none");
  });

  it.each([
    { color: "white", index: 2, point: { x: 5, y: 145 } },
    { color: "black", index: 3, point: { x: 155, y: 145 } },
  ])("[F44.AC3] outlines the $color text under the pointer on its frame", ({ index, point }) => {
    drawShapesAndTexts();
    dispatchDrawingPointer("pointermove", point, NO_KEY);

    expect(readHelps("hover")).toEqual(
      dashedLine("hover", frameOf(index), { dash: "4", width: "2" }),
    );
  });

  it("[F44.AC3] outlines A selected by the same dashed line", () => {
    const { a } = drawShapesAndTexts();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 600, y: 500 }, NO_KEY);

    expect(readHelps("selection")).toEqual(
      dashedLine("selection", contourOf(a), { dash: "4", width: "2" }),
    );

    expect(readStyle("selection-light", "stroke")).toBe("#fff");
    expect(readStyle("selection", "stroke")).toBe("#000");
  });

  it.each([
    // 2 / 2 = 1 unit wide, dashes 4 / 2 = 2; 2 / 0.25 = 8, 4 / 0.25 = 16.
    { dash: "2", scale: 2, width: "1" },
    { dash: "16", scale: 0.25, width: "8" },
  ])("[F44.AC3] draws the outline $width units wide at the scale $scale, dashes $dash", (size) => {
    const { a } = drawShapesAndTexts();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    PAGE_VIEWS.set(document, { origin: { x: 0, y: 0 }, scale: size.scale });
    runResizeObservers("#canvas");

    expect(readHelps("selection")).toEqual(dashedLine("selection", contourOf(a), size));
  });

  it("[F44.AC3] dashes the box of A and B by a line 1 pixel wide, half a pixel inside it", () => {
    const { a, b } = drawShapesAndTexts();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    dragFrom({ x: 200, y: 50 }, { x: 200, y: 50 }, SHIFT);
    dispatchDrawingPointer("pointermove", { x: 600, y: 500 }, NO_KEY);

    // The box from (0, 0) to (250, 100), its line on (0.5, 0.5) to (249.5, 99.5), clockwise from
    // its top-left corner; drawn after the outlines of A and B, each kept.
    expect(readOverlayPaths()).toEqual([
      ...dashedLine("selection", contourOf(a), { dash: "4", width: "2" }),
      ...dashedLine("selection", contourOf(b), { dash: "4", width: "2" }),
      ...dashedLine("selection-box", "M0.5 0.5H249.5V99.5H0.5Z", { dash: "4", width: "1" }),
    ]);

    expect(readStyle("selection-box-light", "stroke")).toBe("#fff");
    expect(readStyle("selection-box", "stroke")).toBe("#000");
  });

  it("[F44.AC3] draws the selection rectangle's line 1 pixel wide, half a pixel inside, its tint unchanged", () => {
    drawShapesAndTexts();
    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);

    // From (−10, −10) to (200, 110): its area the box itself, its line from (−9.5, −9.5) to
    // (199.5, 109.5), drawn last.
    expect(readOverlayPaths().slice(-3)).toEqual([
      { d: "M-10 -10H200V110H-10Z", dashes: undefined, name: "selection-area", width: undefined },
      ...dashedLine("selection-rectangle", "M-9.5 -9.5H199.5V109.5H-9.5Z", {
        dash: "4",
        width: "1",
      }),
    ]);

    expect(readStyle("selection-area", "fill")).toBe(
      "color-mix(in srgb, #4a7bd0 12%, transparent)",
    );

    expect(readStyle("selection-rectangle-light", "stroke")).toBe("#fff");
    expect(readStyle("selection-rectangle", "stroke")).toBe("#000");
  });

  it("[F44.AC3] lays the line of a rectangle 0 wide on its middle, half a pixel inside its ends", () => {
    drawShapesAndTexts();
    // From (5, 300) to (5, 350): its sides moved inward by at most half its width, 0.
    dispatchDrawingPointer("pointerdown", { x: 5, y: 300 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 5, y: 350 }, NO_KEY);

    expect(readHelps("selection-rectangle")).toEqual(
      dashedLine("selection-rectangle", "M5 300.5H5V349.5H5Z", { dash: "4", width: "1" }),
    );
  });

  it("[F44.AC3] strokes no shape nor text of the drawing, and exports none of the helps", () => {
    drawShapesAndTexts();
    dispatchDrawingPointer("pointermove", { x: 600, y: 500 }, NO_KEY);

    const exported = readExportedSvg(document, readDrawnTexts(document));
    const drawing = selectPageElement("#canvas").getHTML();

    dragFrom({ x: 50, y: 50 }, { x: 50, y: 50 }, NO_KEY);
    dragFrom({ x: 200, y: 50 }, { x: 200, y: 50 }, SHIFT);
    dispatchDrawingPointer("pointermove", { x: 5, y: 145 }, NO_KEY);
    dispatchDrawingPointer("pointerdown", { x: -10, y: -10 }, NO_KEY);
    dispatchDrawingPointer("pointermove", { x: 200, y: 110 }, NO_KEY);

    expect(readHelps("selection-rectangle")).toHaveLength(2);
    expect(selectPageElement("#canvas").getHTML()).toBe(drawing);
    expect(drawing).not.toMatch(/stroke/u);
    expect(readExportedSvg(document, readDrawnTexts(document))).toBe(exported);
    expect(exported).not.toMatch(/stroke/u);
  });
});

describe("R outlined by its contour's path", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    // R, 99 999 × 100 000 of radius 100 000, its radius 0 first so that no large rounded shape is
    // drawn on the way.
    typeAndEnter("angle", "0");
    typeAndEnter("radius", "0");
    typeAndEnter("width", "99999");
    typeAndEnter("height", "100000");
    typeAndEnter("shape-x", "0");
    typeAndEnter("shape-y", "0");
    typeAndEnter("radius", "100000");
  });

  it("[F44.AC3] outlines R selected at 100 % by the two strokes of its contour, no band", () => {
    // Its contour 18 647 units from the area shown at the opening view: its band was empty there,
    // its path is whole.
    const d = contourOf(readShapeModel());

    expect(readHelps("selection")).toEqual(dashedLine("selection", d, { dash: "4", width: "2" }));
  });

  it("[F44.AC3] keeps R's outline its contour at 1 %, 200 units wide, dashes 400", () => {
    // Fitted, at 1 %: 2 / 0.01 = 200 units, 4 / 0.01 = 400; once cut to 393 chords a corner.
    PAGE_VIEWS.set(document, { origin: { x: -10_000.5, y: 10_000 }, scale: 0.01 });
    runResizeObservers("#canvas");

    const d = contourOf(readShapeModel());

    expect(readHelps("selection")).toEqual(
      dashedLine("selection", d, { dash: "400", width: "200" }),
    );
  });

  it("[F44.AC3] keeps R's outline its contour after a pan of 700 pixels, the viewBox alone changed", () => {
    const d = contourOf(readShapeModel());

    PAGE_VIEWS.set(document, { origin: { x: 660, y: -40 }, scale: 1 });
    runResizeObservers("#canvas");

    expect(readHelps("selection")).toEqual(dashedLine("selection", d, { dash: "4", width: "2" }));
    expect(selectPageElement("#overlay svg").getAttribute("viewBox")).toBe("660 -40 1200 800");
  });

  it("[F44.AC3] outlines R under the pointer by its contour, the pointer far from it", () => {
    const d = contourOf(readShapeModel());

    PAGE_VIEWS.set(document, { origin: { x: 49_000, y: 49_000 }, scale: 1 });
    runResizeObservers("#canvas");

    // Over the offset (100, 100), R's point (49 100, 49 100), 48 000 units and more from its
    // contour, whose band was empty there.
    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointermove", { bubbles: true, clientX: 200, clientY: 150, pointerId: 1 }),
    );

    expect(readHelps("hover")).toEqual(dashedLine("hover", d, { dash: "4", width: "2" }));
  });

  it(
    "[F44.AC3] draws the layer above 60 times within a second, R selected",
    // R's band took 156 to 200 ms a redraw (note 0031, §5). alone: 73 ms, the median of 5 runs of
    // its file; 960 ms, 60 frames of 16 ms, is 13.2 times it.
    { tags: ["timing"], timeout: 960 },
    () => {
      // 100 % and 200 % in turn, so that each redraw draws its helps at a new scale.
      const scales = Array.from({ length: 30 }, () => [1, 2]).flat();

      for (const scale of scales) {
        PAGE_VIEWS.set(document, { origin: { x: -40, y: -40 }, scale });
        runResizeObservers("#canvas");
      }

      expect(readHelps("selection")).toHaveLength(2);
    },
  );
});
