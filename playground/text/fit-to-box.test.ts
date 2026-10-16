// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  openServedPage,
  readText,
  readValue,
  selectPageElement,
  typeValue,
} from "../user-actions";

// Inter 4.1 Regular 16, kerning included: "Pump P-101" 87.953125 wide, a line 19.359375 tall
// (turned-text-in-place.test.ts); two lines 38.71875 tall. Its box: ⌈87.953125⌉ × ⌈19.359375⌉
// = 88 × 20, then 88 × 39 (DERIV-text-box, step 11).

/** The model the pipeline column shows, the fields this story reads. */
type ShownText = {
  readonly boxHeight: number;
  readonly boxWidth: number;
  readonly isInBox: boolean;
  readonly x: number;
  readonly y: number;
};

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
 * Reads the text model the pipeline column shows.
 *
 * @returns its box, its kind and its position
 */
function readModel(): ShownText {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by JSON.stringify of a Text
  return JSON.parse(readText("#model")) as ShownText;
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

describe("Text tool: a text put in a box keeps its fitted size (US-055)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    typeValue("text-size", "16");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("text-vertical-alignment", "top");
    typeContent("Pump P-101");
    typeValue("shape-x", "47");
    typeValue("shape-y", "61");
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
    typeValue("text-box-width", "50");

    // One line 19.359375 tall fits in 20, not two: the line kept is cut with an ellipsis.
    expect(readLines().map(([content]) => content)).toEqual(["Pum…"]);

    typeValue("text-box-height", "40");

    expect(readLines().map(([content]) => content)).toEqual(["Pump", "P-101"]);
  });

  it("[F17.AC13] measures again at each switch: 88 × 39 once a second line is typed", () => {
    chooseOption("text-fit", "box");
    typeValue("text-box-width", "50");
    chooseOption("text-fit", "content");
    typeContent("Pump P-101\nTank");
    chooseOption("text-fit", "box");

    expect(readBoxFields()).toEqual(["88", "39"]);
  });

  it("[F17.AC13] leaves the box typed when a value other than the kind changes", () => {
    chooseOption("text-fit", "box");
    typeValue("text-box-width", "150");
    typeValue("text-letter-spacing", "10");

    expect(readBoxFields()).toEqual(["150", "20"]);
  });

  it("[F17.AC13] keeps a turned text's corner, X and Y unchanged", () => {
    typeValue("angle", "23");

    // Its turned top-left corner P = (54.27808, 44.58648) (US-054). In a box 88 × 20,
    // k' = (44, 10), R(k') = (44 cos 23° − 10 sin 23°, 44 sin 23° + 10 cos 23°)
    // = (36.59490, 26.39722); b' = P − k' + R(k') = (46.87298, 60.98369).
    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["54.27808", "44.58648"]);
    chooseOption("text-fit", "box");

    const model = readModel();

    expect([readValue("shape-x"), readValue("shape-y")]).toEqual(["54.27808", "44.58648"]);
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
