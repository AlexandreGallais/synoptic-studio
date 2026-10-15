// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  commitField,
  openServedPage,
  readText,
  readValue,
  selectPageElement,
  typeValue,
} from "../user-actions";

/**
 * Types a color or an opacity and leaves the field, as the designer pressing Tab.
 *
 * @param id - the field
 * @param value - what is typed or pasted
 */
function typeAndLeave(id: string, value: string): void {
  typeValue(id, value);
  commitField(id);
}

/**
 * Reads the fill and the fill opacity of the text, and of its background when it has one.
 *
 * @returns the text's fill and opacity, then the background's
 */
function readFills(): readonly (string | null | undefined)[] {
  const text = selectPageElement("#canvas text");
  const band = text.parentElement?.querySelector("path");

  return [
    text.getAttribute("fill"),
    text.getAttribute("fill-opacity"),
    band?.getAttribute("fill"),
    band?.getAttribute("fill-opacity"),
  ];
}

describe("Color fields (US-035)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
  });

  it("[F16.AC1] shows F80 typed as #ff8800 and draws the text with it", () => {
    typeAndLeave("text-color", "F80");

    expect(readValue("text-color")).toBe("#ff8800");
    expect(readFills().slice(0, 2)).toEqual(["#ff8800", "1"]);
    expect(selectPageElement("#text-color-swatch").style.backgroundColor).toBe("#ff8800");
  });

  it("[F16.AC1] takes the alpha of #0000ffcc pasted as the background's opacity, 80", () => {
    clickElement("text-background");
    typeAndLeave("text-background-color", "#0000ffcc");

    // cc = 204, 100 · 204 / 255 = 80.
    expect([readValue("text-background-color"), readValue("text-background-opacity")]).toEqual([
      "#0000ff",
      "80",
    ]);

    expect(readFills().slice(2)).toEqual(["#0000ff", "0.8"]);
  });

  it.each(["red", "#ff800", "rgb(255, 0, 0)"])(
    "[F16.AC1] refuses %s when the field is left: the previous color back, the page says why",
    (typed) => {
      typeAndLeave("text-color", "F80");
      typeAndLeave("text-color", typed);

      expect(readValue("text-color")).toBe("#ff8800");
      expect(readText("#text-color-message")).toContain(`“${typed}” is no hexadecimal color`);
      expect(readFills().slice(0, 2)).toEqual(["#ff8800", "1"]);
    },
  );

  it("refuses a color typed before any was kept: the color the page opened with comes back", () => {
    typeAndLeave("text-background-color", "yellow");

    expect(readValue("text-background-color")).toBe("#ffff00");
    expect(readText("#text-background-color-message")).toContain("#ffff00 kept");
  });

  it("keeps the color the field had when it got the focus, and clears the message once valid", () => {
    selectPageElement("#text-color").dispatchEvent(new FocusEvent("focus"));
    typeAndLeave("text-color", "nope");
    typeAndLeave("text-color", "#ABCDEF");

    // The page opens with #1e293b; then #ABCDEF is kept in lower case.
    expect(readValue("text-color")).toBe("#abcdef");
    expect(readText("#text-color-message")).toBe("");
  });

  it("paints the swatch with the kept color while what is typed is no color", () => {
    typeValue("text-color", "#12");

    expect(selectPageElement("#text-color-swatch").style.backgroundColor).toBe("#1e293b");
  });
});
