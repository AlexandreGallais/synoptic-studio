// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openServedPage,
  readDisplay,
  readModel,
  readText,
  readValue,
  selectPageElement,
  typeAndEnter,
  typeInField,
} from "../user-actions";

/** The fields read only in a box, hidden while the text fits its content. */
const BOX_ONLY = [
  "text-box-width",
  "text-box-height",
  "text-maximum-lines",
  "text-ellipsis",
  "text-vertical-alignment",
];

describe("Text tool: the box's fields (US-033)", () => {
  beforeEach(() => {
    openServedPage("");
    chooseShapeKind("text");
    typeAndEnter("text-size", "16");
    // happy-dom loses the option marked `selected` once the page is moved into its document: the
    // choices the page opens with are chosen again.
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("text-vertical-alignment", "top");
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
    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "60");
    typeAndEnter("text-maximum-lines", "2");
    clickElement("text-ellipsis");
    chooseOption("text-vertical-alignment", "bottom");
    typeAndEnter("text-line-height", "24");
    clickElement("text-trim");

    expect(readModel()).toMatchObject({
      boxHeight: 60,
      boxWidth: 50,
      hasEllipsis: false,
      isInBox: true,
      isTrimmed: true,
      lineHeight: 24,
      maximumLines: 2,
      verticalAlignment: "bottom",
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

  it("[F44.AC4] takes -500 in the box's Height at 0, the text drawn (AUD-046)", () => {
    chooseOption("text-fit", "box");
    typeInField("text-box-height", "-500");

    expect([readValue("text-box-height"), readText("#status")]).toEqual(["0", ""]);
    expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
  });

  it.each([
    { id: "text-box-width", value: "" },
    { id: "text-box-height", value: "abc" },
  ])(
    "refuses $id = '$value', the field showing its value again, the text drawn (US-085)",
    ({ id, value }) => {
      chooseOption("text-fit", "box");

      const model = readText("#model");
      const shown = readValue(id);

      typeAndEnter(id, value);

      expect([readValue(id), readText("#status")]).toEqual([shown, ""]);
      expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
      expect(readText("#model")).toBe(model);
    },
  );

  it.each([
    // 1.5 rounded to 2, a tie up; 100 001 taken at 100 000 (AUD-046).
    { expected: "2", id: "text-maximum-lines", value: "1.5" },
    { expected: "100000", id: "text-line-height", value: "100001" },
  ])("[F44.AC4] takes $id = '$value' as $expected, the text drawn", ({ expected, id, value }) => {
    chooseOption("text-fit", "box");
    typeInField(id, value);

    expect([readValue(id), readText("#status")]).toEqual([expected, ""]);
  });

  it.each([
    { id: "text-maximum-lines", value: "abc" },
    { id: "text-line-height", value: "" },
  ])(
    "refuses $id = '$value' emptied by a script, draws nothing and keeps the last valid text",
    ({ id, value }) => {
      chooseOption("text-fit", "box");

      const shown = readText("#model");

      typeAndEnter(id, value);

      expect(readText("#status")).toContain(
        "the box's width and height, its maximum lines and the line height integers from 0 to 100 000",
      );

      expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
      expect(selectPageElement(`#${id}`).getAttribute("aria-invalid")).toBe("true");
      // No NaN nor refused value reaches the model shown: JSON writes NaN as null.
      expect(readText("#model")).not.toContain("null");
      expect(readText("#model")).toBe(shown);
    },
  );

  it("keeps the box's fields shown while one of them is refused, Fit content chosen", () => {
    chooseOption("text-fit", "box");
    // Emptied by a script, which the field never read: the text refused.
    typeAndEnter("text-maximum-lines", "");
    chooseOption("text-fit", "content");

    expect(readDisplay("text-box-fields") === "none").toBe(false);
    expect(selectPageElement("#text-box-width").getAttribute("aria-invalid")).toBe("true");

    typeAndEnter("text-maximum-lines", "0");
    expect(readDisplay("text-box-fields")).toBe("none");
  });
});
