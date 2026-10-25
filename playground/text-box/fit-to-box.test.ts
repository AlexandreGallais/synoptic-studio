// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openPage,
  readModel,
  readValue,
  selectPageElement,
  typeAndEnter,
  typeValue,
} from "../user-actions";

// Inter 4.1 Regular 16, kerning included: "Pump P-101" 87.953125 wide, a line 19.359375 tall
// (turned-text-in-place.test.ts); two lines 38.71875 tall. Its box: ⌈87.953125⌉ × ⌈19.359375⌉
// = 88 × 20, then 88 × 39 (DERIV-text-box, step 11).

/**
 * Types a content in the Text tool's text area, as the designer would.
 *
 * @param content - the lines, a line feed between two
 */
function typeContent(content: string): void {
  const area = selectPageElement("#text-content");

  if (area instanceof HTMLTextAreaElement) {
    area.value = content;
  }

  area.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Reads the box's width and height fields.
 *
 * @returns their values, the width first
 */
function readBoxFields(): readonly string[] {
  return [readValue("text-box-width"), readValue("text-box-height")];
}

/**
 * Reads where each line of the text is drawn.
 *
 * @returns each `tspan`'s content, x and y
 */
function readLines(): readonly (string | null)[][] {
  return [...document.querySelectorAll("#canvas text tspan")].map((line) => [
    line.textContent,
    line.getAttribute("x"),
    line.getAttribute("y"),
  ]);
}

/**
 * Chooses a kind of text as a browser does: `input`, then `change`.
 *
 * @param kind - `content` or `box`
 */
function chooseKind(kind: string): void {
  const select = selectPageElement("#text-fit");

  if (select instanceof HTMLSelectElement) {
    select.value = kind;
  }

  select.dispatchEvent(new Event("input", { bubbles: true }));
  select.dispatchEvent(new Event("change", { bubbles: true }));
}

describe("Text tool: a text put in a box keeps its fitted size", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    typeAndEnter("text-size", "16");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("text-vertical-alignment", "top");
    typeContent("Pump P-101");
    typeAndEnter("shape-x", "47");
    typeAndEnter("shape-y", "61");
  });

  it("[F17.AC13] writes the block rounded up, 88 × 20, in the box when Box is chosen", () => {
    const lines = readLines();

    expect(readBoxFields()).toEqual(["200", "100"]);
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["88", "20"]);
    expect(readModel()).toMatchObject({ boxHeight: 20, boxWidth: 88, isInBox: true, x: 47, y: 61 });
    // The same line at the same place: left-aligned, the block at the top.
    expect(readLines()).toEqual(lines);
  });

  it("[F17.AC13] does so when the browser sends input before change", () => {
    chooseKind("box");

    expect(readBoxFields()).toEqual(["88", "20"]);
    expect(readModel()).toMatchObject({ boxHeight: 20, boxWidth: 88, isInBox: true });
  });

  it("[F17.AC13] then wraps and cuts as any box when it is resized", () => {
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "50");

    // One line 19.359375 tall fits in 20, not two: the line kept is cut with an ellipsis.
    expect(readLines().map(([content]) => content)).toEqual(["Pum…"]);

    typeAndEnter("text-box-height", "40");

    expect(readLines().map(([content]) => content)).toEqual(["Pump", "P-101"]);
  });

  it("[F17.AC13] measures again at each switch while the box is not typed: 88 × 39 with a second line", () => {
    chooseOption("text-fit", "box");
    chooseOption("text-fit", "content");
    typeContent("Pump P-101\nTank");
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["88", "39"]);
  });

  it("[F17.AC13] leaves the box typed when a value other than the kind changes", () => {
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "150");
    typeAndEnter("text-letter-spacing", "10");

    expect(readBoxFields()).toEqual(["150", "20"]);
  });

  it("[F17.AC13] keeps a turned text's corner, X and Y showing its straight box", () => {
    typeAndEnter("angle", "23");

    // Its turned top-left corner P = (54.27808, 44.58648) (ADR-0057). In a box 88 × 20,
    // k' = (44, 10), R(k') = (44 cos 23° − 10 sin 23°, 44 sin 23° + 10 cos 23°)
    // = (36.59490, 26.39722); b' = P − k' + R(k') = (46.87298, 60.98369). X and Y show its
    // straight box, P.y and P.x − h sin 23°: 19.359375 tall, 54.27808 − 7.56432 =
    // 46.71377; 20 tall, 54.27808 − 7.81462 = 46.46346.
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["46.71377", "44.58648"]);
    chooseOption("text-fit", "box");

    const model = readModel();

    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["46.46346", "44.58648"]);
    expect(readBoxFields()).toEqual(["88", "20"]);
    expect(model.x).toBeCloseTo(46.87298, 5);
    expect(model.y).toBeCloseTo(60.98369, 5);
  });

  it("[F17.AC13] gives an empty text the box 0 × 0", () => {
    typeContent("");
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["0", "0"]);
  });
});

describe("Text tool: the box typed is kept when the text comes back to Box", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    typeAndEnter("text-size", "16");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("text-vertical-alignment", "top");
    typeContent("Pump P-101");
    chooseOption("text-fit", "box");
  });

  it("[F17.AC15] keeps a height typed 15 through Fit content and back: 88 × 15", () => {
    typeAndEnter("text-box-height", "15");
    chooseOption("text-fit", "content");
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["88", "15"]);
    expect(readModel()).toMatchObject({ boxHeight: 15, boxWidth: 88, isInBox: true });
  });

  it("[F17.AC15] keeps a width typed 50 though a second line was typed meanwhile: 50 × 20", () => {
    typeAndEnter("text-box-width", "50");
    chooseOption("text-fit", "content");
    typeContent("Pump P-101\nTank");
    chooseOption("text-fit", "box");

    // Not 88 × 39, the new block rounded up: the box typed comes back.
    expect(readBoxFields()).toEqual(["50", "20"]);
  });

  it("[F17.AC15] keeps the maximum number of lines with the box typed", () => {
    typeAndEnter("text-box-height", "40");
    typeAndEnter("text-maximum-lines", "1");
    chooseOption("text-fit", "content");
    chooseOption("text-fit", "box");

    expect([...readBoxFields(), readValue("text-maximum-lines")]).toEqual(["88", "40", "1"]);
  });

  it("[F17.AC15] Reset puts back the fitted size, the box then measured again as never typed", () => {
    typeAndEnter("text-box-width", "100");
    typeAndEnter("text-box-height", "200");
    clickElement("text-box-reset");

    expect(readBoxFields()).toEqual(["88", "20"]);
    expect(readModel()).toMatchObject({ boxHeight: 20, boxWidth: 88, isInBox: true });

    chooseOption("text-fit", "content");
    typeContent("Pump P-101\nTank");
    chooseOption("text-fit", "box");

    // Measured again, ⌈38.71875⌉ = 39 tall: after Reset the box no longer counts as typed.
    expect(readBoxFields()).toEqual(["88", "39"]);
  });

  it("[F17.AC15] Reset leaves the box as it is when the text cannot be measured", () => {
    typeAndEnter("text-box-width", "100");
    typeAndEnter("text-size", "");
    clickElement("text-box-reset");

    expect(readBoxFields()).toEqual(["100", "20"]);
  });

  it("[F17.AC15] keeps the box typed through another tool and back", () => {
    typeAndEnter("text-box-height", "15");
    chooseShapeKind("rectangle");
    chooseShapeKind("text");
    chooseOption("text-fit", "content");
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["88", "15"]);
  });

  it("[F17.AC15] does not count the maximum number of lines alone as a box typed", () => {
    typeAndEnter("text-maximum-lines", "1");
    chooseOption("text-fit", "content");
    typeContent("Pump P-101\nTank");
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["88", "39"]);
  });

  it("[F17.AC15] measures again a dimension whose field was emptied, the other kept as typed", () => {
    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "15");
    // Emptied and not applied: the field holds no number when the fit is chosen.
    typeValue("text-box-width", "");
    chooseOption("text-fit", "content");
    chooseOption("text-fit", "box");

    // The width emptied counts as not typed: ⌈87.953125⌉ = 88, not 0 nor empty.
    expect(readBoxFields()).toEqual(["88", "15"]);
  });

  it("[F17.AC15] keeps the box typed when an empty width is applied, refused", () => {
    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "15");
    typeAndEnter("text-box-width", "");
    chooseOption("text-fit", "content");
    chooseOption("text-fit", "box");

    // Refused, the field shows 50 again: the box typed, 50 × 15, comes back.
    expect(readBoxFields()).toEqual(["50", "15"]);
  });
});
