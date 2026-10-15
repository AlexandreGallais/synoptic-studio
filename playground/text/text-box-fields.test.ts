// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  openServedPage,
  readDisplay,
  readText,
  selectPageElement,
  typeValue,
} from "../user-actions";

/** The fields read only in a box, hidden while the text fits its content. */
const BOX_ONLY = [
  "text-box-width",
  "text-box-height",
  "text-long-word",
  "text-maximum-lines",
  "text-ellipsis",
  "text-vertical-alignment",
  "text-wrap",
];

/**
 * Reads the text model the pipeline column shows.
 *
 * @returns the JSON written in `#model`, parsed
 */
function readModel(): Record<string, unknown> {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by JSON.stringify of a Text
  return JSON.parse(readText("#model")) as Record<string, unknown>;
}

describe("Text tool: the box's fields (US-033)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    typeValue("text-size", "16");
    // happy-dom loses the option marked `selected` once the page is moved into its document: the
    // choices the page opens with are chosen again.
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("text-long-word", "break");
    chooseOption("text-vertical-alignment", "top");
    chooseOption("text-wrap", "auto");
  });

  it("[F14.AC1] hides the fields read only in a box while the text fits its content", () => {
    expect(readDisplay("text-box-fields")).toBe("none");

    // A field shows no "none" of its own: it is hidden with the group that holds it.
    for (const id of BOX_ONLY) {
      expect([id, selectPageElement(`#${id}`).closest("#text-box-fields") !== null]).toEqual([
        id,
        true,
      ]);
    }

    // Line height and trim apply to both kinds of text.
    for (const id of ["text-line-height", "text-trim"]) {
      expect([id, selectPageElement(`#${id}`).closest("[hidden]") === null]).toEqual([id, true]);
    }

    expect(readModel()).toMatchObject({ isInBox: false });
  });

  it("[F14.AC1] shows them when Box is chosen, and hides them back with Fit content", () => {
    chooseOption("text-fit", "box");

    expect(readDisplay("text-box-fields") === "none").toBe(false);
    chooseOption("text-fit", "content");
    expect(readDisplay("text-box-fields")).toBe("none");
  });

  it("[F14.AC1] reads every option of the box into the model", () => {
    chooseOption("text-fit", "box");
    typeValue("text-box-width", "50");
    typeValue("text-box-height", "60");
    chooseOption("text-long-word", "move");
    typeValue("text-maximum-lines", "2");
    clickElement("text-ellipsis");
    chooseOption("text-vertical-alignment", "bottom");
    chooseOption("text-wrap", "balance");
    typeValue("text-line-height", "24");
    clickElement("text-trim");

    expect(readModel()).toMatchObject({
      boxHeight: 60,
      boxWidth: 50,
      hasEllipsis: false,
      isInBox: true,
      isTrimmed: true,
      lineHeight: 24,
      longWord: "move",
      maximumLines: 2,
      verticalAlignment: "bottom",
      wrapStyle: "balance",
    });
  });

  it("[F14.AC6] opens with the defaults that draw a text of F05: automatic line height, ellipsis on", () => {
    expect(readModel()).toMatchObject({
      hasEllipsis: true,
      isInBox: false,
      isTrimmed: false,
      lineHeight: 0,
      maximumLines: 0,
    });
  });

  it.each([
    { id: "text-box-width", value: "" },
    { id: "text-box-height", value: "-1" },
    { id: "text-maximum-lines", value: "1.5" },
    { id: "text-line-height", value: "100001" },
  ])("refuses $id = '$value', draws nothing and keeps the last valid text", ({ id, value }) => {
    chooseOption("text-fit", "box");
    typeValue(id, value);

    expect(readText("#status")).toContain("integers from 0 to 100 000");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(selectPageElement(`#${id}`).getAttribute("aria-invalid")).toBe("true");
    // No NaN nor refused value reaches the model shown: JSON writes NaN as null.
    expect(readText("#model")).not.toContain("null");
    expect(readModel()).toMatchObject({ isInBox: true, maximumLines: 0 });
  });
});
